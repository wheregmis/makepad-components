use crate::ui::snippets::TOOLTIP_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryTooltipPage = SolidView{
        width: Fill
        height: Fill
        draw_bg.color: (shad_theme.color_background)
        flow: Overlay

        ShadScrollArea{
            ShadPageTitle{
                text: "Tooltip"
            }

            ShadPageSubtitle{
                text: "Thin wrappers over Makepad tooltip primitives for quick hints and callouts."
            }

            ShadSeparator{}

            tooltip_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                tooltip_tabs_row := View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    tooltip_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        tooltip_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        tooltip_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    tooltip_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        tooltip_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        tooltip_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                tooltip_preview_panel := mod.widgets.ShadPanel{
                    tooltip_preview_flip := PageFlip{
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

                                    ShadSectionHeader{ text: "Default" }
                                    View{
                                        width: Fit
                                        height: Fit
                                        flow: Right
                                        spacing: 12.0

                                        tooltip_basic_btn := ShadButtonOutline{text: "Hover tooltip"}
                                        tooltip_callout_btn := ShadButtonOutline{text: "Hover callout"}
                                    }

                                    ShadFieldDescription{text: "Hover the triggers to preview tooltip and callout behavior."}
                                }
                            }
                        }

                        code_page := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            GalleryCodeSnippetSimple{
                                code: #(TOOLTIP_PREVIEW_CODE)
                            }
                        }
                    }
                }
            }
        }

        basic_tooltip := ShadTooltip{}
        callout_tooltip := ShadTooltipCallout{}
    }
}
