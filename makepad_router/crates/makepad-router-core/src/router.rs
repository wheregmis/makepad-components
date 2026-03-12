use crate::navigation::NavigationHistory;
use crate::registry::RouteRegistry;
use crate::route::Route;
use makepad_live_id::*;
use makepad_micro_serde::*;

/// Router configuration and state
#[derive(Clone, Debug, SerBin, DeBin, SerRon, DeRon)]
pub struct Router {
    /// Navigation history
    pub history: NavigationHistory,
    /// Whether to persist router state
    pub persist_state: bool,
    /// Registry for pattern-based navigation (non-visual, usable headless).
    pub route_registry: RouteRegistry,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            history: NavigationHistory::empty(),
            persist_state: false,
            route_registry: RouteRegistry::default(),
        }
    }
}

impl Router {
    /// Create a new router with an initial route
    pub fn new(initial_route: Route) -> Self {
        Self {
            history: NavigationHistory::new(initial_route),
            persist_state: false,
            route_registry: RouteRegistry::default(),
        }
    }

    /// Create a router with state persistence enabled
    pub fn with_persistence(initial_route: Route) -> Self {
        Self {
            history: NavigationHistory::new(initial_route),
            persist_state: true,
            route_registry: RouteRegistry::default(),
        }
    }

    /// Navigate to a new route
    pub fn navigate(&mut self, route: Route) {
        self.history.push(route);
    }

    /// Navigate to a route by ID
    pub fn navigate_to(&mut self, route_id: LiveId) {
        self.navigate(Route::new(route_id));
    }

    /// Replace the current route
    pub fn replace(&mut self, route: Route) {
        self.history.replace(route);
    }

    /// Replace the current route by ID
    pub fn replace_with(&mut self, route_id: LiveId) {
        self.replace(Route::new(route_id));
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        self.history.back()
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        self.history.forward()
    }

    /// Get the current route
    pub fn current_route(&self) -> Option<&Route> {
        self.history.current()
    }

    /// Get the current route ID
    pub fn current_route_id(&self) -> Option<LiveId> {
        self.current_route().map(|r| r.id)
    }

    /// Preview the route selected by `back()` without mutating history.
    pub fn preview_back_route(&self) -> Option<&Route> {
        self.history.preview_back()
    }

    /// Preview the route selected by `forward()` without mutating history.
    pub fn preview_forward_route(&self) -> Option<&Route> {
        self.history.preview_forward()
    }

    /// Preview the route selected by `pop()` without mutating history.
    pub fn preview_pop_route(&self) -> Option<&Route> {
        self.history.preview_pop()
    }

    /// Preview the route selected by `pop_to()` without mutating history.
    pub fn preview_pop_to_route(&self, route_id: LiveId) -> Option<&Route> {
        self.history.preview_pop_to(route_id)
    }

    /// Preview the route selected by `pop_to_root()` without mutating history.
    pub fn preview_pop_to_root_route(&self) -> Option<&Route> {
        self.history.preview_pop_to_root()
    }

    /// Check if we can navigate back
    pub fn can_go_back(&self) -> bool {
        self.history.can_go_back()
    }

    /// Check if we can navigate forward
    pub fn can_go_forward(&self) -> bool {
        self.history.can_go_forward()
    }

    /// Reset to a specific route, clearing all history
    pub fn reset(&mut self, route: Route) {
        self.history.reset(route);
    }

    /// Clear all history except current route
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get the depth of the navigation history
    pub fn depth(&self) -> usize {
        self.history.depth()
    }

    /// Register a pattern-based route for `navigate_by_path`.
    pub fn register_route_pattern(
        &mut self,
        pattern: &str,
        route_id: LiveId,
    ) -> Result<(), String> {
        self.route_registry.register_pattern(pattern, route_id)
    }

    /// Navigate using a path string, using the registered route patterns.
    /// Returns the resolved `Route` on success.
    pub fn navigate_by_path(&mut self, path: &str) -> Result<Route, String> {
        let route = self
            .route_registry
            .resolve_path(path)
            .ok_or_else(|| format!("No route found for path: {}", path))?;
        self.navigate(route.clone());
        Ok(route)
    }

    /// Push a route onto the stack (alias of `navigate`).
    pub fn push(&mut self, route: Route) {
        self.navigate(route);
    }

    /// Pop the current route (stack-style semantics).
    pub fn pop(&mut self) -> bool {
        self.history.pop()
    }

    /// Pop to the given route id (stack-style semantics).
    pub fn pop_to(&mut self, route_id: LiveId) -> bool {
        self.history.pop_to(route_id)
    }

    /// Pop to the root route (stack-style semantics).
    pub fn pop_to_root(&mut self) -> bool {
        self.history.pop_to_root()
    }

    /// Set the entire stack (stack-style semantics).
    pub fn set_stack(&mut self, stack: Vec<Route>) {
        self.history.set_stack(stack);
    }
}

/// Router actions for event handling
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouterAction {
    /// Navigate to a route
    Navigate(Route),
    /// Replace the current route
    Replace(Route),
    /// Go back in history
    Back,
    /// Go forward in history
    Forward,
    /// Reset to a route
    Reset(Route),
    /// Route changed notification
    RouteChanged { from: Option<LiveId>, to: LiveId },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_navigate() {
        let mut router = Router::new(Route::new(live_id!(home)));
        assert_eq!(router.current_route_id(), Some(live_id!(home)));

        router.navigate_to(live_id!(settings));
        assert_eq!(router.current_route_id(), Some(live_id!(settings)));
        assert_eq!(router.depth(), 2);
    }

    #[test]
    fn test_router_back() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.navigate_to(live_id!(settings));
        router.navigate_to(live_id!(profile));

        assert!(router.can_go_back());
        assert!(router.back());
        assert_eq!(router.current_route_id(), Some(live_id!(settings)));

        assert!(router.back());
        assert_eq!(router.current_route_id(), Some(live_id!(home)));
        assert!(!router.can_go_back());
    }

    #[test]
    fn test_router_replace() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.replace_with(live_id!(settings));

        assert_eq!(router.current_route_id(), Some(live_id!(settings)));
        assert_eq!(router.depth(), 1);
        assert!(!router.can_go_back());
    }

    #[test]
    fn test_router_navigate_by_path() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router
            .register_route_pattern("/user/:id", live_id!(user_profile))
            .unwrap();

        router.navigate_by_path("/user/123").unwrap();
        let route = router.current_route().unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert_eq!(
            route.get_param(LiveId::from_str("id")),
            Some(LiveId::from_str("123"))
        );
    }
}
