use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryHr = Hr{
        draw_bg.color: (shad_theme.color_outline_border)
    }

    mod.widgets.GalleryCheckBox = CheckBox{
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
        draw_bg.color: (shad_theme.color_muted_foreground)
        draw_bg.color_hover: (shad_theme.color_secondary_hover)
    }

    mod.widgets.GalleryToggle = Toggle{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
    }
}
