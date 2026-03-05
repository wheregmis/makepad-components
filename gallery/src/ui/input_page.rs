use makepad_components::makepad_widgets::*;
use crate::ui::snippets::INPUT_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryInputPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Input"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired text input field component."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        input_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            input_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                input_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    input_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

                    input_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                input_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    input_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    input_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            input_preview_panel := mod.widgets.GalleryPreviewPanel{
                input_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                Label{
                    text: "Default"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: 320
                    height: Fit

                    ShadInput{
                        empty_text: "Email"
                    }
                }

                GalleryHr{}

                Label{
                    text: "Disabled"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: 320
                    height: Fit

                    ShadInput{
                        is_read_only: true
                        empty_text: "Read Only Value"
                    }
                }

                GalleryHr{}

                Label{
                    text: "With Label"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: 320
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    ShadLabel{ text: "Email" }
                    ShadInput{ empty_text: "Email" }
                }

                Label{
                    text: "With Leading Icon"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: 320
                    height: Fit

                    ShadInputWithIcon{}
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(INPUT_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
