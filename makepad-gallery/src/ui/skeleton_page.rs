use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySkeletonPageBase = #(GallerySkeletonPage::register_widget(vm))

    mod.widgets.GallerySkeletonPage = set_type_default() do mod.widgets.GallerySkeletonPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Skeleton"
            }

            ShadPageSubtitle{
                text: "Use to show a placeholder while content is loading."
            }

            ShadHr{}

            skeleton_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "Preview" }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 12.0
                                    align: Align{x: 0.0, y: 0.5}

                                    ShadSkeleton{
                                        width: 48
                                        height: 48
                                        draw_bg.border_radius: 24.0
                                    }

                                    View{
                                        width: Fit
                                        height: Fit
                                        flow: Down
                                        spacing: 8.0

                                        ShadSkeleton{
                                            width: 200
                                            height: 16
                                        }
                                        ShadSkeleton{
                                            width: 150
                                            height: 16
                                        }
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
                                    code: "View{\\n    width: Fill\\n    height: Fit\\n    flow: Right\\n    spacing: 12.0\\n    align: Align{x: 0.0, y: 0.5}\\n\\n    ShadSkeleton{\\n        width: 48\\n        height: 48\\n        draw_bg.border_radius: 24.0\\n    }\\n\\n    View{\\n        width: Fit\\n        height: Fit\\n        flow: Down\\n        spacing: 8.0\\n\\n        ShadSkeleton{\\n            width: 200\\n            height: 16\\n        }\\n        ShadSkeleton{\\n            width: 150\\n            height: 16\\n        }\\n    }\\n}"
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
pub struct GallerySkeletonPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySkeletonPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
