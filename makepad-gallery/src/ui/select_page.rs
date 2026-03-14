use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    root: ShadScrollArea,
    widget: GallerySelectPage,
    page: select_page,
    title: "Select",
    subtitle: "Select uses the dropdown ref API: read changed(actions) or changed_label(actions), then store the chosen index or label in page state.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 12.0

                    ShadSelect{labels: ["Pending" "In Progress" "Done"]}
                    ShadSelect{labels: ["Toronto" "Montreal" "Vancouver" "Calgary"]}
                }

                ShadFieldDescription{
                    text: "Known limitation: popup-style selects can still be unreliable inside the current gallery PageFlip shell. The splash app remains the best place to verify interaction."
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give the select an id, then get the dropdown ref with view.drop_down(cx, ids!(status_select))."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use changed(actions) when you want the selected index, or changed_label(actions) when the label is enough."}
        mod.widgets.GalleryActionFlowStep{text: "3. Persist the chosen item in page state, then restore it with set_selected_item(cx, ...) or set_selected_by_label(..., cx)."}
        mod.widgets.GalleryActionFlowStep{text: "4. The popup interaction stays inside the component; the page only reacts to the semantic selection result."}
    },
}
