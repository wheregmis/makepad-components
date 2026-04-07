use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadButtonGroup = mod.widgets.ShadSurfacePanel{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 0.0
        draw_bg.color: (shad_theme.color_background)
        draw_bg.border_color: (shad_theme.color_outline_border)
    }

    mod.widgets.ShadButtonGroupItem = set_type_default() do mod.widgets.ShadNavButtonBase{
        width: Fit
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        reset_hover_on_click: true
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
            color_disabled: (shad_theme.color_disabled)
            border_size: 1.0
            border_radius: 0.0
            border_color: #0000
            border_color_hover: (shad_theme.color_outline_border)
            border_color_down: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.color_disabled: (shad_theme.color_disabled_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonGroupItemSm = mod.widgets.ShadButtonGroupItem{
        height: 28
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadButtonGroupItemLg = mod.widgets.ShadButtonGroupItem{
        height: 44
        padding: Inset{left: 20, right: 20, top: 0, bottom: 0}
        draw_text.text_style.font_size: 13
    }

    mod.widgets.ShadButtonGroupItemIcon = mod.widgets.ShadButtonGroupItem{
        width: 36
        spacing: 0.0
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }

    mod.widgets.ShadButtonGroupSeparator = mod.widgets.View{
        width: 1
        height: 18
        draw_bg.color: (shad_theme.color_outline_border_hover)
    }
}
