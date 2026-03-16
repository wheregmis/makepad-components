use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSurface = RoundedView{
        width: Fill
        height: Fit

        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadSurfaceMuted = mod.widgets.ShadSurface{
        draw_bg.color: (shad_theme.color_muted)
    }

    mod.widgets.ShadMediaFrame = mod.widgets.ShadSurface{
        width: Fill
        height: Fill
        flow: Overlay
        clip_x: true
        clip_y: true
        draw_bg.color: (shad_theme.color_secondary)
    }
}
