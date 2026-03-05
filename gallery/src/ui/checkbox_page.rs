use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCheckboxPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Checkbox"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired checkbox component from makepad-components library"
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
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadCheckbox{label: "Accept terms and conditions"}
            ShadCheckbox{label: "Pre-checked option" checked: true}
            ShadCheckbox{label: "Subscribe to newsletter"}
        }

        GalleryHr{}

        Label{
            text: "In a form row"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{y: 0.5}

            ShadCheckbox{label: "Option A"}
            ShadCheckbox{label: "Option B" checked: true}
            ShadCheckbox{label: "Option C"}
        }

        GalleryHr{}

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        checkbox_example_snippet := GalleryCodeSnippet{
            code: "ShadCheckbox{label: \"Accept terms and conditions\"}\nShadCheckbox{label: \"Pre-checked\" checked: true}"
        }
    }
}
