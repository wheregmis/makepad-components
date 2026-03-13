use crate::ui::snippets::TEXTAREA_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryTextareaPageBase = #(GalleryTextareaPage::register_widget(vm))

    mod.widgets.GalleryTextareaPage = set_type_default() do mod.widgets.GalleryTextareaPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Textarea"
            }

            ShadPageSubtitle{
                text: "Textarea uses the same TextInputRef flow as Input, just with multiline editing and larger draft state."
            }

            ShadHr{}

            textarea_preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_flip +: {
                        root_view +: {
                            preview_content +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

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
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Give the textarea an id if you want to drive or inspect it from Rust."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Use view.text_input(cx, ids!(bio_input)).changed(actions) for live draft synchronization."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Use set_text(cx, ...) to restore saved drafts, canned replies, or undoable resets."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Keep the actual note, bio, or message in page state; the textarea is the editor for that state."}
                                    }
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(TEXTAREA_PREVIEW_CODE)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryTextareaPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryTextareaPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
