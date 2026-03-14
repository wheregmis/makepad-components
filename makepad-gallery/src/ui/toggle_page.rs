use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryTogglePage,
    page: toggle_page,
    title: "Toggle",
    subtitle: "Pressed-state buttons for formatting, filtering, and grouped controls. Keep grouped selection in the page/controller and update each item's active state from that source of truth.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Standalone toggles" }
        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            ShadToggle{text: "Bold"}
            ShadToggle{text: "Italic" active: true}
            ShadToggle{text: "Underline"}
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
                ShadToggleGroupItemSm{text: "Sm"}
                ShadToggleGroupItemSm{text: "Active" active: true}
            }

            ShadToggleGroup{
                ShadToggleGroupItem{text: "Default"}
                ShadToggleGroupItem{text: "Active" active: true}
            }

            ShadToggleGroup{
                ShadToggleGroupItemLg{text: "Large"}
                ShadToggleGroupItemLg{text: "Active" active: true}
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Treat pressed and selected values as page/controller state, especially for grouped toggles."}
        mod.widgets.GalleryActionFlowStep{text: "2. On user click, update that state in the page rather than trying to manage each toggle from the app shell."}
        mod.widgets.GalleryActionFlowStep{text: "3. Re-render the matching toggle or toggle-group item with active: true from the current state."}
        mod.widgets.GalleryActionFlowStep{text: "4. This is the same controller pattern used by the Tabs page: local state in the page, visuals derived from that state."}
    },
}
