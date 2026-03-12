use crate::ui::snippets::SCROLL_AREA_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryScrollAreaPage = ShadScrollArea{
        ShadPageTitle{
            text: "Scroll Area"
        }

        ShadPageSubtitle{
            text: "Canonical scroll wrappers for vertical, horizontal, and two-axis overflow."
        }

        ShadSeparator{}

        scroll_area_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            scroll_area_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                scroll_area_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    scroll_area_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    scroll_area_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                scroll_area_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    scroll_area_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    scroll_area_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            scroll_area_preview_panel := mod.widgets.ShadPanel{
                scroll_area_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Vertical" }
                        ShadPanel{
                            scroll_area_demo := ShadScrollArea{
                                width: Fill
                                height: 220

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10.0

                                    ShadLabel{text: "Recent activity"}
                                    ShadSeparator{}
                                    ShadLabel{text: "Project Alpha updated 2 minutes ago"}
                                    ShadLabel{text: "Billing statement exported"}
                                    ShadLabel{text: "New teammate invited to workspace"}
                                    ShadLabel{text: "API key rotated successfully"}
                                    ShadLabel{text: "Audit log downloaded"}
                                    ShadLabel{text: "Staging deploy completed"}
                                    ShadLabel{text: "Design review scheduled for Friday"}
                                    ShadLabel{text: "Feature flag enabled for beta cohort"}
                                    ShadLabel{text: "Customer note added to ticket #438"}
                                }
                            }
                        }

                        ShadSectionHeader{ text: "Horizontal" }
                        ShadPanel{
                            ShadScrollAreaX{
                                width: Fill
                                height: Fit

                                View{
                                    width: Fit
                                    height: Fit
                                    flow: Right
                                    spacing: 12.0

                                    ShadBadge{ label := ShadBadgeLabel{text: "Analytics"} }
                                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Retention"} }
                                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Growth"} }
                                    ShadBadge{ label := ShadBadgeLabel{text: "Revenue"} }
                                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Operations"} }
                                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Launch Week"} }
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
                            code: #(SCROLL_AREA_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
