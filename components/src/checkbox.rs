use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadCheckboxBase = #(ShadCheckbox::register_widget(vm))

    mod.widgets.ShadCheckbox = set_type_default() do mod.widgets.ShadCheckboxBase{
        width: Fit
        height: Fit
        label: "Checkbox"
        checked: false

        draw_bg +: {
            hover: instance(0.0)
            checked_val: instance(0.0)

            color_border: uniform(shad_theme.color_outline_border)
            color_border_hover: uniform(shad_theme.color_outline_border_hover)
            color_primary: uniform(shad_theme.color_primary)
            color_checkmark: uniform(shad_theme.color_primary_foreground)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.clear(vec4(0.0))

                let sz = self.rect_size

                // Box fill: transparent when unchecked, primary color when checked
                sdf.box(1.0, 1.0, sz.x - 2.0, sz.y - 2.0, 4.0)
                sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), self.color_primary, self.checked_val))

                // Border: visible when unchecked, fades to transparent when checked
                sdf.box(0.5, 0.5, sz.x - 1.0, sz.y - 1.0, 4.0)
                let border_col = mix(self.color_border, self.color_border_hover, self.hover)
                sdf.stroke(mix(border_col, vec4(0.0, 0.0, 0.0, 0.0), self.checked_val), 1.0)

                // Checkmark: transparent when unchecked, visible when checked
                let x0 = sz.x * 0.22
                let y0 = sz.y * 0.5
                let x1 = sz.x * 0.41
                let y1 = sz.y * 0.69
                let x2 = sz.x * 0.78
                let y2 = sz.y * 0.25
                sdf.move_to(x0, y0)
                sdf.line_to(x1, y1)
                sdf.line_to(x2, y2)
                sdf.stroke(mix(vec4(0.0, 0.0, 0.0, 0.0), self.color_checkmark, self.checked_val), 1.5)

                return sdf.result
            }
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 11}
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {hover: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Snap}
                    apply: {draw_bg: {hover: 1.0}}
                }
            }

            checked: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {checked_val: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {checked_val: 1.0}}
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadCheckboxAction {
    #[default]
    None,
    Changed(bool),
}

#[derive(Script, Widget, Animator)]
pub struct ShadCheckbox {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[rust]
    area: Area,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[redraw]
    #[live]
    draw_text: DrawText,

    #[live]
    label: String,
    #[live]
    checked: bool,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadCheckbox {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            self.animator_toggle(
                cx,
                self.checked,
                animator::Animate::No,
                ids!(checked.on),
                ids!(checked.off),
            );
        });
    }
}

impl Widget for ShadCheckbox {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_checked) {
            if let Some(args_obj) = args.as_object() {
                let trap = vm.bx.threads.cur().trap.pass();
                let value = vm.bx.heap.vec_value(args_obj, 0, trap);
                if let Some(checked) = value.as_bool() {
                    vm.with_cx_mut(|cx| {
                        self.set_checked(cx, checked, animator::Animate::No);
                    });
                }
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(checked) {
            return ScriptAsyncResult::Return(ScriptValue::from_bool(self.checked));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.area.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerDown(_) => {
                self.checked = !self.checked;
                self.animator_toggle(
                    cx,
                    self.checked,
                    animator::Animate::Yes,
                    ids!(checked.on),
                    ids!(checked.off),
                );
                cx.widget_action_with_data(
                    &self.action_data,
                    uid,
                    ShadCheckboxAction::Changed(self.checked),
                );
                self.area.redraw(cx);
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    if fe.device.has_hovers() {
                        self.animator_play(cx, ids!(hover.on));
                    } else {
                        self.animator_play(cx, ids!(hover.off));
                    }
                } else {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let box_size = 16.0;

        let mut layout = Layout::flow_right().with_align_y(0.5);
        layout.spacing = 8.0;

        cx.begin_turtle(walk, layout);

        self.draw_bg.draw_walk(cx, Walk::fixed(box_size, box_size));

        if !self.label.is_empty() {
            self.draw_text.draw_walk(
                cx,
                Walk::new(Size::fit(), Size::fit()),
                Align { x: 0.0, y: 0.5 },
                self.label.as_ref(),
            );
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl ShadCheckbox {
    pub fn set_checked(&mut self, cx: &mut Cx, checked: bool, animate: animator::Animate) {
        self.checked = checked;
        self.animator_toggle(cx, checked, animate, ids!(checked.on), ids!(checked.off));
        self.area.redraw(cx);
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadCheckboxAction::Changed(v) = item.cast() {
                return Some(v);
            }
        }
        None
    }
}

impl ShadCheckboxRef {
    pub fn set_checked(&self, cx: &mut Cx, checked: bool, animate: animator::Animate) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_checked(cx, checked, animate);
        }
    }

    pub fn is_checked(&self) -> bool {
        self.borrow().map_or(false, |inner| inner.is_checked())
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(inner) = self.borrow() {
            inner.changed(actions)
        } else {
            None
        }
    }
}
