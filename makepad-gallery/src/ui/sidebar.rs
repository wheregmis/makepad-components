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

            mod.widgets.GalleryMobileSidebarIconButton = View{
                width: 36
                height: 36
                flow: Overlay
                align: Align{x: 0.5, y: 0.5}

                button := ShadButtonIconOutline{
                    width: Fill
                    height: Fill
                }
            }

            mod.widgets.GalleryMobileSidebarMenuButton = mod.widgets.GalleryMobileSidebarIconButton{
                icon := IconMenu{
                    width: 18
                    height: 18
                    icon_walk: Walk{width: 18, height: 18}
                    draw_icon.color: (shad_theme.color_primary)
                }
            }

            mod.widgets.GalleryMobileSidebarCloseButton = mod.widgets.GalleryMobileSidebarIconButton{
                icon := IconX{
                    width: 16
                    height: 16
                    icon_walk: Walk{width: 16, height: 16}
                    draw_icon.color: (shad_theme.color_primary)
                }
            }

            mod.widgets.GallerySidebarList = ShadScrollYView{
                width: Fill
                height: Fill
                flow: Down

                $(
                    $sidebar_id := ShadSidebarItem{text: $sidebar_label}
                )*
            }

            mod.widgets.GallerySidebar = ShadSidebar{
                width: 280

                ShadLabel{
                    text: "Makepad Component\nGallery"
                    draw_text.text_style.font_size: 13
                }

                ShadSidebarSectionLabel{text: "Catalog"}

                mod.widgets.GallerySidebarList{}
            }

            mod.widgets.GalleryMobileSidebar = ShadSidebar{
                width: Fill
                spacing: 16.0
                padding: Inset{top: 18, right: 16, bottom: 18, left: 16}

                mobile_sidebar_top_row := View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 10.0

                    mobile_sidebar_close_button := mod.widgets.GalleryMobileSidebarCloseButton{
                        visible: false
                    }

                    mobile_sidebar_meta := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        ShadSectionHeader{
                            text: "Components"
                        }

                        ShadLabel{
                            text: "Makepad Component Gallery"
                            draw_text.text_style.font_size: 12
                            draw_text.color: (shad_theme.color_muted_foreground)
                        }
                    }
                }

                mobile_sidebar_search := ShadButtonIconOutline{
                    width: Fill
                    height: 40
                    text: "Search components"
                }

                ShadSidebarSectionLabel{text: "Catalog"}

                mod.widgets.GallerySidebarList{}
            }
        }
    };
}

gallery_page_entries!(define_gallery_sidebar);
