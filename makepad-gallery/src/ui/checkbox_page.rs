use crate::ui::snippets::CHECKBOX_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCheckboxPageBase = #(GalleryCheckboxPage::register_widget(vm))

    mod.widgets.GalleryCheckboxPage = set_type_default() do mod.widgets.GalleryCheckboxPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Checkbox"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired checkbox component from makepad-components library. Use ShadCheckboxRef::changed(actions) or is_checked() when syncing form state."
            }

            ShadHr{}

            checkbox_preview_section := mod.widgets.GalleryPreviewSection{
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
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12.0

                                    ShadCheckbox{label: "Accept terms and conditions"}
                                    ShadCheckbox{label: "Pre-checked option" checked: true}
                                    ShadCheckbox{label: "Subscribe to newsletter"}
                                }

                                ShadHr{}

                                ShadSectionHeader{ text: "In a form row" }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 24.0
                                    align: Align{y: 0.5}

                                    ShadCheckbox{label: "Option A"}
                                    ShadCheckbox{label: "Option B" checked: true}
                                    ShadCheckbox{label: "Option C"}
                                }

                                ShadHr{}
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Keep each checked value in page or form state, even when the visual checkbox looks self-contained."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Read changed(actions) from ShadCheckboxRef to capture the user's latest choice."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Call set_checked(cx, checked, animator::Animate::No) when loading saved data or resetting a form."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Use is_checked() when submitting or validating without waiting for a fresh action event."}
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
                                    code: #(CHECKBOX_PREVIEW_CODE)
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
pub struct GalleryCheckboxPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryCheckboxPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
