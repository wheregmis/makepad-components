pub const BREADCRUMB_PREVIEW_CODE: &str = "mod.widgets.ShadBreadcrumb{\n    mod.widgets.ShadBreadcrumbLink{ text: \"Home\" }\n    mod.widgets.ShadBreadcrumbSeparator{}\n    mod.widgets.ShadBreadcrumbLink{ text: \"Components\" }\n    mod.widgets.ShadBreadcrumbSeparator{}\n    mod.widgets.ShadBreadcrumbPage{ text: \"Breadcrumb\" }\n}";
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
}

// Controller example (Rust):
// let file_menu = self.view.shad_popover(cx, ids!(file_menu));
// let file_content = file_menu.content_widget();
//
// if file_content.button(cx, ids!(file_new_btn)).clicked(actions) {
//     file_menu.close(cx);
//     self.status = "Selected File -> New file".to_string();
// }"#;
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
}

// Controller example (Rust):
// let products = self.view.shad_popover(cx, ids!(products_menu));
// let content = products.content_widget();
//
// if content.button(cx, ids!(products_sdk_btn)).clicked(actions) {
//     products.close(cx);
//     self.selected_destination = "SDKs".to_string();
// }"#;
pub const PAGINATION_PREVIEW_CODE: &str = r#"projects_pagination := ShadPagination{
    current_page: 5
    page_count: 12
}

// Controller example (Rust):
// let pagination = self.view.shad_pagination(cx, ids!(projects_pagination));
//
// if let Some(page) = pagination.changed(actions) {
//     self.current_page = page;
//     self.reload_rows_for(page);
// }
//
// if self.view.button(cx, ids!(next_page_btn)).clicked(actions) {
//     pagination.next(cx);
// }
//
// pagination.set_page(cx, 1);
// let active_page = pagination.page();
// let total_pages = pagination.page_count();"#;
pub const SIDEBAR_PREVIEW_CODE: &str = r#"mod.widgets.ShadSidebar{
    nav_playground := ShadSidebarItem{text: "Playground"}
    nav_history := ShadSidebarItem{text: "History"}
    nav_settings := ShadSidebarItem{text: "Settings"}
}

// Controller example (Rust):
// if self.ui.button(cx, ids!(nav_playground)).clicked(actions) {
//     self.router.go_to_route(cx, live_id!(playground));
// }
//
// Sidebar items are button actions with navigation styling. Keep the selected
// route or active section in page/app state, then render the matching item as
// active from that state."#;
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
