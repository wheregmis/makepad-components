pub const CONTEXT_MENU_PREVIEW_CODE: &str = r#"context_menu_basic := ShadContextMenu{
    labels: ["Open" "Duplicate" "Share" "Delete"]

    ShadCard{
        width: 360
        height: Fit
        flow: Down
        spacing: 6.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

        ShadLabel{text: "Project brief.md"}
        ShadFieldDescription{text: "Right click this card to open the menu."}
    }
}

context_menu_status := ShadFieldDescription{
    text: "No action selected yet."
}

// Controller example (Rust):
// use makepad_components::context_menu::ShadContextMenuWidgetExt;
//
// if let Some(index) = self.view.shad_context_menu(cx, ids!(context_menu_basic)).changed(actions) {
//     let label = match index {
//         0 => "Open",
//         1 => "Duplicate",
//         2 => "Share",
//         3 => "Delete",
//         _ => "Unknown",
//     };
//     self.view.label(cx, ids!(context_menu_status))
//         .set_text(cx, &format!("Selected: {}", label));
// }"#;
pub const DIALOG_PREVIEW_CODE: &str = r#"// Buttons that open each dialog
View{
    width: Fill
    height: Fit
    flow: Right{wrap: true}
    spacing: 12.0

    open_rename_dialog_btn := ShadButton{
        text: "Rename project"
    }

    open_publish_dialog_btn := ShadButtonOutline{
        text: "Publish changes"
    }

    open_delete_dialog_btn := ShadButtonDestructive{
        text: "Delete project"
    }
}

// Dialog definitions (place at the overlay/after_root level outside the scroll area)
rename_dialog := ShadDialog{
    overlay +: {
        content +: {
            body +: {
                dialog_header := ShadDialogHeader{
                    title := ShadDialogTitle{
                        text: "Rename project"
                    }
                    description := ShadDialogDescription{
                        text: "Update the project name shown across navigation, shares, and release summaries."
                    }
                }

                dialog_content := ShadDialogContent{
                    ShadField{
                        ShadFieldLabel{text: "Project name"}
                        rename_project_input := ShadInput{
                            empty_text: "Northwind Revamp"
                        }
                    }

                    ShadFieldDescription{
                        text: "Keep it concise. Changes apply immediately across the workspace chrome."
                    }
                }

                dialog_footer := ShadDialogFooter{
                    rename_cancel_btn := ShadButtonOutline{
                        text: "Cancel"
                    }
                    rename_save_btn := ShadButton{
                        text: "Save changes"
                    }
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
                    title_label +: { text: "Publish changes?" }
                    description_label +: {
                        text: "Push the latest pricing copy and onboarding updates live for every workspace."
                    }
                }
                footer +: {
                    cancel +: { text: "Keep editing" }
                    confirm +: { text: "Publish now" }
                }
            }
        }
    }
}

delete_dialog := ShadDialogAlertDestructive{
    overlay +: {
        content +: {
            dialog_panel +: {
                dialog_body +: {
                    title_label +: { text: "Delete project?" }
                    description_label +: {
                        text: "This permanently removes the project, API keys, and release history from your workspace."
                    }
                }
                footer +: {
                    cancel +: { text: "Cancel" }
                    confirm +: { text: "Delete project" }
                }
            }
        }
    }
}

// Controller example (Rust):
// fn set_dialog_open(&mut self, cx: &mut Cx, path: &[LiveId], open: bool) {
//     let dialog = self.view.shad_dialog(cx, path);
//     if open { dialog.open(cx); } else { dialog.close(cx); }
// }
//
// if self.view.button(cx, ids!(open_rename_dialog_btn)).clicked(actions) {
//     self.set_dialog_open(cx, ids!(rename_dialog), true);
// }
// if self.view.button(cx, ids!(open_publish_dialog_btn)).clicked(actions) {
//     self.set_dialog_open(cx, ids!(publish_dialog), true);
// }
// if self.view.button(cx, ids!(open_delete_dialog_btn)).clicked(actions) {
//     self.set_dialog_open(cx, ids!(delete_dialog), true);
// }
// for button in [ids!(rename_cancel_btn), ids!(rename_save_btn)] {
//     if self.view.button(cx, button).clicked(actions) {
//         self.set_dialog_open(cx, ids!(rename_dialog), false);
//     }
// }"#;
pub const POPOVER_PREVIEW_CODE: &str = r#"// Basic — bottom/start aligned
profile_popover := ShadPopover{
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
            text: "Quick edits belong in a popover when the current page context should remain visible."
        }

        footer := View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0
            margin: Inset{top: 8}

            popover_close_btn := ShadButtonGhost{
                text: "Cancel"
            }

            popover_apply_btn := ShadButton{
                text: "Save"
            }
        }
    }
}

popover_status := ShadFieldDescription{
    text: "Popover is closed."
}

// Top / End aligned
help_popover := ShadPopover{
    side: "top"
    align: "end"

    trigger := ShadButtonGhost{
        text: "Open top-end help"
    }

    content: ShadPopoverContent{
        width: 280

        title := ShadSectionHeader{
            text: "Keyboard shortcuts"
        }

        description := ShadFieldDescription{
            text: "Use popovers for compact help, profile cards, or lightweight editing flows that should stay attached to a trigger."
        }
    }
}

help_popover_status := ShadFieldDescription{
    text: "Help popover is closed."
}

