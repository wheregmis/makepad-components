use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAlertPage,
    page: alert_page,
    title: "Alert",
    subtitle: "Inline callouts for status, guidance, and destructive messaging. Configure the shell with a tone, title, and description.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Anatomy" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Use ShadAlert with tone: ShadAlertTone.Default for guidance or ShadAlertTone.Destructive for error states."}
            ShadFieldDescription{text: "The alert owns its icon and text stack now, so the gallery can pass title_text and description_text directly on the component."}
            ShadFieldDescription{text: "If you need custom layout, the legacy icon/content children still exist, but the prop-driven root is the preferred shape."}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Default" }

        ShadAlert{
            width: Fill
            title_text: "Heads up!"
            description_text: "You can add components and dependencies to your app using the cli."
        }

        ShadSectionHeader{ text: "Destructive" }

        ShadAlert{
            width: Fill
            tone: ShadAlertTone.Destructive
            title_text: "Error"
            description_text: "Your session has expired. Please log in again."
        }

        ShadHr{}

        ShadSectionHeader{ text: "Usage" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Alerts are inline status UI. They do not own modal state, dismissal logic, or background overlays like dialogs and popovers do."}
            ShadFieldDescription{text: "Use the default tone for neutral guidance, tips, or non-blocking status updates. Use the destructive tone when the copy itself should read as an error or dangerous condition."}
            ShadFieldDescription{text: "If you need actions, add them below the description or as another row inside the alert body so the page keeps ownership of the actual behavior."}
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Choose the shell first: ShadAlert with tone: ShadAlertTone.Default for neutral information, or tone: ShadAlertTone.Destructive for destructive emphasis."}
        mod.widgets.GalleryActionFlowStep{text: "2. Set title_text and description_text on the root alert instead of manually wiring nested title and description labels."}
        mod.widgets.GalleryActionFlowStep{text: "3. Handle any real behavior in page or app code. The alert primitive is a styled layout surface, not a state machine."}
    },
}
