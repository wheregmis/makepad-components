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

            mod.widgets.GalleryMobileSidebarMenuButton = ShadButtonOutline{
                width: Fit
                height: 36
                text: "Menu"
            }

            mod.widgets.GalleryMobileSidebarCloseButton = ShadButtonOutline{
                width: Fit
                height: 36
                text: "Close menu"
            }

            mod.widgets.GallerySidebar = ShadSidebar{
                width: 280

                mobile_sidebar_close_button := mod.widgets.GalleryMobileSidebarCloseButton{
                    visible: false
                }

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
                        $sidebar_id := ShadSidebarItem{text: $sidebar_label}
                    )*
                }
            }
        }
    };
}

gallery_page_entries!(define_gallery_sidebar);
