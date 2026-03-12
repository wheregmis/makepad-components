use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySidebar = ShadSidebar{
        width: 280

        ShadLabel{
            text: "Makepad Component\nGallery"
            draw_text.text_style.font_size: 13
        }

        sidebar_theme_toggle := ShadButtonOutline{
            width: Fill
            text: "Light mode"
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
            sidebar_card := ShadSidebarItem{text: "Card"}
            sidebar_carousel := ShadSidebarItem{text: "Carousel"}
            sidebar_checkbox := ShadSidebarItem{text: "Checkbox"}
            sidebar_collapsible := ShadSidebarItem{text: "Collapsible"}
            sidebar_command_palette := ShadSidebarItem{text: "Command Palette"}
            sidebar_context_menu := ShadSidebarItem{text: "Context Menu"}
            sidebar_dialog := ShadSidebarItem{text: "Dialog"}
            sidebar_input := ShadSidebarItem{text: "Input"}
            sidebar_input_otp := ShadSidebarItem{text: "Input OTP"}
            sidebar_pagination := ShadSidebarItem{text: "Pagination"}
            sidebar_popover := ShadSidebarItem{text: "Popover"}
            sidebar_radio_group := ShadSidebarItem{text: "Radio Group"}
            sidebar_resizable := ShadSidebarItem{text: "Resizable"}
            sidebar_scroll_area := ShadSidebarItem{text: "Scroll Area"}
            sidebar_select := ShadSidebarItem{text: "Select"}
            sidebar_separator := ShadSidebarItem{text: "Separator"}
            sidebar_sheet := ShadSidebarItem{text: "Sheet"}
            sidebar_skeleton := ShadSidebarItem{text: "Skeleton"}
            sidebar_switch := ShadSidebarItem{text: "Switch"}
            sidebar_tabs := ShadSidebarItem{text: "Tabs"}
            sidebar_textarea := ShadSidebarItem{text: "Textarea"}
            sidebar_toggle := ShadSidebarItem{text: "Toggle"}
            sidebar_kbd := ShadSidebarItem{text: "Kbd"}
            sidebar_progress := ShadSidebarItem{text: "Progress"}
            sidebar_label := ShadSidebarItem{text: "Label"}
            sidebar_sidebar := ShadSidebarItem{text: "Sidebar"}
            sidebar_slider := ShadSidebarItem{text: "Slider"}
            sidebar_sonner := ShadSidebarItem{text: "Sonner"}
            sidebar_spinner := ShadSidebarItem{text: "Spinner"}
        }
    }
}
