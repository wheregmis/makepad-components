//! Guard evaluation pipeline for navigation requests.

use crate::{
    guards::{
        RouterAsyncDecision, RouterBeforeLeaveDecision, RouterGuardDecision, RouterNavContext,
        RouterNavKind, RouterRedirectTarget,
    },
    route::Route,
};
use makepad_widgets::*;

use super::{ResolvedPathIntent, RouterAction, RouterBlockReason, RouterNavRequest, RouterWidget};

pub(super) const ROUTER_MAX_REDIRECTS: u8 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingNavPhase {
    BeforeLeaveAsync,
    GuardAsync,
    BundleAsync,
    BootBundleAsync,
}

enum PendingAsyncRx {
    BeforeLeave(ToUIReceiver<RouterBeforeLeaveDecision>),
    Guard(ToUIReceiver<RouterGuardDecision>),
    Bundle {
        route_id: LiveId,
        bundle_id: String,
        rx: ToUIReceiver<Result<(), String>>,
    },
}

pub(super) struct PendingNavigation {
    request: RouterNavRequest,
    context: RouterNavContext,
    resolved_path: Option<ResolvedPathIntent>,
    phase: PendingNavPhase,
    async_index: usize,
    redirect_depth: u8,
    rx: PendingAsyncRx,
}

impl RouterWidget {
    pub(super) fn request_navigation(&mut self, cx: &mut Cx, request: RouterNavRequest) -> bool {
        self.request_navigation_internal(cx, request, false, 0)
    }

    pub(super) fn request_navigation_internal(
        &mut self,
        cx: &mut Cx,
        request: RouterNavRequest,
        skip_before_leave: bool,
        redirect_depth: u8,
    ) -> bool {
        if self.guard_bypass {
            return self.apply_request_bypassing_guards(cx, request);
        }
        if self.pending_navigation.is_some() {
            self.last_blocked_reason = Some(RouterBlockReason::NoHistory);
            return false;
        }
        let Some((context, leaving, resolved_path)) = self.resolve_nav_context(cx, &request) else {
            self.last_blocked_reason = Some(Self::infer_resolution_block_reason(&request));
            return false;
        };

        if !skip_before_leave
            && self.before_leave_hooks().is_empty()
            && !self.has_async_before_leave_hooks()
            && self.route_guards().is_empty()
            && !self.has_async_route_guards()
        {
            return self.apply_request_after_bundle_check(
                cx,
                request,
                context,
                resolved_path,
                redirect_depth,
            );
        }

        if !skip_before_leave && leaving {
            for hook in self.before_leave_hooks() {
                if hook(cx, &context) == RouterBeforeLeaveDecision::Block {
                    self.last_blocked_reason = Some(RouterBlockReason::BeforeLeaveBlocked);
                    return false;
                }
            }
            if self.has_async_before_leave_hooks() {
                return self.run_before_leave_async(
                    cx,
                    request,
                    context,
                    resolved_path,
                    0,
                    redirect_depth,
                );
            }
        }

        self.apply_guards_and_maybe_commit(cx, request, context, resolved_path, redirect_depth)
    }

