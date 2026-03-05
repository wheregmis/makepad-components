use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryBadgePage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Badge"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired badge components from makepad-components library"
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
            align: Align{y: 0.5}
            spacing: 8.0

            ShadBadge{
                label := ShadBadgeLabel{text: "Default"}
            }

            ShadBadgeSecondary{
                label := ShadBadgeSecondaryLabel{text: "Secondary"}
            }

            ShadBadgeDestructive{
                label := ShadBadgeDestructiveLabel{text: "Destructive"}
            }

            ShadBadgeOutline{
                label := ShadBadgeOutlineLabel{text: "Outline"}
            }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        badge_example_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadBadge{\n    label := mod.widgets.ShadBadgeLabel{text: \"Default\"}\n}\nmod.widgets.ShadBadgeDestructive{\n    label := mod.widgets.ShadBadgeDestructiveLabel{text: \"Destructive\"}\n}\nmod.widgets.ShadBadgeOutline{\n    label := mod.widgets.ShadBadgeOutlineLabel{text: \"Outline\"}\n}"
        }
    }
}
