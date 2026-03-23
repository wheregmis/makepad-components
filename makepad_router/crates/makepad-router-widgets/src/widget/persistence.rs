use crate::{route::Route, state::RouterState};
use makepad_widgets::{Cx, WidgetNode};

// Router state persistence helpers.

use super::{RouterAction, RouterBlockReason, RouterWidget};

impl RouterWidget {
    pub(super) fn build_state(&self) -> RouterState {
        if !self.persistence_enabled() {
            return RouterState::default();
        }
        RouterState {
            history: self.router.history.clone(),
            url_path_override: self.url_path_override.clone(),
        }
    }

    pub(super) fn apply_state(&mut self, cx: &mut Cx, state: RouterState) -> bool {
        if !self.persistence_enabled() {
            self.last_blocked_reason = Some(RouterBlockReason::CapabilityDisabled);
            return false;
        }
        let old_route = self.router.current_route().cloned();
        let (stack, current_index) = state.history.into_parts();

        let mut filtered = Vec::<Route>::new();
        let mut new_current = 0usize;
        for (idx, route) in stack.into_iter().enumerate() {
            if !self.routes.templates.contains_key(&route.id) {
                continue;
            }
            if idx <= current_index {
                new_current = filtered.len();
            }
            filtered.push(route);
        }
        if filtered.is_empty() {
            return false;
        }

        self.clear_url_extras();
        self.url_path_override = state.url_path_override;
        self.router.history =
            crate::navigation::NavigationHistory::from_parts(filtered, new_current);
        let Some(new_route) = self.router.current_route().cloned() else {
            return false;
        };
        self.active_route = new_route.id;
        self.transition_rt.state = None;
        self.ensure_route_widget(cx, new_route.id);

        self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
        self.queue_route_actions(
            Some(RouterAction::Reset(new_route.clone())),
            old_route.as_ref().map(|r| r.id),
            &new_route,
        );

        self.redraw(cx);
        true
    }
}
