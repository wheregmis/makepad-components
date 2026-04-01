use crate::ui::registry::gallery_page_entries;
use makepad_components::makepad_widgets::*;

#[allow(unused_macros)]
macro_rules! gallery_route_transition {
    (
        sidebar_id: $sidebar_id:ident,
        sidebar_label: $sidebar_label:literal,
        section: $section:literal,
        shortcut: $shortcut:literal,
        snippet: $snippet:ident,
        transition: $transition:ident,
    ) => {
        route_transition: @$transition
    };
    (
        sidebar_id: $sidebar_id:ident,
        sidebar_label: $sidebar_label:literal,
        section: $section:literal,
        shortcut: $shortcut:literal,
        snippet: $snippet:ident,
    ) => {};
}

macro_rules! define_gallery_root {
    (
        $(
            {
                title: $title:literal,
                route: $route:literal,
                page: $page:ident,
                widget: $widget:ident,
                $($rest:tt)*
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

            mod.widgets.GalleryThemeToggle = ShadButton{
                variant: ShadButtonVariant.Outline
                width: Fit
                height: 36
                text: "Light theme"
            }

            mod.widgets.GalleryGithubButton = ShadButton{
                variant: ShadButtonVariant.Ghost
                width: Fit
                height: 36
                text: "GitHub"
            }

            mod.widgets.GalleryMobileThemeToggle = ShadButton{
                variant: ShadButtonVariant.Outline
                width: Fit
                height: 36
                padding: Inset{left: 10, right: 12, top: 0, bottom: 0}
                spacing: 6.0
                text: "Theme"
            }

            mod.widgets.GalleryMobileGithubButton = ShadButton{
                variant: ShadButtonVariant.Ghost
                width: Fit
                height: 36
                padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
                text: "GitHub"
            }

            mod.widgets.GalleryCommandPaletteHeaderTrigger = View{
                width: Fit
                height: Fit
                flow: Right
                align: Align{y: 0.5}
                spacing: 8.0

                command_palette_trigger := ShadButton{
                    variant: ShadButtonVariant.Outline
                    text: "Search components"
                }

                ShadShortcut{
                    ShadKbd{ label := ShadKbdLabel{text: "Cmd/Ctrl"} }
                    ShadKbdSeparator{}
                    ShadKbd{ label := ShadKbdLabel{text: "K"} }
                }
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
                        gallery_route_transition! { $($rest)* }
                        mod.widgets.$widget{}
                    }
                )*
            }

            mod.widgets.GalleryHeader = AdaptiveView{
                width: Fill
                height: Fit

                Desktop := View{
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

                        header_left := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{y: 0.5}

                            header_meta := View{
                                width: Fit
                                height: Fit
                                flow: Down
                                spacing: 4.0

                                header_caption := ShadSectionHeader{
                                    text: "Makepad Components Gallery"
                                }

                                page_label := ShadLabel{
                                    text: "Components"
                                    draw_text.text_style.font_size: 13
                                }
                            }
                        }

                        header_center := View{
                            width: Fit
                            height: Fit
                            mod.widgets.GalleryCommandPaletteHeaderTrigger{}
                        }

                        header_right := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{x: 1.0, y: 0.5}
                            spacing: 8.0

                            github_button := mod.widgets.GalleryGithubButton{}
                            theme_toggle := mod.widgets.GalleryThemeToggle{}
                        }
                    }

                    ShadSeparator{}
                }

                Mobile := View{
                    width: Fill
                    height: Fit
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

                            header_meta := View{
                                width: Fit
                                height: Fit
                                flow: Down
                                spacing: 2.0

                                header_caption := ShadSectionHeader{
                                    text: "Gallery"
                                }

                                page_label := ShadLabel{
                                    text: "Components"
                                    draw_text.text_style.font_size: 12
                                }
                            }

                            View{
                                width: Fill
                                height: Fit
                            }

                            theme_toggle := mod.widgets.GalleryMobileThemeToggle{}
                        }

                        header_actions_row := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{y: 0.5}
                            spacing: 8.0

                            command_palette_trigger := ShadButton{
                                variant: ShadButtonVariant.Ghost
                                width: Fill
                                text: "Search components"
                            }

                            github_button := mod.widgets.GalleryMobileGithubButton{}
                        }
                    }

                    ShadSeparator{}
                }
            }

            mod.widgets.GalleryMainContent = View{
                width: Fill
                height: Fill
                flow: Down

                header := mod.widgets.GalleryHeader{}
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
