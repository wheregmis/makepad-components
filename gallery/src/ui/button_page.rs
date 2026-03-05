use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryButtonPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Button"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired button components from makepad-components library"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        Label{
            text: "Variants"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0

            ShadButton{text: "Default"}
            ShadButtonDestructive{text: "Destructive"}
            ShadButtonOutline{text: "Outline"}
            ShadButtonSecondary{text: "Secondary"}
            ShadButtonGhost{text: "Ghost"}
            ShadButtonLink{text: "Link"}
        }

        Label{
            text: "Sizes"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButtonSm{text: "Small"}
            ShadButton{text: "Default"}
            ShadButtonLg{text: "Large"}
        }

        Label{
            text: "Destructive Sizes"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButtonDestructive{
                height: 28
                padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                draw_text.text_style.font_size: 10
                text: "Small"
            }
            ShadButtonDestructive{text: "Default"}
            ShadButtonDestructive{
                height: 44
                padding: Inset{left: 32, right: 32, top: 0, bottom: 0}
                draw_text.text_style.font_size: 13
                text: "Large"
            }
        }

        Label{
            text: "Outline Variations"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButtonOutline{text: "Outline"}
            ShadButtonGhost{text: "Ghost"}
            ShadButtonLink{text: "Link"}
        }

        Label{
            text: "Makepad Icon Crate"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 10.0

            IconCheck{}
            IconX{}
            IconSearch{}
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        button_example_snippet := GalleryCodeSnippet{
            code: "View{\n    width: Fit\n    height: Fit\n    flow: Right\n    spacing: 8\n    mod.widgets.ShadButton{text: \"Default\"}\n    mod.widgets.ShadButtonDestructive{text: \"Delete\"}\n    mod.widgets.ShadButtonOutline{text: \"Outline\"}\n}"
        }
    }
}
