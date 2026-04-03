use crate::ui::registry::gallery_page_entries;
use makepad_components::makepad_widgets::*;

macro_rules! define_gallery_root {
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
        #[cfg_attr(not(test), allow(dead_code))]
        pub const ROUTER_BINDINGS: &[(LiveId, &str)] = &[
            $((live_id!($page), $route),)*
        ];

        script_mod! {
            use mod.prelude.widgets.*
            use mod.draw.KeyCode
            use mod.widgets.*

            mod.widgets.GalleryThemeToggleButton = View{
                width: 36
                height: 36
                flow: Overlay
                align: Align{x: 0.5, y: 0.5}

                button := mod.widgets.ShadButtonIconOutline{
                    width: Fill
                    height: Fill
                }
            }

            mod.widgets.GalleryThemeToggleSun = mod.widgets.GalleryThemeToggleButton{
                icon := IconSun{
                    width: 16
                    height: 16
                    icon_walk: Walk{width: 16, height: 16}
                    draw_icon.color: (shad_theme.color_primary)
                }
            }

            mod.widgets.GalleryThemeToggleMoon = mod.widgets.GalleryThemeToggleButton{
                icon := IconMoon{
                    width: 16
                    height: 16
                    icon_walk: Walk{width: 16, height: 16}
                    draw_icon.color: (shad_theme.color_primary)
                }
            }

            mod.widgets.GalleryCommandPaletteHeaderTrigger = View{
                width: Fit
                height: Fit
                flow: Right
                align: Align{y: 0.5}
                spacing: 0.0

                desktop_command_palette_trigger := ShadButtonGhost{text: "Search"}
            }

            mod.widgets.GalleryContentFlip = RouterWidget{
                width: Fill
                height: Fill
                default_route: @accordion_page
                not_found_route: @accordion_page
                sync_browser_url: true
                browser_base_path: "/makepad-components"

                $(
                    $page := RouterRoute{
                        route_pattern: $route
                        $(route_transition: @$transition)?
                        mod.widgets.$widget{}
                    }
                )*
            }

            mod.widgets.GalleryDesktopHeader = View{
                width: Fill
                height: Fit
                flow: Down

                header_bar := View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    padding: Inset{left: 24, right: 24, top: 16, bottom: 14}
                    spacing: 16.0
                    draw_bg.color: (shad_theme.color_background)

                    desktop_header_meta := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        desktop_header_caption := ShadSectionHeader{
                            text: "Makepad Components Gallery"
                        }

                        desktop_page_label := ShadLabel{
                            text: "Components"
                            draw_text.text_style.font_size: 13
                        }
                    }

                    mod.widgets.GalleryCommandPaletteHeaderTrigger{}

                    desktop_header_actions := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{x: 1.0, y: 0.5}
                        spacing: 8.0

                        desktop_theme_toggle_sun := mod.widgets.GalleryThemeToggleSun{}
                        desktop_theme_toggle_moon := mod.widgets.GalleryThemeToggleMoon{visible: false}
                    }
                }

                ShadSeparator{}
            }

            mod.widgets.GalleryMobileHeader = View{
                width: Fill
                height: Fit
                visible: false
                flow: Down

                header_bar := View{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 16, right: 16, top: 12, bottom: 12}
                    spacing: 10.0
                    draw_bg.color: (shad_theme.color_background)

                    header_top_row := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        spacing: 12.0

                        mobile_sidebar_menu_button := mod.widgets.GalleryMobileSidebarMenuButton{}

                        mobile_header_meta := View{
                            width: Fit
                            height: Fit
                            flow: Down
                            spacing: 2.0

                            mobile_header_caption := ShadSectionHeader{
                                text: "Gallery"
                            }

                            mobile_page_label := ShadLabel{
                                text: "Components"
                                draw_text.text_style.font_size: 12
                            }
                        }

                        View{
                            width: Fill
                            height: Fit
                        }

                        mobile_theme_toggle_sun := mod.widgets.GalleryThemeToggleSun{}
                        mobile_theme_toggle_moon := mod.widgets.GalleryThemeToggleMoon{visible: false}
                    }

                    mobile_command_palette_trigger := ShadButtonGhost{
                        width: Fill
                        text: "Search components"
                    }
                }

                ShadSeparator{}
            }

            mod.widgets.GalleryMainContent = View{
                width: Fill
                height: Fill
                flow: Down

                desktop_header := mod.widgets.GalleryDesktopHeader{}
                mobile_header := mod.widgets.GalleryMobileHeader{}
                content_flip := mod.widgets.GalleryContentFlip{}
            }

            mod.widgets.GalleryAppShell = View{
                width: Fill
                height: Fill
                flow: Right
                spacing: 0.0

                sidebar_shell := View{
                    width: 280
                    height: Fill
                    flow: Overlay
                    clip_x: true
                    clip_y: true

                    sidebar := mod.widgets.GallerySidebar{}
                }

                main_content := mod.widgets.GalleryMainContent{
                    width: Fill
                    height: Fill
                }
            }

            mod.widgets.GalleryAppUi = Root{
                main_window := Window{
                    window.inner_size: vec2(1400 900)
                    window.title: "Makepad Components Gallery"
                    pass +: { clear_color: (shad_theme.color_background) }
                    window_menu +: {
                        command_palette_menu := MenuItem.Item {
                            name: "Command Palette"
                            key: KeyCode.KeyK
                            enabled: true
                        }
                        view_menu := MenuItem.Sub {
                            name: "View"
                            items: [@zoom_in, @zoom_out, @line9, @command_palette_menu, @fullscreen]
                        }
                    }
                    body +: {
                        width: Fill
                        height: Fill
                        flow: Overlay
                        draw_bg.color: (shad_theme.color_background)

                        app_shell := mod.widgets.GalleryAppShell{}
                        command_palette := mod.widgets.GalleryCommandPalette{}
                    }
                }
            }
        }
    };
}

gallery_page_entries!(define_gallery_root);
