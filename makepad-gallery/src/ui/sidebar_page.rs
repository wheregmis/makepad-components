use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySidebarPage,
    page: sidebar_page,
    title: "Sidebar",
    subtitle: "Sidebar primitives are navigation-flavored button actions. Name the items you care about, then route or swap page state from their clicks.",
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
                ShadSidebarItem{text: "Playground"}
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
        mod.widgets.GalleryActionFlowStep{text: "1. Treat each ShadSidebarItem like a named button action with sidebar styling."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read clicks with ui.button(cx, ids!(nav_playground)).clicked(actions), then route or swap the selected page."}
        mod.widgets.GalleryActionFlowStep{text: "3. Keep the active route in page or app state, not inside the sidebar primitive."}
        mod.widgets.GalleryActionFlowStep{text: "4. Render the active item from that route state so sidebar, router, and content stay in sync."}
    },
}