    fn resolve_nav_context(
        &mut self,
        _cx: &mut Cx,
        request: &RouterNavRequest,
    ) -> Option<(RouterNavContext, bool, Option<ResolvedPathIntent>)> {
        let from = self.router.current_route().cloned();
        let kind = Self::request_kind(request);
        let to: Option<Route>;
        let mut to_path: Option<String> = None;
        let mut resolved_path: Option<ResolvedPathIntent> = None;

        match request {
            RouterNavRequest::Navigate { route_id }
            | RouterNavRequest::NavigateWithTransition { route_id, .. }
            | RouterNavRequest::Replace { route_id }
            | RouterNavRequest::ReplaceWithTransition { route_id, .. } => {
                if !self.routes.templates.contains_key(route_id) {
                    return None;
                }
                to = Some(Route::new(*route_id));
            }
            RouterNavRequest::NavigateByPath { path }
            | RouterNavRequest::ReplaceByPath { path, .. } => {
                let (replace, clear_extras) = match request {
                    RouterNavRequest::NavigateByPath { .. } => (false, true),
                    RouterNavRequest::ReplaceByPath { clear_extras, .. } => (true, *clear_extras),
                    _ => (false, true),
                };
                let intent = self.resolve_path_intent(path, replace, clear_extras)?;
                to_path = Some(intent.path.clone());
                to = Some(intent.route.clone());
                resolved_path = Some(intent);
            }
            RouterNavRequest::Back { .. } => {
                let Some(next) = self.router.preview_back_route() else {
                    return None;
                };
                to = Some(next.clone());
            }
            RouterNavRequest::Forward { .. } => {
                let Some(next) = self.router.preview_forward_route() else {
                    return None;
                };
                to = Some(next.clone());
            }
            RouterNavRequest::Reset { route } => {
                if !self.routes.templates.contains_key(&route.id) {
                    return None;
                }
                to = Some(route.clone());
            }
            RouterNavRequest::SetStack { stack } => {
                let filtered: Vec<Route> = stack
                    .iter()
                    .filter(|r| self.routes.templates.contains_key(&r.id))
                    .cloned()
                    .collect();
                if filtered.is_empty() {
                    return None;
                }
                to = filtered.last().cloned();
            }
            RouterNavRequest::Pop => {
                let Some(next) = self.router.preview_pop_route() else {
                    return None;
                };
                to = Some(next.clone());
            }
            RouterNavRequest::PopTo { route_id } => {
                let Some(next) = self.router.preview_pop_to_route(*route_id) else {
                    return None;
                };
                to = Some(next.clone());
            }
            RouterNavRequest::PopToRoot => {
                let Some(next) = self.router.preview_pop_to_root_route() else {
                    return None;
                };
                to = Some(next.clone());
            }
        }

        let leaving = match (&from, &to) {
            (Some(from), Some(to)) => from.id != to.id,
            (Some(_), None) => false,
            _ => false,
        };

        Some((
            RouterNavContext {
                kind,
                from,
                to,
                to_path,
            },
            leaving,
            resolved_path,
        ))
    }

    fn request_kind(request: &RouterNavRequest) -> RouterNavKind {
        match request {
            RouterNavRequest::Navigate { .. } | RouterNavRequest::NavigateWithTransition { .. } => {
                RouterNavKind::Navigate
            }
            RouterNavRequest::Replace { .. } | RouterNavRequest::ReplaceWithTransition { .. } => {
                RouterNavKind::Replace
            }
            RouterNavRequest::NavigateByPath { .. } => RouterNavKind::NavigateByPath,
            RouterNavRequest::ReplaceByPath { .. } => RouterNavKind::ReplaceByPath,
            RouterNavRequest::Back { .. } => RouterNavKind::Back,
            RouterNavRequest::Forward { .. } => RouterNavKind::Forward,
            RouterNavRequest::Reset { .. } => RouterNavKind::Reset,
            RouterNavRequest::SetStack { .. } => RouterNavKind::SetStack,
            RouterNavRequest::Pop => RouterNavKind::Pop,
            RouterNavRequest::PopTo { .. } => RouterNavKind::PopTo,
            RouterNavRequest::PopToRoot => RouterNavKind::PopToRoot,
        }
    }

    fn fail_bundle_load(
        &mut self,
        route_id: LiveId,
        bundle_id: String,
        error: Option<String>,
    ) -> bool {
        if let Some(error) = error {
            log!(
                "Router: failed to load wasm bundle `{}` for route {:?}: {}",
                bundle_id,
                route_id,
                error
            );
        }
        self.last_blocked_reason = Some(RouterBlockReason::BundleLoadFailed);
        self.pending_actions.push(RouterAction::BundleLoadFailed {
            route_id,
            bundle_id,
        });
        false
    }

