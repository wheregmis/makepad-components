pub const CONTEXT_MENU_PREVIEW_CODE: &str = r#"ShadContextMenu{
    labels: ["Open" "Duplicate" "Share" "Delete"]

    ShadSurface{
        width: 360
        height: Fit
        flow: Down
        spacing: 6.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        ShadLabel{text: "Project brief.md"}
        ShadFieldDescription{text: "Right click this card to open the menu."}
    }
}"#;
pub const DIALOG_PREVIEW_CODE: &str = r#"open_rename_dialog_btn := ShadButton{
    text: "Rename project"
}

rename_dialog := ShadDialog{
    overlay +: {
        content +: {
            body +: {
                dialog_header := ShadDialogHeader{
                    title := ShadDialogTitle{text: "Rename project"}
                    description := ShadDialogDescription{
                        text: "Update the project name shown across navigation and shares."
                    }
                }

                dialog_content := ShadDialogContent{
                    ShadField{
                        ShadFieldLabel{text: "Project name"}
                        ShadInput{empty_text: "Northwind Revamp"}
                    }

                    ShadFieldDescription{
                        text: "Keep it concise. Changes apply immediately."
                    }
                }

                dialog_footer := ShadDialogFooter{
                    rename_cancel_btn := ShadButtonOutline{text: "Cancel"}
                    rename_save_btn := ShadButton{text: "Save changes"}
                }
            }
        }
    }
}

publish_dialog := ShadDialogAlert{
    overlay +: {
        content +: {
            dialog_panel +: {
                dialog_body +: {
                    title_label +: {text: "Publish changes?"}
                    description_label +: {
                        text: "Push the latest updates live for every workspace."
                    }
                }

                footer +: {
                    cancel +: {text: "Keep editing"}
                    confirm +: {text: "Publish now"}
                }
            }
        }
    }
}"#;
pub const POPOVER_PREVIEW_CODE: &str = r#"profile_popover := ShadPopover{
    side: "bottom"
    align: "start"

    trigger := ShadButtonOutline{
        text: "Open profile editor"
    }

    content: ShadPopoverContent{
        title := ShadSectionHeader{
            text: "Edit profile"
        }

        description := ShadFieldDescription{
            text: "Quick edits belong in a popover when the page context should stay visible."
        }

        footer := View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0

            popover_close_btn := ShadButtonGhost{
                text: "Cancel"
            }

            popover_apply_btn := ShadButton{
                text: "Save"
            }
        }
    }
}"#;
pub const SHEET_PREVIEW_CODE: &str = r#"open_right_sheet_btn := ShadButton{
    text: "Open editor"
}

right_sheet := ShadSheet{
    side: "right"
    sheet_size: 360.0

    overlay +: {
        content +: {
            sheet_frame +: {
                header +: {
                    title +: {text: "Edit workspace"}
                    description +: {text: "Keep editing in context without leaving the current dashboard."}
                }

                body +: {
                    ShadField{
                        ShadFieldLabel{text: "Workspace name"}
                        ShadInput{empty_text: "Northwind"}
                    }

                    ShadField{
                        ShadFieldLabel{text: "Default team"}
                        ShadSelect{labels: ["Design" "Engineering" "Ops"]}
                    }
                }

                footer +: {
                    close_right_sheet_btn := ShadButtonOutline{text: "Cancel"}
                    save_right_sheet_btn := ShadButton{text: "Save changes"}
                }
            }
        }
    }
}"#;
