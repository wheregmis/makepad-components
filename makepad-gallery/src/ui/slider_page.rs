use crate::ui::snippets::SLIDER_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySliderPageBase = #(GallerySliderPage::register_widget(vm))

    mod.widgets.GallerySliderPage = set_type_default() do mod.widgets.GallerySliderPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Slider"
            }

            ShadPageSubtitle{
                text: "Sliders expose continuous and committed values separately, so pages can preview during drag and commit on release."
            }

            ShadHr{}

            slider_preview_section := mod.widgets.GalleryPreviewSection{
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

                                volume_slider := ShadSlider{default: 0.5}
                                ShadSlider{default: 0.8}
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Name the slider you want to observe, then get it with view.slider(cx, ids!(volume_slider))."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Use slided(actions) for live preview while the thumb is moving."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Use end_slide(actions) when you want to commit the final value to saved state."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Restore or override the control from outside with set_value(cx, f64), and inspect the current value() when needed."}
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
                                    code: #(SLIDER_PREVIEW_CODE)
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
pub struct GallerySliderPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySliderPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
