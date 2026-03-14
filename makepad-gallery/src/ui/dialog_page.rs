use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::dialog::ShadDialogWidgetExt;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryDialogPage,
    page: dialog_page,
    title: "Dialog",
    subtitle: "Modal dialogs with variants: generic (custom body), alert (title + description + Cancel/Confirm), and destructive. Use ShadDialogRef::open/close plus `open_changed(actions)` to control visibility.",
    divider: { ShadHr{} },
    preview_spacing: 24.0,
    preview: {
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
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Grab a ShadDialogRef for the specific dialog instance your page owns."}
        mod.widgets.GalleryActionFlowStep{text: "2. Call open(cx) or close(cx) from page buttons, command handlers, or async completions."}
        mod.widgets.GalleryActionFlowStep{text: "3. Listen to `open_changed(actions)` when the rest of the page reacts to the dialog lifecycle."}
        mod.widgets.GalleryActionFlowStep{text: "4. Backdrop dismissal, Escape, and internal close controls remain inside the component, so the shell does not need to know those ids."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryDialogPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryDialogPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self.view.button(cx, ids!(open_dialog_btn)).clicked(actions) {
                self.view.shad_dialog(cx, ids!(default_dialog)).open(cx);
            }
            if self
                .view
                .button(cx, ids!(open_default_btn))
                .clicked(actions)
            {
                self.view
                    .shad_dialog(cx, ids!(alert_default_dialog))
                    .open(cx);
            }
            if self
                .view
                .button(cx, ids!(open_destructive_btn))
                .clicked(actions)
            {
                self.view.shad_dialog(cx, ids!(destructive_dialog)).open(cx);
            }

            let default_dialog = self.view.shad_dialog(cx, ids!(default_dialog));
            if default_dialog.button(cx, ids!(close_btn)).clicked(actions) {
                default_dialog.close(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
