use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSidebar = View{
        width: 280
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_secondary)
        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
        spacing: 10.0
    }

    mod.widgets.ShadSidebarSectionLabel = Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 9
    }

    mod.widgets.ShadSidebarItem = ButtonFlatter{
        width: Fill
        height: 32
        draw_text.text_style.font_size: 10
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_bg.color: #0000
        draw_bg.color_hover: (shad_theme.color_ghost_hover)
        draw_bg.color_down: (shad_theme.color_ghost_down)
        padding: Inset{left: 10, right: 10}
        align: Align{x: 0.0, y: 0.5}
        text: "Item"
    }
}
