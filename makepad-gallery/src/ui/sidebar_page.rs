use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySidebarPage,
    page: sidebar_page,
    title: "Sidebar",
    subtitle: "Sidebar items expose an optional active state so route-driven navigation can stay in the component instead of script-level color patching.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        View{
            width: Fill
            height: Fit
            flow: Down
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
                ShadSidebarItem{text: "Playground" active: true}
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
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Treat each ShadSidebarItem like a named button action with sidebar styling and optional active state."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read clicks with ui.shad_sidebar_item(cx, ids!(nav_playground)).clicked(actions), then route or swap the selected page."}
        mod.widgets.GalleryActionFlowStep{text: "3. Keep the active route in page or app state, then call view.shad_sidebar_item(cx, ids!(nav_playground)).set_active(cx, true) for the matching item."}
        mod.widgets.GalleryActionFlowStep{text: "4. That route-driven active state keeps sidebar, router, and content in sync without manual color overrides."}
    },
}
