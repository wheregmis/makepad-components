use crate::ui::snippets::ASPECT_RATIO_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAspectRatioPage = ShadScrollYView{
        ShadPageTitle{
            text: "Aspect Ratio"
        }

        ShadPageSubtitle{
            text: "Displays content within a desired ratio."
        }

        ShadHr{}

        aspect_ratio_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            aspect_ratio_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                aspect_ratio_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    aspect_ratio_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    aspect_ratio_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                aspect_ratio_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    aspect_ratio_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    aspect_ratio_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            aspect_ratio_preview_panel := mod.widgets.ShadPanel{
                aspect_ratio_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                ShadSectionHeader{ text: "16:9 Preview" }

                RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                    draw_bg +: {
                        color: #0000
                        border_size: 1.0
                        border_radius: (shad_theme.radius)
                        border_color: (shad_theme.color_outline_border)
                    }

                    ShadAspectRatio{
                        width: Fill
                        ratio: 1.7777777778

                        RoundedView{
                            width: Fill
                            height: Fill
                            flow: Overlay
                            align: Align{x: 0.5, y: 0.5}
                            draw_bg +: {
                                color: (shad_theme.color_secondary)
                                border_radius: (shad_theme.radius)
                            }

                            ShadSectionHeader{
                                text: "16:9"
                                draw_text.text_style.font_size: 11
                            }
                        }
                    }
                }

                ShadSectionHeader{ text: "Common Ratios" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 12.0

                    ShadAspectRatio{
                        width: 180
                        ratio: 1.0

                        RoundedView{
                            width: Fill
                            height: Fill
                            flow: Overlay
                            align: Align{x: 0.5, y: 0.5}
                            draw_bg +: {
                                color: (shad_theme.color_secondary)
                                border_radius: (shad_theme.radius)
                            }
                            ShadSectionHeader{ text: "1:1" }
                        }
                    }

                    ShadAspectRatio{
                        width: 180
                        ratio: 1.3333333333

                        RoundedView{
                            width: Fill
                            height: Fill
                            flow: Overlay
                            align: Align{x: 0.5, y: 0.5}
                            draw_bg +: {
                                color: (shad_theme.color_secondary)
                                border_radius: (shad_theme.radius)
                            }
                            ShadSectionHeader{ text: "4:3" }
                        }
                    }

                    ShadAspectRatio{
                        width: 180
                        ratio: 0.5625

                        RoundedView{
                            width: Fill
                            height: Fill
                            flow: Overlay
                            align: Align{x: 0.5, y: 0.5}
                            draw_bg +: {
                                color: (shad_theme.color_secondary)
                                border_radius: (shad_theme.radius)
                            }
                            ShadSectionHeader{ text: "9:16" }
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
                            code: #(ASPECT_RATIO_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
