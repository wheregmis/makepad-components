use crate::internal::actions::first_widget_action;
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

    let DialogPanel = mod.widgets.RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 0.0

        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadDialogHeader = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 6.0
        padding: Inset{left: 20, right: 20, top: 20, bottom: 12}
    }

    mod.widgets.ShadDialogTitle = mod.widgets.ShadAlertTitle{}

    mod.widgets.ShadDialogDescription = mod.widgets.ShadAlertDescription{}

    mod.widgets.ShadDialogContent = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 12.0
        padding: Inset{left: 20, right: 20, top: 0, bottom: 16}
    }

    mod.widgets.ShadDialogFooter = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{x: 1.0, y: 0.5}
        spacing: 8.0
        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
    }

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

                body := DialogPanel{}
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

                dialog_panel := DialogPanel{
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

                dialog_panel := DialogPanel{
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
    #[rust]
    last_content_width: Option<f64>,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadDialog {
    const VIEWPORT_MARGIN: f64 = 12.0;
    const DEFAULT_CONTENT_WIDTH: f64 = 360.0;

    fn sync_content_width(&mut self, cx: &mut Cx) {
        let available_width = (cx.current_pass_size().x - Self::VIEWPORT_MARGIN * 2.0).max(0.0);
        let content_width = Self::DEFAULT_CONTENT_WIDTH.min(available_width);
        if self.last_content_width == Some(content_width) {
            return;
        }
        self.last_content_width = Some(content_width);

        let mut content = self.overlay.widget(cx, ids!(content));
        script_apply_eval!(cx, content, {
            width: #(content_width)
        });
    }

    fn sync_open_state(&mut self, cx: &mut Cx) {
        self.sync_content_width(cx);
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
        if let Some(ShadDialogAction::OpenChanged(open)) =
            first_widget_action::<ShadDialogAction>(actions, self.widget_uid())
        {
            return Some(open);
        }
        None
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
