use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // Single switch/toggle component: themed Toggle (same as gallery uses)
    mod.widgets.ShadSwitch = Toggle{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10

        draw_bg.border_color: (shad_theme.color_outline_border)
        draw_bg.border_color_active: (shad_theme.color_outline_border)
        draw_bg.border_color_hover: (shad_theme.color_outline_border_hover)
    }
}
