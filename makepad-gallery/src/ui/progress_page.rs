use crate::ui::snippets::PROGRESS_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryProgressPageBase = #(GalleryProgressPage::register_widget(vm))

    mod.widgets.GalleryProgressPage = set_type_default() do mod.widgets.GalleryProgressPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Progress"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired progress bars. Determinate (value 0–1) and indeterminate (animated)."
            }

            ShadHr{}

            progress_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "Determinate" }

                                ShadProgress33{}
                                ShadProgress66{}
                                ShadProgressFull{}

                                ShadSectionHeader{ text: "Indeterminate (animated)" }

                                ShadProgressIndeterminate{}
                            }
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(PROGRESS_PREVIEW_CODE)
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
pub struct GalleryProgressPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryProgressPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
