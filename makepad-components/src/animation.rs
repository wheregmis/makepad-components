use makepad_widgets::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AnimationStep {
    Stop,
    Idle,
    Redraw { delta: f64 },
}

#[derive(Clone, Debug, Default)]
pub(crate) struct AnimationClock {
    last_tick: Option<f64>,
}

impl AnimationClock {
    pub(crate) fn reset(&mut self) {
        self.last_tick = None;
    }

    pub(crate) fn step(&mut self, time: f64, animate: bool, fps: f64) -> AnimationStep {
        if !animate {
            self.reset();
            return AnimationStep::Stop;
        }

        let Some(last_tick) = self.last_tick else {
            self.last_tick = Some(time);
            return AnimationStep::Idle;
        };

        let delta = (time - last_tick).max(0.0);
        let min_delta = 1.0 / fps.max(1.0);
        if delta + 1e-9 < min_delta {
            return AnimationStep::Idle;
        }

        self.last_tick = Some(time);
        AnimationStep::Redraw { delta }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct AnimationTicker {
    next_frame: NextFrame,
    clock: AnimationClock,
}

impl AnimationTicker {
    pub(crate) fn ensure_started(&mut self, cx: &mut Cx, animate: bool) {
        if animate {
            if self.next_frame == NextFrame::default() {
                self.next_frame = cx.new_next_frame();
            }
        } else {
            self.stop();
        }
    }

    pub(crate) fn stop(&mut self) {
        self.next_frame = NextFrame::default();
        self.clock.reset();
    }

    pub(crate) fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        animate: bool,
        fps: f64,
    ) -> AnimationStep {
        let Some(next_frame) = self.next_frame.is_event(event) else {
            if !animate {
                self.stop();
            }
            return AnimationStep::Idle;
        };

        let step = self.clock.step(next_frame.time, animate, fps);
        match step {
            AnimationStep::Stop => self.next_frame = NextFrame::default(),
            AnimationStep::Idle | AnimationStep::Redraw { .. } => {
                self.next_frame = cx.new_next_frame();
            }
        }
        step
    }
}

pub(crate) fn advance_phase(phase: f32, delta: f64, cycles_per_second: f64) -> f32 {
    let next = phase as f64 + delta.max(0.0) * cycles_per_second.max(0.0);
    next.rem_euclid(1.0) as f32
}

#[cfg(test)]
mod tests {
    use super::{advance_phase, AnimationClock, AnimationStep};

    #[test]
    fn advance_phase_wraps_into_unit_interval() {
        let phase = advance_phase(0.9, 0.3, 1.0);
        assert!((phase - 0.2).abs() < 1e-6);
    }

    #[test]
    fn animation_clock_throttles_by_requested_fps() {
        let mut clock = AnimationClock::default();

        assert_eq!(clock.step(0.0, true, 30.0), AnimationStep::Idle);
        assert_eq!(clock.step(0.01, true, 30.0), AnimationStep::Idle);
        match clock.step(0.04, true, 30.0) {
            AnimationStep::Redraw { delta } => assert!((delta - 0.04).abs() < 1e-9),
            other => panic!("expected redraw step, got {other:?}"),
        }
    }

    #[test]
    fn animation_clock_resumes_cleanly_after_reset() {
        let mut clock = AnimationClock::default();

        assert_eq!(clock.step(0.0, true, 30.0), AnimationStep::Idle);

        clock.reset();

        assert_eq!(clock.step(0.08, true, 30.0), AnimationStep::Idle);
        match clock.step(0.12, true, 30.0) {
            AnimationStep::Redraw { delta } => assert!((delta - 0.04).abs() < 1e-9),
            other => panic!("expected redraw step after resume, got {other:?}"),
        }
    }

    #[test]
    fn animation_clock_stops_when_animation_is_disabled() {
        let mut clock = AnimationClock::default();
        assert_eq!(clock.step(0.0, false, 30.0), AnimationStep::Stop);
    }
}
