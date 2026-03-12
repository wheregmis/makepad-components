use crate::ui::snippets::RADIO_GROUP_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryRadioGroupPageBase = #(GalleryRadioGroupPage::register_widget(vm))

    mod.widgets.GalleryRadioGroupPage = set_type_default() do mod.widgets.GalleryRadioGroupPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollArea{
            ShadPageTitle{
                text: "Radio Group"
            }

            ShadPageSubtitle{
                text: "Single-choice groups styled around Makepad radio buttons."
            }

            ShadSeparator{}

            radio_group_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                radio_group_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    radio_group_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        radio_group_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        radio_group_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    radio_group_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        radio_group_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        radio_group_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                radio_group_preview_panel := mod.widgets.ShadPanel{
                    radio_group_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Stacked options" }
                            ShadPanel{
                                ShadRadioGroup{
                                    ShadRadioItem{text: "Starter"}
                                    ShadRadioItem{text: "Pro"}
                                    ShadRadioItem{text: "Enterprise"}
                                }
                            }

                            ShadSectionHeader{ text: "Inline options" }
                            ShadPanel{
                                ShadRadioGroupInline{
                                    ShadRadioItem{text: "Weekly"}
                                    ShadRadioItem{text: "Monthly"}
                                    ShadRadioItem{text: "Yearly"}
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                GalleryCodeSnippet{
                                    code_view +: { text: #(RADIO_GROUP_PREVIEW_CODE) }
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
pub struct GalleryRadioGroupPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryRadioGroupPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
