use crate::{
    route::{Route, RouteQuery},
    router::RouterAction,
};
use makepad_widgets::*;

use super::{
    ResolvedPathIntent, ResolvedPathKind, RouterActionKind, RouterTransitionDirection, RouterWidget,
};

impl RouterWidget {
    pub(super) fn resolve_path_intent(
        &mut self,
        path: &str,
        replace: bool,
        clear_extras: bool,
    ) -> Option<ResolvedPathIntent> {
        let parsed = self.parse_url_cached(path);
        let query = RouteQuery::from_query_string(&parsed.query);

        // Optimization: Remove unnecessary `.clone()` on hash and query when creating intent.
        // Previously: called `.clone()` on these values multiple times in the return arms below,
        //             which caused unnecessary `String` and `HashMap` heap allocations during path resolution.
        // Now: we move ownership directly into the `Route` struct in each mutually exclusive return arm,
        //      eliminating the heap churn while keeping the code perfectly safe.
        let hash = parsed.hash;
        let normalized_path = parsed.path;

        // 1) Full match in this router.
        if let Some(mut route) = self.router.route_registry.resolve_path(&normalized_path) {
            if self.routes.templates.contains_key(&route.id) {
                route.query = query;
                route.hash = hash;
                return Some(ResolvedPathIntent {
                    path: normalized_path,
                    route,
                    kind: ResolvedPathKind::FullMatch,
                    clear_extras,
                    replace,
                });
            }
            return None;
        }

        // 2) Prefix match for nested routing: activate a parent route and delegate the tail.
        if let Some((route_id, params, pattern, tail)) =
            self.resolve_nested_prefix(&normalized_path)
        {
            if self.routes.templates.contains_key(&route_id) {
                return Some(ResolvedPathIntent {
                    path: normalized_path,
                    route: Route {
                        id: route_id,
                        params,
                        query,
                        hash,
                        pattern: Some(pattern),
                    },
                    kind: ResolvedPathKind::NestedPrefix { tail },
                    clear_extras,
                    replace,
                });
            }
            return None;
        }

        // 3) Not-found fallback.
        if self.not_found_route.0 != 0 && self.routes.templates.contains_key(&self.not_found_route)
        {
            if !replace && self.current_route_id() == Some(self.not_found_route) {
                return None;
            }
            let mut nf = Route::new(self.not_found_route);
            nf.query = query;
            nf.hash = hash;
            return Some(ResolvedPathIntent {
                path: normalized_path,
                route: nf,
                kind: ResolvedPathKind::NotFoundFallback,
                clear_extras,
                replace,
            });
        }

        None
    }

    // Optimization: avoid multiple allocations of `Route` and its parameters during path resolution
    // Previously: passed `&ResolvedPathIntent` and called `intent.route.clone()` multiple times
    // Now: pass `ResolvedPathIntent` by value and move the underlying fields to eliminate intermediate heap churn
    pub(super) fn apply_resolved_path_intent(
        &mut self,
        cx: &mut Cx,
        intent: ResolvedPathIntent,
    ) -> bool {
        if intent.clear_extras {
            self.clear_url_extras();
        } else {
            self.url_path_override = None;
        }

        if matches!(intent.kind, ResolvedPathKind::NotFoundFallback) {
            self.url_path_override = Some(intent.path.clone());
        }

        let old_route = self.router.current_route().cloned();
        let replace = intent.replace;
        let route = intent.route;

        if replace {
            self.router.replace(route.clone());
        } else {
            self.router.navigate(route.clone());
        }
        self.active_route = route.id;

        self.ensure_route_widget(cx, route.id);
        self.start_transition(
            cx,
            old_route.as_ref().map(|r| r.id),
            route.id,
            if replace {
                RouterActionKind::Replace
            } else {
                RouterActionKind::Push
            },
            RouterTransitionDirection::Forward,
            None,
        );

        self.dispatch_route_change(cx, old_route.as_ref(), &route);
        let primary_action = if replace {
            RouterAction::Replace(route.clone())
        } else {
            RouterAction::Navigate(route.clone())
        };
        self.queue_route_actions(
            Some(primary_action.clone()),
            old_route.as_ref().map(|r| r.id),
            &route,
        );
        self.sync_browser_with_action(cx, &primary_action);

        match &intent.kind {
            ResolvedPathKind::NestedPrefix { tail } => {
                let _ = self.delegate_tail_to_child(cx, route.id, tail);
            }
            ResolvedPathKind::FullMatch => {
                if self.child_routers.contains_key(&route.id) {
                    if let Some(pattern) = &route.pattern {
                        if let Some((_params, tail)) =
                            pattern.matches_prefix_with_tail(&intent.path)
                        {
                            let _ = self.delegate_tail_to_child(cx, route.id, &tail);
                        }
                    }
                }
            }
            ResolvedPathKind::NotFoundFallback => {}
        }

        self.redraw(cx);
        true
    }

    pub(super) fn navigate_by_path_internal(
        &mut self,
        cx: &mut Cx,
        path: &str,
        clear_extras: bool,
    ) -> bool {
        if let Some(intent) = self.resolve_path_intent(path, false, clear_extras) {
            return self.apply_resolved_path_intent(cx, intent);
        }

        log!("Router: No route found for path: {}", path);
        false
    }

    pub(super) fn replace_by_path_internal(
        &mut self,
        cx: &mut Cx,
        path: &str,
        clear_extras: bool,
    ) -> bool {
        if let Some(intent) = self.resolve_path_intent(path, true, clear_extras) {
            return self.apply_resolved_path_intent(cx, intent);
        }

        false
    }
}
