use crate::ui::snippets::DIALOG_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryDialogPage = ShadScrollYView{
        ShadPageTitle{
            text: "Dialog"
        }

        ShadPageSubtitle{
            text: "Modal dialogs with variants: generic (custom body), alert (title + description + Cancel/Confirm), and destructive. Use set_open(bool) and is_open() to control visibility."
        }

        ShadHr{}

        dialog_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            dialog_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                dialog_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    dialog_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    dialog_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                dialog_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    dialog_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    dialog_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            dialog_preview_panel := mod.widgets.ShadPanel{
                dialog_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 24.0

                        ShadSectionHeader{ text: "Generic" }

                        open_dialog_btn := mod.widgets.ShadButton{
                            text: "Open dialog"
                        }

                        View{
                            width: Fill
                            height: 280
                            default_dialog := ShadDialog{
                                width: Fill
                                height: Fill
                                open: false
                                overlay +: {
                                    content +: {
                                        body +: {
                                            title_label := ShadAlertTitle{
                                                text: "Dialog title"
                                            }
                                            desc_label := ShadAlertDescription{
                                                text: "This is a generic dialog. Put any content in the body. Close via the button below or click the backdrop / press Escape."
                                            }
                                            close_btn := mod.widgets.ShadButton{
                                                text: "Close"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        ShadSectionHeader{ text: "Alert" }

                        open_default_btn := mod.widgets.ShadButton{
                            text: "Open alert dialog"
                        }

                        View{
                            width: Fill
                            height: 280
                            alert_default_dialog := ShadDialogAlert{
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
                            destructive_dialog := ShadDialogAlertDestructive{
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
                            code: #(DIALOG_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
