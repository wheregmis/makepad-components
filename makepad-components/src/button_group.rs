use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadButtonGroup = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 0.0
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadButtonGroupItem = set_type_default() do mod.widgets.ShadNavButtonBase{
        width: Fit
        size_is_managed: true
        variant_is_managed: true
        variant: ShadButtonVariant.Ghost
        managed_radius: 0.0
        size_small_height: 28
        size_default_height: 36
        size_large_height: 44
        size_small_padding_x: 12
        size_default_padding_x: 16
        size_large_padding_x: 20
        size_small_font_size: 10
        size_default_font_size: 11
        size_large_font_size: 13
        destructive_fill: #0000
        destructive_fill_hover: (shad_theme.color_destructive_hover)
        destructive_fill_down: (shad_theme.color_destructive_down)
        destructive_fill_focus: (shad_theme.color_destructive_hover)
        destructive_text: (shad_theme.color_destructive)
        destructive_text_hover: (shad_theme.color_destructive)
        destructive_text_down: (shad_theme.color_destructive_foreground)
        destructive_text_focus: (shad_theme.color_destructive)
        reset_hover_on_click: true
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonGroupSeparator = mod.widgets.View{
        width: 1
        height: 16
        draw_bg.color: (shad_theme.color_outline_border)
    }
}
