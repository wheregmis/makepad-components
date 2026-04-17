use crate::ui::registry::gallery_page_entries;
use makepad_components::makepad_widgets::*;

macro_rules! define_gallery_sidebar {
    ($(
        $section_name:literal => {
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
        }
    )*) => {
        script_mod! {
            use mod.prelude.widgets.*
            use mod.widgets.*

            mod.widgets.GallerySidebarTitle = View {
                width: Fill
                height: Fit
                flow: Right
                align: Align {y: 0.5}
                spacing: 10.0
                padding: Inset {top: 8, bottom: 20}

                logo := View {
                    width: 28
                    height: 28
                    flow: Overlay
                    align: Align {x: 0.5, y: 0.5}
                    draw_bg +: {
                        color: (shad_theme.color_primary)
                        border_radius: 6.0
                    }
                    icon := IconBox {
                        width: 16
                        height: 16
                        draw_icon.color: (shad_theme.color_primary_foreground)
                    }
                }

                label := ShadLabel {
                    text: "Makepad UI"
                    draw_text.text_style: theme.font_bold{font_size: 14.0}
                    draw_text.color: (shad_theme.color_primary)
                }

                version_tag := View {
                    width: Fit
                    height: Fit
                    padding: Inset {left: 6, right: 6, top: 2, bottom: 2}
                    margin: Inset {left: 4}
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: 4.0
                    }
                    ShadLabel {
                        text: "v0.1"
                        draw_text.text_style.font_size: 8.0
                        draw_text.color: (shad_theme.color_muted_foreground)
                    }
                }
            }

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
                padding: Inset{right: 8}

                $(
                    ShadSidebarSectionLabel{
                        text: $section_name
                        margin: Inset{top: 24.0, bottom: 8.0, left: 12.0, right: 0.0}
                        draw_text.text_style: theme.font_bold{font_size: 9.0}
                    }
                    $(
                        $sidebar_id := ShadSidebarItem{
                            text: $sidebar_label
                            height: 32
                            draw_text.text_style.font_size: 10.0
                        }
                    )*
                )*
            }

            mod.widgets.GallerySidebar = ShadSidebar{
                width: 260
                padding: Inset{top: 24, right: 16, bottom: 16, left: 24}
                draw_bg.border_size: 0.0
                draw_bg.color: (shad_theme.color_background)

                mod.widgets.GallerySidebarTitle{}

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

                mod.widgets.GallerySidebarList{}
            }
        }
    };
}

gallery_page_entries!(define_gallery_sidebar);
