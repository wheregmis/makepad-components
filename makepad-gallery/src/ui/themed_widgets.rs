use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""

        container := SolidView{
            width: Fill
            height: Fit
            padding: Inset{top: 12, right: 12, bottom: 12, left: 12}
            draw_bg +: {
                color: (shad_theme.color_muted)
                border_radius: (shad_theme.radius)
            }

            code_view := CodeView{
                text: ""
                editor +: {
                    width: Fill
                    height: Fit
                }
            }
        }
    }

    mod.widgets.GalleryActionFlowStep = ShadFieldDescription{
        width: Fill
    }

    mod.widgets.GalleryActionFlow = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
        draw_bg +: {
            color: (shad_theme.color_muted)
            border_radius: (shad_theme.radius)
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
            self.view
                .widget(cx, ids!(container.code_view))
                .set_text(cx, self.code.as_ref());
        });
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
