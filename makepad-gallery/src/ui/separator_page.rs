use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySeparatorPage,
    page: separator_page,
    title: "Separator",
    subtitle: "A lightweight divider for grouping related content sections.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Stacked content" }
        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                ShadLabel{text: "Account"}
                ShadFieldDescription{text: "Profile settings and personal information."}
                ShadSeparator{}
                ShadLabel{text: "Billing"}
                ShadFieldDescription{text: "Invoices, payment methods, and tax details."}
                ShadSeparator{}
                ShadLabel{text: "Security"}
                ShadFieldDescription{text: "Sessions, MFA, and access tokens."}
            }
        }
    },
}
