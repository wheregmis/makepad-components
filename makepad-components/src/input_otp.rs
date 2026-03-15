use crate::internal::script_args::string_arg;
use crate::models::input_otp::{sanitize as sanitize_otp, visible_cells as clamped_visible_cells};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

const MAX_OTP_SLOTS: usize = 6;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadInputOtpSlot = RoundedView{
        width: 48
        height: 56
        align: Align{x: 0.5, y: 0.5}
        draw_bg +: {
            color: #0000
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadInputOtpBase = #(ShadInputOtp::register_widget(vm))

    mod.widgets.ShadInputOtp = set_type_default() do mod.widgets.ShadInputOtpBase{
        width: Fit
        height: Fit
        flow: Overlay
        cell_count: 6
        value: ""

        slot_wrap := View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            slot_0 := mod.widgets.ShadInputOtpSlot{
                label_0 := Label{
                    text: ""
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_1 := mod.widgets.ShadInputOtpSlot{
                label_1 := Label{
                    text: ""
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_2 := mod.widgets.ShadInputOtpSlot{
                label_2 := Label{
                    text: ""
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_3 := mod.widgets.ShadInputOtpSlot{
                label_3 := Label{
                    text: ""
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_4 := mod.widgets.ShadInputOtpSlot{
                label_4 := Label{
                    text: ""
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_5 := mod.widgets.ShadInputOtpSlot{
                label_5 := Label{
                    text: ""
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
        }

        controller := TextInput{
            width: Fill
            height: Fill
            empty_text: ""
            is_numeric_only: true
            padding: Inset{left: 0, right: 0, top: 0, bottom: 0}

            draw_bg +: {
                border_size: 0.0
                color: #0000
                color_hover: #0000
                color_focus: #0000
                color_down: #0000
                color_empty: #0000
                color_disabled: #0000
                border_color: #0000
                border_color_hover: #0000
                border_color_focus: #0000
                border_color_down: #0000
                border_color_empty: #0000
                border_color_disabled: #0000
            }

            draw_text +: {
                color: #0000
                color_hover: #0000
                color_focus: #0000
                color_down: #0000
                color_empty: #0000
                color_disabled: #0000
            }

            draw_cursor +: {
                color: #0000
            }

            draw_selection +: {
                color: #0000
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadInputOtpAction {
    Changed(String),
    Completed(String),
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadInputOtp {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    cell_count: u32,
    #[live]
    value: String,

    #[rust]
    last_completed: Option<String>,
    #[rust]
    synced_controller_value: String,
    #[rust]
    synced_slots_value: String,
    #[rust]
    synced_visible_cells: usize,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadInputOtp {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.value = self.sanitize(&self.value);
            self.sync_controller(cx);
            self.sync_slots(cx);
        });
    }
}

impl ShadInputOtp {
    fn controller_ref(&self, cx: &Cx) -> TextInputRef {
        self.view.text_input(cx, ids!(controller))
    }

    fn visible_cells(&self) -> usize {
        clamped_visible_cells(self.cell_count, MAX_OTP_SLOTS)
    }

    fn sanitize(&self, value: &str) -> String {
        sanitize_otp(value, self.visible_cells())
    }

    fn slot_ref(&self, cx: &Cx, index: usize) -> ViewRef {
        match index {
            0 => self.view.view(cx, ids!(slot_0)),
            1 => self.view.view(cx, ids!(slot_1)),
            2 => self.view.view(cx, ids!(slot_2)),
            3 => self.view.view(cx, ids!(slot_3)),
            4 => self.view.view(cx, ids!(slot_4)),
            _ => self.view.view(cx, ids!(slot_5)),
        }
    }

    fn set_slot_label(&self, cx: &mut Cx, index: usize, value: &str) {
        match index {
            0 => self.view.label(cx, ids!(label_0)).set_text(cx, value),
            1 => self.view.label(cx, ids!(label_1)).set_text(cx, value),
            2 => self.view.label(cx, ids!(label_2)).set_text(cx, value),
            3 => self.view.label(cx, ids!(label_3)).set_text(cx, value),
            4 => self.view.label(cx, ids!(label_4)).set_text(cx, value),
            _ => self.view.label(cx, ids!(label_5)).set_text(cx, value),
        }
    }

    fn sync_slots(&mut self, cx: &mut Cx) {
        let visible_cells = self.visible_cells();
        if self.synced_slots_value == self.value && self.synced_visible_cells == visible_cells {
            return;
        }

        let mut chars = self.value.chars();

        for index in 0..MAX_OTP_SLOTS {
            let slot = self.slot_ref(cx, index);
            let is_visible = index < visible_cells;
            slot.set_visible(cx, is_visible);
            if !is_visible {
                continue;
            }

            let digit = chars.next().map(|c| c.to_string()).unwrap_or_default();
            self.set_slot_label(cx, index, &digit);
        }

        self.synced_slots_value.clone_from(&self.value);
        self.synced_visible_cells = visible_cells;
    }

    fn emit_completed_if_needed(&mut self, cx: &mut Cx) {
        if self.value.len() == self.visible_cells() {
            if self.last_completed.as_deref() != Some(self.value.as_str()) {
                self.last_completed = Some(self.value.clone());
                cx.widget_action_with_data(
                    &self.action_data,
                    self.widget_uid(),
                    ShadInputOtpAction::Completed(self.value.clone()),
                );
            }
        } else {
            self.last_completed = None;
        }
    }

    fn sync_controller(&mut self, cx: &mut Cx) {
        if self.synced_controller_value == self.value {
            return;
        }

        self.controller_ref(cx).set_text(cx, &self.value);
        self.synced_controller_value.clone_from(&self.value);
    }

    pub fn set_value(&mut self, cx: &mut Cx, next: String) {
        let sanitized = self.sanitize(&next);
        if sanitized != self.value {
            self.value = sanitized.clone();
            cx.widget_action_with_data(
                &self.action_data,
                self.widget_uid(),
                ShadInputOtpAction::Changed(sanitized),
            );
        }
        self.emit_completed_if_needed(cx);
        self.sync_controller(cx);
        self.sync_slots(cx);
    }

    pub fn changed(&self, actions: &Actions) -> Option<String> {
        for action in actions.filter_widget_actions_cast::<ShadInputOtpAction>(self.widget_uid()) {
            if let ShadInputOtpAction::Changed(value) = action {
                return Some(value);
            }
        }
        None
    }

    pub fn completed(&self, actions: &Actions) -> Option<String> {
        for action in actions.filter_widget_actions_cast::<ShadInputOtpAction>(self.widget_uid()) {
            if let ShadInputOtpAction::Completed(value) = action {
                return Some(value);
            }
        }
        None
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Widget for ShadInputOtp {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_value) {
            if let Some(next) = string_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_value(cx, next));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(value) {
            if let Some(value) = ScriptValue::from_inline_string(&self.value) {
                return ScriptAsyncResult::Return(value);
            }
            return ScriptAsyncResult::Return(vm.bx.heap.new_string_from_str(&self.value));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let controller = self.controller_ref(cx);
            if let Some(text) = controller.changed(actions) {
                self.set_value(cx, text);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_controller(cx);
        self.sync_slots(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShadInputOtpRef {
    pub fn set_value(&self, cx: &mut Cx, value: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value.into());
        }
    }

    pub fn changed(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }

    pub fn completed(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.completed(actions))
    }

    pub fn value(&self) -> Option<String> {
        self.borrow().map(|inner| inner.value().to_string())
    }
}
