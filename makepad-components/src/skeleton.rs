use crate::animation::{advance_phase, AnimationStep, AnimationTicker};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSkeletonBase = #(ShadSkeleton::register_widget(vm))
    mod.widgets.ShadSkeleton = set_type_default() do mod.widgets.ShadSkeletonBase{
        width: 100
        height: 20
        animate: true
        animation_fps: 30.0
        shimmer_speed: 2.0

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let base = self.color
                let highlight = mix(base, vec4(1.0, 1.0, 1.0, base.w), 0.75)
                let band_width = clamp(self.rect_size.x * 0.35, 12.0, max(12.0, self.rect_size.x))
                let band_center = -band_width * 0.5 + (self.rect_size.x + band_width) * fract(self.pad1)
                let x = self.pos.x * self.rect_size.x
                let distance = abs(x - band_center) / max(1.0, band_width * 0.5)
                let taper = clamp(1.0 - distance, 0.0, 1.0)
                let blend = taper * taper * (3.0 - 2.0 * taper)
                let fill_color = mix(base, highlight, blend)

                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    max(1.0, self.border_radius),
                )
                sdf.fill_keep(fill_color)

                if self.border_size > 0.0 {
                    sdf.stroke(self.border_color, self.border_size)
                }

                return sdf.result
            }
        }
    }
}

#[derive(Script, Widget)]
pub struct ShadSkeleton {
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
    #[live(2.0)]
    shimmer_speed: f32,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

impl ShadSkeleton {
    fn is_animating(&self) -> bool {
        self.animate && self.animation_fps > 0.0 && self.shimmer_speed > 0.0001
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
        self.phase = advance_phase(self.phase, delta, self.shimmer_speed as f64);
        self.sync_phase_to_shader();
    }
}

impl ScriptHook for ShadSkeleton {
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

impl Widget for ShadSkeleton {
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
