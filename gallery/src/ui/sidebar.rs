use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySidebar = ShadSidebar{
        width: 280

        Label{
            text: "Makepad Component\nGallery"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 13
        }

        ShadSidebarSectionLabel{text: "Components"}

        ScrollYView{
            width: Fill
            height: Fill
            flow: Down

            sidebar_accordion := ShadSidebarItem{text: "Accordion"}
            sidebar_alert := ShadSidebarItem{text: "Alert"}
            sidebar_aspect_ratio := ShadSidebarItem{text: "Aspect Ratio"}
            sidebar_avatar := ShadSidebarItem{text: "Avatar"}
            sidebar_badge := ShadSidebarItem{text: "Badge"}
            sidebar_breadcrumb := ShadSidebarItem{text: "Breadcrumb"}
            sidebar_button := ShadSidebarItem{text: "Button"}
            sidebar_button_group := ShadSidebarItem{text: "Button Group"}
            sidebar_checkbox := ShadSidebarItem{text: "Checkbox"}
            sidebar_collapsible := ShadSidebarItem{text: "Collapsible"}
            sidebar_input := ShadSidebarItem{text: "Input"}
            sidebar_label := ShadSidebarItem{text: "Label"}
            sidebar_sidebar := ShadSidebarItem{text: "Sidebar"}
        }
    }
}
