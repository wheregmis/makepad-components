use crate::ui::registry::gallery_page_entries;
use makepad_components::makepad_widgets::*;

macro_rules! define_gallery_sidebar {
    (
        $(
            {
                title: $title:literal,
                route: $route:literal,
                page: $page:ident,
                widget: $widget:ident,
                sidebar_id: $sidebar_id:ident,
                sidebar_label: $sidebar_label:literal,
                section: $section:literal,
                shortcut: $shortcut:literal,
                snippet: $snippet:ident,
                $(transition: $transition:ident,)?
            }
        )*
    ) => {
        script_mod! {
            use mod.prelude.widgets.*
            use mod.widgets.*

            mod.widgets.GallerySidebarItem = set_type_default() do mod.widgets.ShadNavButtonBase{
                width: Fill
                height: 32
                padding: Inset{left: 10, right: 10}
                align: Align{x: 0.0, y: 0.5}
                reset_hover_on_click: true
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_secondary_hover)
                    color_down: (shad_theme.color_secondary_down)
                    border_radius: (shad_theme.radius)
                    color_focus: (shad_theme.color_secondary_hover)
                    border_size: 0.0
                    border_color: #0000
                }
                draw_text +: {
                    color: (shad_theme.color_primary)
                    color_hover: (shad_theme.color_primary)
                    color_down: (shad_theme.color_primary)
                    color_focus: (shad_theme.color_primary)
                    text_style.font_size: 10
                }
                text: "Item"
            }

            mod.widgets.GallerySidebar = ShadSidebar{
                width: 280

                ShadLabel{
                    text: "Makepad Component\nGallery"
                    draw_text.text_style.font_size: 13
                }

                ShadSidebarSectionLabel{text: "Catalog"}

                ShadScrollYView{
                    width: Fill
                    height: Fill
                    flow: Down

                    $(
                        $sidebar_id := GallerySidebarItem{text: $sidebar_label}
                    )*
                }
            }
        }
    };
}

gallery_page_entries!(define_gallery_sidebar);
