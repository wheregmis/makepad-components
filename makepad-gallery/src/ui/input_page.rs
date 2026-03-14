use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryInputPage,
    page: input_page,
    title: "Input",
    subtitle: "Inputs are page-owned draft state: use TextInputRef methods for live changes, submit-on-return, and restoring text from external state.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        View{
            width: 320
            height: Fit

            ShadInput{
                empty_text: "Email"
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "Disabled" }

        View{
            width: 320
            height: Fit

            ShadInput{
                is_read_only: true
                empty_text: "Read Only Value"
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "With Label" }

        View{
            width: 320
            height: Fit
            flow: Down
            spacing: 6.0

            ShadLabel{ text: "Email" }
            ShadInput{ empty_text: "Email" }
        }

        ShadSectionHeader{ text: "With Leading Icon" }

        View{
            width: 320
            height: Fit

            ShadInputWithIcon{}
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give any input you need to drive an id, like email_input := ShadInput{...}."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read live edits with view.text_input(cx, ids!(email_input)).changed(actions)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use returned(actions) when Enter should submit or confirm the current draft."}
        mod.widgets.GalleryActionFlowStep{text: "4. When external state changes, push it back into the field with set_text(cx, ...)."}
    },
}
