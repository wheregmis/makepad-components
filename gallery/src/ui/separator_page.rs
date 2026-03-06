use makepad_components::makepad_widgets::*;
use crate::ui::snippets::SEPARATOR_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySeparatorPage = ShadScrollArea{
        ShadPageTitle{
            text: "Separator"
        }

        ShadPageSubtitle{
            text: "A lightweight divider for grouping related content sections."
        }

        ShadSeparator{}

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

        GalleryCodeSnippetSimple{
            code: #(SEPARATOR_PREVIEW_CODE)
        }
    }
}
