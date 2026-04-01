use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::button::ShadButtonWidgetExt;
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
                ShadField{
                    width: Fill
                    spacing: 4.0
                    ShadFieldLabel{text: "Current project name"}
                    current_project_name_value := ShadLabel{text: "Northwind Revamp"}
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Right{wrap: true}
                    spacing: 12.0

                    open_rename_dialog_btn := ShadButton{
                        text: "Rename project"
                    }

                    open_publish_dialog_btn := ShadButton{
                        variant: ShadButtonVariant.Outline
                        text: "Publish changes"
                    }

                    open_delete_dialog_btn := ShadButton{
                        variant: ShadButtonVariant.Destructive
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
                ShadFieldDescription{text: "Use ShadDialogAlert when the flow is just title, description, tone, and confirm/cancel copy. Drop to ShadDialog when the body or footer needs custom fields and actions."}
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep one ShadDialogRef for each workflow your page can open, such as rename, publish, or delete."}
        mod.widgets.GalleryActionFlowStep{text: "2. Trigger `open(cx)` from the page button or row action that starts the workflow."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use ShadDialogAlert for prop-driven confirm flows, setting alert_tone, alert_title_text, alert_description_text, alert_confirm_text, and alert_cancel_text on the instance."}
        mod.widgets.GalleryActionFlowStep{text: "4. For generic dialogs, wire your own footer actions and call `close(cx)` when the workflow completes or cancels."}
        mod.widgets.GalleryActionFlowStep{text: "5. Use `open_changed(actions)` only when the surrounding page needs to react to the dialog lifecycle; backdrop and Escape dismissal stay component-owned."}
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

                        footer := ShadDialogFooter{
                            cancel := ShadButton{
                                variant: ShadButtonVariant.Outline
                                text: "Cancel"
                            }

                            confirm := ShadButton{
                                text: "Save changes"
                            }
                        }
                    }
                }
            }
        }

        publish_dialog := ShadDialogAlert{
            alert_title_text: "Publish changes?"
            alert_description_text: "Push the latest pricing copy and onboarding updates live for every workspace."
            alert_cancel_text: "Keep editing"
            alert_confirm_text: "Publish now"
        }

        delete_dialog := ShadDialogAlert{
            alert_tone: ShadDialogAlertTone.Destructive
            alert_title_text: "Delete project?"
            alert_description_text: "This permanently removes the project, API keys, and release history from your workspace."
            alert_cancel_text: "Cancel"
            alert_confirm_text: "Delete project"
        }
    },
}

#[derive(Script, Widget)]
pub struct GalleryDialogPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    rename_draft: String,
    #[rust]
    saved_project_name: String,
}

impl GalleryDialogPage {
    const DEFAULT_PROJECT_NAME: &'static str = "Northwind Revamp";

    fn ensure_initialized(&mut self) {
        if self.saved_project_name.is_empty() {
            self.saved_project_name = Self::DEFAULT_PROJECT_NAME.to_string();
        }
        if self.rename_draft.is_empty() {
            self.rename_draft = self.saved_project_name.clone();
        }
    }

    fn sync_project_name_label(&self, cx: &mut Cx) {
        self.view
            .label(cx, ids!(current_project_name_value))
            .set_text(cx, &self.saved_project_name);
    }

    fn sync_rename_input(&self, cx: &mut Cx) {
        self.view
            .text_input(cx, ids!(rename_project_input))
            .set_text(cx, &self.rename_draft);
    }

    fn set_dialog_open(&mut self, cx: &mut Cx, path: &[LiveId], open: bool) {
        let dialog = self.view.shad_dialog(cx, path);
        if open {
            dialog.open(cx);
        } else {
            dialog.close(cx);
        }
    }

    fn open_rename_dialog(&mut self, cx: &mut Cx) {
        self.ensure_initialized();
        self.rename_draft = self.saved_project_name.clone();
        self.sync_rename_input(cx);
        self.set_dialog_open(cx, ids!(rename_dialog), true);
        self.view
            .text_input(cx, ids!(rename_project_input))
            .set_key_focus(cx);
    }

    fn cancel_rename(&mut self, cx: &mut Cx) {
        self.ensure_initialized();
        self.rename_draft = self.saved_project_name.clone();
        self.sync_rename_input(cx);
        self.set_dialog_open(cx, ids!(rename_dialog), false);
    }

    fn commit_rename(&mut self, cx: &mut Cx) {
        self.ensure_initialized();
        let next_name = self.rename_draft.trim();
        if !next_name.is_empty() {
            self.saved_project_name = next_name.to_string();
        }
        self.rename_draft = self.saved_project_name.clone();
        self.sync_project_name_label(cx);
        self.sync_rename_input(cx);
        self.set_dialog_open(cx, ids!(rename_dialog), false);
    }
}

impl ScriptHook for GalleryDialogPage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        self.ensure_initialized();
        vm.with_cx_mut(|cx| {
            self.sync_project_name_label(cx);
            self.sync_rename_input(cx);
        });
    }
}

impl Widget for GalleryDialogPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            self.ensure_initialized();

            if self
                .view
                .shad_button(cx, ids!(open_rename_dialog_btn))
                .clicked(actions)
            {
                self.open_rename_dialog(cx);
            }

            for (button, path) in [
                (ids!(open_publish_dialog_btn), ids!(publish_dialog)),
                (ids!(open_delete_dialog_btn), ids!(delete_dialog)),
            ] {
                if self.view.shad_button(cx, button).clicked(actions) {
                    self.set_dialog_open(cx, path, true);
                }
            }

            let rename_input = self.view.text_input(cx, ids!(rename_project_input));
            if let Some(text) = rename_input.changed(actions) {
                self.rename_draft = text;
            }
            if let Some((text, _modifiers)) = rename_input.returned(actions) {
                self.rename_draft = text;
                self.commit_rename(cx);
                return;
            }

            if self
                .view
                .button(cx, ids!(rename_cancel_btn))
                .clicked(actions)
            {
                self.cancel_rename(cx);
                return;
            }

            if self.view.button(cx, ids!(rename_save_btn)).clicked(actions) {
                self.commit_rename(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
