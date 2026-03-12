//! Public guard API plumbing for RouterWidget.

use crate::guards::{
    RouterAsyncDecision, RouterAsyncGuard, RouterBeforeLeaveAsync, RouterBeforeLeaveDecision,
    RouterBeforeLeaveSync, RouterGuardDecision, RouterNavContext, RouterSyncGuard,
};
use makepad_widgets::Cx;

use super::{RouterBlockReason, RouterWidget};

impl RouterWidget {
    pub fn add_route_guard<F>(&mut self, guard: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterGuardDecision + Send + Sync + 'static,
    {
        if !self.guards_sync_enabled() {
            self.last_blocked_reason = Some(RouterBlockReason::CapabilityDisabled);
            return Err(RouterBlockReason::CapabilityDisabled);
        }
        self.guards.route_guards.push(Box::new(guard));
        Ok(())
    }

    pub fn add_route_guard_async<F>(&mut self, guard: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterGuardDecision>
            + Send
            + Sync
            + 'static,
    {
        if !self.guards_async_enabled() {
            self.last_blocked_reason = Some(RouterBlockReason::CapabilityDisabled);
            return Err(RouterBlockReason::CapabilityDisabled);
        }
        self.guards.route_guards_async.push(Box::new(guard));
        Ok(())
    }

    pub fn add_before_leave_hook<F>(&mut self, hook: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterBeforeLeaveDecision + Send + Sync + 'static,
    {
        if !self.guards_sync_enabled() {
            self.last_blocked_reason = Some(RouterBlockReason::CapabilityDisabled);
            return Err(RouterBlockReason::CapabilityDisabled);
        }
        self.guards.before_leave_hooks.push(Box::new(hook));
        Ok(())
    }

    pub fn add_before_leave_hook_async<F>(&mut self, hook: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterBeforeLeaveDecision>
            + Send
            + Sync
            + 'static,
    {
        if !self.guards_async_enabled() {
            self.last_blocked_reason = Some(RouterBlockReason::CapabilityDisabled);
            return Err(RouterBlockReason::CapabilityDisabled);
        }
        self.guards.before_leave_hooks_async.push(Box::new(hook));
        Ok(())
    }

    pub(super) fn has_async_before_leave_hooks(&self) -> bool {
        if !self.guards_async_enabled() {
            return false;
        }
        !self.guards.before_leave_hooks_async.is_empty()
    }

    pub(super) fn has_async_route_guards(&self) -> bool {
        if !self.guards_async_enabled() {
            return false;
        }
        !self.guards.route_guards_async.is_empty()
    }

    pub(super) fn before_leave_hooks(&self) -> &[RouterBeforeLeaveSync] {
        if !self.guards_sync_enabled() {
            return &[];
        }
        &self.guards.before_leave_hooks
    }

    pub(super) fn before_leave_hooks_async(&self) -> &[RouterBeforeLeaveAsync] {
        if !self.guards_async_enabled() {
            return &[];
        }
        &self.guards.before_leave_hooks_async
    }

    pub(super) fn route_guards(&self) -> &[RouterSyncGuard] {
        if !self.guards_sync_enabled() {
            return &[];
        }
        &self.guards.route_guards
    }

    pub(super) fn route_guards_async(&self) -> &[RouterAsyncGuard] {
        if !self.guards_async_enabled() {
            return &[];
        }
        &self.guards.route_guards_async
    }
}
