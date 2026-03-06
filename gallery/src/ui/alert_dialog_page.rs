use makepad_components::makepad_widgets::*;
use crate::ui::snippets::ALERT_DIALOG_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAlertDialogPage = ShadScrollYView{
        ShadPageTitle{
            text: "Alert Dialog"
        }

        ShadPageSubtitle{
            text: "Modal dialog with title, description, and action buttons (Cancel / Continue or OK). Use set_open(bool) and is_open() to control visibility."
        }

        ShadHr{}

        alert_dialog_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            alert_dialog_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                alert_dialog_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    alert_dialog_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    alert_dialog_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                alert_dialog_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    alert_dialog_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    alert_dialog_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            alert_dialog_preview_panel := mod.widgets.ShadPanel{
                alert_dialog_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 24.0

                        ShadSectionHeader{ text: "Default" }

                        open_default_btn := mod.widgets.ShadButton{
                            text: "Open dialog"
                        }

                        View{
                            width: Fill
                            height: 280
                            default_dialog := ShadAlertDialog{
                                width: Fill
                                height: Fill
                                open: false
                            }
                        }

                        ShadSectionHeader{ text: "Destructive" }

                        open_destructive_btn := mod.widgets.ShadButtonDestructive{
                            text: "Open destructive dialog"
                        }

                        View{
                            width: Fill
                            height: 280
                            destructive_dialog := ShadAlertDialogDestructive{
                                width: Fill
                                height: Fill
                                open: false
                            }
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(ALERT_DIALOG_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
