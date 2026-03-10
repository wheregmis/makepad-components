use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

#[derive(Clone)]
enum DrawState {
    DrawBody,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadCollapsibleBase = #(ShadCollapsible::register_widget(vm))

    mod.widgets.ShadCollapsible = set_type_default() do mod.widgets.ShadCollapsibleBase{
        width: Fill
        height: Fit
        flow: Down
        is_open: false
        active: 0.0
        title: "Collapsible"

        draw_bg +: {
            hover: instance(0.0)
            active: instance(0.0)
            header_height: uniform(44.0)
            border_size: uniform(1.0)
            corner_radius: uniform(shad_theme.radius)

            color_bg: uniform(shad_theme.color_secondary)
            color_hover: uniform(shad_theme.color_secondary_hover)
            color_border: uniform(shad_theme.color_outline_border)
            color_border_hover: uniform(shad_theme.color_outline_border_hover)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.clear(vec4(0.0))

                let border = mix(self.color_border, self.color_border_hover, self.hover)
                let hh = clamp(self.header_height, 0.0, self.rect_size.y)

                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.corner_radius)
                sdf.fill(self.color_bg)
                sdf.stroke(border, self.border_size)

                sdf.rect(1.0, 1.0, self.rect_size.x - 2.0, hh - 1.0)
                sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), self.color_hover, self.hover))

                sdf.rect(1.0, hh, self.rect_size.x - 2.0, 1.0)
                sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), border, self.active))

                return sdf.result
            }
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 11}
        }

        draw_icon +: {
            active: instance(0.0)
            hover: instance(0.0)

            color: uniform(shad_theme.color_muted_foreground)
            color_hover: uniform(shad_theme.color_primary)

            pixel: fn() {
                let sz = 4.0
                let c = self.rect_size * 0.5
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.clear(vec4(0.0))

                // Use filled triangle for crisp edges (stroke was thin/aliased).
                sdf.rotate(self.active * 0.5 * PI, c.x, c.y)
                sdf.move_to(c.x - sz, c.y - sz)
                sdf.line_to(c.x + sz, c.y)
                sdf.line_to(c.x - sz, c.y + sz)
                sdf.close_path()
                sdf.fill(mix(self.color, self.color_hover, self.hover))

                return sdf.result
            }
        }

        body: View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 16, right: 16, top: 0, bottom: 14}
            spacing: 8.0
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.10}}
                    ease: InOutQuad
                    apply: {
                        draw_bg: {hover: 0.0}
                        draw_icon: {hover: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.12}}
                    ease: InOutQuad
                    apply: {
                        draw_bg: {hover: 1.0}
                        draw_icon: {hover: 1.0}
                    }
                }
            }

            active: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.24}}
                    ease: InOutQuad
                    redraw: true
                    apply: {
                        active: 0.0
                        draw_bg: {active: 0.0}
                        draw_icon: {active: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.24}}
                    ease: InOutQuad
                    redraw: true
                    apply: {
                        active: 1.0
                        draw_bg: {active: 1.0}
                        draw_icon: {active: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadCollapsibleAction {
    #[default]
    None,
    Opening,
    Closing,
    Animating(f64),
}

#[derive(Script, Widget, Animator)]
pub struct ShadCollapsible {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    area: Area,
    #[rust]
    header_area: Area,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[redraw]
    #[live]
    draw_icon: DrawQuad,

    #[find]
    #[redraw]
    #[live]
    body: WidgetRef,

    #[live]
    title: String,
    #[live]
    is_open: bool,
    #[live]
    active: f64,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadCollapsible {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            self.animator_toggle(
                cx,
                self.is_open,
                animator::Animate::No,
                ids!(active.on),
                ids!(active.off),
            );
        });
    }
}

