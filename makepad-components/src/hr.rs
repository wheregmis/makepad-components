use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSeparator = Hr{
        draw_bg.color: (shad_theme.color_outline_border)
    }

    mod.widgets.ShadHr = mod.widgets.ShadSeparator{}
}
