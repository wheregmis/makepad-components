use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySwitchPage,
    page: switch_page,
    title: "Switch",
    subtitle: "Switches are boolean page state with toggle styling. Read changed(actions), then push external state back with set_active(cx, bool).",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            email_alerts_switch := ShadSwitch{text: "Enable notifications"}
            ShadSwitch{text: "Dark mode"}
            ShadSwitch{text: "Use cellular data"}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Inline with label" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{y: 0.5}

            ShadSwitch{text: "Email alerts"}
            ShadSwitch{text: "SMS alerts"}
        }

        ShadHr{}
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. ShadSwitch is styled like a switch, but the runtime ref is the same boolean check-box/toggle family."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read changes with view.check_box(cx, ids!(email_alerts_switch)).changed(actions)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Store that boolean in page state or preferences, not in the app shell."}
        mod.widgets.GalleryActionFlowStep{text: "4. Restore the switch from external state with set_active(cx, bool), and inspect active(cx) when reconciling state."}
    },
}
