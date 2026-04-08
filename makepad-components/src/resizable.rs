use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadResizable = Splitter{
        size: 12.0
        min_horizontal: 72.0
        max_horizontal: 72.0
        min_vertical: 120.0
        max_vertical: 120.0

        draw_bg +: {
            size: 3.0
            splitter_pad: 1.0
            border_radius: 999.0
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_outline_border_hover)
            color_drag: (shad_theme.color_primary)
        }
    }
}
