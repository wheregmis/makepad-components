use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadAspectRatioBase = #(ShadAspectRatio::register_widget(vm))

    mod.widgets.ShadAspectRatio = set_type_default() do mod.widgets.ShadAspectRatioBase{
        width: Fill
        height: Fit
        ratio: 1.7777777778
        flow: Overlay
        align: Align{x: 0.5, y: 0.5}
        draw_bg.color: #0000
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadAspectRatio {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live(1.7777777778)]
    ratio: f64,
}

impl ShadAspectRatio {
    fn effective_ratio(&self) -> f64 {
        if self.ratio > 0.0 {
            self.ratio
        } else {
            1.0
        }
    }

    fn with_ratio_walk(&self, cx: &Cx2d, mut walk: Walk) -> Walk {
        let ratio = self.effective_ratio();
        let rect = cx.peek_walk_turtle(walk);

        let width_fixed = match walk.width {
            Size::Fixed(v) => Some(v),
            _ => None,
        };
        let height_fixed = match walk.height {
            Size::Fixed(v) => Some(v),
            _ => None,
        };

        let (width, height) = match (width_fixed, height_fixed) {
            (Some(width), Some(height)) => (width, height),
            (Some(width), None) => (width, width / ratio),
            (None, Some(height)) => (height * ratio, height),
            (None, None) => {
                let mut width = rect.size.x.max(0.0);
                if width <= 0.0 {
                    width = rect.size.y.max(0.0) * ratio;
                }
                (width, width / ratio)
            }
        };

        walk.width = Size::Fixed(width);
        walk.height = Size::Fixed(height);
        walk
    }
}

impl Widget for ShadAspectRatio {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let walk = self.with_ratio_walk(cx, walk);
        self.view.draw_walk(cx, scope, walk)
    }
}
