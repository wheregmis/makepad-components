use makepad_widgets::*;

use super::{RouterTransitionDirection, RouterTransitionPreset, RouterWidget};

impl RouterWidget {
    pub(super) fn draw_routes_with_transition(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        rect: Rect,
    ) {
        if let Some(state) = self.transition_rt.state.clone() {
            let from_effect =
                Self::compute_effect(state.preset, state.direction, state.progress, false, rect);
            Self::draw_route_into_draw_list(
                cx,
                scope,
                &mut self.draw_lists.from,
                &mut self.routes.widgets,
                state.from_route,
                rect,
                from_effect,
                false,
            );

            let to_effect =
                Self::compute_effect(state.preset, state.direction, state.progress, true, rect);
            Self::draw_route_into_draw_list(
                cx,
                scope,
                &mut self.draw_lists.to,
                &mut self.routes.widgets,
                state.to_route,
                rect,
                to_effect,
                false,
            );
        } else {
            let effect = Self::compute_effect(
                RouterTransitionPreset::None,
                RouterTransitionDirection::Forward,
                0.0,
                true,
                rect,
            );
            Self::draw_route_into_draw_list(
                cx,
                scope,
                &mut self.draw_lists.to,
                &mut self.routes.widgets,
                self.active_route,
                rect,
                effect,
                false,
            );
        }
    }
}
