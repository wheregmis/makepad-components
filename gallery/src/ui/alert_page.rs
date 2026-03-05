use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAlertPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Alert"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired alert components from makepad-components library"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        Label{
            text: "Default"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        ShadAlert{
            width: Fill
            icon := ShadAlertIcon{text: "ⓘ"}
            content := ShadAlertContent{
                title := ShadAlertTitle{text: "Heads up!"}
                description := ShadAlertDescription{
                    text: "You can add components and dependencies to your app using the cli."
                }
            }
        }

        Label{
            text: "Destructive"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        ShadAlertDestructive{
            width: Fill
            icon := ShadAlertDestructiveIcon{text: "✕"}
            content := ShadAlertContent{
                title := ShadAlertDestructiveTitle{text: "Error"}
                description := ShadAlertDescription{
                    text: "Your session has expired. Please log in again."
                }
            }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        alert_example_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadAlert{\n    width: Fill\n    icon := mod.widgets.ShadAlertIcon{text: \"ⓘ\"}\n    content := mod.widgets.ShadAlertContent{\n        title := mod.widgets.ShadAlertTitle{text: \"Heads up!\"}\n        description := mod.widgets.ShadAlertDescription{text: \"Action complete.\"}\n    }\n}"
        }
    }
}
