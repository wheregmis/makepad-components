use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    root: ShadScrollArea,
    widget: GalleryRadioGroupPage,
    page: radio_group_page,
    title: "Radio Group",
    subtitle: "Radio groups are page-owned single-choice state: use RadioButtonSet::selected(cx, actions) to map clicks back into a domain value.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Stacked options" }
        ShadPanel{
            ShadRadioGroup{
                ShadRadioItem{text: "Starter"}
                ShadRadioItem{text: "Pro"}
                ShadRadioItem{text: "Enterprise"}
            }
        }

        ShadSectionHeader{ text: "Inline options" }
        ShadPanel{
            ShadRadioGroupInline{
                ShadRadioItem{text: "Weekly"}
                ShadRadioItem{text: "Monthly"}
                ShadRadioItem{text: "Yearly"}
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Name the individual radio items that belong to one logical group."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read the selected index with view.radio_button_set(ids!(starter_plan, pro_plan, enterprise_plan)).selected(cx, actions)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Convert that index into your domain enum or model value in the page controller."}
        mod.widgets.GalleryActionFlowStep{text: "4. When restoring state, call set_active(cx, ...) on the matching item so the UI reflects the domain value again."}
    },
}
