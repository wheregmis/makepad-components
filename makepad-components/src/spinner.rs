use crate::animation::{AnimationStep, AnimationTicker};
use crate::button::ShadControlSize;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let SpinnerVisual = mod.widgets.RoundedView{
        width: Fill
        height: Fill

        draw_bg +: {
            color: uniform(shad_theme.color_primary)
            rotation_speed: uniform(1.0)
            stroke_width: uniform(2.5)
            arc_gap: uniform(0.25)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let radius = min(self.rect_size.x * 0.5, self.rect_size.y * 0.5) - self.stroke_width * 0.5
                let center = self.rect_size * 0.5
                let rotation = self.draw_pass.time * self.rotation_speed * 2.0 * PI
                let gap_radians = self.arc_gap * 2.0 * PI
                let start_angle = rotation
                sdf.arc_round_caps(
                    center.x
                    center.y
                    radius
                    start_angle
                    start_angle + 2.0 * PI - gap_radians
                    self.stroke_width
                )
                return sdf.fill(self.color)
            }
        }
    }

    mod.widgets.ShadSpinnerBase = #(ShadSpinner::register_widget(vm))

    mod.widgets.ShadSpinner = set_type_default() do mod.widgets.ShadSpinnerBase{
        width: 24
        height: 24
        animate: true
        animation_fps: 24.0
        size: ShadControlSize.Default

        spinner_body := SpinnerVisual{}
    }

}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ManagedSpinnerSize {
    diameter: f64,
    stroke_width: f32,
}

fn spinner_size_spec(
    size: ShadControlSize,
    small_diameter: f64,
    default_diameter: f64,
    large_diameter: f64,
    small_stroke_width: f64,
    default_stroke_width: f64,
    large_stroke_width: f64,
) -> ManagedSpinnerSize {
    match size {
        ShadControlSize::Small => ManagedSpinnerSize {
            diameter: small_diameter,
            stroke_width: small_stroke_width as f32,
        },
        ShadControlSize::Default => ManagedSpinnerSize {
            diameter: default_diameter,
            stroke_width: default_stroke_width as f32,
        },
        ShadControlSize::Large => ManagedSpinnerSize {
            diameter: large_diameter,
            stroke_width: large_stroke_width as f32,
        },
    }
}

#[derive(Script, Widget)]
pub struct ShadSpinner {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live(true)]
    animate: bool,
    #[live(24.0)]
    animation_fps: f64,
    #[live(ShadControlSize::Default)]
    size: ShadControlSize,
    #[live(16.0)]
    size_small_diameter: f64,
    #[live(24.0)]
    size_default_diameter: f64,
    #[live(32.0)]
    size_large_diameter: f64,
    #[live(2.0)]
    size_small_stroke_width: f64,
    #[live(2.5)]
    size_default_stroke_width: f64,
    #[live(3.0)]
    size_large_stroke_width: f64,
    #[rust]
    applied_size: Option<ManagedSpinnerSize>,
    #[rust]
    ticker: AnimationTicker,
}

impl ShadSpinner {
    fn managed_size(&self) -> ManagedSpinnerSize {
        spinner_size_spec(
            self.size,
            self.size_small_diameter,
            self.size_default_diameter,
            self.size_large_diameter,
            self.size_small_stroke_width,
            self.size_default_stroke_width,
            self.size_large_stroke_width,
        )
    }

    fn sync_managed_size(&mut self, cx: &mut Cx) {
        let size = self.managed_size();
        if self.applied_size == Some(size) {
            return;
        }

        self.view.walk.width = Size::Fixed(size.diameter);
        self.view.walk.height = Size::Fixed(size.diameter);

        let mut spinner_body = self.view.widget(cx, ids!(spinner_body));
        script_apply_eval!(cx, spinner_body, {
            draw_bg +: {
                stroke_width: #(size.stroke_width)
            }
        });

        self.applied_size = Some(size);
        self.view.redraw(cx);
    }

    fn is_visible(&self, cx: &Cx) -> bool {
        let area = self.view.area();
        if !area.is_valid(cx) {
            return false;
        }
        let clipped = area.clipped_rect(cx);
        clipped.size.x > 0.0 && clipped.size.y > 0.0
    }
}

impl ScriptHook for ShadSpinner {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| self.sync_managed_size(cx));
    }
}

impl Widget for ShadSpinner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let animate = self.animate && self.animation_fps > 0.0 && self.is_visible(cx);
        if let AnimationStep::Redraw { .. } =
            self.ticker
                .handle_event(cx, event, animate, self.animation_fps)
        {
            self.view.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_managed_size(&mut *cx);
        let should_tick =
            self.animate && self.animation_fps > 0.0 && cx.walk_turtle_would_be_visible(walk);
        self.ticker.ensure_started(cx, should_tick);
        self.view.draw_walk(cx, scope, walk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_size_specs_match_expected_tokens() {
        assert_eq!(
            spinner_size_spec(ShadControlSize::Small, 16.0, 24.0, 32.0, 2.0, 2.5, 3.0),
            ManagedSpinnerSize {
                diameter: 16.0,
                stroke_width: 2.0,
            }
        );

        assert_eq!(
            spinner_size_spec(ShadControlSize::Default, 16.0, 24.0, 32.0, 2.0, 2.5, 3.0),
            ManagedSpinnerSize {
                diameter: 24.0,
                stroke_width: 2.5,
            }
        );

        assert_eq!(
            spinner_size_spec(ShadControlSize::Large, 16.0, 24.0, 32.0, 2.0, 2.5, 3.0),
            ManagedSpinnerSize {
                diameter: 32.0,
                stroke_width: 3.0,
            }
        );
    }
}
