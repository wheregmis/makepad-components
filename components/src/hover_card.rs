use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadHoverCard = mod.widgets.CalloutTooltipInner{
        content +: {
            width: Fit
            height: Fit
            padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

            draw_bg +: {
                color: (shad_theme.color_background)
                border_color: (shad_theme.color_outline_border)
                border_radius: (shad_theme.radius)
                background_color: (shad_theme.color_background)
                border_size: 1.0
            }

            hover_content := View{
                width: 250
                height: Fit
                flow: Down
                spacing: 4.0

                draw_bg +: {
                    color: #0000
                }
            }
        }
    }
}
