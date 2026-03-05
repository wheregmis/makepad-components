use makepad_components::makepad_widgets::*;
use crate::ui::snippets::COLLAPSIBLE_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCollapsiblePage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Collapsible"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Single section toggle inspired by shadcn/ui collapsible."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        collapsible_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            collapsible_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                collapsible_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    collapsible_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

                    collapsible_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                collapsible_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    collapsible_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    collapsible_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            collapsible_preview_panel := mod.widgets.GalleryPreviewPanel{
                collapsible_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadCollapsible{
                    margin: Inset{top: 12, right: 12}
                    title: "Order #4189"
                    is_open: true
                    body: View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 8.0

                        RoundedView{
                            width: Fill
                            height: Fit
                            flow: Right
                            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                            draw_bg +: {
                                color: #0000
                                border_size: 1.0
                                border_radius: 6.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            Label{
                                width: Fill
                                text: "Status"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }
                            Label{
                                text: "Shipped"
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 10
                            }
                        }

                        RoundedView{
                            width: Fill
                            height: Fit
                            flow: Down
                            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                            spacing: 4.0
                            draw_bg +: {
                                color: #0000
                                border_size: 1.0
                                border_radius: 6.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            Label{
                                text: "Shipping address"
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 10
                            }
                            Label{
                                text: "100 Market St, San Francisco"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }
                        }

                        RoundedView{
                            width: Fill
                            height: Fit
                            flow: Down
                            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                            spacing: 4.0
                            draw_bg +: {
                                color: #0000
                                border_size: 1.0
                                border_radius: 6.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            Label{
                                text: "Items"
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 10
                            }
                            Label{
                                text: "2x Studio Headphones"
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
                            code: #(COLLAPSIBLE_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
