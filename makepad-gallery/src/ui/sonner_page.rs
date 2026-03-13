use crate::ui::snippets::SONNER_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;
use makepad_components::sonner::ShadSonnerWidgetExt;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySonnerPageBase = #(GallerySonnerPage::register_widget(vm))

    mod.widgets.GallerySonnerPage = set_type_default() do mod.widgets.GallerySonnerPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Sonner / Toast"
            }

            ShadPageSubtitle{
                text: "Toast notifications with Modal overlay. Use ShadSonnerRef::show/hide and opened/closed actions when a page or app shell triggers toasts."
            }

            ShadHr{}

            sonner_preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_flip +: {
                        root_view +: {
                            preview_content +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 16.0

                                ShadSectionHeader{ text: "Basic" }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 8.0

                                    toast_event_btn := ShadButton{text: "Event created"}
                                    toast_desc_btn := ShadButton{text: "Toast with description"}
                                }

                                View{
                                    width: Fill
                                    height: 200
                                    toast_event := ShadSonner{
                                        width: Fill
                                        height: Fill
                                        open: false
                                    }
                                    toast_desc := ShadSonnerWithDescription{
                                        width: Fill
                                        height: Fill
                                        open: false
                                    }
                                }

                                ShadHr{}

                                ShadSectionHeader{ text: "With Close Button" }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 8.0

                                    toast_close_btn := ShadButton{text: "Show toast with close"}
                                }

                                View{
                                    width: Fill
                                    height: 200
                                    toast_close := ShadSonnerWithClose{
                                        width: Fill
                                        height: Fill
                                        open: false
                                    }
                                }
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Keep one ShadSonnerRef per toast variant the page can trigger."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Call show(cx) from buttons, async completions, or other semantic page events."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Use opened(actions) and closed(actions) when the page or shell reacts to toast lifecycle."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Close buttons and modal dismissal remain component-owned, so the page just triggers and observes."}
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
                                    code: #(SONNER_PREVIEW_CODE)
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
pub struct GallerySonnerPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySonnerPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self.view.button(cx, ids!(toast_event_btn)).clicked(actions) {
                self.view.shad_sonner(cx, ids!(toast_event)).show(cx);
            }
            if self.view.button(cx, ids!(toast_desc_btn)).clicked(actions) {
                self.view.shad_sonner(cx, ids!(toast_desc)).show(cx);
            }
            if self.view.button(cx, ids!(toast_close_btn)).clicked(actions) {
                self.view.shad_sonner(cx, ids!(toast_close)).show(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
