use crate::ui::snippets::ALERT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAlertPage = ShadScrollYView{
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
                height: Fit
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
                alert_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
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

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(ALERT_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
