use crate::ui::snippets::ASPECT_RATIO_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAspectRatioPageBase = #(GalleryAspectRatioPage::register_widget(vm))

    mod.widgets.GalleryAspectRatioPage = set_type_default() do mod.widgets.GalleryAspectRatioPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Aspect Ratio"
            }

            ShadPageSubtitle{
                text: "Displays content within a desired ratio."
            }

            ShadHr{}

            aspect_ratio_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "16:9 Preview" }

                                RoundedView{
                                    width: Fill
                                    height: Fit
                                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                    draw_bg +: {
                                        color: #0000
                                        border_size: 1.0
                                        border_radius: (shad_theme.radius)
                                        border_color: (shad_theme.color_outline_border)
                                    }

                                    ShadAspectRatio{
                                        width: Fill
                                        ratio: 1.7777777778

                                        RoundedView{
                                            width: Fill
                                            height: Fill
                                            flow: Overlay
                                            align: Align{x: 0.5, y: 0.5}
                                            draw_bg +: {
                                                color: (shad_theme.color_secondary)
                                                border_radius: (shad_theme.radius)
                                            }

                                            ShadSectionHeader{
                                                text: "16:9"
                                                draw_text.text_style.font_size: 11
                                            }
                                        }
                                    }
                                }

                                ShadSectionHeader{ text: "Common Ratios" }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 12.0

                                    ShadAspectRatio{
                                        width: 180
                                        ratio: 1.0

                                        RoundedView{
                                            width: Fill
                                            height: Fill
                                            flow: Overlay
                                            align: Align{x: 0.5, y: 0.5}
                                            draw_bg +: {
                                                color: (shad_theme.color_secondary)
                                                border_radius: (shad_theme.radius)
                                            }
                                            ShadSectionHeader{ text: "1:1" }
                                        }
                                    }

                                    ShadAspectRatio{
                                        width: 180
                                        ratio: 1.3333333333

                                        RoundedView{
                                            width: Fill
                                            height: Fill
                                            flow: Overlay
                                            align: Align{x: 0.5, y: 0.5}
                                            draw_bg +: {
                                                color: (shad_theme.color_secondary)
                                                border_radius: (shad_theme.radius)
                                            }
                                            ShadSectionHeader{ text: "4:3" }
                                        }
                                    }

                                    ShadAspectRatio{
                                        width: 180
                                        ratio: 0.5625

                                        RoundedView{
                                            width: Fill
                                            height: Fill
                                            flow: Overlay
                                            align: Align{x: 0.5, y: 0.5}
                                            draw_bg +: {
                                                color: (shad_theme.color_secondary)
                                                border_radius: (shad_theme.radius)
                                            }
                                            ShadSectionHeader{ text: "9:16" }
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
                                    code: #(ASPECT_RATIO_PREVIEW_CODE)
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
pub struct GalleryAspectRatioPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryAspectRatioPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
