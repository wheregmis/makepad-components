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
            color: (shad_theme.color_popover)
            border_radius: (shad_theme.radius)
            border_size: (shad_theme.border_size)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadCardHeader = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 4.0
        padding: Inset{left: 20, right: 20, top: 20, bottom: 14}
    }

    mod.widgets.ShadCardTitle = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 16
    }

    mod.widgets.ShadCardDescription = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadCardContent = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 12.0
        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
    }

    mod.widgets.ShadCardFooter = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 8.0
        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
    }
}
