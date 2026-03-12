use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadLabel = mod.widgets.Label {
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadPageTitle = mod.widgets.Label{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 18
    }

    mod.widgets.ShadPageSubtitle = mod.widgets.Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadSectionHeader = mod.widgets.Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
    }
}
