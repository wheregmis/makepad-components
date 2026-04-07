use crate::internal::actions::open_changed_action;
use crate::internal::overlay::{
    button_clicked, draw_modal_overlay, modal_dismissed, set_modal_widget_open,
    sync_modal_open_state,
};
use crate::internal::script_args::bool_arg;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadDialogHeader = mod.widgets.ShadSurfaceHeader{}

    mod.widgets.ShadDialogTitle = mod.widgets.ShadAlertTitle{}

    mod.widgets.ShadDialogDescription = mod.widgets.ShadAlertDescription{}

    mod.widgets.ShadDialogContent = mod.widgets.ShadSurfaceContent{}

    mod.widgets.ShadDialogFooter = mod.widgets.ShadSurfaceFooter{}

    mod.widgets.ShadDialogBase = #(ShadDialog::register_widget(vm))

    mod.widgets.ShadDialog = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: (shad_theme.color_overlay)
            }

            content +: {
                width: 360
                height: Fit

                body := mod.widgets.ShadSurfacePanel{}
            }
        }
    }

    mod.widgets.ShadDialogAlert = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: (shad_theme.color_overlay)
            }

            content +: {
                width: 360
                height: Fit

                dialog_panel := mod.widgets.ShadSurfacePanel{
                    dialog_body := mod.widgets.ShadDialogHeader{
                        title_label := mod.widgets.ShadDialogTitle{
                            text: "Are you absolutely sure?"
                        }
                        description_label := mod.widgets.ShadDialogDescription{
                            text: "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    footer := mod.widgets.ShadDialogFooter{
                        cancel := ShadButtonOutline{
                            text: "Cancel"
                        }

                        confirm := ShadButton{
                            text: "Continue"
                        }
                    }
                }
            }
        }
    }

    mod.widgets.ShadDialogAlertDestructive = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: (shad_theme.color_overlay)
            }

            content +: {
                width: 360
                height: Fit

                dialog_panel := mod.widgets.ShadSurfacePanel{
                    dialog_body := mod.widgets.ShadDialogHeader{
                        title_label := mod.widgets.ShadDialogTitle{
                            text: "Are you absolutely sure?"
                        }
                        description_label := mod.widgets.ShadDialogDescription{
                            text: "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    footer := mod.widgets.ShadDialogFooter{
                        cancel := ShadButtonOutline{
                            text: "Cancel"
                        }

                        confirm := ShadButtonDestructive{
                            text: "Delete"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadDialogAction {
    OpenChanged(bool),
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadDialog {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,

    #[find]
    #[redraw]
    #[live]
    overlay: WidgetRef,

    #[live]
    open: bool,
    #[rust]
    is_synced_open: bool,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadDialog {
    fn sync_open_state(&mut self, cx: &mut Cx) {
        sync_modal_open_state(cx, &mut self.overlay, &mut self.is_synced_open, self.open);
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        let uid = self.widget_uid();
        set_modal_widget_open(
            cx,
            &mut self.overlay,
            &mut self.open,
            &mut self.is_synced_open,
            &self.action_data,
            uid,
            open,
            ShadDialogAction::OpenChanged,
        );
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.set_open(cx, true);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.set_open(cx, false);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        open_changed_action::<ShadDialogAction, _>(actions, self.widget_uid(), |action| {
            if let ShadDialogAction::OpenChanged(open) = action {
                Some(open)
            } else {
                None
            }
        })
    }
}

impl Widget for ShadDialog {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_open) {
            if let Some(open) = bool_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_open(cx, open));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(is_open) {
            return ScriptAsyncResult::Return(ScriptValue::from_bool(self.open));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_open_state(cx);

        if self.open {
            self.overlay.handle_event(cx, event, scope);
            // Close when modal is dismissed (backdrop/Escape) or when cancel/confirm clicked (alert variants)
            if let Event::Actions(actions) = event {
                if modal_dismissed(&self.overlay, cx, actions) {
                    self.close(cx);
                }
                if button_clicked(
                    &self.overlay,
                    cx,
                    &[
                        live_id!(content),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(cancel),
                    ],
                    actions,
                ) {
                    self.close(cx);
                }
                if button_clicked(
                    &self.overlay,
                    cx,
                    &[
                        live_id!(content),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(confirm),
                    ],
                    actions,
                ) {
                    self.close(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_open_state(cx);
        draw_modal_overlay(cx, scope, walk, self.layout, self.open, &mut self.overlay)
    }
}

impl ShadDialogRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }
}
