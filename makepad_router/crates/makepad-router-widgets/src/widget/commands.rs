use crate::route::Route;
use crate::router::RouterAction;

use super::RouterTransitionSpec;
use makepad_widgets::LiveId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouterBlockReason {
    GuardBlocked,
    BeforeLeaveBlocked,
    RouteMissing,
    NoHistory,
    CapabilityDisabled,
    RedirectLimit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouterCapabilities {
    pub guards_sync: bool,
    pub guards_async: bool,
    pub transitions: bool,
    pub nested: bool,
    pub persistence: bool,
}

impl Default for RouterCapabilities {
    fn default() -> Self {
        Self {
            guards_sync: false,
            guards_async: false,
            transitions: false,
            nested: false,
            persistence: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouterConfig {
    pub capabilities: RouterCapabilities,
    pub default_route: LiveId,
    pub not_found_route: LiveId,
    pub default_transition: LiveId,
    pub persist_state: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            capabilities: RouterCapabilities::default(),
            default_route: LiveId(0),
            not_found_route: LiveId(0),
            default_transition: LiveId(0),
            persist_state: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum RouterCommand {
    GoToRoute {
        route_id: LiveId,
        transition: Option<RouterTransitionSpec>,
    },
    GoToPath {
        path: String,
    },
    ReplaceRoute {
        route_id: LiveId,
        transition: Option<RouterTransitionSpec>,
    },
    ReplacePath {
        path: String,
        clear_extras: bool,
    },
    Back {
        transition: Option<RouterTransitionSpec>,
    },
    Forward {
        transition: Option<RouterTransitionSpec>,
    },
    Reset {
        route: Route,
    },
    Push {
        route_id: LiveId,
        transition: Option<RouterTransitionSpec>,
    },
    Pop,
    PopTo {
        route_id: LiveId,
    },
    PopToRoot,
    SetStack {
        stack: Vec<Route>,
    },
}

#[derive(Clone, Debug)]
pub struct RouterDispatchResult {
    pub changed: bool,
    pub from: Option<Route>,
    pub to: Option<Route>,
    pub action: Option<RouterAction>,
    pub blocked_reason: Option<RouterBlockReason>,
}

impl RouterDispatchResult {
    pub fn blocked(from: Option<Route>, to: Option<Route>, reason: RouterBlockReason) -> Self {
        Self {
            changed: false,
            from,
            to,
            action: None,
            blocked_reason: Some(reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_capabilities_are_opt_in() {
        let caps = RouterCapabilities::default();
        assert!(!caps.guards_sync);
        assert!(!caps.guards_async);
        assert!(!caps.transitions);
        assert!(!caps.nested);
        assert!(!caps.persistence);
    }

    #[test]
    fn blocked_result_marks_failure() {
        let result = RouterDispatchResult::blocked(None, None, RouterBlockReason::NoHistory);
        assert!(!result.changed);
        assert_eq!(result.blocked_reason, Some(RouterBlockReason::NoHistory));
    }
}
