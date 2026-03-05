use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySidebarItem = mod.widgets.ButtonFlatter{
        width: Fill
        height: 32
        draw_text.text_style.font_size: 10
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_bg.color: #0000
        padding: Inset{left: 10, right: 10}
        align: Align{x: 0.0, y: 0.5}
        text: "Item"
    }

    mod.widgets.GallerySectionLabel = mod.widgets.Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 9
    }

    mod.widgets.GallerySidebar = View{
        width: 280
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_secondary)
        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
        spacing: 10.0

        Label{
            text: "Makepad Component\nGallery"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 13
        }

        mod.widgets.GallerySectionLabel{text: "Components"}

        ScrollYView{
            width: Fill
            height: Fill
            flow: Down

            sidebar_accordion := mod.widgets.GallerySidebarItem{text: "Accordion"}
            sidebar_alert := mod.widgets.GallerySidebarItem{text: "Alert"}
            sidebar_avatar := mod.widgets.GallerySidebarItem{text: "Avatar"}
            sidebar_badge := mod.widgets.GallerySidebarItem{text: "Badge"}
            sidebar_button := mod.widgets.GallerySidebarItem{text: "Button"}
            sidebar_checkbox := mod.widgets.GallerySidebarItem{text: "Checkbox"}
        }
    }
}
