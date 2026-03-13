use crate::ui::snippets::CARD_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCardPageBase = #(GalleryCardPage::register_widget(vm))

    mod.widgets.GalleryCardPage = set_type_default() do mod.widgets.GalleryCardPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Card"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired card component from makepad-components library"
            }

            ShadHr{}

            card_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "Default" }

                                mod.widgets.ShadCard{
                                    header := mod.widgets.ShadCardHeader{
                                        title := mod.widgets.ShadCardTitle{text: "Card title"}
                                        description := mod.widgets.ShadCardDescription{text: "Card description goes here."}
                                    }
                                    content := mod.widgets.ShadCardContent{
                                        ShadLabel{
                                            text: "Card content area. Put any widgets here."
                                            draw_text.text_style.font_size: 14
                                        }
                                    }
                                    footer := mod.widgets.ShadCardFooter{
                                        mod.widgets.ShadButton{text: "Cancel"}
                                        mod.widgets.ShadButton{text: "Save"}
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
                                    code: #(CARD_PREVIEW_CODE)
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
pub struct GalleryCardPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryCardPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
