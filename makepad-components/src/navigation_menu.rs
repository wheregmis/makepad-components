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
        flow: Right
        align: Align{y: 0.5}
        spacing: 4.0
    }

    mod.widgets.ShadNavigationMenuTrigger = ButtonFlat{
        height: 36
        padding: Inset{left: 14, right: 14, top: 0, bottom: 0}

        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
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

    mod.widgets.ShadNavigationMenuContent = RoundedView{
        width: 320
        height: Fit
        flow: Down
        spacing: 12.0
        padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

        draw_bg +: {
            color: (shad_theme.color_popover)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadNavigationMenuSectionLabel = Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 9
        text: "Section"
    }

    mod.widgets.ShadNavigationMenuCallout = RoundedView{
        width: 196
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadNavigationMenuPanel = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 6.0
        padding: Inset{left: 12, right: 12, top: 12, bottom: 12}

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
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
