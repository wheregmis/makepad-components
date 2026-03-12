use crate::route::Route;
use makepad_live_id::LiveId;
use makepad_widgets::{Cx, ToUIReceiver};

// Guard and before-leave types for RouterWidget.

/// Kind of navigation request being evaluated by guards/hooks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouterNavKind {
    Navigate,
    Replace,
    NavigateByPath,
    ReplaceByPath,
    Back,
    Forward,
    Reset,
    SetStack,
    Push,
    Pop,
    PopTo,
    PopToRoot,
}

/// Context passed to route guards and before-leave hooks.
#[derive(Clone, Debug)]
pub struct RouterNavContext {
    pub kind: RouterNavKind,
    pub from: Option<Route>,
    pub to: Option<Route>,
    pub to_path: Option<String>,
}

/// Target used by a guard to redirect navigation.
#[derive(Clone, Debug)]
pub enum RouterRedirectTarget {
    Route(LiveId),
    Path(String),
}

/// Redirect instruction returned by a guard.
#[derive(Clone, Debug)]
pub struct RouterRedirect {
    pub target: RouterRedirectTarget,
    pub replace: bool,
}

/// Result of a guard evaluation.
#[derive(Clone, Debug)]
pub enum RouterGuardDecision {
    Allow,
    Block,
    Redirect(RouterRedirect),
}

/// Result of a before-leave hook.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouterBeforeLeaveDecision {
    Allow,
    Block,
}

/// Async decision type used by async guards/before-leave hooks.
pub enum RouterAsyncDecision<T> {
    Immediate(T),
    Pending(ToUIReceiver<T>),
}

/// Sync guard hook. Return `Allow`, `Block`, or `Redirect`.
pub type RouterSyncGuard =
    Box<dyn Fn(&mut Cx, &RouterNavContext) -> RouterGuardDecision + Send + Sync>;
/// Async guard hook. Return an immediate decision or a `ToUIReceiver`.
pub type RouterAsyncGuard = Box<
    dyn Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterGuardDecision> + Send + Sync,
>;

/// Sync before-leave hook.
pub type RouterBeforeLeaveSync =
    Box<dyn Fn(&mut Cx, &RouterNavContext) -> RouterBeforeLeaveDecision + Send + Sync>;
/// Async before-leave hook.
pub type RouterBeforeLeaveAsync = Box<
    dyn Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterBeforeLeaveDecision>
        + Send
        + Sync,
>;
