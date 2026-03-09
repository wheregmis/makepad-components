use crate::ui::snippets::RADIO_GROUP_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryRadioGroupPage = ShadScrollArea{
        ShadPageTitle{
            text: "Radio Group"
        }

        ShadPageSubtitle{
            text: "Single-choice groups styled around Makepad radio buttons."
        }

        ShadSeparator{}

        ShadSectionHeader{ text: "Stacked options" }
        ShadPanel{
            ShadRadioGroup{
                ShadRadioItem{text: "Starter"}
                ShadRadioItem{text: "Pro"}
                ShadRadioItem{text: "Enterprise"}
            }
        }

        ShadSectionHeader{ text: "Inline options" }
        ShadPanel{
            ShadRadioGroupInline{
                ShadRadioItem{text: "Weekly"}
                ShadRadioItem{text: "Monthly"}
                ShadRadioItem{text: "Yearly"}
            }
        }

        GalleryCodeSnippetSimple{
            code: #(RADIO_GROUP_PREVIEW_CODE)
        }
    }
}
