use crate::button::ShadButtonVariant;
use crate::internal::actions::first_widget_action;
use crate::internal::overlay::{
    button_clicked, draw_modal_overlay, modal_dismissed, set_modal_widget_open,
    sync_modal_open_state,
};
use crate::internal::script_args::bool_arg;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Script, ScriptHook)]
#[repr(u32)]
pub enum ShadDialogAlertTone {
    #[pick]
    #[default]
    Default,
    Destructive,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ShadDialogAlertTone = set_type_default() do #(ShadDialogAlertTone::script_api(vm))
    mod.widgets.ShadDialogAlertTone = ShadDialogAlertTone

    let DialogPanel = mod.widgets.RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 0.0

        draw_bg +: {
            color: (shad_theme.color_popover)
            border_radius: (shad_theme.radius)
            border_size: (shad_theme.border_size)
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
        alert_mode: true
        alert_tone: ShadDialogAlertTone.Default
        alert_title_text: "Alert title"
        alert_description_text: "Alert description"
        alert_confirm_text: "Continue"
        alert_cancel_text: "Cancel"
        default_border_color: (shad_theme.color_outline_border)
        default_title_color: (shad_theme.color_primary)
        default_description_color: (shad_theme.color_muted_foreground)
        destructive_border_color: (shad_theme.color_destructive)
        destructive_title_color: (shad_theme.color_destructive)
        destructive_description_color: (shad_theme.color_destructive)
        destructive_confirm_bg: (shad_theme.color_destructive)
        destructive_confirm_bg_hover: (shad_theme.color_destructive_hover)
        destructive_confirm_bg_down: (shad_theme.color_destructive_down)
        destructive_confirm_text: (shad_theme.color_destructive_foreground)

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
                            text: "Alert title"
                        }
                        description_label := mod.widgets.ShadDialogDescription{
                            text: "Alert description"
                        }
                    }

                    footer := mod.widgets.ShadDialogFooter{
                        cancel := ShadButton{
                            variant: ShadButtonVariant.Outline
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

}

#[derive(Clone, Debug, Default)]
pub enum ShadDialogAction {
    OpenChanged(bool),
    #[default]
    None,
}

#[derive(Script, Widget)]
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
    #[live(false)]
    alert_mode: bool,
    #[live(ShadDialogAlertTone::Default)]
    alert_tone: ShadDialogAlertTone,
    #[live]
    alert_title_text: ArcStringMut,
    #[live]
    alert_description_text: ArcStringMut,
    #[live]
    alert_confirm_text: ArcStringMut,
    #[live]
    alert_cancel_text: ArcStringMut,
    #[live]
    default_border_color: Vec4,
    #[live]
    default_title_color: Vec4,
    #[live]
    default_description_color: Vec4,
    #[live]
    destructive_border_color: Vec4,
    #[live]
    destructive_title_color: Vec4,
    #[live]
    destructive_description_color: Vec4,
    #[live]
    destructive_confirm_bg: Vec4,
    #[live]
    destructive_confirm_bg_hover: Vec4,
    #[live]
    destructive_confirm_bg_down: Vec4,
    #[live]
    destructive_confirm_text: Vec4,
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

impl ScriptHook for ShadDialog {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        if !self.alert_mode {
            return;
        }

        let title_text = self.alert_title_text.as_ref().to_string();
        let description_text = self.alert_description_text.as_ref().to_string();
        let confirm_text = self.alert_confirm_text.as_ref().to_string();
        let cancel_text = self.alert_cancel_text.as_ref().to_string();
        let confirm_variant = match self.alert_tone {
            ShadDialogAlertTone::Default => ShadButtonVariant::Primary,
            ShadDialogAlertTone::Destructive => ShadButtonVariant::Destructive,
        };
        let (border_color, title_color, description_color) = match self.alert_tone {
            ShadDialogAlertTone::Default => (
                self.default_border_color,
                self.default_title_color,
                self.default_description_color,
            ),
            ShadDialogAlertTone::Destructive => (
                self.destructive_border_color,
                self.destructive_title_color,
                self.destructive_description_color,
            ),
        };

        vm.with_cx_mut(|cx| {
            let mut dialog_panel = self
                .overlay
                .widget(cx, ids!(content.dialog_panel));
            script_apply_eval!(cx, dialog_panel, {
                draw_bg +: {
                    border_color: #(border_color)
                }
            });

            let mut title = self
                .overlay
                .widget(cx, ids!(content.dialog_panel.dialog_body.title_label));
            if !title_text.is_empty() {
                script_apply_eval!(cx, title, {
                    text: #(title_text.clone())
                    draw_text.color: #(title_color)
                });
            } else {
                script_apply_eval!(cx, title, {
                    draw_text.color: #(title_color)
                });
            }

            let mut description = self
                .overlay
                .widget(cx, ids!(content.dialog_panel.dialog_body.description_label));
            if !description_text.is_empty() {
                script_apply_eval!(cx, description, {
                    text: #(description_text.clone())
                    draw_text.color: #(description_color)
                });
            } else {
                script_apply_eval!(cx, description, {
                    draw_text.color: #(description_color)
                });
            }

            if !cancel_text.is_empty() {
                let mut cancel = self
                    .overlay
                    .widget(cx, ids!(content.dialog_panel.footer.cancel));
                script_apply_eval!(cx, cancel, {
                    text: #(cancel_text.clone())
                });
            }

            if !confirm_text.is_empty() {
                let mut confirm = self
                    .overlay
                    .widget(cx, ids!(content.dialog_panel.footer.confirm));
                script_apply_eval!(cx, confirm, {
                    variant: #(confirm_variant)
                    text: #(confirm_text.clone())
                });

                if matches!(self.alert_tone, ShadDialogAlertTone::Destructive) {
                    script_apply_eval!(cx, confirm, {
                        draw_bg +: {
                            color: #(self.destructive_confirm_bg)
                            color_hover: #(self.destructive_confirm_bg_hover)
                            color_down: #(self.destructive_confirm_bg_down)
                            color_focus: #(self.destructive_confirm_bg_hover)
                        }
                        draw_text +: {
                            color: #(self.destructive_confirm_text)
                            color_hover: #(self.destructive_confirm_text)
                            color_down: #(self.destructive_confirm_text)
                            color_focus: #(self.destructive_confirm_text)
                        }
                    });
                }
            } else {
                let mut confirm = self
                    .overlay
                    .widget(cx, ids!(content.dialog_panel.footer.confirm));
                script_apply_eval!(cx, confirm, {
                    variant: #(confirm_variant)
                });
            }
        });
    }
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
                for path in [
                    &[
                        live_id!(content),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(cancel),
                    ][..],
                    &[
                        live_id!(content),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(confirm),
                    ][..],
                    &[
                        live_id!(content),
                        live_id!(body),
                        live_id!(footer),
                        live_id!(cancel),
                    ][..],
                    &[
                        live_id!(content),
                        live_id!(body),
                        live_id!(footer),
                        live_id!(confirm),
                    ][..],
                ] {
                    if button_clicked(&self.overlay, cx, path, actions) {
                        self.close(cx);
                        break;
                    }
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
