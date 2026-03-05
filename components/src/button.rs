use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadButton = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #18181b
            color_hover: #27272a
            color_down: #3f3f46
            border_size: 0.0
            border_radius: 6.0
            border_color: #0000
        }
        draw_text.color: #fafafa
        draw_text.color_hover: #fafafa
        draw_text.color_down: #fafafa
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonDestructive = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #dc2626
            color_hover: #b91c1c
            color_down: #991b1b
            border_size: 0.0
            border_radius: 6.0
            border_color: #0000
        }
        draw_text.color: #fafafa
        draw_text.color_hover: #fafafa
        draw_text.color_down: #fafafa
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonOutline = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: #27272a
            color_down: #3f3f46
            border_size: 1.0
            border_radius: 6.0
            border_color: #3f3f46
            border_color_hover: #52525b
            border_color_down: #71717a
        }
        draw_text.color: #fafafa
        draw_text.color_hover: #fafafa
        draw_text.color_down: #fafafa
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonSecondary = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #27272a
            color_hover: #3f3f46
            color_down: #52525b
            border_size: 0.0
            border_radius: 6.0
            border_color: #0000
        }
        draw_text.color: #a1a1aa
        draw_text.color_hover: #d4d4d8
        draw_text.color_down: #fafafa
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonGhost = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: #27272a
            color_down: #3f3f46
            border_size: 0.0
            border_radius: 6.0
            border_color: #0000
        }
        draw_text.color: #fafafa
        draw_text.color_hover: #fafafa
        draw_text.color_down: #fafafa
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonLink = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 4, right: 4, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: #0000
            color_down: #0000
            border_size: 0.0
            border_radius: 0.0
            border_color: #0000
        }
        draw_text.color: #a1a1aa
        draw_text.color_hover: #fafafa
        draw_text.color_down: #d4d4d8
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonSm = mod.widgets.ShadButton{
        height: 28
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadButtonLg = mod.widgets.ShadButton{
        height: 44
        padding: Inset{left: 32, right: 32, top: 0, bottom: 0}
        draw_text.text_style.font_size: 13
    }

    mod.widgets.ShadButtonIcon = mod.widgets.ShadButton{
        width: 36
        height: 36
        spacing: 0.0
        text: ""
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }
}
