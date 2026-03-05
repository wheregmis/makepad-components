use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadAvatar = mod.widgets.RoundedView{
        width: 40
        height: 40
        flow: Overlay
        align: Align{x: 0.5, y: 0.5}
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: 20.0
            border_size: 0.0
        }
    }

    mod.widgets.ShadAvatarFallback = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 12
        text: "CN"
    }

    mod.widgets.ShadAvatarSm = mod.widgets.ShadAvatar{
        width: 32
        height: 32
        draw_bg +: {
            border_radius: 16.0
        }
    }

    mod.widgets.ShadAvatarLg = mod.widgets.ShadAvatar{
        width: 56
        height: 56
        draw_bg +: {
            border_radius: 28.0
        }
    }
}
