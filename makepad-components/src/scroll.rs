use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadScrollBar = mod.widgets.ScrollBar{
        bar_size: 8.0
        bar_side_margin: 2.0
        min_handle_size: 28.0
        draw_bg +: {
            size: 4.0
            border_size: 0.0
            border_radius: 999.0
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_outline_border_hover)
            color_drag: (shad_theme.color_primary)
            border_color: #0000
            border_color_hover: #0000
            border_color_drag: #0000
        }
    }

    mod.widgets.ShadScrollBarsX = mod.widgets.ScrollBars{
        show_scroll_x: true
        show_scroll_y: false
        scroll_bar_x: mod.widgets.ShadScrollBar{
            drag_scrolling: true
            use_vertical_finger_scroll: true
        }
    }

    mod.widgets.ShadScrollBarsY = mod.widgets.ScrollBars{
        show_scroll_x: false
        show_scroll_y: true
        scroll_bar_y: mod.widgets.ShadScrollBar{
            drag_scrolling: true
        }
    }

    mod.widgets.ShadScrollBarsXY = mod.widgets.ScrollBars{
        show_scroll_x: true
        show_scroll_y: true
        scroll_bar_x: mod.widgets.ShadScrollBar{
            drag_scrolling: true
            use_vertical_finger_scroll: true
        }
        scroll_bar_y: mod.widgets.ShadScrollBar{
            drag_scrolling: true
        }
    }

    mod.widgets.ShadScrollArea = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        clip_x: true
        clip_y: true
        scroll_bars: mod.widgets.ShadScrollBarsY{}
        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
        padding: Inset{top: 18, right: 18, bottom: 18, left: 18}
        spacing: 12.0
    }

    mod.widgets.ShadScrollAreaX = ScrollXView{
        width: Fill
        height: Fit
        flow: Right
        clip_x: true
        clip_y: true
        scroll_bars: mod.widgets.ShadScrollBarsX{}
        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
        padding: Inset{top: 12, right: 14, bottom: 14, left: 14}
        spacing: 12.0
    }

    mod.widgets.ShadScrollAreaXY = ScrollXYView{
        width: Fill
        height: Fill
        flow: Down
        clip_x: true
        clip_y: true
        scroll_bars: mod.widgets.ShadScrollBarsXY{}
        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
        padding: Inset{top: 18, right: 18, bottom: 18, left: 18}
        spacing: 12.0
    }

    mod.widgets.ShadScrollYView = mod.widgets.ShadScrollArea{}
}
