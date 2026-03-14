use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryTextareaPage,
    page: textarea_page,
    title: "Textarea",
    subtitle: "Textarea uses the same TextInputRef flow as Input, just with multiline editing and larger draft state.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        View{
            width: 420
            height: Fit

            bio_input := ShadTextarea{
                empty_text: "Type your message here."
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "With label and helper text" }

        View{
            width: 420
            height: Fit
            flow: Down
            spacing: 6.0

            ShadLabel{ text: "Bio" }
            ShadTextareaSm{
                empty_text: "Tell us a little bit about yourself"
            }
            ShadFieldDescription{
                text: "Keep it short. You can always edit this later."
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "Large" }

        View{
            width: 520
            height: Fit

            ShadTextareaLg{
                empty_text: "Draft a longer response..."
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give the textarea an id if you want to drive or inspect it from Rust."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use view.text_input(cx, ids!(bio_input)).changed(actions) for live draft synchronization."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use set_text(cx, ...) to restore saved drafts, canned replies, or undoable resets."}
        mod.widgets.GalleryActionFlowStep{text: "4. Keep the actual note, bio, or message in page state; the textarea is the editor for that state."}
    },
}