    fn apply_request_after_bundle_check(
        &mut self,
        cx: &mut Cx,
        request: RouterNavRequest,
        context: RouterNavContext,
        resolved_path: Option<ResolvedPathIntent>,
        redirect_depth: u8,
    ) -> bool {
        let Some(route_id) = context.to.as_ref().map(|route| route.id) else {
            return self.apply_request_bypassing_guards_resolved(cx, request, resolved_path);
        };
        let Some(bundle_id) = self.route_bundle_id(route_id) else {
            return self.apply_request_bypassing_guards_resolved(cx, request, resolved_path);
        };

        let rx = cx.ensure_wasm_bundle(&bundle_id);
        match rx.try_recv_flush() {
            Ok(Ok(())) => self.apply_request_bypassing_guards_resolved(cx, request, resolved_path),
            Ok(Err(error)) => self.fail_bundle_load(route_id, bundle_id, Some(error)),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                self.pending_navigation = Some(PendingNavigation {
                    request,
                    context,
                    resolved_path,
                    phase: PendingNavPhase::BundleAsync,
                    async_index: 0,
                    redirect_depth,
                    rx: PendingAsyncRx::Bundle {
                        route_id,
                        bundle_id,
                        rx,
                    },
                });
                true
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => self.fail_bundle_load(
                route_id,
                bundle_id,
                Some("bundle load channel disconnected".to_string()),
            ),
        }
    }

    pub(super) fn ensure_boot_active_route(&mut self, cx: &mut Cx) {
        if self.pending_navigation.is_some()
            || self.active_route.0 == 0
            || self.routes.widgets.contains_key(&self.active_route)
        {
            return;
        }

        let Some(bundle_id) = self.route_bundle_id(self.active_route) else {
            self.ensure_route_widget(cx, self.active_route);
            return;
        };

        let rx = cx.ensure_wasm_bundle(&bundle_id);
        match rx.try_recv_flush() {
            Ok(Ok(())) => {
                self.ensure_route_widget(cx, self.active_route);
            }
            Ok(Err(error)) => {
                let _ = self.fail_bundle_load(self.active_route, bundle_id, Some(error));
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                let route = self
                    .router
                    .current_route()
                    .cloned()
                    .unwrap_or_else(|| Route::new(self.active_route));
                self.pending_navigation = Some(PendingNavigation {
                    request: RouterNavRequest::Reset {
                        route: route.clone(),
                    },
                    context: RouterNavContext {
                        kind: RouterNavKind::Reset,
                        from: None,
                        to: Some(route),
                        to_path: None,
                    },
                    resolved_path: None,
                    phase: PendingNavPhase::BootBundleAsync,
                    async_index: 0,
                    redirect_depth: 0,
                    rx: PendingAsyncRx::Bundle {
                        route_id: self.active_route,
                        bundle_id,
                        rx,
                    },
                });
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                let _ = self.fail_bundle_load(
                    self.active_route,
                    bundle_id,
                    Some("bundle load channel disconnected".to_string()),
                );
            }
        }
    }

    fn apply_guards_and_maybe_commit(
        &mut self,
        cx: &mut Cx,
        mut request: RouterNavRequest,
        mut context: RouterNavContext,
        mut resolved_path: Option<ResolvedPathIntent>,
        mut redirect_depth: u8,
    ) -> bool {
        loop {
            let mut redirected = None;
            for guard in self.route_guards() {
                match guard(cx, &context) {
                    RouterGuardDecision::Allow => {}
                    RouterGuardDecision::Block => {
                        self.last_blocked_reason = Some(RouterBlockReason::GuardBlocked);
                        return false;
                    }
                    RouterGuardDecision::Redirect(redirect) => {
                        if redirect_depth >= ROUTER_MAX_REDIRECTS {
                            self.last_blocked_reason = Some(RouterBlockReason::RedirectLimit);
                            log!("Router: guard redirect limit reached");
                            return false;
                        }
                        redirect_depth += 1;
                        request = Self::redirect_to_request(redirect.target, redirect.replace);
                        let Some((next_context, _, next_resolved_path)) =
                            self.resolve_nav_context(cx, &request)
                        else {
                            return false;
                        };
                        context = next_context;
                        resolved_path = next_resolved_path;
                        redirected = Some(());
                        break;
                    }
                }
            }
            if redirected.is_some() {
                continue;
            }

            if self.has_async_route_guards() {
                return self.run_guard_async(
                    cx,
                    request,
                    context,
                    resolved_path,
                    0,
                    redirect_depth,
                );
            }

            return self.apply_request_after_bundle_check(
                cx,
                request,
                context,
                resolved_path,
                redirect_depth,
            );
        }
    }

