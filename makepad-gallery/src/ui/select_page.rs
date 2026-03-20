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
                spacing: 16.0

                ShadFieldDescription{
                    width: Fill
                    text: "Use explicit field stacks on mobile so each trigger gets the full row width instead of competing inside a tight horizontal layout."
                }

                ShadField{
                    width: Fill
                    ShadFieldLabel{text: "Status"}
                    status_select := ShadSelect{
                        width: Fill
                        labels: ["Pending" "In Progress" "Done"]
                    }
                    ShadFieldDescription{
                        width: Fill
                        text: "Good for short state lists where the surrounding page owns the selected index or label."
                    }
                }

                ShadField{
                    width: Fill
                    ShadFieldLabel{text: "City"}
                    city_select := ShadSelect{
                        width: Fill
                        labels: ["Toronto" "Montreal" "Vancouver" "Calgary"]
                    }
                    ShadFieldDescription{
                        width: Fill
                        text: "Treat the select like any other form control: let the container decide width, then keep helper copy below it."
                    }
                }

                ShadFieldDescription{
                    width: Fill
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
