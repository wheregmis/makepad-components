use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySwitchPage,
    page: switch_page,
    title: "Switch",
    subtitle: "Switches expose a typed `ShadSwitchRef` for boolean page state and support `ShadControlSize.Small/Default/Large`. Read changed(actions), then push external state back with set_active(cx, bool).",
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

        ShadSectionHeader{ text: "Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadSwitch{
                size: ShadControlSize.Small
                text: "Small switch"
            }
            ShadSwitch{
                text: "Default switch"
            }
            ShadSwitch{
                size: ShadControlSize.Large
                text: "Large switch"
            }
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
        mod.widgets.GalleryActionFlowStep{text: "1. ShadSwitch is styled like a switch and exposes a typed `ShadSwitchRef` on top of the toggle/check-box runtime."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read changes with view.shad_switch(cx, ids!(email_alerts_switch)).changed(actions)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Set `size: ShadControlSize.Small/Default/Large` declaratively, or call set_size(cx, size) when page state changes the density."}
        mod.widgets.GalleryActionFlowStep{text: "4. Store the boolean in page state or preferences, not in the app shell, then restore with set_active(cx, bool) and inspect active(cx) when reconciling state."}
    },
}
