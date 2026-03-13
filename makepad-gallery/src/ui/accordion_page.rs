use crate::ui::snippets::ACCORDION_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAccordionPageBase = #(GalleryAccordionPage::register_widget(vm))

    mod.widgets.GalleryAccordionPage = set_type_default() do mod.widgets.GalleryAccordionPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Accordion"
            }

            ShadPageSubtitle{
                text: "Accordion component from makepad-components library. Use ShadAccordionItemRef::set_is_open(cx, ..) and opening/closing(actions) when external state drives expansion."
            }

            ShadHr{}

            accordion_preview_section := mod.widgets.GalleryPreviewSection{
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

                                accordion_demo_shell := View{
                                    width: 840
                                    height: Fit
                                    flow: Down
                                    spacing: 12.0

                                    ShadFieldDescription{
                                        width: Fill
                                        text: "Contained preview with realistic spacing, nested content, and divider rhythm."
                                    }

                                    accordion_panel := ShadAccordion{
                                        width: Fill
                                        item_accessible := ShadAccordionItem{
                                            title: "Is it accessible?"
                                            is_open: true
                                            body: View{
                                                width: Fill
                                                height: Fit
                                                flow: Down
                                                padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                                                ShadFieldDescription{
                                                    width: Fill
                                                    text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior."
                                                }
                                            }
                                        }

                                        item_styled := ShadAccordionItem{
                                            title: "Is it styled with complex elements?"
                                            body: View{
                                                width: Fill
                                                height: Fit
                                                flow: Down
                                                padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                                                spacing: 10.0

                                                ShadFieldDescription{
                                                    width: Fill
                                                    text: "We can put any view here, like a row with toggles."
                                                }

                                                View{
                                                    width: Fit
                                                    height: Fit
                                                    flow: Right
                                                    spacing: 20.0
                                                    align: Align{y: 0.5}

                                                    ShadSwitch{text: "Switch"}
                                                    ShadCheckbox{label: "Or a CheckBox"}
                                                }
                                            }
                                        }

                                        item_third := ShadAccordionItem{
                                            title: "This is third accordion"
                                            body: View{
                                                width: Fill
                                                height: Fit
                                                flow: Down
                                                padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                                                ShadFieldDescription{
                                                    width: Fill
                                                    text: "This is third accordion content. It can be any view, like a text view or a button."
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Keep expansion decisions in the page or feature state, not inside the app shell."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Get a ShadAccordionItemRef for the specific item you want to drive, then call set_is_open(cx, true/false, animator::Animate::Yes)."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Use opening(actions) and closing(actions) when surrounding UI needs to react to the transition."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Use is_open(cx) when restoring layout or reconciling external state back into the page."}
                                    }
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                                code_snippet +: {
                                    code: #(ACCORDION_PREVIEW_CODE)
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
pub struct GalleryAccordionPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryAccordionPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
