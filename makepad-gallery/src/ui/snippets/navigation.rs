pub const BREADCRUMB_PREVIEW_CODE: &str = r#"// Default trail
ShadSurfaceMuted{
    width: Fill
    height: Fit
    flow: Down
    spacing: 10.0
    padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
    draw_bg +: {
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    ShadBreadcrumb{
        ShadBreadcrumbLink{ text: "Workspace" }
        ShadBreadcrumbSeparator{}
        ShadBreadcrumbLink{ text: "Settings" }
        ShadBreadcrumbSeparator{}
        ShadBreadcrumbPage{ text: "Billing" }
    }

    View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 4.0

        ShadLabel{
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 16
            text: "Billing Settings"
        }
        ShadFieldDescription{
            text: "Manage invoices, tax details, and workspace-level billing contacts."
        }
    }
}

// Collapsed / Ellipsis trail
ShadSurfaceMuted{
    width: Fill
    height: Fit
    flow: Down
    spacing: 10.0
    padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
    draw_bg +: {
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    ShadBreadcrumb{
        ShadBreadcrumbLink{ text: "Workspace" }
        ShadBreadcrumbSeparator{}
        ShadBreadcrumbEllipsis{}
        ShadBreadcrumbSeparator{}
        ShadBreadcrumbLink{ text: "Invoices" }
        ShadBreadcrumbSeparator{}
        ShadBreadcrumbPage{ text: "Archive" }
    }

    View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 4.0

        ShadLabel{
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 16
            text: "Archived Invoice #2048"
        }
        ShadFieldDescription{
            text: "Use ellipsis when the hierarchy is deep and the hidden ancestors do not need an interactive overflow control yet."
        }
    }
}"#;
pub const COMMAND_PALETTE_PREVIEW_CODE: &str = r#"mod.widgets.ShadButton{text: "Open Command Palette"}

// The gallery listens for Cmd/Ctrl + K globally.
//
// Page action flow:
// 1. The page-local trigger emits GalleryCommandPalettePageAction::OpenRequested.
// 2. The app shell listens to command_palette_page.open_requested(actions).
// 3. The shell opens the shared command palette overlay.
//
// This keeps page-local button clicks separate from shell-owned modal state."#;
pub const MENUBAR_PREVIEW_CODE: &str = r#"ShadCard{
    spacing: 14.0
    padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

    menubar_demo := ShadMenubar{
        file_menu := ShadMenubarMenu{
            trigger := ShadMenubarTrigger{text: "File"}
            content: ShadMenubarContent{
                ShadMenubarLabel{text: "Project"}
                file_new_btn := ShadMenubarItem{text: "New file"}
                file_open_btn := ShadMenubarItem{text: "Open recent"}
                ShadMenubarSeparator{}
                file_share_btn := ShadMenubarItem{text: "Share"}
            }
        }

        edit_menu := ShadMenubarMenu{
            trigger := ShadMenubarTrigger{text: "Edit"}
            content: ShadMenubarContent{
                ShadMenubarLabel{text: "History"}
                edit_undo_btn := ShadMenubarItem{text: "Undo"}
                edit_redo_btn := ShadMenubarItem{text: "Redo"}
                ShadMenubarSeparator{}
                edit_find_btn := ShadMenubarItem{text: "Find in files"}
            }
        }

        view_menu := ShadMenubarMenu{
            trigger := ShadMenubarTrigger{text: "View"}
            content: ShadMenubarContent{
                ShadMenubarLabel{text: "Workspace"}
                view_toggle_sidebar_btn := ShadMenubarItem{text: "Toggle sidebar"}
                view_zen_mode_btn := ShadMenubarItem{text: "Enter zen mode"}
            }
        }
    }

    menubar_status := ShadFieldDescription{
        text: "Choose a menubar action."
    }
}