impl Widget for ShadCollapsible {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_is_open) {
            if let Some(args_obj) = args.as_object() {
                let trap = vm.bx.threads.cur().trap.pass();
                let value = vm.bx.heap.vec_value(args_obj, 0, trap);
                if let Some(is_open) = value.as_bool() {
                    vm.with_cx_mut(|cx| {
                        self.set_is_open(cx, is_open, animator::Animate::No);
                    });
                }
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(is_open) {
            let is_open = vm.with_cx(|cx| self.is_open(cx));
            return ScriptAsyncResult::Return(ScriptValue::from_bool(is_open));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            cx.widget_action_with_data(
                &self.action_data,
                uid,
                ShadCollapsibleAction::Animating(self.active),
            );
            self.area.redraw(cx);
        }

        if self.active > 0.0 {
            self.body.handle_event(cx, event, scope);
        }

        match event.hits(cx, self.header_area) {
            Hit::FingerDown(_) => {
                if self.animator_in_state(cx, ids!(active.on)) {
                    self.is_open = false;
                    self.animator_play(cx, ids!(active.off));
                    cx.widget_action_with_data(
                        &self.action_data,
                        uid,
                        ShadCollapsibleAction::Closing,
                    );
                } else {
                    self.is_open = true;
                    self.animator_play(cx, ids!(active.on));
                    cx.widget_action_with_data(
                        &self.action_data,
                        uid,
                        ShadCollapsibleAction::Opening,
                    );
                }
                self.animator_play(cx, ids!(hover.on));
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

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let header_height = 44.0;
        let icon_size = 12.0;

        if self.draw_state.begin(cx, DrawState::DrawBody) {
            self.draw_bg.begin(cx, walk, self.layout);

            let mut header_layout = Layout::flow_right().with_align_y(0.5);
            header_layout.padding = Inset {
                left: 16.0,
                right: 20.0,
                top: 10.0,
                bottom: 10.0,
            };
            header_layout.spacing = 8.0;

            cx.begin_turtle(
                Walk::new(Size::fill(), Size::Fixed(header_height)),
                header_layout,
            );
            self.draw_text.draw_walk(
                cx,
                Walk::new(Size::fill(), Size::fit()),
                Align { x: 0.0, y: 0.5 },
                self.title.as_ref(),
            );
            self.draw_icon
                .draw_walk(cx, Walk::fixed(icon_size, icon_size));
            cx.end_turtle_with_area(&mut self.header_area);
        }

        if let Some(DrawState::DrawBody) = self.draw_state.get() {
            if self.active > 0.0 {
                let body_walk = self.body.walk(cx);
                self.body.draw_walk(cx, scope, body_walk)?;
            }
            self.draw_bg.end(cx);
            self.area = self.draw_bg.area();
            self.draw_state.end();
        }
        DrawStep::done()
    }
}

impl ShadCollapsible {
    pub fn set_is_open(&mut self, cx: &mut Cx, is_open: bool, animate: animator::Animate) {
        self.is_open = is_open;
        self.animator_toggle(cx, is_open, animate, ids!(active.on), ids!(active.off));
        self.area.redraw(cx);
    }

    pub fn is_open(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(active.on))
    }

    pub fn opening(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(
                item.cast::<ShadCollapsibleAction>(),
                ShadCollapsibleAction::Opening
            )
        } else {
            false
        }
    }

    pub fn closing(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(
                item.cast::<ShadCollapsibleAction>(),
                ShadCollapsibleAction::Closing
            )
        } else {
            false
        }
    }

    pub fn animating(&self, actions: &Actions) -> Option<f64> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadCollapsibleAction::Animating(v) = item.cast() {
                return Some(v);
            }
        }
        None
    }
}

impl ShadCollapsibleRef {
    pub fn set_is_open(&self, cx: &mut Cx, is_open: bool, animate: animator::Animate) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_is_open(cx, is_open, animate);
        }
    }

    pub fn is_open(&self, cx: &Cx) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open(cx))
    }

    pub fn opening(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            inner.opening(actions)
        } else {
            false
        }
    }

    pub fn closing(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            inner.closing(actions)
        } else {
            false
        }
    }

    pub fn animating(&self, actions: &Actions) -> Option<f64> {
        if let Some(inner) = self.borrow() {
            inner.animating(actions)
        } else {
            None
        }
    }
}
