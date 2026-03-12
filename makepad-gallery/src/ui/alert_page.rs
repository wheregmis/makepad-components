use crate::ui::snippets::ALERT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAlertPage = ShadScrollYView{
        ShadPageTitle{
            text: "Alert"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired alert components from makepad-components library"
        }

        ShadHr{}

        alert_preview_section := mod.widgets.GalleryPreviewSection{
            width: Fill
            height: Fit

            code_snippet +: {
                code: #(ALERT_PREVIEW_CODE)
            }

            preview_flip +: {
                root_view +: {
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 12.0

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
                }
            }
        }
    }
}
