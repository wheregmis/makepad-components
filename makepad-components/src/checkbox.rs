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
        grab_key_focus: true

        draw_bg +: {
            hover: instance(0.0)
            focus: instance(0.0)
            checked_val: instance(0.0)

            color_border: uniform(shad_theme.color_outline_border)
            color_border_hover: uniform(shad_theme.color_outline_border_hover)
            color_primary: uniform(shad_theme.color_primary)
            color_checkmark: uniform(shad_theme.color_primary_foreground)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.clear(vec4(0.0))

                let sz = self.rect_size
                let box_radius = 2.0
                let inset = 1.5

                // Checkbox box: square with subtle rounding (Shadcn-style)
                sdf.box(inset, inset, sz.x - inset * 2.0, sz.y - inset * 2.0, box_radius)
                sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), self.color_primary, self.checked_val))

                // Border: visible when unchecked, fades when checked; thicker on hover
                let border_col = mix(self.color_border, self.color_border_hover, self.hover)
                let border_w = mix(1.0, 1.25, self.hover)
                sdf.box(inset, inset, sz.x - inset * 2.0, sz.y - inset * 2.0, box_radius)
                sdf.stroke(mix(border_col, vec4(0.0, 0.0, 0.0, 0.0), self.checked_val), border_w)

                // Checkmark: scaled down so tick has clear padding inside the box
                let cx = 0.51
                let cy = 0.48
                let scale = 0.72
                let x0 = sz.x * (cx + (0.22 - cx) * scale)
                let y0 = sz.y * (cy + (0.54 - cy) * scale)
                let x1 = sz.x * (cx + (0.42 - cx) * scale)
                let y1 = sz.y * (cy + (0.74 - cy) * scale)
                let x2 = sz.x * (cx + (0.80 - cx) * scale)
                let y2 = sz.y * (cy + (0.22 - cy) * scale)
                sdf.move_to(x0, y0)
                sdf.line_to(x1, y1)
                sdf.line_to(x2, y2)
                sdf.stroke(mix(vec4(0.0, 0.0, 0.0, 0.0), self.color_checkmark, self.checked_val), 1.75)

                // Focus ring (drawn last so it's visible)
                sdf.box(0.0, 0.0, sz.x, sz.y, box_radius + 1.0)
                sdf.stroke(mix(vec4(0.0, 0.0, 0.0, 0.0), self.color_primary, self.focus), 2.0)

                return sdf.result
            }
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 12}
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

            focus: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {focus: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Snap}
                    apply: {draw_bg: {focus: 1.0}}
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
    #[live]
    grab_key_focus: bool,

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
            Hit::KeyDown(ke) => {
                if let KeyCode::Space | KeyCode::ReturnKey = ke.key_code {
                    let checked = !self.checked;
                    if self.sync_checked_state(cx, checked, animator::Animate::Yes) {
                        cx.widget_action_with_data(
                            &self.action_data,
                            uid,
                            ShadCheckboxAction::Changed(checked),
                        );
                    }
                }
            }
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area);
                }
                let checked = !self.checked;
                if self.sync_checked_state(cx, checked, animator::Animate::Yes) {
                    cx.widget_action_with_data(
                        &self.action_data,
                        uid,
                        ShadCheckboxAction::Changed(checked),
                    );
                }
            }
            Hit::KeyFocus(_) => {
                self.animator_play(cx, ids!(focus.on));
            }
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, ids!(focus.off));
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
        let box_size = 18.0;

        let mut layout = Layout::flow_right().with_align_y(0.5);
        layout.spacing = 10.0;

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
    fn sync_checked_state(
        &mut self,
        cx: &mut Cx,
        checked: bool,
        animate: animator::Animate,
    ) -> bool {
        if self.checked == checked {
            return false;
        }

        self.checked = checked;
        self.animator_toggle(cx, checked, animate, ids!(checked.on), ids!(checked.off));
        self.area.redraw(cx);
        true
    }

    pub fn set_checked(&mut self, cx: &mut Cx, checked: bool, animate: animator::Animate) {
        self.sync_checked_state(cx, checked, animate);
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
        self.borrow().is_some_and(|inner| inner.is_checked())
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(inner) = self.borrow() {
            inner.changed(actions)
        } else {
            None
        }
    }
}
