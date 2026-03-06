use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadCard = mod.widgets.RoundedView{
        width: Fill
        height: Fit
        flow: Down
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadCardHeader = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 2.0
        padding: Inset{left: 24, right: 24, top: 24, bottom: 16}
    }

    mod.widgets.ShadCardTitle = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 18
    }

    mod.widgets.ShadCardDescription = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 14
    }

    mod.widgets.ShadCardContent = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        padding: Inset{left: 24, right: 24, top: 0, bottom: 24}
    }

    mod.widgets.ShadCardFooter = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 8.0
        padding: Inset{left: 24, right: 24, top: 16, bottom: 24}
    }
}