    fn run_before_leave_async(
        &mut self,
        cx: &mut Cx,
        request: RouterNavRequest,
        context: RouterNavContext,
        resolved_path: Option<ResolvedPathIntent>,
        start_index: usize,
        redirect_depth: u8,
    ) -> bool {
        let mut idx = start_index;
        let hooks = self.before_leave_hooks_async();
        while idx < hooks.len() {
            match (hooks[idx])(cx, &context) {
                RouterAsyncDecision::Immediate(RouterBeforeLeaveDecision::Allow) => {
                    idx += 1;
                }
                RouterAsyncDecision::Immediate(RouterBeforeLeaveDecision::Block) => {
                    self.last_blocked_reason = Some(RouterBlockReason::BeforeLeaveBlocked);
                    return false;
                }
                RouterAsyncDecision::Pending(rx) => {
                    self.pending_navigation = Some(PendingNavigation {
                        request,
                        context,
                        resolved_path,
                        phase: PendingNavPhase::BeforeLeaveAsync,
                        async_index: idx,
                        redirect_depth,
                        rx: PendingAsyncRx::BeforeLeave(rx),
                    });
                    return true;
                }
            }
        }

        self.apply_guards_and_maybe_commit(cx, request, context, resolved_path, redirect_depth)
    }

    fn run_guard_async(
        &mut self,
        cx: &mut Cx,
        request: RouterNavRequest,
        context: RouterNavContext,
        resolved_path: Option<ResolvedPathIntent>,
        start_index: usize,
        redirect_depth: u8,
    ) -> bool {
        let mut idx = start_index;
        let guards = self.route_guards_async();
        while idx < guards.len() {
            match (guards[idx])(cx, &context) {
                RouterAsyncDecision::Immediate(RouterGuardDecision::Allow) => idx += 1,
                RouterAsyncDecision::Immediate(RouterGuardDecision::Block) => {
                    self.last_blocked_reason = Some(RouterBlockReason::GuardBlocked);
                    return false;
                }
                RouterAsyncDecision::Immediate(RouterGuardDecision::Redirect(redirect)) => {
                    if redirect_depth >= ROUTER_MAX_REDIRECTS {
                        self.last_blocked_reason = Some(RouterBlockReason::RedirectLimit);
                        log!("Router: guard redirect limit reached");
                        return false;
                    }
                    let next_request = Self::redirect_to_request(redirect.target, redirect.replace);
                    return self.request_navigation_internal(
                        cx,
                        next_request,
                        true,
                        redirect_depth.saturating_add(1),
                    );
                }
                RouterAsyncDecision::Pending(rx) => {
                    self.pending_navigation = Some(PendingNavigation {
                        request,
                        context,
                        resolved_path,
                        phase: PendingNavPhase::GuardAsync,
                        async_index: idx,
                        redirect_depth,
                        rx: PendingAsyncRx::Guard(rx),
                    });
                    return true;
                }
            }
        }

        self.apply_request_after_bundle_check(cx, request, context, resolved_path, redirect_depth)
    }

