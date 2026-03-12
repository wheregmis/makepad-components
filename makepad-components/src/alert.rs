use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadAlert = mod.widgets.RoundedView{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.0}
        spacing: 12.0
        padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
        draw_bg +: {
            color: #0000
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadAlertIcon = mod.widgets.IconInfo{
        icon_walk: Walk{width: 16, height: 16}
        draw_icon.color: (shad_theme.color_primary)
    }

    mod.widgets.ShadAlertContent = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 4.0
    }

    mod.widgets.ShadAlertTitle = mod.widgets.Label{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadAlertDescription = mod.widgets.Label{
        width: Fill
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadAlertDestructive = mod.widgets.ShadAlert{
        draw_bg.border_color: (shad_theme.color_destructive)
    }

    mod.widgets.ShadAlertDestructiveIcon = mod.widgets.IconX{
        icon_walk: Walk{width: 16, height: 16}
        draw_icon.color: (shad_theme.color_destructive)
    }

    mod.widgets.ShadAlertDestructiveTitle = mod.widgets.ShadAlertTitle{
        draw_text.color: (shad_theme.color_destructive_foreground)
    }
}
