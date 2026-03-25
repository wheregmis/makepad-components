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
}

// Controller example (Rust):
// let menu = self.ui.shad_context_menu(cx, ids!(context_menu_basic));
//
// if let Some(index) = menu.changed(actions) {
//     match index {
//         0 => self.open_file(),
//         1 => self.duplicate_file(),
//         2 => self.share_file(),
//         3 => self.delete_file(),
//         _ => {}
//     }
// }"#;
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
                    rename_cancel_btn := ShadButton{
                        variant: ShadButtonVariant.Outline
                        text: "Cancel"
                    }
                    rename_save_btn := ShadButton{text: "Save changes"}
                }
            }
        }
    }
}

publish_dialog := ShadDialogAlert{
    alert_title_text: "Publish changes?"
    alert_description_text: "Push the latest updates live for every workspace."
    alert_cancel_text: "Keep editing"
    alert_confirm_text: "Publish now"
}

// Controller example (Rust):
// let dialog = self.ui.shad_dialog(cx, ids!(rename_dialog));
//
// if self.ui.shad_button(cx, ids!(open_rename_dialog_btn)).clicked(actions) {
//     dialog.open(cx);
// }
//
// if self.ui.shad_button(cx, ids!(rename_cancel_btn)).clicked(actions)
//     || self.ui.shad_button(cx, ids!(rename_save_btn)).clicked(actions)
// {
//     dialog.close(cx);
// }
//
// if let Some(is_open) = dialog.open_changed(actions) {
//     log!("Dialog open: {}", is_open);
// }"#;
pub const POPOVER_PREVIEW_CODE: &str = r#"profile_popover := ShadPopover{
    side: "bottom"
    align: "start"

    trigger := ShadButton{
        variant: ShadButtonVariant.Outline
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

            popover_close_btn := ShadButton{
                variant: ShadButtonVariant.Ghost
                text: "Cancel"
            }

            popover_apply_btn := ShadButton{
                text: "Save"
            }
        }
    }
}

// Controller example (Rust):
// let popover = self.view.shad_popover(cx, ids!(profile_popover));
// let content = popover.content_widget();
//
// if content.shad_button(cx, ids!(popover_apply_btn)).clicked(actions) {
//     self.save_profile_changes();
//     popover.close(cx);
// }
//
// if let Some(is_open) = popover.open_changed(actions) {
//     log!("Popover open: {}", is_open);
// }"#;
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
                    close_right_sheet_btn := ShadButton{
                        variant: ShadButtonVariant.Outline
                        text: "Cancel"
                    }
                    save_right_sheet_btn := ShadButton{text: "Save changes"}
                }
            }
        }
    }
}

// Controller example (Rust):
// let sheet = self.ui.shad_sheet(cx, ids!(right_sheet));
//
// if self.ui.shad_button(cx, ids!(open_right_sheet_btn)).clicked(actions) {
//     sheet.open(cx);
// }
//
// if self.ui.shad_button(cx, ids!(close_right_sheet_btn)).clicked(actions)
//     || self.ui.shad_button(cx, ids!(save_right_sheet_btn)).clicked(actions)
// {
//     sheet.close(cx);
// }
//
// if let Some(is_open) = sheet.open_changed(actions) {
//     log!("Sheet open: {}", is_open);
// }
//
// side and sheet_size are declarative configuration.
// The page opens and closes the sheet; backdrop dismissal and Escape stay inside ShadSheet."#;