// Controller example (Rust):
// use makepad_components::popover::ShadPopoverWidgetExt;
//
// let file_menu = self.view.shad_popover(cx, ids!(file_menu));
// let file_content = file_menu.content_widget();
//
// if file_content.button(cx, ids!(file_new_btn)).clicked(actions) {
//     file_menu.close(cx);
//     self.view.label(cx, ids!(menubar_status)).set_text(cx, "Selected File -> New file");
// }
//
// // Close other menus when one opens (mutual-exclusion pattern):
// if matches!(file_menu.open_changed(actions), Some(true)) {
//     self.view.shad_popover(cx, ids!(edit_menu)).close(cx);
//     self.view.shad_popover(cx, ids!(view_menu)).close(cx);
// }"#;
pub const NAVIGATION_MENU_PREVIEW_CODE: &str = r#"ShadCard{
    spacing: 14.0
    padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

    navigation_menu_demo := ShadNavigationMenu{
        navigation_list := ShadNavigationMenuList{
            products_menu := ShadNavigationMenuItem{
                trigger := ShadNavigationMenuTrigger{text: "Products"}

                content: ShadNavigationMenuContent{
                    menu_row := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 12.0

                        ShadNavigationMenuCallout{
                            ShadSectionHeader{text: "Ship faster"}
                            ShadFieldDescription{text: "Launch with billing, auth, and analytics primitives that already fit the design system."}
                            products_trial_btn := ShadButton{text: "Start trial"}
                        }

                        View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 10.0

                            sdk_panel := ShadNavigationMenuPanel{
                                products_sdk_btn := ShadButtonGhost{
                                    width: Fill
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "SDKs"
                                }
                                ShadFieldDescription{text: "Type-safe clients for web, desktop, and native apps."}
                            }

                            dashboard_panel := ShadNavigationMenuPanel{
                                products_dashboard_btn := ShadButtonGhost{
                                    width: Fill
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Dashboards"
                                }
                                ShadFieldDescription{text: "Operational views for metrics, queues, and release health."}
                            }
                        }
                    }
                }
            }

            resources_menu := ShadNavigationMenuItem{
                trigger := ShadNavigationMenuTrigger{text: "Resources"}

                content: ShadNavigationMenuContent{
                    ShadNavigationMenuSectionLabel{text: "Learn"}
                    guides_btn := ShadButtonGhost{
                        width: Fill
                        align: Align{x: 0.0, y: 0.5}
                        text: "Guides"
                    }
                    ShadFieldDescription{text: "Architecture notes, implementation walkthroughs, and migration recipes."}
                    examples_btn := ShadButtonGhost{
                        width: Fill
                        align: Align{x: 0.0, y: 0.5}
                        text: "Examples"
                    }
                    ShadFieldDescription{text: "Reference compositions for onboarding, billing, and analytics flows."}
                }
            }

            pricing_link := ShadButtonGhost{
                text: "Pricing"
            }
        }
    }

    navigation_selection_status := ShadFieldDescription{
        text: "Selected destination: none yet."
    }
}

// Controller example (Rust):
// use makepad_components::popover::ShadPopoverWidgetExt;
//
// let products = self.view.shad_popover(cx, ids!(products_menu));
// let products_content = products.content_widget();
//
// if products_content.button(cx, ids!(products_trial_btn)).clicked(actions) {
//     products.close(cx);
//     self.view.label(cx, ids!(navigation_selection_status))
//         .set_text(cx, "Selected destination: Start trial");
// }
//
// if self.view.button(cx, ids!(pricing_link)).clicked(actions) {
//     self.view.label(cx, ids!(navigation_selection_status))
//         .set_text(cx, "Selected destination: Pricing");
// }
//
// // Mutual-exclusion: close other menu when one opens
// if matches!(products.open_changed(actions), Some(true)) {
//     self.view.shad_popover(cx, ids!(resources_menu)).close(cx);
// }"#;
pub const PAGINATION_PREVIEW_CODE: &str = r#"// Controlled pagination
pagination_demo := ShadPagination{
    current_page: 5
    page_count: 12
}

View{
    width: Fit
    height: Fit
    flow: Right
    spacing: 8.0

    prev_external_btn := ShadButtonOutline{
        text: "Previous"
    }

    next_external_btn := ShadButtonOutline{
        text: "Next"
    }

    jump_last_btn := ShadButtonGhost{
        text: "Jump to last"
    }
}

pagination_status := ShadFieldDescription{
    text: "Current page: 5 of 12"
}

// Compact range
pagination_compact := ShadPagination{
    current_page: 21
    page_count: 42
    max_visible_pages: 5
}

pagination_compact_status := ShadFieldDescription{
    text: "Current page: 21 of 42"
}

