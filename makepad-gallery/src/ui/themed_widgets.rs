use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""

        container := ShadSurfaceMuted{
            width: Fill
            height: Fit
            padding: Inset{top: 12, right: 12, bottom: 12, left: 12}
            draw_bg +: {
                color: (shad_theme.color_muted)
                border_radius: (shad_theme.radius)
                border_size: 0.0
            }

            code_scroll := ScrollXView{
                width: Fill
                height: Fit
                flow: Right

                code_text := Label{
                    width: Fit
                    height: Fit
                    padding: 0.0
                    draw_text +: {
                        color: (shad_theme.color_primary_foreground)
                        text_style: theme.font_code{
                            font_size: 11.0
                            line_spacing: 1.35
                        }
                    }
                    text: ""
                }
            }
        }
    }

    mod.widgets.GalleryActionFlowStep = ShadFieldDescription{
        width: Fill
    }

    mod.widgets.GalleryActionFlow = ShadSurfaceMuted{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
        draw_bg +: {
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        title := ShadSectionHeader{
            text: "Action Flow"
        }

        body := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 8.0
        }
    }

    mod.widgets.GalleryPreviewStackNavigation = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 16.0

        preview_title := ShadSectionHeader{
            text: "Preview"
        }

        root_view := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0
        }

        code_page := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            code_title := ShadSectionHeader{
                text: "Code"
            }

            body := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0
            }
        }
    }

    mod.widgets.GalleryPreviewSection = View{
        width: Fill
        height: Fit
        flow: Down

        preview_panel := mod.widgets.ShadPanel{
            preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                width: Fill
                height: Fit

                root_view +: {
                    preview_content := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                    }

                    action_flow := View{
                        visible: false
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                    }
                }

                code_page +: {
                    body +: {
                        code_snippet := mod.widgets.GalleryCodeSnippet{}
                    }
                }
            }
        }
    }
}

#[derive(Script, Widget)]
pub struct GalleryCodeSnippet {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    code: ArcStringMut,
    #[live]
    code_resource: Option<ScriptHandleRef>,
    #[rust]
    last_code: String,
}

impl ScriptHook for GalleryCodeSnippet {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.refresh_text(cx);
        });
    }
}

impl GalleryCodeSnippet {
    fn set_code_view_text(&mut self, cx: &mut Cx, text: &str) {
        if self.last_code == text {
            return;
        }
        self.last_code.clear();
        self.last_code.push_str(text);
        self.view.label(cx, ids!(container.code_scroll.code_text)).set_text(cx, text);
    }

    fn resource_text(&mut self, cx: &mut Cx) -> Option<String> {
        let handle = self.code_resource.as_ref()?.as_handle();
        cx.load_script_resource(handle);
        if let Some(data) = cx.get_resource(handle) {
            return Some(
                String::from_utf8(data.as_ref().clone()).unwrap_or_else(|_| {
                    "// Failed to decode snippet resource as UTF-8.\n".to_string()
                }),
            );
        }

        let resources = cx.script_data.resources.resources.borrow();
        resources
            .iter()
            .find(|resource| resource.handle == handle && resource.is_error())
            .map(|_| "// Failed to load snippet resource.\n".to_string())
    }

    fn refresh_text(&mut self, cx: &mut Cx) {
        if let Some(text) = self.resource_text(cx) {
            self.set_code_view_text(cx, &text);
        } else {
            let inline_code = self.code.as_ref().to_string();
            self.set_code_view_text(cx, &inline_code);
        }
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.refresh_text(cx);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.refresh_text(cx.cx);
        self.view.draw_walk(cx, scope, walk)
    }
}
