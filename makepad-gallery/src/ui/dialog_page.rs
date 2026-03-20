use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::dialog::ShadDialogWidgetExt;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    root: ShadScrollArea,
    shell: {
        draw_bg.color: (shad_theme.color_background)
        flow: Overlay
    },
    widget: GalleryDialogPage,
    page: dialog_page,
    title: "Dialog",
    subtitle: "Workflow dialogs for short blocking tasks, confirmations, and destructive actions.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                ShadSectionHeader{text: "Workflow Examples"}
                ShadFieldDescription{text: "Use dialogs when the user needs to stop, make one focused decision, and return to the page with a clear next state."}

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
            }
        }

        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 8.0

                ShadSectionHeader{text: "When to use dialog"}
                ShadFieldDescription{text: "Choose dialog for blocking confirmations or one short workflow like renaming, publishing, or deleting. Use ShadSheet when the work can stay alongside the current screen, and use inline cards for content that should remain visible all the time."}
                ShadFieldDescription{text: "The dialog component owns the backdrop, Escape dismissal, and alert confirm/cancel behavior. Page code should only open or close the specific dialog instance it controls."}
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep one ShadDialogRef for each workflow your page can open, such as rename, publish, or delete."}
        mod.widgets.GalleryActionFlowStep{text: "2. Trigger `open(cx)` from the page button or row action that starts the workflow."}
        mod.widgets.GalleryActionFlowStep{text: "3. For generic dialogs, wire your own footer actions and call `close(cx)` when the workflow completes or cancels."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use `open_changed(actions)` only when the surrounding page needs to react to the dialog lifecycle; backdrop and Escape dismissal stay component-owned."}
    },
    after_root: {
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
                            title_label +: {
                                text: "Publish changes?"
                            }
                            description_label +: {
                                text: "Push the latest pricing copy and onboarding updates live for every workspace."
                            }
                        }

                        footer +: {
                            cancel +: {
                                text: "Keep editing"
                            }
                            confirm +: {
                                text: "Publish now"
                            }
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
                            title_label +: {
                                text: "Delete project?"
                            }
                            description_label +: {
                                text: "This permanently removes the project, API keys, and release history from your workspace."
                            }
                        }

                        footer +: {
                            cancel +: {
                                text: "Cancel"
                            }
                            confirm +: {
                                text: "Delete project"
                            }
                        }
                    }
                }
            }
        }
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryDialogPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryDialogPage {
    fn set_dialog_open(&mut self, cx: &mut Cx, path: &[LiveId], open: bool) {
        let dialog = self.view.shad_dialog(cx, path);
        if open {
            dialog.open(cx);
        } else {
            dialog.close(cx);
        }
    }
}

impl Widget for GalleryDialogPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            for (button, path) in [
                (ids!(open_rename_dialog_btn), ids!(rename_dialog)),
                (ids!(open_publish_dialog_btn), ids!(publish_dialog)),
                (ids!(open_delete_dialog_btn), ids!(delete_dialog)),
            ] {
                if self.view.button(cx, button).clicked(actions) {
                    self.set_dialog_open(cx, path, true);
                }
            }

            for button in [ids!(rename_cancel_btn), ids!(rename_save_btn)] {
                if self.view.button(cx, button).clicked(actions) {
                    self.set_dialog_open(cx, ids!(rename_dialog), false);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
