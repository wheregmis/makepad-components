use crate::ui::snippets::SELECT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySelectPage = ShadScrollArea{
        ShadPageTitle{
            text: "Select"
        }

        ShadPageSubtitle{
            text: "Single-choice, non-searchable selection built on the shared popup-menu stack."
        }

        ShadSeparator{}

        select_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            select_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                select_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    select_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    select_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                select_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    select_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    select_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            select_preview_panel := mod.widgets.ShadPanel{
                select_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadPanel{
                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                View{
                                    width: Fit
                                    height: Fit
                                    flow: Right
                                    spacing: 12.0

                                    ShadSelect{labels: ["Pending" "In Progress" "Done"]}
                                    ShadSelect{labels: ["Toronto" "Montreal" "Vancouver" "Calgary"]}
                                }

                                ShadFieldDescription{
                                    text: "Known limitation: popup-style selects can still be unreliable inside the current gallery PageFlip shell. The splash app remains the best place to verify interaction."
                                }
                            }
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(SELECT_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
