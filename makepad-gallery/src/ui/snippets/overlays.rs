pub const CONTEXT_MENU_PREVIEW_CODE: &str = r#"ShadContextMenu{
    labels: ["Open" "Duplicate" "Share" "Delete"]

    RoundedView{
        width: 360
        height: Fit
        flow: Down
        spacing: 6.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
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
pub const DIALOG_PREVIEW_CODE: &str = r#"// Generic: ShadDialog with overlay +: { content +: { body +: { ... } } }
// Alert: ShadDialogAlert{ open: false } — closes on Cancel/Confirm/backdrop
// Destructive: ShadDialogAlertDestructive{ open: false }

mod.widgets.ShadButton{text: "Open dialog"}
mod.widgets.ShadDialog{ open: false }
mod.widgets.ShadDialogAlert{ open: false }
mod.widgets.ShadDialogAlertDestructive{ open: false }

// Controller example (Rust):
// let dialog = self.ui.shad_dialog(cx, ids!(default_dialog));
//
// if self.ui.button(cx, ids!(open_dialog_btn)).clicked(actions) {
//     dialog.open(cx);
// }
//
// if self.ui.button(cx, ids!(close_btn)).clicked(actions) {
//     dialog.close(cx);
// }
//
// if let Some(is_open) = dialog.open_changed(actions) {
//     log!("Dialog open: {}", is_open);
// }"#;
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
}

// Controller example (Rust):
// let popover = self.view.shad_popover(cx, ids!(profile_popover));
// let content = popover.content_widget();
//
// if content.button(cx, ids!(popover_apply_btn)).clicked(actions) {
//     self.save_profile_changes();
//     popover.close(cx);
// }
//
// if let Some(is_open) = popover.open_changed(actions) {
//     log!("Popover open: {}", is_open);
// }"#;
pub const SHEET_PREVIEW_CODE: &str = r#"ShadButton{text: "Open sheet"}
ShadSheet{open: false}

// Controller example (Rust):
// let sheet = self.ui.shad_sheet(cx, ids!(right_sheet));
//
// if self.ui.button(cx, ids!(open_right_sheet_btn)).clicked(actions) {
//     sheet.open(cx);
// }
//
// if self.ui.button(cx, ids!(close_right_sheet_btn)).clicked(actions) {
//     sheet.close(cx);
// }
//
// if let Some(is_open) = sheet.open_changed(actions) {
//     log!("Sheet open: {}", is_open);
// }
//
// Override overlay.content.sheet_frame align/size for left/top/bottom variants."#;
