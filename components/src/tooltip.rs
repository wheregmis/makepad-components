use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadTooltip = Tooltip{
        content +: {
            RoundedView{
                width: Fit
                height: Fit
                padding: Inset{left: 12, right: 12, top: 10, bottom: 10}

                draw_bg +: {
                    color: (shad_theme.color_background)
                    border_color: (shad_theme.color_outline_border)
                    radius: (shad_theme.radius)
                    border_size: 1.0
                }

                tooltip_label := Label{
                    width: 220
                    draw_text +: {
                        color: (shad_theme.color_primary)
                        text_style.font_size: 10.0
                        flow: Flow.Right{wrap: true}
                    }
                }
            }
        }
    }

    mod.widgets.ShadTooltipCallout = CalloutTooltip{
        tooltip := CalloutTooltipInner{
            content +: {
                padding: Inset{left: 12, right: 12, top: 12, bottom: 12}

                draw_bg +: {
                    border_color: (shad_theme.color_outline_border)
                    border_radius: (shad_theme.radius)
                    background_color: (shad_theme.color_background)
                    border_size: 1.0
                }

                tooltip_label := Label{
                    width: 220
                    draw_text +: {
                        color: (shad_theme.color_primary)
                        text_style.font_size: 10.0
                    }
                }
            }
        }
    }
}
