use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAvatarPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Avatar"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired avatar components from makepad-components library"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

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
            spacing: 12.0

            ShadAvatarSm{
                fallback := ShadAvatarFallback{text: "SM"}
            }

            ShadAvatar{
                fallback := ShadAvatarFallback{text: "CN"}
            }

            ShadAvatarLg{
                fallback := ShadAvatarFallback{text: "LG"}
            }
        }

        Label{
            text: "Fallback Variants"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 12.0

            ShadAvatar{
                fallback := ShadAvatarFallback{text: "JD"}
            }

            ShadAvatar{
                fallback := ShadAvatarFallback{text: "AB"}
            }

            ShadAvatar{
                fallback := ShadAvatarFallback{text: "?"}
            }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        avatar_example_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadAvatar{\n    fallback := mod.widgets.ShadAvatarFallback{text: \"CN\"}\n}\nmod.widgets.ShadAvatarSm{\n    fallback := mod.widgets.ShadAvatarFallback{text: \"SM\"}\n}\nmod.widgets.ShadAvatarLg{\n    fallback := mod.widgets.ShadAvatarFallback{text: \"LG\"}\n}"
        }
    }
}
