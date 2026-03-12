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

            input_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                input_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    input_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        input_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        input_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    input_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        input_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        input_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                input_preview_panel := mod.widgets.ShadPanel{
                    input_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
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

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. Give any input you need to drive an id, like email_input := ShadInput{...}."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. Read live edits with view.text_input(cx, ids!(email_input)).changed(actions)."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. Use returned(actions) when Enter should submit or confirm the current draft."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. When external state changes, push it back into the field with set_text(cx, ...)."}
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                GalleryCodeSnippet{
                                    code_view +: { text: #(INPUT_PREVIEW_CODE) }
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
