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
                text: "Multiline text input with the same field styling as the base input."
            }

            ShadHr{}

            textarea_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                textarea_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    textarea_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        textarea_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        textarea_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    textarea_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        textarea_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        textarea_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                textarea_preview_panel := mod.widgets.ShadPanel{
                    textarea_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Default" }

                            View{
                                width: 420
                                height: Fit

                                ShadTextarea{
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

                        code_page +: {
                            body +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            GalleryCodeSnippet{
                                code_view +: { text: #(TEXTAREA_PREVIEW_CODE) }
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
