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

            breadcrumb_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                breadcrumb_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    breadcrumb_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        breadcrumb_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}
                        breadcrumb_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    breadcrumb_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        breadcrumb_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}
                        breadcrumb_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                breadcrumb_preview_panel := mod.widgets.ShadPanel{
                    breadcrumb_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
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

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                GalleryCodeSnippet{
                                    code_view +: { text: #(BREADCRUMB_PREVIEW_CODE) }
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