    fn redirect_to_request(target: RouterRedirectTarget, replace: bool) -> RouterNavRequest {
        match (target, replace) {
            (RouterRedirectTarget::Route(route_id), false) => {
                RouterNavRequest::Navigate { route_id }
            }
            (RouterRedirectTarget::Route(route_id), true) => RouterNavRequest::Replace { route_id },
            (RouterRedirectTarget::Path(path), false) => RouterNavRequest::NavigateByPath { path },
            (RouterRedirectTarget::Path(path), true) => RouterNavRequest::ReplaceByPath {
                path,
                clear_extras: true,
            },
        }
    }

    fn apply_request_bypassing_guards(&mut self, cx: &mut Cx, request: RouterNavRequest) -> bool {
        self.apply_request_bypassing_guards_resolved(cx, request, None)
    }

    fn apply_request_bypassing_guards_resolved(
        &mut self,
        cx: &mut Cx,
        request: RouterNavRequest,
        resolved_path: Option<ResolvedPathIntent>,
    ) -> bool {
        let prev = self.guard_bypass;
        self.guard_bypass = true;
        let out = match request {
            RouterNavRequest::Navigate { route_id } => self.navigate(cx, route_id),
            RouterNavRequest::NavigateWithTransition {
                route_id,
                transition,
            } => self.navigate_with_transition(cx, route_id, transition),
            RouterNavRequest::Replace { route_id } => self.replace(cx, route_id),
            RouterNavRequest::ReplaceWithTransition {
                route_id,
                transition,
            } => self.replace_with_transition(cx, route_id, transition),
            RouterNavRequest::NavigateByPath { path } => match resolved_path.as_ref() {
                Some(intent) => self.apply_resolved_path_intent(cx, intent),
                None => self.navigate_by_path(cx, &path),
            },
            RouterNavRequest::ReplaceByPath { path, clear_extras } => {
                match resolved_path.as_ref() {
                    Some(intent) => self.apply_resolved_path_intent(cx, intent),
                    None => self.replace_by_path_internal(cx, &path, clear_extras),
                }
            }
            RouterNavRequest::Back { transition } => match transition {
                Some(t) => self.back_with_transition(cx, t),
                None => self.back(cx),
            },
            RouterNavRequest::Forward { transition } => match transition {
                Some(t) => self.forward_with_transition(cx, t),
                None => self.forward(cx),
            },
            RouterNavRequest::Reset { route } => self.reset(cx, route),
            RouterNavRequest::SetStack { stack } => self.set_stack(cx, stack),
            RouterNavRequest::Pop => self.pop(cx),
            RouterNavRequest::PopTo { route_id } => self.pop_to(cx, route_id),
            RouterNavRequest::PopToRoot => self.pop_to_root(cx),
        };
        self.guard_bypass = prev;
        out
    }

    fn infer_resolution_block_reason(request: &RouterNavRequest) -> RouterBlockReason {
        match request {
            RouterNavRequest::Back { .. }
            | RouterNavRequest::Forward { .. }
            | RouterNavRequest::Pop
            | RouterNavRequest::PopTo { .. }
            | RouterNavRequest::PopToRoot => RouterBlockReason::NoHistory,
            _ => RouterBlockReason::RouteMissing,
        }
    }

