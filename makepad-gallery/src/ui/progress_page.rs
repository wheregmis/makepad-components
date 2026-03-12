use crate::ui::snippets::PROGRESS_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryProgressPageBase = #(GalleryProgressPage::register_widget(vm))

    mod.widgets.GalleryProgressPage = set_type_default() do mod.widgets.GalleryProgressPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Progress"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired progress bars. Determinate (value 0–1) and indeterminate (animated)."
            }

            ShadHr{}

            progress_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                progress_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    progress_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        progress_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        progress_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    progress_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        progress_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        progress_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                progress_preview_panel := mod.widgets.ShadPanel{
                    progress_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Determinate" }

                            ShadProgress33{}
                            ShadProgress66{}
                            ShadProgressFull{}

                            ShadSectionHeader{ text: "Indeterminate (animated)" }

                            ShadProgressIndeterminate{}
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                GalleryCodeSnippet{
                                    code_view +: { text: #(PROGRESS_PREVIEW_CODE) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryProgressPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryProgressPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
