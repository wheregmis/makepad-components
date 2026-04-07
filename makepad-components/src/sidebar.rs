use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSidebar = mod.widgets.ShadSurfacePanel{
        width: 280
        height: Fill
        flow: Down
        clip_x: true
        clip_y: true
        draw_bg.color: (shad_theme.color_background)
        draw_bg.border_size: 1.0
        draw_bg.border_color: (shad_theme.color_outline_border)
        padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
        spacing: 12.0
    }

    mod.widgets.ShadSidebarSectionLabel = Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadSidebarItem = mod.widgets.ButtonFlat{
        width: Fill
        height: 40
        new_batch: true
        enable_long_press: true
        padding: Inset{left: 14, right: 14}
        align: Align{x: 0.0, y: 0.5}
        reset_hover_on_click: true
        grab_key_focus: true
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_secondary_hover)
            color_down: (shad_theme.color_secondary_down)
            border_radius: (shad_theme.radius)
            color_focus: (shad_theme.color_secondary_hover)
            border_size: 1.0
            border_color: #0000
            border_color_hover: #0000
            border_color_down: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
        }
        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            text_style.font_size: 11
        }
        text: "Item"
    }
}
