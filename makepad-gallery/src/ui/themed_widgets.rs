use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryPageRoot = mod.widgets.ShadScrollYView {
        padding: Inset{top: 0, right: 64, bottom: 80, left: 64}
        draw_bg.border_size: 0.0
        draw_bg.color: (shad_theme.color_background)
    }

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

    mod.widgets.GalleryActionFlow = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 12.0
        margin: Inset{top: 16}
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        draw_bg +: {
            color: (shad_theme.color_muted)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_border)
        }

        title := ShadSectionHeader{
            text: "Action Flow"
            draw_text.text_style.font_size: 10.0
            draw_text.color: (shad_theme.color_primary)
        }

        body := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0
        }
    }

    mod.widgets.GalleryPreviewStackNavigation = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 16.0

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
        margin: Inset{top: 32, bottom: 32}

        preview_panel := RoundedView{
            width: Fill
            height: Fit
            flow: Down
            padding: 0.0
            draw_bg +: {
                color: (shad_theme.color_background)
                border_radius: 8.0
                border_size: 1.0
                border_color: (shad_theme.color_border)
            }

            preview_header := View {
                width: Fill
                height: 40
                flow: Right
                align: Align{y: 0.5}
                padding: Inset{left: 16, right: 16}
                draw_bg +: {
                    color: (shad_theme.color_muted)
                    border_radius: vec4(8.0, 8.0, 0.0, 0.0)
                }

                dots := View {
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 6.0
                    View { width: 8, height: 8, draw_bg +: { color: #ff5f57, border_radius: 4.0 } }
                    View { width: 8, height: 8, draw_bg +: { color: #febc2e, border_radius: 4.0 } }
                    View { width: 8, height: 8, draw_bg +: { color: #28c840, border_radius: 4.0 } }

                }

                spacing := View { width: Fill, height: 1 }

                ShadLabel {
                    text: "Component Preview"
                    draw_text.text_style.font_size: 9.0
                    draw_text.color: (shad_theme.color_muted_foreground)
                }
            }

            preview_content_wrapper := View {
                width: Fill
                height: Fit
                padding: Inset{top: 48, right: 48, bottom: 48, left: 48}

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
