use crate::{route::Route, router::RouterAction, state::RouterState};
use makepad_widgets::*;

use super::{
    RouterActionKind, RouterNavRequest, RouterTransitionDirection, RouterTransitionSpec,
    RouterWidget,
};

impl RouterWidget {
    /// Navigate to a route by id. Returns true if the route exists.
    pub fn navigate(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Navigate { route_id });
        }
        if self.routes.templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Push,
                RouterTransitionDirection::Forward,
                None,
            );

            if let Some(new_route) = self.router.current_route().cloned() {
                let primary_action = RouterAction::Navigate(new_route.clone());
                self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
                self.queue_route_actions(
                    Some(primary_action.clone()),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
                self.sync_browser_with_action(cx, &primary_action);
            }

            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    /// Navigate with an explicit transition override.
    pub fn navigate_with_transition(
        &mut self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateWithTransition {
                    route_id,
                    transition,
                },
            );
        }
        if self.routes.templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Push,
                RouterTransitionDirection::Forward,
                Some(transition),
            );

            if let Some(new_route) = self.router.current_route().cloned() {
                let primary_action = RouterAction::Navigate(new_route.clone());
                self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
                self.queue_route_actions(
                    Some(primary_action.clone()),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
                self.sync_browser_with_action(cx, &primary_action);
            }

            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    /// Replace the current route by id (does not add to history).
    pub fn replace(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Replace { route_id });
        }
        if self.routes.templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.replace_with(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Replace,
                RouterTransitionDirection::Forward,
                None,
            );

            if let Some(new_route) = self.router.current_route().cloned() {
                let primary_action = RouterAction::Replace(new_route.clone());
                self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
                self.queue_route_actions(
                    Some(primary_action.clone()),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
                self.sync_browser_with_action(cx, &primary_action);
            }

            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    /// Replace with an explicit transition override.
    pub fn replace_with_transition(
        &mut self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::ReplaceWithTransition {
                    route_id,
                    transition,
                },
            );
        }
        if self.routes.templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.replace_with(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Replace,
                RouterTransitionDirection::Forward,
                Some(transition),
            );

            if let Some(new_route) = self.router.current_route().cloned() {
                let primary_action = RouterAction::Replace(new_route.clone());
                self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
                self.queue_route_actions(
                    Some(primary_action.clone()),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
                self.sync_browser_with_action(cx, &primary_action);
            }

            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    /// Go back in history. Returns true if navigation occurred.
    pub fn back(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Back { transition: None });
        }
        let old_route = self.router.current_route().cloned();
        if self.router.back() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );

                self.dispatch_route_change(cx, old_route.as_ref(), &route);

                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Back),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.sync_browser_with_action(cx, &RouterAction::Back);

                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Go back in history with an explicit transition override.
    pub fn back_with_transition(&mut self, cx: &mut Cx, transition: RouterTransitionSpec) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::Back {
                    transition: Some(transition),
                },
            );
        }
        let old_route = self.router.current_route().cloned();
        if self.router.back() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    Some(transition),
                );
                self.dispatch_route_change(cx, old_route.as_ref(), &route);
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Back),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.sync_browser_with_action(cx, &RouterAction::Back);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Go forward in history. Returns true if navigation occurred.
    pub fn forward(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Forward { transition: None });
        }
        let old_route = self.router.current_route().cloned();
        if self.router.forward() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.as_ref(), &route);
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Forward),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.sync_browser_with_action(cx, &RouterAction::Forward);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Go forward in history. Returns true if navigation occurred.
    pub fn forward_with_transition(
        &mut self,
        cx: &mut Cx,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::Forward {
                    transition: Some(transition),
                },
            );
        }
        let old_route = self.router.current_route().cloned();
        if self.router.forward() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    Some(transition),
                );
                self.dispatch_route_change(cx, old_route.as_ref(), &route);
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Forward),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.sync_browser_with_action(cx, &RouterAction::Forward);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Whether there is a previous route in history.
    pub fn can_go_back(&self) -> bool {
        self.router.can_go_back()
    }

    /// Whether there is a forward route in history.
    pub fn can_go_forward(&self) -> bool {
        self.router.can_go_forward()
    }

    /// History depth.
    pub fn depth(&self) -> usize {
        self.router.depth()
    }

    /// Current route id, if any.
    pub fn current_route_id(&self) -> Option<LiveId> {
        self.router.current_route_id()
    }

    /// Snapshot current router state (for persistence).
    pub fn get_state(&self) -> RouterState {
        self.build_state()
    }

    /// Restore router state. Returns true on success.
    pub fn set_state(&mut self, cx: &mut Cx, state: RouterState) -> bool {
        self.apply_state(cx, state)
    }

    /// Clear history, keeping the current route.
    pub fn clear_history(&mut self, cx: &mut Cx) {
        self.router.clear_history();
        self.redraw(cx);
    }

    /// Reset history to a single route.
    pub fn reset(&mut self, cx: &mut Cx, route: Route) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Reset { route });
        }
        if !self.routes.templates.contains_key(&route.id) {
            log!("Router: Route template not found for {:?}", route.id);
            return false;
        }
        self.clear_url_extras();
        let old_route = self.router.current_route().cloned();
        self.router.reset(route.clone());
        self.active_route = route.id;
        self.ensure_route_widget(cx, route.id);
        self.start_transition(
            cx,
            old_route.as_ref().map(|r| r.id),
            route.id,
            RouterActionKind::Replace,
            RouterTransitionDirection::Forward,
            None,
        );

        if let Some(new_route) = self.router.current_route().cloned() {
            let primary_action = RouterAction::Reset(new_route.clone());
            self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
            self.queue_route_actions(
                Some(primary_action.clone()),
                old_route.as_ref().map(|r| r.id),
                &new_route,
            );
            self.sync_browser_with_action(cx, &primary_action);
        }

        self.redraw(cx);
        true
    }

    /// Push a route (alias of `navigate`).
    pub fn push(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        self.navigate(cx, route_id)
    }

    /// Pop the current route (stack-style semantics).
    pub fn pop(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Pop);
        }
        let old_depth = self.router.depth();
        let old_route = self.router.current_route().cloned();
        if self.router.pop() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.sync_browser_after_pop(cx, old_depth.saturating_sub(self.router.depth()));
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    /// Pop to a route id (stack-style semantics).
    pub fn pop_to(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::PopTo { route_id });
        }
        let old_depth = self.router.depth();
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to(route_id) {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.sync_browser_after_pop(cx, old_depth.saturating_sub(self.router.depth()));
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    /// Pop to the root route (stack-style semantics).
    pub fn pop_to_root(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::PopToRoot);
        }
        let old_depth = self.router.depth();
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to_root() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.sync_browser_after_pop(cx, old_depth.saturating_sub(self.router.depth()));
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    /// Set the entire stack (stack-style semantics).
    pub fn set_stack(&mut self, cx: &mut Cx, stack: Vec<Route>) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::SetStack { stack });
        }
        let filtered: Vec<Route> = stack
            .into_iter()
            .filter(|r| self.routes.templates.contains_key(&r.id))
            .collect();
        if filtered.is_empty() {
            return false;
        }
        self.clear_url_extras();
        let old_route = self.router.current_route().cloned();
        self.router.set_stack(filtered);
        let Some(new_route) = self.router.current_route().cloned() else {
            return false;
        };
        self.active_route = new_route.id;
        self.ensure_route_widget(cx, new_route.id);
        self.start_transition(
            cx,
            old_route.as_ref().map(|r| r.id),
            new_route.id,
            RouterActionKind::Replace,
            RouterTransitionDirection::Forward,
            None,
        );
        self.dispatch_route_change(cx, old_route.as_ref(), &new_route);
        let primary_action = RouterAction::Reset(new_route.clone());
        self.queue_route_actions(
            Some(primary_action.clone()),
            old_route.as_ref().map(|r| r.id),
            &new_route,
        );
        self.sync_browser_with_action(cx, &primary_action);
        self.redraw(cx);
        true
    }

    /// Navigate by path string (matches registered route patterns).
    pub fn navigate_by_path(&mut self, cx: &mut Cx, path: &str) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateByPath {
                    path: path.to_string(),
                },
            );
        }
        self.navigate_by_path_internal(cx, path, true)
    }
}
