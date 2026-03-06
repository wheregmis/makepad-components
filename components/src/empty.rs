use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadEmpty = mod.widgets.RoundedView{
        width: Fill
        height: Fit
        flow: Down
        align: Center
        spacing: 12.0
        padding: Inset{left: 24, right: 24, top: 48, bottom: 48}

        draw_bg +: {
            color: #0000
            border_size: 1.0
            border_style: Dashed
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadEmptyIconContainer = mod.widgets.View{
        width: 48
        height: 48
        align: Center
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: 24.0
            border_size: 0.0
        }
    }

    mod.widgets.ShadEmptyIcon = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 20
    }

    mod.widgets.ShadEmptyContent = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        align: Center
        spacing: 4.0
    }

    mod.widgets.ShadEmptyTitle = mod.widgets.Label{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 12
        // font_weight: Bold (if supported)
    }

    mod.widgets.ShadEmptyDescription = mod.widgets.Label{
        width: Fill
        align: Align{x: 0.5, y: 0.0}
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 11
        draw_text.wrap: Word
    }

    mod.widgets.ShadEmptyAction = mod.widgets.View{
        width: Fill
        height: Fit
        align: Center
        padding: Inset{top: 8}
    }
}
