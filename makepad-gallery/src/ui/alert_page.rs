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

            alert_preview_section := mod.widgets.GalleryPreviewSection{
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
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(ALERT_PREVIEW_CODE)
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
