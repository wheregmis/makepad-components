use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryTogglePage,
    page: toggle_page,
    title: "Toggle",
    subtitle: "Pressed-state buttons for formatting, filtering, and grouped controls. Use `size: ShadControlSize.*` for scale, then keep grouped selection in the page/controller as the source of truth.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Standalone toggles" }
        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            ShadToggle{size: ShadControlSize.Small text: "Bold"}
            ShadToggle{text: "Italic" active: true}
            ShadToggle{size: ShadControlSize.Large text: "Underline"}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Toggle group" }
        ShadToggleGroup{
            ShadToggleGroupItem{text: "Left"}
            ShadToggleGroupItem{text: "Center" active: true}
            ShadToggleGroupItem{text: "Right"}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Sizes" }
        View{
            width: Fit
            height: Fit
            flow: Down
            spacing: 12.0

            ShadToggleGroup{
                ShadToggleGroupItem{size: ShadControlSize.Small text: "Sm"}
                ShadToggleGroupItem{size: ShadControlSize.Small text: "Active" active: true}
            }

            ShadToggleGroup{
                ShadToggleGroupItem{text: "Default"}
                ShadToggleGroupItem{text: "Active" active: true}
            }

            ShadToggleGroup{
                ShadToggleGroupItem{size: ShadControlSize.Large text: "Large"}
                ShadToggleGroupItem{size: ShadControlSize.Large text: "Active" active: true}
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Treat pressed and selected values as page/controller state, especially for grouped toggles."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use size: ShadControlSize.Small, Default, or Large on ShadToggle / ShadToggleGroupItem instead of size-specific alias widgets."}
        mod.widgets.GalleryActionFlowStep{text: "3. On user click, update that state in the page rather than trying to manage each toggle from the app shell."}
        mod.widgets.GalleryActionFlowStep{text: "4. Re-render the matching toggle or toggle-group item with active: true from the current state."}
        mod.widgets.GalleryActionFlowStep{text: "5. This is the same controller pattern used by the Tabs page: local state in the page, visuals derived from that state."}
    },
}
