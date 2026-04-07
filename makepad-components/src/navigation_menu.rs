use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadNavigationMenu = View{
        width: Fit
        height: Fit
        flow: Down
        spacing: 8.0
    }

    mod.widgets.ShadNavigationMenuList = View{
        width: Fit
        height: Fit
        new_batch: true
        flow: Right
        align: Align{y: 0.5}
        spacing: 6.0
        padding: Inset{left: 6, right: 6, top: 6, bottom: 6}
        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadNavigationMenuTrigger = ButtonFlat{
        height: 36
        new_batch: true
        enable_long_press: true
        padding: Inset{left: 14, right: 14, top: 0, bottom: 0}

        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
            color_disabled: (shad_theme.color_disabled)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: #0000
            border_color_hover: (shad_theme.color_outline_border)
            border_color_down: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
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

    mod.widgets.ShadNavigationMenuContent = mod.widgets.ShadSurfacePopover{
        width: 320
        height: Fit
        spacing: 14.0
        padding: Inset{left: 18, right: 18, top: 18, bottom: 18}
        draw_bg.border_color: (shad_theme.color_outline_border_hover)
    }

    mod.widgets.ShadNavigationMenuSectionLabel = Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 9
        text: "Section"
    }

    mod.widgets.ShadNavigationMenuCallout = mod.widgets.ShadSurfacePanel{
        width: 196
        height: Fit
        spacing: 8.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
        draw_bg.color: (shad_theme.color_background)
        draw_bg.border_color: (shad_theme.color_outline_border_hover)
    }

    mod.widgets.ShadNavigationMenuPanel = mod.widgets.ShadSurfacePanel{
        width: Fill
        height: Fit
        spacing: 6.0
        padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
        draw_bg.color: (shad_theme.color_background)
        draw_bg.border_color: (shad_theme.color_outline_border)
    }

    mod.widgets.ShadNavigationMenuItem = mod.widgets.ShadPopover{
        side: "bottom"
        align: "center"
        side_offset: 8.0
        viewport_padding: 16.0
        can_dismiss: true
        open_on_hover: true

        trigger := mod.widgets.ShadNavigationMenuTrigger{
            text: "Overview"
        }

        content: mod.widgets.ShadNavigationMenuContent{}
    }
}
