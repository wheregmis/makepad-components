use makepad_components::makepad_widgets::*;

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

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        input_code_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadInput{ empty_message: \"Email\" }"
        }
    }
}
