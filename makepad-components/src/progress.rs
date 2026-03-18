use crate::animation::{advance_phase, AnimationStep, AnimationTicker};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadProgressBase = mod.widgets.RoundedView{
        width: Fill
        height: 8

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

    mod.widgets.ShadProgress = mod.widgets.ShadProgressBase{}
    mod.widgets.ShadProgress33 = mod.widgets.ShadProgressBase{ draw_bg +: { progress: instance(0.33) } }
    mod.widgets.ShadProgress66 = mod.widgets.ShadProgressBase{ draw_bg +: { progress: instance(0.66) } }
    mod.widgets.ShadProgressFull = mod.widgets.ShadProgressBase{ draw_bg +: { progress: instance(1.0) } }

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
