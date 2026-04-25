use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadTextPrimary = mod.widgets.Label {
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadTextMuted = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadTextDestructive = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_destructive)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadLabel = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadPageTitle = mod.widgets.Label{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style: theme.font_bold{font_size: 24.0}
        margin: Inset{top: 40, bottom: 8}
    }

    mod.widgets.ShadPageSubtitle = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style: theme.font_regular{font_size: 11.0}
        margin: Inset{bottom: 32}
    }

    mod.widgets.ShadSectionHeader = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style: theme.font_bold{font_size: 10.0}
    }
}
