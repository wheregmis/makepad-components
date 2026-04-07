use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadBadge = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        new_batch: true
        flow: Right
        align: Align{x: 0.5, y: 0.5}
        padding: Inset{left: 8, right: 8, top: 3, bottom: 3}
        draw_bg +: {
            color: (shad_theme.color_primary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000
        }
    }

    mod.widgets.ShadBadgeLabel = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary_foreground)
        draw_text.text_style.font_size: 9
        text: "Badge"
    }

    mod.widgets.ShadBadgeSecondary = mod.widgets.ShadBadge{
        draw_bg +: {
            color: (shad_theme.color_secondary)
        }
    }

    mod.widgets.ShadBadgeSecondaryLabel = mod.widgets.ShadBadgeLabel{
        draw_text.color: (shad_theme.color_secondary_foreground)
    }

    mod.widgets.ShadBadgeDestructive = mod.widgets.ShadBadge{
        draw_bg +: {
            color: (shad_theme.color_destructive)
        }
    }

    mod.widgets.ShadBadgeDestructiveLabel = mod.widgets.ShadBadgeLabel{
        draw_text.color: (shad_theme.color_destructive_foreground)
    }

    mod.widgets.ShadBadgeSuccess = mod.widgets.ShadBadge{
        draw_bg +: {
            color: (shad_theme.color_success_subtle)
            border_size: 1.0
            border_color: #0000
        }
    }

    mod.widgets.ShadBadgeSuccessLabel = mod.widgets.ShadBadgeLabel{
        draw_text.color: (shad_theme.color_success)
    }

    mod.widgets.ShadBadgeWarning = mod.widgets.ShadBadge{
        draw_bg +: {
            color: (shad_theme.color_warning_subtle)
            border_size: 1.0
            border_color: #0000
        }
    }

    mod.widgets.ShadBadgeWarningLabel = mod.widgets.ShadBadgeLabel{
        draw_text.color: (shad_theme.color_warning)
    }

    mod.widgets.ShadBadgeOutline = mod.widgets.ShadBadge{
        draw_bg +: {
            color: #0000
            border_size: 1.0
            border_color: (shad_theme.color_outline_border_hover)
        }
    }

    mod.widgets.ShadBadgeOutlineLabel = mod.widgets.ShadBadgeLabel{
        draw_text.color: (shad_theme.color_muted_foreground)
    }
}
