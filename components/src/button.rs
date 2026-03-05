use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadButton = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_primary_foreground)
            color_hover: (shad_theme.color_secondary)
            color_down: (shad_theme.color_secondary_hover)
            color_focus: (shad_theme.color_primary_foreground)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonDestructive = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_destructive)
            color_hover: (shad_theme.color_destructive_hover)
            color_down: (shad_theme.color_destructive_down)
            color_focus: (shad_theme.color_destructive)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_destructive_foreground)
        draw_text.color_hover: (shad_theme.color_destructive_foreground)
        draw_text.color_down: (shad_theme.color_destructive_foreground)
        draw_text.color_focus: (shad_theme.color_destructive_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonOutline = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: #0000
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_focus: (shad_theme.color_outline_border)
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonSecondary = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_secondary)
            color_hover: (shad_theme.color_secondary_hover)
            color_down: (shad_theme.color_secondary_down)
            color_focus: (shad_theme.color_secondary)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_secondary_foreground)
        draw_text.color_hover: (shad_theme.color_secondary_foreground)
        draw_text.color_down: (shad_theme.color_secondary_foreground)
        draw_text.color_focus: (shad_theme.color_secondary_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonGhost = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: #0000
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonLink = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 4, right: 4, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: #0000
            color_down: #0000
            color_focus: #0000
            border_size: 0.0
            border_radius: 0.0
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary_down)
        draw_text.color_focus: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonSm = mod.widgets.ShadButton{
        height: 28
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadButtonLg = mod.widgets.ShadButton{
        height: 44
        padding: Inset{left: 32, right: 32, top: 0, bottom: 0}
        draw_text.text_style.font_size: 13
    }

    mod.widgets.ShadButtonIcon = mod.widgets.ShadButton{
        width: 36
        height: 36
        spacing: 0.0
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }
}
