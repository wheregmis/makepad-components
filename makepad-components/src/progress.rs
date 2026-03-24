use crate::animation::{advance_phase, AnimationStep, AnimationTicker};
use crate::internal::script_args::number_arg;
use makepad_widgets::*;

fn clamp_progress_value(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadProgressBase = #(ShadProgress::register_widget(vm))
    mod.widgets.ShadProgress = set_type_default() do mod.widgets.ShadProgressBase{
        width: Fill
        height: 8
        value: 0.5

        draw_bg +: {
            color: (shad_theme.color_secondary)
            color_fill: (shad_theme.color_primary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000
            progress: instance(0.5)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let p = clamp(self.progress, 0.0, 1.0)
                let fill_w = self.rect_size.x * p
                let r = max(1.0, self.border_radius)

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, r)
                sdf.fill_keep(self.color)

                if p > 0.001 {
                    sdf.box(0.0, 0.0, fill_w, self.rect_size.y, r)
                    sdf.intersect()
                    sdf.fill_keep(self.color_fill)
                }

                if self.border_size > 0.0 {
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, r)
                    sdf.stroke(self.border_color, self.border_size)
                }

                return sdf.result
            }
        }
    }

    mod.widgets.ShadProgress33 = mod.widgets.ShadProgress{ value: 0.33 }
    mod.widgets.ShadProgress66 = mod.widgets.ShadProgress{ value: 0.66 }
    mod.widgets.ShadProgressFull = mod.widgets.ShadProgress{ value: 1.0 }

    mod.widgets.ShadProgressIndeterminateBase = #(ShadProgressIndeterminate::register_widget(vm))
    mod.widgets.ShadProgressIndeterminate = set_type_default() do mod.widgets.ShadProgressIndeterminateBase{
        width: Fill
        height: 8
        animate: true
        animation_fps: 30.0
        sweep_duration: 1.5

        draw_bg +: {
            color: (shad_theme.color_secondary)
            color_fill: (shad_theme.color_primary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000
            bar_width: uniform(0.4)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let radius = max(1.0, self.border_radius)
                let bar_width = self.rect_size.x * clamp(self.bar_width, 0.01, 1.0)
                let start_x = -bar_width + (self.rect_size.x + bar_width) * fract(self.pad1)

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, radius)
                sdf.fill_keep(self.color)

                sdf.box(start_x, 0.0, bar_width, self.rect_size.y, radius)
                sdf.intersect()
                sdf.fill_keep(self.color_fill)

                if self.border_size > 0.0 {
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, radius)
                    sdf.stroke(self.border_color, self.border_size)
                }

                return sdf.result
            }
        }
    }
}

#[derive(Script, Widget)]
pub struct ShadProgress {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live(0.5)]
    value: f64,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[rust]
    area: Area,
}

impl ScriptHook for ShadProgress {
    fn on_after_apply(
        &mut self,
        _vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        self.value = clamp_progress_value(self.value);
    }
}

impl ShadProgress {
    fn normalized_value(&self) -> f64 {
        clamp_progress_value(self.value)
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: f64) {
        let value = clamp_progress_value(value);
        if (self.value - value).abs() <= f64::EPSILON {
            return;
        }
        self.value = value;
        self.area.redraw(cx);
    }

    pub fn value(&self) -> f64 {
        self.normalized_value()
    }
}

impl Widget for ShadProgress {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(value) {
            return ScriptAsyncResult::Return(ScriptValue::from_f64(self.normalized_value()));
        }
        if method == live_id!(set_value) {
            if let Some(value) = number_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_value(cx, value));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let value = self.normalized_value();
        self.draw_bg.begin(cx, walk, self.layout);
        script_apply_eval!(cx, self, {
            draw_bg +: {
                progress: #(value)
            }
        });
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl ShadProgressRef {
    pub fn value(&self) -> f64 {
        self.borrow().map(|inner| inner.value()).unwrap_or(0.0)
    }

    pub fn set_value(&self, cx: &mut Cx, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }
}

#[derive(Script, Widget)]
pub struct ShadProgressIndeterminate {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[rust]
    ticker: AnimationTicker,
    #[rust]
    area: Area,
    #[rust]
    phase: f32,
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live(true)]
    animate: bool,
    #[live(30.0)]
    animation_fps: f64,
    #[live(1.5)]
    sweep_duration: f32,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

impl ShadProgressIndeterminate {
    fn is_animating(&self) -> bool {
        self.animate && self.animation_fps > 0.0 && self.sweep_duration > 0.0001
    }

    fn is_area_visible(&self, cx: &Cx) -> bool {
        if !self.area.is_valid(cx) {
            return false;
        }
        let clipped = self.area.clipped_rect(cx);
        clipped.size.x > 0.0 && clipped.size.y > 0.0
    }

    fn sync_phase_to_shader(&mut self) {
        self.draw_bg.pad1 = self.phase;
    }

    fn advance(&mut self, delta: f64) {
        let cycles_per_second = 1.0 / self.sweep_duration.max(0.0001) as f64;
        self.phase = advance_phase(self.phase, delta, cycles_per_second);
        self.sync_phase_to_shader();
    }
}

impl ScriptHook for ShadProgressIndeterminate {
    fn on_after_apply(
        &mut self,
        _vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        self.sync_phase_to_shader();
    }
}

impl Widget for ShadProgressIndeterminate {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let animate = self.is_animating() && self.is_area_visible(cx);
        match self
            .ticker
            .handle_event(cx, event, animate, self.animation_fps)
        {
            AnimationStep::Redraw { delta } => {
                self.advance(delta);
                self.area.redraw(cx);
            }
            AnimationStep::Stop | AnimationStep::Idle => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let should_tick = self.is_animating() && cx.walk_turtle_would_be_visible(walk);
        self.ticker.ensure_started(cx, should_tick);
        self.sync_phase_to_shader();

        cx.begin_turtle(walk, self.layout);
        let rect = cx.turtle().rect();
        self.draw_bg.draw_abs(cx, rect);
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

#[cfg(test)]
mod tests {
    use super::clamp_progress_value;

    #[test]
    fn clamp_progress_value_keeps_fractional_values() {
        assert_eq!(clamp_progress_value(0.42), 0.42);
    }

    #[test]
    fn clamp_progress_value_clamps_out_of_range_values() {
        assert_eq!(clamp_progress_value(-1.0), 0.0);
        assert_eq!(clamp_progress_value(1.4), 1.0);
    }
}