// Controller example (Rust):
// let pagination = self.view.shad_pagination(cx, ids!(pagination_demo));
//
// if self.view.button(cx, ids!(prev_external_btn)).clicked(actions) {
//     pagination.prev(cx);
//     self.sync_ui(cx);
// }
//
// if self.view.button(cx, ids!(next_external_btn)).clicked(actions) {
//     pagination.next(cx);
//     self.sync_ui(cx);
// }
//
// if self.view.button(cx, ids!(jump_last_btn)).clicked(actions) {
//     pagination.set_page(cx, pagination.page_count());
//     self.sync_ui(cx);
// }
//
// if pagination.changed(actions).is_some() {
//     self.sync_ui(cx);
// }
//
// fn sync_ui(&self, cx: &mut Cx) {
//     let page = self.view.shad_pagination(cx, ids!(pagination_demo)).page();
//     let page_count = self.view.shad_pagination(cx, ids!(pagination_demo)).page_count();
//     self.view.label(cx, ids!(pagination_status))
//         .set_text(cx, &format!("Current page: {page} of {page_count}"));
// }"#;
pub const SIDEBAR_PREVIEW_CODE: &str = r#"AdaptiveView{
    Desktop := View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 12.0
        align: Align{y: 0.0}

        ShadSidebar{
            width: 280
            height: 320
            ShadLabel{
                text: "Acme Inc"
                draw_text.text_style.font_size: 12
            }
            ShadSidebarSectionLabel{text: "Platform"}
            nav_playground := ShadSidebarItem{text: "Playground"}
            ShadSidebarItem{text: "History"}
            ShadSidebarItem{text: "Settings"}
        }

        View{
            width: Fill
            height: 320
            draw_bg.color: #0000
            draw_bg.border_size: 1.0
            draw_bg.border_color: (shad_theme.color_outline_border)
            draw_bg.border_radius: (shad_theme.radius)
        }
    }

    Mobile := View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 12.0

        ShadSidebar{
            width: Fill
            height: Fit
            ShadLabel{
                text: "Acme Inc"
                draw_text.text_style.font_size: 12
            }
            ShadSidebarSectionLabel{text: "Platform"}
            ShadSidebarItem{text: "Playground"}
            ShadSidebarItem{text: "History"}
            ShadSidebarItem{text: "Settings"}
        }

        View{
            width: Fill
            height: 160
            draw_bg.color: #0000
            draw_bg.border_size: 1.0
            draw_bg.border_color: (shad_theme.color_outline_border)
            draw_bg.border_radius: (shad_theme.radius)
        }
    }
}

// Controller example (Rust):
// if self.ui.button(cx, ids!(nav_playground)).clicked(actions) {
//     self.router.go_to_route(cx, live_id!(playground));
// }
//
// Sidebar items are button actions with navigation styling. Keep the selected
// route or active section in page/app state, then render the matching item as
// active from that state."#;
pub const TABS_PREVIEW_CODE: &str = r#"tabs_preview_shell := ShadSurface{
    width: 380
    flow: Down
    spacing: 14.0
    padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

    ShadTabs{
        tabs_row := ShadTabsList{
            overview_group := View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 4.0

                tabs_overview_trigger := ShadTabsTrigger{text: "Overview & Activity"}
                tabs_overview_indicator := ShadTabsIndicator{}
            }

            usage_group := View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 4.0

                tabs_usage_trigger := ShadTabsTrigger{text: "Implementation Notes"}
                tabs_usage_indicator := ShadTabsIndicator{
                    visible: false
                }
            }

            settings_group := View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 4.0

                tabs_settings_trigger := ShadTabsTrigger{text: "Settings & Access"}
                tabs_settings_indicator := ShadTabsIndicator{
                    visible: false
                }
            }
        }

        tabs_content_flip := mod.widgets.RouterWidget{
            width: Fill
            height: Fit
            default_route: @overview_page
            not_found_route: @overview_page

            overview_page := mod.widgets.RouterRoute{
                route_pattern: "/"
                ShadSectionHeader{text: "Overview"}
                ShadFieldDescription{text: "Keep related content grouped, even when the trigger row overflows on mobile or inside compact panels."}
            }

            usage_page := mod.widgets.RouterRoute{
                route_pattern: "/usage"
                ShadSectionHeader{text: "Usage"}
                ShadFieldDescription{text: "Pair ShadTabsTrigger with PageFlip, RouterWidget, or any app-owned state holder."}
            }

            settings_page := mod.widgets.RouterRoute{
                route_pattern: "/settings"
                ShadSectionHeader{text: "Settings"}
                ShadFieldDescription{text: "The compact shell forces horizontal overflow so tab-row scrolling stays visible in the demo."}
            }
        }
    }
}

// Controller example (Rust):
// use makepad_router::widget::RouterWidgetWidgetExt;
//
// fn set_selected_tab(&mut self, cx: &mut Cx, page: LiveId) {
//     self.view.router_widget(cx, ids!(tabs_content_flip)).go_to_route(cx, page);
//     self.view.view(cx, ids!(tabs_overview_indicator)).set_visible(cx, page == live_id!(overview_page));
//     self.view.view(cx, ids!(tabs_usage_indicator)).set_visible(cx, page == live_id!(usage_page));
//     self.view.view(cx, ids!(tabs_settings_indicator)).set_visible(cx, page == live_id!(settings_page));
// }
//
// // In handle_event:
// if self.view.button(cx, ids!(tabs_overview_trigger)).clicked(actions) {
//     self.set_selected_tab(cx, live_id!(overview_page));
// }
// if self.view.button(cx, ids!(tabs_usage_trigger)).clicked(actions) {
//     self.set_selected_tab(cx, live_id!(usage_page));
// }"#;