// Controller example (Rust):
// use makepad_components::popover::ShadPopoverWidgetExt;
//
// let profile = self.view.shad_popover(cx, ids!(profile_popover));
// let profile_content = profile.content_widget();
//
// if profile_content.button(cx, ids!(popover_apply_btn)).clicked(actions) {
//     profile.close(cx);
//     self.view.label(cx, ids!(popover_status))
//         .set_text(cx, "Saved changes and closed the popover.");
// }
// if profile_content.button(cx, ids!(popover_close_btn)).clicked(actions) {
//     profile.close(cx);
// }
//
// if profile.open_changed(actions).is_some()
//     || self.view.shad_popover(cx, ids!(help_popover)).open_changed(actions).is_some()
// {
//     // sync status labels from is_open()
// }"#;
pub const SHEET_PREVIEW_CODE: &str = r#"// Trigger buttons (inside the page scroll area)
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0

    ShadCard{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{left: 14, right: 14, top: 14, bottom: 14}

        ShadLabel{text: "Right / 360px"}
        open_right_sheet_btn := ShadButton{text: "Open editor"}
    }

    ShadCard{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{left: 14, right: 14, top: 14, bottom: 14}

        ShadLabel{text: "Left / 360px"}
        open_left_sheet_btn := ShadButtonOutline{text: "Open navigation"}
    }

    ShadCard{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{left: 14, right: 14, top: 14, bottom: 14}

        ShadLabel{text: "Top / 220px"}
        open_top_sheet_btn := ShadButtonOutline{text: "Open filters"}
    }

    ShadCard{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{left: 14, right: 14, top: 14, bottom: 14}

        ShadLabel{text: "Bottom / 220px"}
        open_bottom_sheet_btn := ShadButtonOutline{text: "Open activity"}
    }
}

// Sheet definitions (place at the overlay/after_root level, outside the scroll area)
right_sheet := ShadSheet{
    side: "right"
    sheet_size: 360.0
    overlay +: {
        content +: {
            sheet_frame +: {
                header +: {
                    title +: {text: "Edit workspace"}
                    description +: {text: "Keep editing in context without leaving the dashboard you are already working in."}
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
}

left_sheet := ShadSheet{
    side: "left"
    sheet_size: 360.0
    overlay +: {
        content +: {
            sheet_frame +: {
                header +: {
                    title +: {text: "Project navigation"}
                    description +: {text: "Use left sheets for drill-in navigation and supporting menus."}
                }
                body +: {
                    ShadSidebarItem{text: "Roadmap"}
                    ShadSidebarItem{text: "Backlog"}
                    ShadSidebarItem{text: "Releases"}
                    ShadSidebarItem{text: "Postmortems"}
                }
                footer +: {
                    close_left_sheet_btn := ShadButtonOutline{text: "Close"}
                }
            }
        }
    }
}

top_sheet := ShadSheet{
    side: "top"
    sheet_size: 220.0
    overlay +: {
        content +: {
            sheet_frame +: {
                header +: {
                    title +: {text: "Filter panel"}
                    description +: {text: "Top sheets work well for lightweight filters and global controls."}
                }
                body +: {
                    ShadField{
                        ShadFieldLabel{text: "Team"}
                        ShadSelect{labels: ["All teams" "Design" "Engineering" "Ops"]}
                    }
                    ShadField{
                        ShadFieldLabel{text: "Status"}
                        ShadSelect{labels: ["Any status" "Open" "Blocked" "Done"]}
                    }
                }
                footer +: {
                    close_top_sheet_btn := ShadButtonOutline{text: "Close"}
                    apply_top_sheet_btn := ShadButton{text: "Apply filters"}
                }
            }
        }
    }
}

bottom_sheet := ShadSheet{
    side: "bottom"
    sheet_size: 220.0
    overlay +: {
        content +: {
            sheet_frame +: {
                header +: {
                    title +: {text: "Activity feed"}
                    description +: {text: "Bottom sheets suit notifications, activity, and short-lived supporting context."}
                }
                body +: {
                    ShadLabel{text: "Latest updates"}
                    ShadFieldDescription{text: "Lucas mentioned your team in release notes."}
                    ShadFieldDescription{text: "A new build finished successfully 3 minutes ago."}
                }
                footer +: {
                    close_bottom_sheet_btn := ShadButtonOutline{text: "Dismiss"}
                    mark_bottom_sheet_btn := ShadButton{text: "Mark all read"}
                }
            }
        }
    }
}

// Controller example (Rust):
// use makepad_components::sheet::ShadSheetWidgetExt;
//
// fn set_sheet_open(&mut self, cx: &mut Cx, path: &[LiveId], open: bool) {
//     let sheet = self.view.shad_sheet(cx, path);
//     if open { sheet.open(cx); } else { sheet.close(cx); }
// }
//
// if self.view.button(cx, ids!(open_right_sheet_btn)).clicked(actions) {
//     self.set_sheet_open(cx, ids!(right_sheet), true);
// }
// if self.view.button(cx, ids!(close_right_sheet_btn)).clicked(actions)
//     || self.view.button(cx, ids!(save_right_sheet_btn)).clicked(actions) {
//     self.set_sheet_open(cx, ids!(right_sheet), false);
// }
//
// if let Some(is_open) = self.view.shad_sheet(cx, ids!(right_sheet)).open_changed(actions) {
//     log!("Sheet open: {}", is_open);
// }"#;
