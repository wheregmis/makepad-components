use crate::ui::snippets::RADIO_GROUP_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryRadioGroupPage = ShadScrollArea{
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
                height: Fit
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
                radio_group_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
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

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(RADIO_GROUP_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
