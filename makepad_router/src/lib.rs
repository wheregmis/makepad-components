pub use makepad_draw;
pub use makepad_live_id;
pub use makepad_micro_serde;
pub use makepad_router_core;
pub use makepad_router_widgets;
pub use makepad_widgets;

pub mod guards {
    pub use makepad_router_widgets::guards::*;
}
pub mod widget {
    pub use makepad_router_widgets::widget::*;
}

pub mod navigation {
    pub use makepad_router_core::navigation::*;
}
pub mod pattern {
    pub use makepad_router_core::pattern::*;
}
pub mod registry {
    pub use makepad_router_core::registry::*;
}
pub mod route {
    pub use makepad_router_core::route::*;
}
pub mod router {
    pub use makepad_router_core::router::*;
}
pub mod state {
    pub use makepad_router_core::state::*;
}
pub mod url {
    pub use makepad_router_core::url::*;
}

// Public API (explicit re-exports only; no wildcard exports).
pub use crate::guards::{
    RouterAsyncDecision, RouterAsyncGuard, RouterBeforeLeaveAsync, RouterBeforeLeaveDecision,
    RouterBeforeLeaveSync, RouterGuardDecision, RouterNavContext, RouterNavKind, RouterRedirect,
    RouterRedirectTarget, RouterSyncGuard,
};
pub use crate::navigation::NavigationHistory;
pub use crate::pattern::{RouteParams, RoutePattern, RouteSegment};
pub use crate::registry::RouteRegistry;
pub use crate::route::{Route, RouteQuery};
pub use crate::router::{Router, RouterAction};
pub use crate::state::RouterState;
pub use crate::url::{build_query_string, parse_query_map, RouterUrl};
pub use crate::widget::script_mod;
pub use crate::widget::{
    RouterBlockReason, RouterCapabilities, RouterCommand, RouterConfig, RouterDispatchResult,
    RouterRoute, RouterTransitionPreset, RouterTransitionSpec, RouterWidget, RouterWidgetRef,
    RouterWidgetWidgetRefExt,
};

/// Convenience re-exports for common usage patterns.
pub mod prelude {
    pub use crate::guards::{
        RouterAsyncDecision, RouterBeforeLeaveDecision, RouterGuardDecision, RouterNavContext,
        RouterRedirect, RouterRedirectTarget,
    };
    pub use crate::pattern::{RouteParams, RoutePattern, RouteSegment};
    pub use crate::route::{Route, RouteQuery};
    pub use crate::router::{Router, RouterAction};
    pub use crate::state::RouterState;
    pub use crate::url::RouterUrl;
    pub use crate::widget::{
        RouterBlockReason, RouterCapabilities, RouterCommand, RouterConfig, RouterDispatchResult,
        RouterRoute, RouterTransitionPreset, RouterTransitionSpec, RouterWidget, RouterWidgetRef,
        RouterWidgetWidgetRefExt,
    };
}
