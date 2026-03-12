use crate::ui::snippets::ALERT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAlertPageBase = #(GalleryAlertPage::register_widget(vm))

    mod.widgets.GalleryAlertPage = set_type_default() do mod.widgets.GalleryAlertPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Alert"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired alert components from makepad-components library"
            }

            ShadHr{}

            alert_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                alert_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    alert_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        alert_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        alert_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    alert_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        alert_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        alert_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                alert_preview_panel := mod.widgets.ShadPanel{
                    alert_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Default" }

                            ShadAlert{
                                width: Fill
                                icon := ShadAlertIcon{}
                                content := ShadAlertContent{
                                    title := ShadAlertTitle{text: "Heads up!"}
                                    description := ShadAlertDescription{
                                        text: "You can add components and dependencies to your app using the cli."
                                    }
                                }
                            }

                            ShadSectionHeader{ text: "Destructive" }

                            ShadAlertDestructive{
                                width: Fill
                                icon := ShadAlertDestructiveIcon{}
                                content := ShadAlertContent{
                                    title := ShadAlertDestructiveTitle{text: "Error"}
                                    description := ShadAlertDescription{
                                        text: "Your session has expired. Please log in again."
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

                                GalleryCodeSnippet{
                                    code_view +: { text: #(ALERT_PREVIEW_CODE) }
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
pub struct GalleryAlertPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryAlertPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
