use crate::ui::snippets::BREADCRUMB_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryBreadcrumbPageBase = #(GalleryBreadcrumbPage::register_widget(vm))

    mod.widgets.GalleryBreadcrumbPage = set_type_default() do mod.widgets.GalleryBreadcrumbPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Breadcrumb"
            }

            ShadPageSubtitle{
                text: "Breadcrumb flows for navigation hierarchies."
            }

            ShadHr{}

            breadcrumb_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "Default" }
                                ShadBreadcrumb{
                                    ShadBreadcrumbLink{ text: "Home" }
                                    ShadBreadcrumbSeparator{}
                                    ShadBreadcrumbLink{ text: "Components" }
                                    ShadBreadcrumbSeparator{}
                                    ShadBreadcrumbPage{ text: "Breadcrumb" }
                                }

                                ShadHr{}
                                ShadSectionHeader{ text: "Collapsed / Ellipsis" }
                                ShadBreadcrumb{
                                    ShadBreadcrumbLink{ text: "Home" }
                                    ShadBreadcrumbSeparator{}
                                    ShadBreadcrumbEllipsis{}
                                    ShadBreadcrumbSeparator{}
                                    ShadBreadcrumbLink{ text: "Components" }
                                    ShadBreadcrumbSeparator{}
                                    ShadBreadcrumbPage{ text: "Breadcrumb" }
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
                                    code: #(BREADCRUMB_PREVIEW_CODE)
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
pub struct GalleryBreadcrumbPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryBreadcrumbPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
