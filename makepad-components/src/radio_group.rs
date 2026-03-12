use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadRadioGroup = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0
    }

    mod.widgets.ShadRadioGroupInline = View{
        width: Fit
        height: Fit
        flow: Right
        spacing: 16.0
        align: Align{y: 0.5}
    }

    mod.widgets.ShadRadioItem = RadioButtonFlat{
        width: Fit
        height: Fit
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}

        draw_bg +: {
            size: 16.0
            color: #0000
            color_hover: #0000
            color_down: #0000
            color_focus: #0000
            color_disabled: #0000
            color_active: #0000

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_primary)
            border_color_focus: (shad_theme.color_primary)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)

            mark_color: #0000
            mark_color_active: (shad_theme.color_primary)
            mark_color_disabled: (shad_theme.color_muted_foreground)
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11.0
        }
    }
}
