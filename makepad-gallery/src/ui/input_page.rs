use crate::ui::snippets::INPUT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryInputPageBase = #(GalleryInputPage::register_widget(vm))

    mod.widgets.GalleryInputPage = set_type_default() do mod.widgets.GalleryInputPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Input"
            }

            ShadPageSubtitle{
                text: "Inputs are page-owned draft state: use TextInputRef methods for live changes, submit-on-return, and restoring text from external state."
            }

            ShadHr{}

            input_preview_section := mod.widgets.GalleryPreviewSection{
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
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Give any input you need to drive an id, like email_input := ShadInput{...}."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Read live edits with view.text_input(cx, ids!(email_input)).changed(actions)."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Use returned(actions) when Enter should submit or confirm the current draft."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. When external state changes, push it back into the field with set_text(cx, ...)."}
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
                                    code: #(INPUT_PREVIEW_CODE)
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
pub struct GalleryInputPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryInputPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
