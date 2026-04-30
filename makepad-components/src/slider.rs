use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSlider = mod.widgets.SliderRoundFlat{
        width: Fill
        height: 16
        margin: Inset{}
        default: 0.5
        min: 0.0
        max: 1.0
        step: 0.0

        text_input +: {
            width: 0
            height: 0
        }

        draw_bg +: {
            label_size: 0.0
            color: uniform(shad_theme.color_outline_border)
            color_hover: uniform(shad_theme.color_outline_border_hover)
            color_focus: uniform(shad_theme.color_outline_border)
            color_drag: uniform(shad_theme.color_outline_border_hover)
            color_disabled: uniform(shad_theme.color_muted_foreground)

            val_color: uniform(shad_theme.color_primary)
            val_color_hover: uniform(shad_theme.color_primary)
            val_color_focus: uniform(shad_theme.color_primary)
            val_color_drag: uniform(shad_theme.color_primary)
            val_color_disabled: uniform(shad_theme.color_muted_foreground)

            handle_color: uniform(shad_theme.color_primary)
            handle_color_hover: uniform(shad_theme.color_primary)
            handle_color_focus: uniform(shad_theme.color_primary)
            handle_color_drag: uniform(shad_theme.color_primary)
            handle_color_disabled: uniform(shad_theme.color_muted_foreground)

            border_color: uniform(shad_theme.color_outline_border)
            border_color_hover: uniform(shad_theme.color_outline_border_hover)
            border_color_focus: uniform(shad_theme.color_primary)
            border_color_drag: uniform(shad_theme.color_outline_border_hover)
            border_color_disabled: uniform(shad_theme.color_muted_foreground)

            border_radius: uniform(shad_theme.radius)
        }
    }
}