    pub(super) fn poll_pending_navigation(&mut self, cx: &mut Cx) {
        let Some(pending) = self.pending_navigation.take() else {
            return;
        };

        let PendingNavigation {
            request,
            context,
            resolved_path,
            phase,
            async_index,
            redirect_depth,
            rx,
        } = pending;

        match phase {
            PendingNavPhase::BeforeLeaveAsync => {
                let PendingAsyncRx::BeforeLeave(rx) = rx else {
                    debug_assert!(
                        false,
                        "router before-leave phase paired with non before-leave rx"
                    );
                    return;
                };
                let decision = match rx.try_recv_flush() {
                    Ok(v) => v,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        self.pending_navigation = Some(PendingNavigation {
                            request,
                            context,
                            resolved_path,
                            phase,
                            async_index,
                            redirect_depth,
                            rx: PendingAsyncRx::BeforeLeave(rx),
                        });
                        return;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => return,
                };

                if decision != RouterBeforeLeaveDecision::Allow {
                    self.last_blocked_reason = Some(RouterBlockReason::BeforeLeaveBlocked);
                    return;
                }
                let _ = self.run_before_leave_async(
                    cx,
                    request,
                    context,
                    resolved_path,
                    async_index + 1,
                    redirect_depth,
                );
            }
            PendingNavPhase::GuardAsync => {
                let PendingAsyncRx::Guard(rx) = rx else {
                    debug_assert!(false, "router guard phase paired with non guard rx");
                    return;
                };
                let decision = match rx.try_recv_flush() {
                    Ok(v) => v,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        self.pending_navigation = Some(PendingNavigation {
                            request,
                            context,
                            resolved_path,
                            phase,
                            async_index,
                            redirect_depth,
                            rx: PendingAsyncRx::Guard(rx),
                        });
                        return;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => return,
                };

                match decision {
                    RouterGuardDecision::Allow => {
                        let _ = self.run_guard_async(
                            cx,
                            request,
                            context,
                            resolved_path,
                            async_index + 1,
                            redirect_depth,
                        );
                    }
                    RouterGuardDecision::Block => {
                        self.last_blocked_reason = Some(RouterBlockReason::GuardBlocked);
                    }
                    RouterGuardDecision::Redirect(redirect) => {
                        if redirect_depth >= ROUTER_MAX_REDIRECTS {
                            self.last_blocked_reason = Some(RouterBlockReason::RedirectLimit);
                            log!("Router: guard redirect limit reached");
                            return;
                        }
                        let next_request =
                            Self::redirect_to_request(redirect.target, redirect.replace);
                        let _ = self.request_navigation_internal(
                            cx,
                            next_request,
                            true,
                            redirect_depth.saturating_add(1),
                        );
                    }
                }
            }
            PendingNavPhase::BundleAsync => {
                let PendingAsyncRx::Bundle {
                    route_id,
                    bundle_id,
                    rx,
                } = rx
                else {
                    debug_assert!(false, "router bundle phase paired with non bundle rx");
                    return;
                };
                let result = match rx.try_recv_flush() {
                    Ok(v) => v,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        self.pending_navigation = Some(PendingNavigation {
                            request,
                            context,
                            resolved_path,
                            phase,
                            async_index,
                            redirect_depth,
                            rx: PendingAsyncRx::Bundle {
                                route_id,
                                bundle_id,
                                rx,
                            },
                        });
                        return;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        let _ = self.fail_bundle_load(
                            route_id,
                            bundle_id,
                            Some("bundle load channel disconnected".to_string()),
                        );
                        return;
                    }
                };

                match result {
                    Ok(()) => {
                        let _ = self.apply_request_bypassing_guards_resolved(
                            cx,
                            request,
                            resolved_path,
                        );
                    }
                    Err(error) => {
                        let _ = self.fail_bundle_load(route_id, bundle_id, Some(error));
                    }
                }
            }
            PendingNavPhase::BootBundleAsync => {
                let PendingAsyncRx::Bundle {
                    route_id,
                    bundle_id,
                    rx,
                } = rx
                else {
                    debug_assert!(false, "router boot bundle phase paired with non bundle rx");
                    return;
                };
                let result = match rx.try_recv_flush() {
                    Ok(v) => v,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        self.pending_navigation = Some(PendingNavigation {
                            request,
                            context,
                            resolved_path,
                            phase,
                            async_index,
                            redirect_depth,
                            rx: PendingAsyncRx::Bundle {
                                route_id,
                                bundle_id,
                                rx,
                            },
                        });
                        return;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        let _ = self.fail_bundle_load(
                            route_id,
                            bundle_id,
                            Some("bundle load channel disconnected".to_string()),
                        );
                        return;
                    }
                };

                match result {
                    Ok(()) => {
                        self.ensure_route_widget(cx, route_id);
                    }
                    Err(error) => {
                        let _ = self.fail_bundle_load(route_id, bundle_id, Some(error));
                    }
                }
            }
        }
    }
}
