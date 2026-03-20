use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSelectItem = mod.widgets.PopupMenuItem{
        width: Fill
        height: Fit
        align: Align{y: 0.5}
        padding: Inset{left: 24, right: 12, top: 8, bottom: 8}

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11
        }

        draw_bg +: {
            border_size: 0.0
            border_radius: 6.0
            color: #0000
            color_hover: (shad_theme.color_secondary)
            color_active: (shad_theme.color_secondary_hover)
            color_disabled: #0000
            border_color: #0000
            border_color_hover: #0000
            border_color_active: #0000
            border_color_disabled: #0000
            mark_color: #0000
            mark_color_active: (shad_theme.color_primary)
            mark_color_disabled: (shad_theme.color_muted_foreground)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelectPopupMenu = mod.widgets.PopupMenu{
        width: 220
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}
        menu_item: mod.widgets.ShadSelectItem{}

        draw_bg +: {
            border_size: 1.0
            border_radius: (shad_theme.radius)
            color: (shad_theme.color_background)
            border_color: (shad_theme.color_outline_border)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelect = mod.widgets.DropDownFlat{
        width: 220
        height: 36
        align: Align{x: 0.0, y: 0.5}

        padding: Inset{left: 12, right: 28, top: 0, bottom: 0}

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11
        }

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_focus: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_active: (shad_theme.color_ghost_hover)
            color_disabled: #0000
            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
            border_color_down: (shad_theme.color_primary)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)
            arrow_color: (shad_theme.color_primary)
            arrow_color_hover: (shad_theme.color_primary)
            arrow_color_focus: (shad_theme.color_primary)
            arrow_color_down: (shad_theme.color_primary)
            arrow_color_disabled: (shad_theme.color_muted_foreground)
        }

        popup_menu: mod.widgets.ShadSelectPopupMenu{}
    }
}
