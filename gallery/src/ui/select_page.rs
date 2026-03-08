use crate::ui::snippets::SELECT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySelectPage = ShadScrollArea{
        ShadPageTitle{
            text: "Select"
        }

        ShadPageSubtitle{
            text: "Single-choice, non-searchable selection built on the same popup stack as Dropdown Menu."
        }

        ShadSeparator{}

        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 12.0

                    ShadSelect{labels: ["Pending" "In Progress" "Done"]}
                    ShadSelect{labels: ["Toronto" "Montreal" "Vancouver" "Calgary"]}
                }

                ShadFieldDescription{
                    text: "Known limitation: popup-style selects can still be unreliable inside the current gallery PageFlip shell. The splash app remains the best place to verify interaction."
                }
            }
        }

        GalleryCodeSnippetSimple{
            code: #(SELECT_PREVIEW_CODE)
        }
    }
}
