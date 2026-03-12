use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadKbd = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        flow: Right
        align: Align{x: 0.5, y: 0.5}
        padding: Inset{left: 6, right: 6, top: 4, bottom: 4}
        draw_bg +: {
            color: (shad_theme.color_kbd_bg)
            border_radius: 5.0
            border_size: 1.0
            border_color: (shad_theme.color_kbd_border)
        }
    }

    mod.widgets.ShadKbdLabel = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_kbd_foreground)
        draw_text.text_style.font_size: 11
        text: "k"
    }

    mod.widgets.ShadKbdSeparator = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_kbd_foreground)
        draw_text.text_style.font_size: 11
        text: " + "
    }
}
