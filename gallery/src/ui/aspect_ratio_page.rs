use makepad_components::makepad_widgets::*;
use crate::ui::snippets::ASPECT_RATIO_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAspectRatioPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Aspect Ratio"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Displays content within a desired ratio."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

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

                    aspect_ratio_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

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

                    aspect_ratio_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    aspect_ratio_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            aspect_ratio_preview_panel := mod.widgets.GalleryPreviewPanel{
                aspect_ratio_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                Label{
                    text: "16:9 Preview"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

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

                            Label{
                                text: "16:9"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 11
                            }
                        }
                    }
                }

                Label{
                    text: "Common Ratios"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

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
                            Label{
                                text: "1:1"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }
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
                            Label{
                                text: "4:3"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }
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
                            Label{
                                text: "9:16"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }
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
