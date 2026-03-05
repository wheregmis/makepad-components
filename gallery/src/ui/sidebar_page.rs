use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySidebarPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Sidebar"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Composable sidebar primitives used by the gallery navigation."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 12.0
            align: Align{y: 0.0}

            ShadSidebar{
                width: 300
                height: 320
                Label{
                    text: "Acme Inc"
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 12
                }
                ShadSidebarSectionLabel{text: "Platform"}
                ShadSidebarItem{text: "Playground"}
                ShadSidebarItem{text: "History"}
                ShadSidebarItem{text: "Settings"}
            }

            View{
                width: Fill
                height: 320
                draw_bg.color: #0000
                draw_bg.border_size: 1.0
                draw_bg.border_color: (shad_theme.color_outline_border)
                draw_bg.border_radius: (shad_theme.radius)
            }
        }

        sidebar_example_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadSidebar{\n    width: 300\n    Label{text: \"Acme Inc\"}\n    ShadSidebarSectionLabel{text: \"Platform\"}\n    ShadSidebarItem{text: \"Playground\"}\n    ShadSidebarItem{text: \"History\"}\n}"
        }
    }
}
