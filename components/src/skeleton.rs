use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.ShadSkeleton = mod.widgets.RoundedView{
        width: 100
        height: 20
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000
        }
    }
}
