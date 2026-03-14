use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryCheckboxPage,
    page: checkbox_page,
    title: "Checkbox",
    subtitle: "Shadcn-inspired checkbox component from makepad-components library. Use ShadCheckboxRef::changed(actions) or is_checked() when syncing form state.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadCheckbox{label: "Accept terms and conditions"}
            ShadCheckbox{label: "Pre-checked option" checked: true}
            ShadCheckbox{label: "Subscribe to newsletter"}
        }

        ShadHr{}

        ShadSectionHeader{ text: "In a form row" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{y: 0.5}

            ShadCheckbox{label: "Option A"}
            ShadCheckbox{label: "Option B" checked: true}
            ShadCheckbox{label: "Option C"}
        }

        ShadHr{}
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep each checked value in page or form state, even when the visual checkbox looks self-contained."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read changed(actions) from ShadCheckboxRef to capture the user's latest choice."}
        mod.widgets.GalleryActionFlowStep{text: "3. Call set_checked(cx, checked, animator::Animate::No) when loading saved data or resetting a form."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use is_checked() when submitting or validating without waiting for a fresh action event."}
    },
}
