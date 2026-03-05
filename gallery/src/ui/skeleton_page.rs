use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySkeletonPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Skeleton"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Use to show a placeholder while content is loading."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        Label{
            text: "Preview"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 12.0
            align: Align{x: 0.0, y: 0.5}

            ShadSkeleton{
                width: 48
                height: 48
                draw_bg.border_radius: 24.0
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0

                ShadSkeleton{
                    width: 200
                    height: 16
                }
                ShadSkeleton{
                    width: 150
                    height: 16
                }
            }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        skeleton_code_snippet := GalleryCodeSnippet{
            code: "View{\n    width: Fill\n    height: Fit\n    flow: Right\n    spacing: 12.0\n    align: Align{x: 0.0, y: 0.5}\n\n    ShadSkeleton{\n        width: 48\n        height: 48\n        draw_bg.border_radius: 24.0\n    }\n\n    View{\n        width: Fit\n        height: Fit\n        flow: Down\n        spacing: 8.0\n\n        ShadSkeleton{\n            width: 200\n            height: 16\n        }\n        ShadSkeleton{\n            width: 150\n            height: 16\n        }\n    }\n}"
        }
    }
}

