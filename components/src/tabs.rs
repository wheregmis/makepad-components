use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadTabs = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 12.0
    }

    mod.widgets.ShadTabsList = RoundedView{
        width: Fit
        height: Fit
        flow: Right
        spacing: 4.0
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}

        draw_bg +: {
            color: (shad_theme.color_muted)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadTabsTrigger = ButtonFlat{
        height: 32
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 0.0
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            text_style.font_size: 11.0
        }
    }

    mod.widgets.ShadTabsContent = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

        draw_bg +: {
            color: #0000
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }
}
