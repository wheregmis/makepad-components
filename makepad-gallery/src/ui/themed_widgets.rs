use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""
        code_container := SolidView{
            width: Fill
            height: Fit
            padding: Inset{top: 12, right: 12, bottom: 12, left: 12}
            draw_bg +: {
                color: (shad_theme.color_muted)
                border_radius: (shad_theme.radius)
            }

            code_label := Label{
                width: Fill
                height: Fit
                padding: 0
                draw_text +: {
                    color: (shad_theme.color_primary)
                    text_style: theme.font_code{
                        font_size: theme.font_size_code
                        line_spacing: theme.font_longform_line_spacing
                    }
                }
            }
        }
    }

    mod.widgets.GalleryPreviewStackNavigation = mod.widgets.PageFlip{
        width: Fill
        height: Fit
        active_page: @root_view

        root_view := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0
        }

        code_page := View{
            width: Fill
            height: Fit

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

        tabs_row := View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 20.0
            margin: Inset{top: 4, bottom: 12}

            demo_tab_group := View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 6.0

                demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                demo_indicator := SolidView{
                    width: Fill
                    height: 2
                    draw_bg.color: (shad_theme.color_primary)
                }
            }

            code_tab_group := View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 6.0

                code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                code_indicator := SolidView{
                    width: Fill
                    height: 2
                    visible: false
                    draw_bg.color: (shad_theme.color_primary)
                }
            }
        }

        preview_panel := mod.widgets.ShadPanel{
            preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                width: Fill
                height: Fit

                code_page +: {
                    body +: {
                        code_snippet := GalleryCodeSnippet{
                            code: ""
                        }
                    }
                }
            }
        }
    }

}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryCodeSnippet {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    code: ArcStringMut,
    #[rust]
    last_code: String,
}

impl GalleryCodeSnippet {
    fn sync_code(&mut self, cx: &mut Cx) {
        let current_raw = self.code.as_ref();
        if current_raw == self.last_code.as_str() {
            return;
        }

        let label = self.view.widget(cx, ids!(code_label));
        if label.is_empty() {
            return;
        }

        label.set_text(cx, current_raw.trim());
        self.last_code = current_raw.to_string();
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_code(cx);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_code(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}
