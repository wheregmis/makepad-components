use makepad_draw::draw_list_2d::DrawListExt;
use makepad_widgets::*;

// Route transition presets and runtime state.

use super::RouterWidget;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouterTransitionPreset {
    None,
    Fade,
    SlideLeft,
    SlideRight,
    Scale,
    SharedAxis,
}

impl RouterTransitionPreset {
    pub fn from_live_id(id: LiveId) -> Self {
        match id {
            x if x == live_id!(none) || x == live_id!(None) => RouterTransitionPreset::None,
            x if x == live_id!(fade) || x == live_id!(Fade) => RouterTransitionPreset::Fade,
            x if x == live_id!(slide_left) || x == live_id!(SlideLeft) => {
                RouterTransitionPreset::SlideLeft
            }
            x if x == live_id!(slide_right) || x == live_id!(SlideRight) => {
                RouterTransitionPreset::SlideRight
            }
            x if x == live_id!(scale) || x == live_id!(Scale) => RouterTransitionPreset::Scale,
            x if x == live_id!(shared_axis) || x == live_id!(SharedAxis) => {
                RouterTransitionPreset::SharedAxis
            }
            _ => RouterTransitionPreset::None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RouterTransitionSpec {
    pub preset: RouterTransitionPreset,
    pub duration: f64,
}

impl RouterTransitionSpec {
    pub fn none() -> Self {
        Self {
            preset: RouterTransitionPreset::None,
            duration: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RouterTransitionDirection {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RouterActionKind {
    Push,
    Pop,
    Replace,
}

#[derive(Clone, Debug)]
pub(super) struct RouterTransitionState {
    pub(super) from_route: LiveId,
    pub(super) to_route: LiveId,
    pub(super) preset: RouterTransitionPreset,
    pub(super) direction: RouterTransitionDirection,
    pub(super) start_time: Option<f64>,
    pub(super) duration: f64,
    pub(super) progress: f64,
}

impl RouterTransitionState {
    fn tick(&mut self, time: f64) -> bool {
        let start = self.start_time.get_or_insert(time);
        let elapsed = (time - *start).max(0.0);
        let mut t = elapsed / self.duration.max(0.000_1);
        if t >= 1.0 {
            t = 1.0;
        }
        self.progress = t;
        t >= 1.0
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct TransitionEffect {
    abs_pos: Vec2d,
    view_transform: Mat4f,
}

impl RouterWidget {
    fn route_transition_spec(&self, route_id: LiveId) -> Option<RouterTransitionSpec> {
        let preset_id = self.routes.transition_overrides.get(&route_id).copied()?;
        let preset = RouterTransitionPreset::from_live_id(preset_id);
        let duration = self
            .routes
            .transition_duration_overrides
            .get(&route_id)
            .copied()
            .unwrap_or(self.transition_duration);
        Some(RouterTransitionSpec { preset, duration })
    }

    fn default_transition_spec(&self, kind: RouterActionKind) -> RouterTransitionSpec {
        let preset_id = match kind {
            RouterActionKind::Push => self.push_transition,
            RouterActionKind::Pop => self.pop_transition,
            RouterActionKind::Replace => self.replace_transition,
        };
        let preset = RouterTransitionPreset::from_live_id(preset_id);
        if preset == RouterTransitionPreset::None {
            return RouterTransitionSpec::none();
        }
        RouterTransitionSpec {
            preset,
            duration: self.transition_duration,
        }
    }

    pub(super) fn start_transition(
        &mut self,
        cx: &mut Cx,
        from_route: Option<LiveId>,
        to_route: LiveId,
        kind: RouterActionKind,
        direction: RouterTransitionDirection,
        override_spec: Option<RouterTransitionSpec>,
    ) {
        if !self.transitions_enabled() {
            self.transition_rt.state = None;
            return;
        }
        let Some(from_route) = from_route else {
            self.transition_rt.state = None;
            return;
        };
        if from_route == to_route {
            self.transition_rt.state = None;
            return;
        }

        // Ensure the previous route gets a few more input events (especially FingerUp) so widgets
        // can release hover/pressed state even though we don't dispatch events to inactive routes.
        self.pointer_cleanup.route = Some(from_route);
        self.pointer_cleanup.budget = 8;

        let mut spec = override_spec
            .or_else(|| self.route_transition_spec(to_route))
            .unwrap_or_else(|| self.default_transition_spec(kind));

        if spec.preset == RouterTransitionPreset::None {
            self.transition_rt.state = None;
            return;
        }

        if !spec.duration.is_finite() || spec.duration <= 0.0 {
            spec.duration = self.transition_duration.max(0.000_1);
        }

        self.transition_rt.state = Some(RouterTransitionState {
            from_route,
            to_route,
            preset: spec.preset,
            direction,
            start_time: None,
            duration: spec.duration,
            progress: 0.0,
        });
        self.transition_rt.next_frame = cx.new_next_frame();
        self.redraw(cx);
    }

    pub(super) fn update_transition(&mut self, cx: &mut Cx, time: f64) {
        let Some(state) = &mut self.transition_rt.state else {
            return;
        };
        if !state.tick(time) {
            self.transition_rt.next_frame = cx.new_next_frame();
        } else {
            self.transition_rt.state = None;
        }
        self.redraw_transition_frame(cx);
    }

    pub(super) fn ease_in_out(t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    pub(super) fn compute_effect(
        preset: RouterTransitionPreset,
        direction: RouterTransitionDirection,
        t: f64,
        is_to: bool,
        rect: Rect,
    ) -> TransitionEffect {
        let t = Self::ease_in_out(t);

        let mut pos_from = rect.pos;
        let mut pos_to = rect.pos;
        let mut transform_from = Mat4f::identity();
        let mut transform_to = Mat4f::identity();

        match preset {
            RouterTransitionPreset::None => {}
            RouterTransitionPreset::Fade => {}
            RouterTransitionPreset::SlideLeft => {
                pos_from.x += -(rect.size.x * t);
                pos_to.x += rect.size.x * (1.0 - t);
            }
            RouterTransitionPreset::SlideRight => {
                pos_from.x += rect.size.x * t;
                pos_to.x += -(rect.size.x * (1.0 - t));
            }
            RouterTransitionPreset::Scale => {
                let center_x = (rect.pos.x + rect.size.x * 0.5) as f32;
                let center_y = (rect.pos.y + rect.size.y * 0.5) as f32;
                let from_s = (1.0 - 0.05 * t) as f32;
                let to_s = (0.95 + 0.05 * t) as f32;
                transform_from = Mat4f::mul(
                    &Mat4f::mul(
                        &Mat4f::translation(vec3(center_x, center_y, 0.0)),
                        &Mat4f::scale(from_s),
                    ),
                    &Mat4f::translation(vec3(-center_x, -center_y, 0.0)),
                );
                transform_to = Mat4f::mul(
                    &Mat4f::mul(
                        &Mat4f::translation(vec3(center_x, center_y, 0.0)),
                        &Mat4f::scale(to_s),
                    ),
                    &Mat4f::translation(vec3(-center_x, -center_y, 0.0)),
                );
            }
            RouterTransitionPreset::SharedAxis => {
                let dir = match direction {
                    RouterTransitionDirection::Forward => 1.0f32,
                    RouterTransitionDirection::Backward => -1.0f32,
                };
                pos_from.x += -(rect.size.x * 0.2) * t * (dir as f64);
                pos_to.x += (rect.size.x * 0.2) * (1.0 - t) * (dir as f64);

                let center_x = (rect.pos.x + rect.size.x * 0.5) as f32;
                let center_y = (rect.pos.y + rect.size.y * 0.5) as f32;
                let from_s = (1.0 - 0.02 * t) as f32;
                let to_s = (0.92 + 0.08 * t) as f32;
                transform_from = Mat4f::mul(
                    &Mat4f::mul(
                        &Mat4f::translation(vec3(center_x, center_y, 0.0)),
                        &Mat4f::scale(from_s),
                    ),
                    &Mat4f::translation(vec3(-center_x, -center_y, 0.0)),
                );
                transform_to = Mat4f::mul(
                    &Mat4f::mul(
                        &Mat4f::translation(vec3(center_x, center_y, 0.0)),
                        &Mat4f::scale(to_s),
                    ),
                    &Mat4f::translation(vec3(-center_x, -center_y, 0.0)),
                );
            }
        }

        let (abs_pos, view_transform) = if is_to {
            (pos_to, transform_to)
        } else {
            (pos_from, transform_from)
        };

        TransitionEffect {
            abs_pos,
            view_transform,
        }
    }

    pub(super) fn draw_route_into_draw_list(
        cx: &mut Cx2d,
        scope: &mut Scope,
        draw_list: &mut DrawList2d,
        route_widgets: &mut ComponentMap<LiveId, WidgetRef>,
        route_id: LiveId,
        rect: Rect,
        effect: TransitionEffect,
        force_redraw: bool,
    ) {
        let walk = Walk::abs_rect(Rect {
            pos: effect.abs_pos,
            size: rect.size,
        });
        let should_draw_contents = if force_redraw {
            draw_list.begin_always(cx);
            true
        } else {
            !draw_list.begin(cx, walk).is_not_redrawing()
        };

        let draw_list_id = draw_list.id();
        {
            let dl = &mut cx.cx.cx.draw_lists[draw_list_id];
            dl.draw_list_uniforms.view_shift = vec2(0.0, 0.0);
            dl.draw_list_uniforms.view_transform = effect.view_transform;
        }

        if !should_draw_contents {
            // Reused draw-lists still need walk advancement and transform updates each frame.
            cx.walk_turtle(walk);
            return;
        }

        if let Some(widget) = route_widgets.get_mut(&route_id) {
            let _ = widget.draw_walk(cx, scope, walk);
        }

        draw_list.end(cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use makepad_live_id::live_id;

    #[test]
    fn transition_preset_from_live_id_parses_known_values() {
        assert_eq!(
            RouterTransitionPreset::from_live_id(live_id!(Fade)),
            RouterTransitionPreset::Fade
        );
        assert_eq!(
            RouterTransitionPreset::from_live_id(live_id!(fade)),
            RouterTransitionPreset::Fade
        );
        assert_eq!(
            RouterTransitionPreset::from_live_id(live_id!(SlideLeft)),
            RouterTransitionPreset::SlideLeft
        );
        assert_eq!(
            RouterTransitionPreset::from_live_id(live_id!(shared_axis)),
            RouterTransitionPreset::SharedAxis
        );
        assert_eq!(
            RouterTransitionPreset::from_live_id(live_id!(does_not_exist)),
            RouterTransitionPreset::None
        );
    }

    #[test]
    fn transition_state_ticks_to_completion() {
        let mut state = RouterTransitionState {
            from_route: live_id!(a),
            to_route: live_id!(b),
            preset: RouterTransitionPreset::Fade,
            direction: RouterTransitionDirection::Forward,
            start_time: None,
            duration: 1.0,
            progress: 0.0,
        };

        assert!(!state.tick(10.0));
        assert_eq!(state.progress, 0.0);
        assert!(!state.tick(10.5));
        assert!((state.progress - 0.5).abs() < 1e-9);
        assert!(state.tick(11.0));
        assert_eq!(state.progress, 1.0);
        assert!(state.tick(20.0));
        assert_eq!(state.progress, 1.0);
    }
}
