use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadMenubar = RoundedView{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 4.0
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}

        draw_bg +: {
            color: (shad_theme.color_background)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadMenubarTrigger = ButtonFlat{
        height: 30
        padding: Inset{left: 10, right: 10, top: 0, bottom: 0}

        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_secondary)
            color_down: (shad_theme.color_secondary_hover)
            color_focus: (shad_theme.color_secondary)
            color_disabled: (shad_theme.color_disabled)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_disabled_foreground)
            text_style.font_size: 11
        }
    }

    mod.widgets.ShadMenubarContent = RoundedView{
        width: 220
        height: Fit
        flow: Down
        spacing: 4.0
        padding: Inset{left: 6, right: 6, top: 6, bottom: 6}

        draw_bg +: {
            color: (shad_theme.color_popover)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadMenubarLabel = Label{
        margin: Inset{left: 8, right: 8, top: 4, bottom: 2}
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 9
        text: "Section"
    }

    mod.widgets.ShadMenubarHint = Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
        text: "Shortcut"
    }

    mod.widgets.ShadMenubarItem = mod.widgets.ShadButtonGhost{
        width: Fill
        height: 32
        align: Align{x: 0.0, y: 0.5}
        padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
        grab_key_focus: false
        reset_hover_on_click: true

        draw_bg +: {
            border_radius: (shad_theme.radius)
            color_focus: (shad_theme.color_ghost_hover)
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            text_style.font_size: 11
        }

        text: "Menu item"
    }

    mod.widgets.ShadMenubarSeparator = ShadSeparator{
        margin: Inset{left: 2, right: 2, top: 2, bottom: 2}
    }

    mod.widgets.ShadMenubarMenu = mod.widgets.ShadPopover{
        side: "bottom"
        align: "start"
        side_offset: 6.0
        viewport_padding: 10.0
        can_dismiss: true
        open_on_hover: true

        trigger := mod.widgets.ShadMenubarTrigger{
            text: "Menu"
        }

        content: mod.widgets.ShadMenubarContent{}
    }
}
