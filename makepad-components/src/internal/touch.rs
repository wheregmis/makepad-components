use makepad_widgets::*;

pub(crate) fn is_primary_tap(fe: &FingerUpEvent) -> bool {
    fe.is_primary_hit() && fe.is_over && fe.was_tap()
}

#[cfg(test)]
mod tests {
    use super::is_primary_tap;
    use makepad_widgets::event::DigitId;
    use makepad_widgets::*;

    fn finger_up(device: DigitDevice, abs_start: Vec2d, abs: Vec2d, is_over: bool) -> FingerUpEvent {
        FingerUpEvent {
            window_id: WindowId(0, 0),
            abs,
            abs_start,
            capture_time: 0.0,
            time: 0.1,
            digit_id: DigitId::default(),
            device,
            has_long_press_occurred: false,
            tap_count: 1,
            modifiers: KeyModifiers::default(),
            rect: Rect::default(),
            is_over,
            is_sweep: false,
        }
    }

    #[test]
    fn primary_tap_inside_returns_true() {
        let fe = finger_up(
            DigitDevice::Mouse {
                button: MouseButton::PRIMARY,
            },
            dvec2(0.0, 0.0),
            dvec2(1.0, 1.0),
            true,
        );
        assert!(is_primary_tap(&fe));
    }

    #[test]
    fn drag_release_returns_false() {
        let fe = finger_up(
            DigitDevice::Touch { uid: 7 },
            dvec2(0.0, 0.0),
            dvec2(12.0, 0.0),
            true,
        );
        assert!(!is_primary_tap(&fe));
    }

    #[test]
    fn release_outside_returns_false() {
        let fe = finger_up(
            DigitDevice::Touch { uid: 7 },
            dvec2(0.0, 0.0),
            dvec2(1.0, 1.0),
            false,
        );
        assert!(!is_primary_tap(&fe));
    }

    #[test]
    fn secondary_mouse_returns_false() {
        let fe = finger_up(
            DigitDevice::Mouse {
                button: MouseButton::SECONDARY,
            },
            dvec2(0.0, 0.0),
            dvec2(1.0, 1.0),
            true,
        );
        assert!(!is_primary_tap(&fe));
    }
}
