use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadTabs = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 14.0
    }

    mod.widgets.ShadTabsList = ScrollXView{
        width: Fill
        height: Fit
        flow: Right
        clip_x: true
        clip_y: true
        scroll_bars: mod.widgets.ShadScrollBarsX{
            scroll_bar_x: mod.widgets.ShadScrollBar{
                bar_size: 7.0
                min_handle_size: 24.0
                use_vertical_finger_scroll: true
            }
        }
        draw_bg +: {
            color: (shad_theme.color_muted)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
        spacing: 6.0
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}
    }

    mod.widgets.ShadTabsTrigger = ButtonFlat{
        width: Fit
        height: 40
        enable_long_press: true
        reset_hover_on_click: true
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_disabled: (shad_theme.color_disabled)
            border_color: #0000
            border_color_hover: #0000
            border_color_down: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_disabled_foreground)
            text_style.font_size: 11.0
        }
    }

    mod.widgets.ShadTabsIndicator = SolidView{
        width: Fill
        height: 2
        draw_bg.color: (shad_theme.color_primary)
    }

    mod.widgets.ShadTabsContent = mod.widgets.ShadSurfaceTransparent{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0
        padding: Inset{left: 18, right: 18, top: 18, bottom: 18}
    }
}
