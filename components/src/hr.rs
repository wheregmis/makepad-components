use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadHr = Hr{
        draw_bg.color: (shad_theme.color_outline_border)
    }
}
