use crate::calendar::ShadCalendarWidgetRefExt;
use crate::internal::actions::{emit_widget_action, widget_action_map};
use crate::internal::script_args::{bool_arg, string_arg};
use crate::calendar::ShadDate;
use crate::popover::ShadPopoverWidgetExt;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadDatePickerBase = #(ShadDatePicker::register_widget(vm))

    mod.widgets.ShadDatePicker = set_type_default() do mod.widgets.ShadDatePickerBase{
        width: Fit
        height: Fit
        value: ""
        placeholder: "Pick a date"
        open: false

        popover := ShadPopover{
            open: false
            side: "bottom"
            align: "start"

            trigger := ShadButtonOutline{
                width: 220
                align: Align{x: 0.0, y: 0.5}
                text: "Pick a date"
            }

            content: View{
                width: Fit
                height: Fit
                calendar := ShadCalendar{}
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadDatePickerAction {
    Changed(ShadDate),
    OpenChanged(bool),
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadDatePicker {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    value: ArcStringMut,
    #[live]
    placeholder: ArcStringMut,
    #[live(false)]
    open: bool,

    #[rust]
    value_date: Option<ShadDate>,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadDatePicker {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.value_date = ShadDate::parse_iso(self.value.as_ref());
            self.sync_to_children(cx);
        });
    }
}

impl ShadDatePicker {
    fn format_value(&self) -> String {
        self.value_date
            .map(|value| value.format_iso())
            .unwrap_or_else(|| self.placeholder.as_ref().to_owned())
    }

    fn calendar_ref(&self, cx: &Cx) -> Option<crate::calendar::ShadCalendarRef> {
        let content = self.view.shad_popover(cx, ids!(popover)).content_widget();
        if content.is_empty() {
            None
        } else {
            Some(content.shad_calendar(cx, ids!(calendar)))
        }
    }

    fn sync_to_children(&mut self, cx: &mut Cx) {
        self.view
            .button(cx, ids!(popover.trigger))
            .set_text(cx, &self.format_value());
        let popover = self.view.shad_popover(cx, ids!(popover));
        popover.set_open(cx, self.open);
        if let Some(calendar) = self.calendar_ref(cx) {
            calendar.set_value(cx, self.value_date);
        }
    }

    fn emit_open_state(&self, cx: &mut Cx, open: bool) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadDatePickerAction::OpenChanged(open),
        );
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: Option<ShadDate>) {
        if self.value_date == value {
            return;
        }
        self.value_date = value;
        self.sync_to_children(cx);
        if let Some(value) = value {
            emit_widget_action(
                cx,
                &self.action_data,
                self.widget_uid(),
                ShadDatePickerAction::Changed(value),
            );
        }
    }

    pub fn clear(&mut self, cx: &mut Cx) {
        if self.value_date.take().is_some() {
            self.sync_to_children(cx);
        }
    }

    pub fn value(&self) -> Option<ShadDate> {
        self.value_date
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if self.open == open {
            return;
        }
        self.open = open;
        self.sync_to_children(cx);
        self.emit_open_state(cx, open);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn changed(&self, actions: &Actions) -> Option<ShadDate> {
        widget_action_map::<ShadDatePickerAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadDatePickerAction::Changed(value) = action {
                Some(value)
            } else {
                None
            }
        })
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        widget_action_map::<ShadDatePickerAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadDatePickerAction::OpenChanged(open) = action {
                Some(open)
            } else {
                None
            }
        })
    }
}

impl Widget for ShadDatePicker {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_value) {
            if let Some(next) = string_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_value(cx, ShadDate::parse_iso(&next)));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(clear) {
            vm.with_cx_mut(|cx| self.clear(cx));
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(value) {
            if let Some(date) = self.value_date {
                let value = date.format_iso();
                if let Some(inline) = ScriptValue::from_inline_string(&value) {
                    return ScriptAsyncResult::Return(inline);
                }
                return ScriptAsyncResult::Return(vm.bx.heap.new_string_from_str(&value).into());
            }
            return ScriptAsyncResult::Return(NIL);
        }
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
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let popover = self.view.shad_popover(cx, ids!(popover));
            if let Some(open) = popover.open_changed(actions) {
                if open != self.open {
                    self.open = open;
                    self.emit_open_state(cx, open);
                }
            }
            if let Some(calendar) = self.calendar_ref(cx) {
                if let Some(date) = calendar.changed(actions) {
                    self.set_value(cx, Some(date));
                    if self.open {
                        self.open = false;
                        popover.set_open(cx, false);
                        self.emit_open_state(cx, false);
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShadDatePickerRef {
    pub fn set_value(&self, cx: &mut Cx, value: Option<ShadDate>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }

    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear(cx);
        }
    }

    pub fn value(&self) -> Option<ShadDate> {
        self.borrow().and_then(|inner| inner.value())
    }

    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }

    pub fn changed(&self, actions: &Actions) -> Option<ShadDate> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }
}
