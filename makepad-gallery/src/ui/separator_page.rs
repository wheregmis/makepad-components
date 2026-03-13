use crate::ui::snippets::SEPARATOR_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySeparatorPageBase = #(GallerySeparatorPage::register_widget(vm))

    mod.widgets.GallerySeparatorPage = set_type_default() do mod.widgets.GallerySeparatorPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Separator"
            }

            ShadPageSubtitle{
                text: "A lightweight divider for grouping related content sections."
            }

            ShadSeparator{}

            separator_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "Stacked content" }
                                ShadPanel{
                                    View{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 12.0

                                        ShadLabel{text: "Account"}
                                        ShadFieldDescription{text: "Profile settings and personal information."}
                                        ShadSeparator{}
                                        ShadLabel{text: "Billing"}
                                        ShadFieldDescription{text: "Invoices, payment methods, and tax details."}
                                        ShadSeparator{}
                                        ShadLabel{text: "Security"}
                                        ShadFieldDescription{text: "Sessions, MFA, and access tokens."}
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
                                    code: #(SEPARATOR_PREVIEW_CODE)
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
pub struct GallerySeparatorPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySeparatorPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
