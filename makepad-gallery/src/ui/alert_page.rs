use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAlertPage,
    page: alert_page,
    title: "Alert",
    subtitle: "Shadcn-inspired alert components from makepad-components library",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        ShadAlert{
            width: Fill
            icon := ShadAlertIcon{}
            content := ShadAlertContent{
                title := ShadAlertTitle{text: "Heads up!"}
                description := ShadAlertDescription{
                    text: "You can add components and dependencies to your app using the cli."
                }
            }
        }

        ShadSectionHeader{ text: "Destructive" }

        ShadAlertDestructive{
            width: Fill
            icon := ShadAlertDestructiveIcon{}
            content := ShadAlertContent{
                title := ShadAlertDestructiveTitle{text: "Error"}
                description := ShadAlertDescription{
                    text: "Your session has expired. Please log in again."
                }
            }
        }
    },
}
