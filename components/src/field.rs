use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadField = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
    }

    mod.widgets.ShadFieldLabel = mod.widgets.Label{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadFieldDescription = mod.widgets.Label{
        width: Fill
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadFieldError = mod.widgets.Label{
        width: Fill
        draw_text.color: (shad_theme.color_destructive)
        draw_text.text_style.font_size: 10
    }
}
