use crate::pattern::{RouteParams, RoutePatternRef};
use crate::route::Route;
use makepad_widgets::*;
// Nested router discovery and child router registration.

use super::{RouterWidget, RouterWidgetRef, RouterWidgetWidgetRefExt};

impl RouterWidget {
    pub(super) fn resolve_nested_prefix(
        &mut self,
        path: &str,
    ) -> Option<(LiveId, RouteParams, RoutePatternRef, String)> {
        if !self.nested_enabled() {
            return None;
        }
        if self.caches.nested_prefix_cache_epoch == self.caches.route_registry_epoch
            && self.caches.nested_prefix_cache_path == path
        {
            return self.caches.nested_prefix_cache_result.clone();
        }

        let mut best: Option<(LiveId, RouteParams, RoutePatternRef, String, usize)> = None;

        for route_id in self
            .routes
            .patterns
            .keys()
            .cloned()
            .chain(self.child_routers.keys().cloned())
        {
            let Some(pattern_obj) = self.router.route_registry.get_pattern(route_id) else {
                continue;
            };
            let Some((params, tail)) = pattern_obj.matches_prefix_with_tail(path) else {
                continue;
            };
            let priority = pattern_obj.priority();
            match &best {
                Some((_id, _p, _pat, _tail, best_prio)) if *best_prio <= priority => {}
                _ => {
                    best = Some((route_id, params, pattern_obj.clone(), tail, priority));
                }
            }
        }

        let out = best.map(|(id, params, pattern, tail, _prio)| (id, params, pattern, tail));
        self.caches.nested_prefix_cache_epoch = self.caches.route_registry_epoch;
        self.caches.nested_prefix_cache_path = path.to_string();
        self.caches.nested_prefix_cache_result = out.clone();
        out
    }

    pub(super) fn delegate_tail_to_child(
        &mut self,
        cx: &mut Cx,
        parent_route_id: LiveId,
        tail: &str,
    ) -> bool {
        if !self.nested_enabled() {
            return false;
        }
        if tail.is_empty() {
            return true;
        }
        self.detect_child_routers(cx);
        let child_router = self.child_routers.get(&parent_route_id).cloned();
        if let Some(child_router) = child_router {
            if let Some(mut child) = child_router.borrow_mut() {
                return child.navigate_by_path(cx, tail);
            }
        }
        false
    }

    /// Automatically detect and register child routers in route widgets.
    ///
    /// We scan the instantiated route widget tree for nested `RouterWidget` instances.
    pub(super) fn detect_child_routers(&mut self, _cx: &mut Cx) {
        if !self.nested_enabled() {
            return;
        }
        if self.caches.child_router_scan_epoch == self.caches.route_registry_epoch
            && self.caches.child_router_scan_widget_count == self.routes.widgets.len()
        {
            return;
        }
        for (route_id, route_widget) in self.routes.widgets.iter() {
            if self.child_routers.contains_key(route_id) {
                continue;
            }

            if let Some(child_router) = Self::find_first_child_router(route_widget) {
                self.child_routers.insert(*route_id, child_router);
            }
        }
        self.caches.child_router_scan_epoch = self.caches.route_registry_epoch;
        self.caches.child_router_scan_widget_count = self.routes.widgets.len();
    }

    fn find_first_child_router(widget: &WidgetRef) -> Option<RouterWidgetRef> {
        if widget.borrow::<RouterWidget>().is_some() {
            return Some(widget.as_router_widget());
        }

        let mut children = Vec::new();
        widget.children(&mut |_id, child| children.push(child));
        for child in children {
            if let Some(found) = Self::find_first_child_router(&child) {
                return Some(found);
            }
        }
        None
    }

    /// Navigate to a nested route.
    pub fn navigate_nested(&mut self, cx: &mut Cx, path: &[LiveId], route: Route) -> bool {
        if !self.nested_enabled() {
            self.last_blocked_reason = Some(super::RouterBlockReason::CapabilityDisabled);
            return false;
        }
        if path.is_empty() {
            // Navigate in current router.
            if self.routes.templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                self.router.navigate(route.clone());
                self.active_route = route.id;

                self.ensure_route_widget(cx, route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    super::RouterActionKind::Push,
                    super::RouterTransitionDirection::Forward,
                    None,
                );

                self.redraw(cx);
                return true;
            }
            return false;
        }

        // Navigate to child router.
        let first = path[0];
        let child_router_opt = self.child_routers.get(&first).cloned();
        if let Some(child_router) = child_router_opt {
            if let Some(mut child) = child_router.borrow_mut() {
                if child.navigate_nested(cx, &path[1..], route) {
                    self.redraw(cx);
                    return true;
                }
            }
        }

        false
    }
}
