use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadPanel = SolidView{
        width: Fill
        height: Fit
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        draw_bg.color: (shad_theme.color_clear)
        draw_bg.border_size: (shad_theme.border_size)
        draw_bg.border_color: (shad_theme.color_outline_border)
        draw_bg.border_radius: (shad_theme.radius)
    }
}
