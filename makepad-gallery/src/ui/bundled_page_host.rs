use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.GalleryBundledPageHost = #(GalleryBundledPageHost::register_widget(vm)) {
        width: Fill
        height: Fill
    }
}

#[derive(Script, WidgetRef, WidgetRegister)]
pub struct GalleryBundledPageHost {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[rust]
    area: Area,
    #[live]
    page_id: LiveId,
    #[rust]
    widget: Option<WidgetRef>,
}

impl GalleryBundledPageHost {
    fn sync_widget_with_vm(&mut self, vm: &mut ScriptVm, apply: &Apply, scope: &mut Scope) {
        if self.page_id == LiveId(0) {
            return;
        }

        if let Err(error) = crate::ui::ensure_gallery_page_registered_vm(vm, self.page_id) {
            error!(
                "GalleryBundledPageHost: failed to register page {:?}: {}",
                self.page_id, error
            );
            return;
        }

        let Some(template_value) = crate::ui::gallery_page_template_value_vm(vm, self.page_id)
        else {
            error!(
                "GalleryBundledPageHost: missing template for page {:?}",
                self.page_id
            );
            return;
        };

        if let Some(widget) = &mut self.widget {
            widget.script_apply(vm, apply, scope, template_value);
        } else {
            let widget = WidgetRef::script_from_value_scoped(vm, scope, template_value);
            self.widget = Some(widget);
        }

        if let Some(widget) = &self.widget {
            vm.cx_mut()
                .widget_tree_insert_child_deep(self.uid, self.page_id, widget.clone());
        }
    }

    fn ensure_widget(&mut self, cx: &mut Cx) {
        if self.widget.is_some() || self.page_id == LiveId(0) {
            return;
        }

        let page_id = self.page_id;
        if let Err(error) = crate::ui::ensure_gallery_page_registered(cx, page_id) {
            error!(
                "GalleryBundledPageHost: failed to register page {:?}: {}",
                self.page_id, error
            );
            return;
        }

        let widget = cx.with_vm(|vm| {
            let Some(template_value) = crate::ui::gallery_page_template_value_vm(vm, page_id)
            else {
                return Err(format!("Missing gallery template for {:?}", page_id));
            };
            Ok(WidgetRef::script_from_value(vm, template_value))
        });

        match widget {
            Ok(widget) => {
                cx.widget_tree_insert_child_deep(self.uid, self.page_id, widget.clone());
                self.widget = Some(widget);
            }
            Err(error) => {
                error!(
                    "GalleryBundledPageHost: failed to materialize page {:?}: {}",
                    self.page_id, error
                );
            }
        }
    }
}

impl ScriptHook for GalleryBundledPageHost {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        scope: &mut Scope,
        _value: ScriptValue,
    ) {
        self.sync_widget_with_vm(vm, apply, scope);
    }
}

impl WidgetNode for GalleryBundledPageHost {
    fn widget_uid(&self) -> WidgetUid {
        self.uid
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        if let Some(widget) = &self.widget {
            widget.walk(cx)
        } else {
            self.walk
        }
    }

    fn area(&self) -> Area {
        if let Some(widget) = &self.widget {
            widget.area()
        } else {
            self.area
        }
    }

    fn children(&self, visit: &mut dyn FnMut(LiveId, WidgetRef)) {
        if let Some(widget) = &self.widget {
            visit(self.page_id, widget.clone());
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        if let Some(widget) = &self.widget {
            widget.redraw(cx);
        }
    }
}

impl Widget for GalleryBundledPageHost {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ensure_widget(cx);
        if let Some(widget) = &self.widget {
            widget.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(widget) = &self.widget {
            widget.draw_walk(cx, scope, walk)
        } else {
            DrawStep::done()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bootstrap_gallery_vm(vm: &mut ScriptVm) {
        crate::makepad_widgets::script_mod(vm);
        makepad_components::theme::script_mod(vm);
        script_eval!(vm, {
            mod.widgets.shad_theme = mod.widgets.shad_themes.dark
        });
        makepad_components::script_mod_without_theme(vm);
        crate::makepad_code_editor::script_mod(vm);
        makepad_router::script_mod(vm);
        crate::ui::script_mod(vm);
    }

    #[test]
    fn bundled_host_materializes_page_once() {
        let mut cx = Cx::new(Box::new(|_, _| {}));

        cx.with_vm(|vm| {
            bootstrap_gallery_vm(vm);

            let host_value = script_eval!(vm, {
                mod.widgets.GalleryBundledPageHost {
                    page_id: @alert_page
                }
            });
            let mut host = WidgetRef::script_from_value(vm, host_value);

            let first_uid = host
                .borrow::<GalleryBundledPageHost>()
                .and_then(|inner| inner.widget.as_ref().map(|widget| widget.widget_uid()))
                .expect("bundled host should materialize its page child");

            host.script_apply(vm, &Apply::Reload, &mut Scope::empty(), host_value);

            let second_uid = host
                .borrow::<GalleryBundledPageHost>()
                .and_then(|inner| inner.widget.as_ref().map(|widget| widget.widget_uid()))
                .expect("bundled host should retain its page child");

            assert_eq!(first_uid, second_uid);
        });
    }

    #[test]
    fn bundled_host_ignores_unknown_pages() {
        let mut cx = Cx::new(Box::new(|_, _| {}));

        cx.with_vm(|vm| {
            bootstrap_gallery_vm(vm);

            let host_value = script_eval!(vm, {
                mod.widgets.GalleryBundledPageHost {
                    page_id: @missing_gallery_page
                }
            });
            let host = WidgetRef::script_from_value(vm, host_value);

            assert!(host
                .borrow::<GalleryBundledPageHost>()
                .is_some_and(|inner| inner.widget.is_none()));
        });
    }
}
