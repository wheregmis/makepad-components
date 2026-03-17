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

            mod.widgets.GalleryThemeToggle = ButtonFlatIcon{
                width: 36
                height: 36
                icon_walk: Walk{width: 16, height: 16}
                draw_icon.svg: crate_resource("self://resources/icons/sun-moon.svg")
                draw_icon.color: (shad_theme.color_primary)

                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                    border_color_hover: (shad_theme.color_outline_border_hover)
                    border_color_down: (shad_theme.color_outline_border_down)
                    border_color_focus: (shad_theme.color_outline_border_hover)
                }
            }

            mod.widgets.GalleryCommandPaletteHeaderTrigger = View{
                width: Fit
                height: Fit
                flow: Right
                align: Align{y: 0.5}
                spacing: 8.0

                desktop_command_palette_trigger := ShadButtonOutline{text: "Search components"}

                ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
                ShadKbdSeparator{}
                ShadKbd{ label := ShadKbdLabel{text: "K"} }
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
                        width: Fit
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

                    View{
                        width: Fill
                        height: Fit
                    }

                    desktop_theme_toggle := mod.widgets.GalleryThemeToggle{}
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
                    flow: Right
                    align: Align{y: 0.5}
                    padding: Inset{left: 16, right: 16, top: 12, bottom: 12}
                    spacing: 12.0
                    draw_bg.color: (shad_theme.color_background)

                    mobile_sidebar_toggle := View{
                        width: 36
                        height: 36
                        flow: Overlay

                        mobile_sidebar_menu_button := IconButtonMenu{
                            width: 36
                            height: 36
                            draw_bg +: {
                                color: #0000
                                color_hover: (shad_theme.color_ghost_hover)
                                color_down: (shad_theme.color_ghost_down)
                                color_focus: (shad_theme.color_ghost_hover)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                                border_color: #0000
                            }
                            draw_icon.color: (shad_theme.color_primary)
                        }

                        mobile_sidebar_close_button := IconButtonX{
                            visible: false
                            width: 36
                            height: 36
                            draw_bg +: {
                                color: #0000
                                color_hover: (shad_theme.color_ghost_hover)
                                color_down: (shad_theme.color_ghost_down)
                                color_focus: (shad_theme.color_ghost_hover)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                                border_color: #0000
                            }
                            draw_icon.color: (shad_theme.color_primary)
                        }
                    }

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

                    mobile_command_palette_trigger := ShadButtonGhost{
                        text: "Search"
                    }

                    View{
                        width: Fill
                        height: Fit
                    }

                    mobile_theme_toggle := mod.widgets.GalleryThemeToggle{}
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
                sidebar := mod.widgets.GallerySidebar{}
                main_content := mod.widgets.GalleryMainContent{}
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
