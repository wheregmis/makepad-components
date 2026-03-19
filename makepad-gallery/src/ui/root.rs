use crate::ui::registry::gallery_page_entries;
use makepad_components::makepad_widgets::*;

macro_rules! define_gallery_root {
    (@build
        [
            $($bindings:tt)*
        ]
        [
            $($routes:tt)*
        ]
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
            bundle: base,
            components: [$($components:ident),* $(,)?],
            icons: [$($icons:ident),* $(,)?],
            icon_policy: $icon_policy:ident,
            $(transition: $transition:ident,)?
        }
        $($rest:tt)*
    ) => {
        define_gallery_root!(
            @build
            [
                $($bindings)*
                (live_id!($page), $route),
            ]
            [
                $($routes)*
                $page := RouterRoute{
                    route_pattern: $route
                    $(route_transition: @$transition)?
                    mod.widgets.$widget{}
                }
            ]
            $($rest)*
        );
    };

    (@build
        [
            $($bindings:tt)*
        ]
        [
            $($routes:tt)*
        ]
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
            bundle: page,
            components: [$($components:ident),* $(,)?],
            icons: [$($icons:ident),* $(,)?],
            icon_policy: $icon_policy:ident,
            $(transition: $transition:ident,)?
        }
        $($rest:tt)*
    ) => {
        define_gallery_root!(
            @build
            [
                $($bindings)*
                (live_id!($page), $route),
            ]
            [
                $($routes)*
                $page := RouterRoute{
                    route_pattern: $route
                    route_bundle: #(stringify!($page))
                    $(route_transition: @$transition)?
                    mod.widgets.GalleryBundledPageHost{
                        page_id: @$page
                    }
                }
            ]
            $($rest)*
        );
    };

    (@build
        [
            $($bindings:tt)*
        ]
        [
            $($routes:tt)*
        ]
    ) => {
        #[cfg_attr(not(test), allow(dead_code))]
        pub const ROUTER_BINDINGS: &[(LiveId, &str)] = &[
            $($bindings)*
        ];

        script_mod! {
            use mod.prelude.widgets.*
            use mod.draw.KeyCode
            use mod.widgets.*

            mod.widgets.GalleryThemeToggleDarkButton = mod.widgets.IconButtonMoon{
                width: Fit
                height: 36
                spacing: 8.0
                padding: Inset{left: 12, right: 14, top: 0, bottom: 0}
                text: "Dark theme"
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                    border_color_hover: (shad_theme.color_outline_border_hover)
                    border_color_down: (shad_theme.color_outline_border_down)
                    border_color_focus: (shad_theme.color_outline_border_hover)
                    border_color_disabled: (shad_theme.color_disabled_border)
                }
                draw_icon.color: (shad_theme.color_primary)
                draw_text.color: (shad_theme.color_primary)
                draw_text.color_hover: (shad_theme.color_primary)
                draw_text.color_down: (shad_theme.color_primary)
                draw_text.color_focus: (shad_theme.color_primary)
                draw_text.color_disabled: (shad_theme.color_disabled_foreground)
                draw_text.text_style.font_size: 11
            }

            mod.widgets.GalleryThemeToggleLightButton = mod.widgets.IconButtonSun{
                width: Fit
                height: 36
                spacing: 8.0
                padding: Inset{left: 12, right: 14, top: 0, bottom: 0}
                text: "Light theme"
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                    border_color_hover: (shad_theme.color_outline_border_hover)
                    border_color_down: (shad_theme.color_outline_border_down)
                    border_color_focus: (shad_theme.color_outline_border_hover)
                    border_color_disabled: (shad_theme.color_disabled_border)
                }
                draw_icon.color: (shad_theme.color_primary)
                draw_text.color: (shad_theme.color_primary)
                draw_text.color_hover: (shad_theme.color_primary)
                draw_text.color_down: (shad_theme.color_primary)
                draw_text.color_focus: (shad_theme.color_primary)
                draw_text.color_disabled: (shad_theme.color_disabled_foreground)
                draw_text.text_style.font_size: 11
            }

            mod.widgets.GalleryMobileThemeToggleDarkButton = mod.widgets.GalleryThemeToggleDarkButton{
                width: 36
                height: 36
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                spacing: 0.0
                text: ""
            }

            mod.widgets.GalleryMobileThemeToggleLightButton = mod.widgets.GalleryThemeToggleLightButton{
                width: 36
                height: 36
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                spacing: 0.0
                text: ""
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

                ShadSectionHeader{
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                    text: "or"
                }

                ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
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

                $($routes)*
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

                    desktop_theme_toggle := View{
                        width: Fit
                        height: 36
                        flow: Overlay

                        desktop_theme_dark_button := mod.widgets.GalleryThemeToggleDarkButton{}
                        desktop_theme_light_button := mod.widgets.GalleryThemeToggleLightButton{
                            visible: false
                        }
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

                        View{
                            width: Fill
                            height: Fit
                        }

                        mobile_theme_toggle := View{
                            width: 36
                            height: 36
                            flow: Overlay

                            mobile_theme_dark_button := mod.widgets.GalleryMobileThemeToggleDarkButton{}
                            mobile_theme_light_button := mod.widgets.GalleryMobileThemeToggleLightButton{
                                visible: false
                            }
                        }
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

    ($($entries:tt)*) => {
        define_gallery_root!(@build [] [] $($entries)*);
    };
}

gallery_page_entries!(define_gallery_root);

#[cfg(test)]
mod tests {
    use super::*;
    use makepad_components::makepad_widgets::NoTrap;
    use std::collections::HashMap;

    fn bootstrap_gallery_vm(vm: &mut ScriptVm) {
        crate::ui::register_gallery_shell_widgets(vm);
        makepad_components::theme::script_mod(vm);
        script_eval!(vm, {
            mod.widgets.shad_theme = mod.widgets.shad_themes.dark
        });
        crate::ui::set_gallery_bundle_vm(vm);
        crate::ui::register_gallery_shell_dependencies(vm);
        makepad_router::script_mod(vm);
        crate::ui::script_mod(vm);
    }

    fn router_route_templates(vm: &mut ScriptVm) -> HashMap<LiveId, ScriptObject> {
        let template = script_eval!(vm, {
            mod.widgets.GalleryContentFlip{}
        });
        let template_obj = template.as_object().unwrap();
        let proto = vm.bx.heap.proto(template_obj).as_object().unwrap();
        let mut routes = HashMap::new();

        vm.vec_with(proto, |_vm, vec| {
            for kv in vec {
                let Some(route_id) = kv.key.as_id() else {
                    continue;
                };
                let Some(route_obj) = kv.value.as_object() else {
                    continue;
                };
                routes.insert(route_id, route_obj);
            }
        });

        routes
    }

    fn string_property(vm: &mut ScriptVm, obj: ScriptObject, prop: LiveId) -> Option<String> {
        vm.string_with(vm.bx.heap.value(obj, prop.into(), NoTrap), |_vm, value| {
            value.to_string()
        })
    }

    #[test]
    fn bundled_routes_emit_route_bundle_metadata() {
        let mut cx = Cx::new(Box::new(|_, _| {}));

        cx.with_vm(|vm| {
            bootstrap_gallery_vm(vm);
            let routes = router_route_templates(vm);

            assert_eq!(routes.len(), ROUTER_BINDINGS.len());

            let accordion_route = routes.get(&live_id!(accordion_page)).unwrap();
            assert_eq!(
                string_property(vm, *accordion_route, id!(route_pattern)),
                Some("/".to_string())
            );
            assert_eq!(
                string_property(vm, *accordion_route, id!(route_bundle)),
                None
            );

            let alert_route = routes.get(&live_id!(alert_page)).unwrap();
            assert_eq!(
                string_property(vm, *alert_route, id!(route_pattern)),
                Some("/alert".to_string())
            );
            assert_eq!(
                string_property(vm, *alert_route, id!(route_bundle)),
                Some("alert_page".to_string())
            );
        });
    }
}
