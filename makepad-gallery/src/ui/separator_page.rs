use crate::ui::snippets::SEPARATOR_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySeparatorPage = ShadScrollArea{
        ShadPageTitle{
            text: "Separator"
        }

        ShadPageSubtitle{
            text: "A lightweight divider for grouping related content sections."
        }

        ShadSeparator{}

        separator_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            separator_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                separator_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    separator_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    separator_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                separator_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    separator_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    separator_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            separator_preview_panel := mod.widgets.ShadPanel{
                separator_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Stacked content" }
                        ShadPanel{
                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                ShadLabel{text: "Account"}
                                ShadFieldDescription{text: "Profile settings and personal information."}
                                ShadSeparator{}
                                ShadLabel{text: "Billing"}
                                ShadFieldDescription{text: "Invoices, payment methods, and tax details."}
                                ShadSeparator{}
                                ShadLabel{text: "Security"}
                                ShadFieldDescription{text: "Sessions, MFA, and access tokens."}
                            }
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(SEPARATOR_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
