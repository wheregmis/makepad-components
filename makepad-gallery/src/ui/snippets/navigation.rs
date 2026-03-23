pub const BREADCRUMB_PREVIEW_CODE: &str = r#"// Keep the breadcrumb close to the page title it describes.
ShadSurfaceMuted{
    width: 360
    height: Fit
    flow: Down
    spacing: 10.0
    padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
    draw_bg +: {
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    mod.widgets.ShadBreadcrumb{
        mod.widgets.ShadBreadcrumbLink{ text: "Workspace" }
        mod.widgets.ShadBreadcrumbSeparator{}
        mod.widgets.ShadBreadcrumbLink{ text: "Settings" }
        mod.widgets.ShadBreadcrumbSeparator{}
        mod.widgets.ShadBreadcrumbPage{ text: "Billing" }
    }

    View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 4.0

        mod.widgets.ShadLabel{
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 16
            text: "Billing Settings"
        }
        mod.widgets.ShadFieldDescription{
            text: "Manage invoices, tax details, and workspace-level billing contacts."
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
pub const MENUBAR_PREVIEW_CODE: &str = r#"app_menubar := ShadMenubar{
    file_menu := ShadMenubarMenu{
        trigger := ShadMenubarTrigger{text: "File"}
        content: ShadMenubarContent{
            file_new_btn := ShadMenubarItem{text: "New file"}
            file_open_btn := ShadMenubarItem{text: "Open recent"}
            ShadMenubarSeparator{}
            file_share_btn := ShadMenubarItem{text: "Share"}
        }
    }

    edit_menu := ShadMenubarMenu{
        trigger := ShadMenubarTrigger{text: "Edit"}
        content: ShadMenubarContent{
            edit_undo_btn := ShadMenubarItem{text: "Undo"}
            edit_redo_btn := ShadMenubarItem{text: "Redo"}
        }
    }
}"#;
pub const NAVIGATION_MENU_PREVIEW_CODE: &str = r#"site_nav := ShadNavigationMenu{
    navigation_list := ShadNavigationMenuList{
        products_menu := ShadNavigationMenuItem{
            trigger := ShadNavigationMenuTrigger{text: "Products"}
            content: ShadNavigationMenuContent{
                products_trial_btn := ShadButton{text: "Start trial"}
                products_sdk_btn := ShadButtonGhost{
                    width: Fill
                    align: Align{x: 0.0, y: 0.5}
                    text: "SDKs"
                }
            }
        }

        resources_menu := ShadNavigationMenuItem{
            trigger := ShadNavigationMenuTrigger{text: "Resources"}
            content: ShadNavigationMenuContent{
                guides_btn := ShadButtonGhost{
                    width: Fill
                    align: Align{x: 0.0, y: 0.5}
                    text: "Guides"
                }
            }
        }
    }
}"#;
pub const PAGINATION_PREVIEW_CODE: &str = r#"projects_pagination := ShadPagination{
    current_page: 5
    page_count: 12
}"#;
pub const SIDEBAR_PREVIEW_CODE: &str = r#"mod.widgets.ShadSidebar{
    nav_playground := ShadSidebarItem{text: "Playground"}
    nav_history := ShadSidebarItem{text: "History"}
    nav_settings := ShadSidebarItem{text: "Settings"}
}"#;
pub const TABS_PREVIEW_CODE: &str = r#"ShadTabs{
    ShadTabsList{
        ShadTabsTrigger{text: "Overview"}
        ShadTabsTrigger{text: "Usage"}
        ShadTabsTrigger{text: "Settings"}
    }
    ShadTabsContent{
        ShadLabel{text: "Switch content in app code with PageFlip or another state holder."}
    }
}

// Page-controller example (Rust):
// if self.ui.button(cx, ids!(tabs_usage_trigger)).clicked(actions) {
//     self.set_selected_tab(cx, live_id!(usage));
// }
//
// fn set_selected_tab(&mut self, cx: &mut Cx, page: LiveId) {
//     self.view.router_widget(cx, ids!(tabs_content_flip)).go_to_route(cx, page);
//     // Also update the active indicator visibility here.
// }"#;
