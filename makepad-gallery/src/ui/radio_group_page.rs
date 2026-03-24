use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    root: ShadScrollArea,
    widget: GalleryRadioGroupPage,
    page: radio_group_page,
    title: "Radio Group",
    subtitle: "Radio groups expose a typed `ShadRadioGroupRef` so single-choice state no longer has to reach down to `RadioButtonSet` directly.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Stacked options" }
        ShadPanel{
            ShadRadioGroup{
                starter_plan := ShadRadioItem{text: "Starter"}
                pro_plan := ShadRadioItem{text: "Pro"}
                enterprise_plan := ShadRadioItem{text: "Enterprise"}
            }
        }

        ShadSectionHeader{ text: "Inline options" }
        ShadPanel{
            ShadRadioGroupInline{
                weekly_interval := ShadRadioItem{text: "Weekly"}
                monthly_interval := ShadRadioItem{text: "Monthly"}
                yearly_interval := ShadRadioItem{text: "Yearly"}
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Name the individual radio items that belong to one logical group."}
        mod.widgets.GalleryActionFlowStep{text: "2. Build a typed group with view.shad_radio_group(cx, [ids!(starter_plan), ids!(pro_plan), ids!(enterprise_plan)])."}
        mod.widgets.GalleryActionFlowStep{text: "3. Read the selected index with group.selected(cx, actions) and convert it into your domain enum or model value."}
        mod.widgets.GalleryActionFlowStep{text: "4. When restoring state, call group.set_selected(cx, Some(index)) so the UI reflects the domain value again."}
    },
}
