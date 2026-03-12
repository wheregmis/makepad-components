use crate::ui::snippets::CAROUSEL_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCarouselPageBase = #(GalleryCarouselPage::register_widget(vm))

    mod.widgets.GalleryCarouselPage = set_type_default() do mod.widgets.GalleryCarouselPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            width: Fill
            height: Fill

            ShadPageTitle{
                text: "Carousel"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired carousel with prev/next navigation and slide indicators. Use ShadCarouselRef::next/prev/go_to and changed(actions) to sync app state."
            }

            ShadHr{}

            carousel_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                carousel_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    carousel_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        carousel_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        carousel_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    carousel_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        carousel_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        carousel_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                carousel_preview_panel := mod.widgets.ShadPanel{
                    carousel_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Default" }

                            carousel_demo := mod.widgets.ShadCarousel{}

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. Treat the carousel as a component that owns its prev/next buttons and dot wiring internally."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. Use a ShadCarouselRef when outside UI wants to move it: next(cx), prev(cx), or go_to(cx, index)."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. Listen to changed(actions) when labels, badges, analytics, or related content need the active slide index."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. Use current() when a page redraws or restores state and needs to know which slide is active now."}
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
                                code_view +: { text: #(CAROUSEL_PREVIEW_CODE) }
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
pub struct GalleryCarouselPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryCarouselPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
