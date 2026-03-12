use crate::ui::snippets::SPINNER_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySpinnerPageBase = #(GallerySpinnerPage::register_widget(vm))

    mod.widgets.GallerySpinnerPage = set_type_default() do mod.widgets.GallerySpinnerPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Spinner"
            }

            ShadPageSubtitle{
                text: "Circular loading indicator. Use for async operations and loading states."
            }

            ShadHr{}

            spinner_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                spinner_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    spinner_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        spinner_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        spinner_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    spinner_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        spinner_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        spinner_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                spinner_preview_panel := mod.widgets.ShadPanel{
                    spinner_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 24.0
                                align: Align{x: 0.5, y: 0.5}

                                ShadSpinner{}
                            }
                        }

                        code_page +: {
                            body +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            GalleryCodeSnippet{
                                code_view +: { text: #(SPINNER_PREVIEW_CODE) }
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
pub struct GallerySpinnerPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySpinnerPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
