use crate::internal::actions::{emit_widget_action, widget_action_map};
use crate::internal::script_args::bool_arg;
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
    OpenChanged(bool),
    AnimationProgress(f64),
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
        if method == live_id!(set_open) {
            if let Some(is_open) = bool_arg(vm, args) {
                vm.with_cx_mut(|cx| {
                    self.set_open(cx, is_open, animator::Animate::No);
                });
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
            emit_widget_action(
                cx,
                &self.action_data,
                uid,
                ShadCollapsibleAction::AnimationProgress(self.active),
            );
            self.area.redraw(cx);
        }

        if self.active > 0.0 {
            self.body.handle_event(cx, event, scope);
        }

        match event.hits(cx, self.header_area) {
            Hit::FingerDown(_) => {
                let next_is_open = !self.animator_in_state(cx, ids!(active.on));
                if self.sync_open_state(cx, next_is_open, animator::Animate::Yes) {
                    emit_widget_action(
                        cx,
                        &self.action_data,
                        uid,
                        ShadCollapsibleAction::OpenChanged(next_is_open),
                    );
                }
                self.animator_play(cx, ids!(hover.on));
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
    fn sync_open_state(&mut self, cx: &mut Cx, is_open: bool, animate: animator::Animate) -> bool {
        if self.is_open == is_open {
            return false;
        }

        self.is_open = is_open;
        self.animator_toggle(cx, is_open, animate, ids!(active.on), ids!(active.off));
        self.area.redraw(cx);
        true
    }

    pub fn set_open(&mut self, cx: &mut Cx, is_open: bool, animate: animator::Animate) {
        self.sync_open_state(cx, is_open, animate);
    }

    pub fn is_open(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(active.on))
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        widget_action_map::<ShadCollapsibleAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadCollapsibleAction::OpenChanged(open) = action {
                Some(open)
            } else {
                None
            }
        })
    }

    pub fn animation_progress(&self, actions: &Actions) -> Option<f64> {
        widget_action_map::<ShadCollapsibleAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadCollapsibleAction::AnimationProgress(value) = action {
                Some(value)
            } else {
                None
            }
        })
    }
}

impl ShadCollapsibleRef {
    pub fn set_open(&self, cx: &mut Cx, is_open: bool, animate: animator::Animate) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, is_open, animate);
        }
    }

    pub fn is_open(&self, cx: &Cx) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open(cx))
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }

    pub fn animation_progress(&self, actions: &Actions) -> Option<f64> {
        self.borrow().and_then(|inner| inner.animation_progress(actions))
    }
}
