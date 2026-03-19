use crate::route::Route;
use crate::router::RouterAction;
use makepad_widgets::*;

use super::{
    RouterBlockReason, RouterCommand, RouterConfig, RouterDispatchResult, RouterWidget,
    RouterWidgetRef,
};

impl RouterWidget {
    pub fn apply_config(&mut self, config: RouterConfig) {
        self.cap_guards_sync = config.capabilities.guards_sync;
        self.cap_guards_async = config.capabilities.guards_async;
        self.cap_transitions = config.capabilities.transitions;
        self.cap_nested = config.capabilities.nested;
        self.cap_persistence = config.capabilities.persistence;

        if config.default_route.0 != 0 {
            self.default_route = config.default_route;
        }
        if config.not_found_route.0 != 0 {
            self.not_found_route = config.not_found_route;
        }
        if config.default_transition.0 != 0 {
            self.push_transition = config.default_transition;
            self.pop_transition = config.default_transition;
            self.replace_transition = config.default_transition;
        }

        self.persist_state = config.persist_state;
        if let Some(browser_base_path) = config.browser_base_path {
            self.browser_base_path
                .as_mut_empty()
                .push_str(&browser_base_path);
            self.inferred_browser_base_path.clear();
        }
    }

    fn primary_action_for_command(
        command: &RouterCommand,
        to: Option<&Route>,
    ) -> Option<RouterAction> {
        match command {
            RouterCommand::GoToRoute { .. }
            | RouterCommand::GoToPath { .. }
            | RouterCommand::Push { .. } => to.cloned().map(RouterAction::Navigate),
            RouterCommand::ReplaceRoute { .. } | RouterCommand::ReplacePath { .. } => {
                to.cloned().map(RouterAction::Replace)
            }
            RouterCommand::Back { .. } => Some(RouterAction::Back),
            RouterCommand::Forward { .. } => Some(RouterAction::Forward),
            RouterCommand::Reset { .. } | RouterCommand::SetStack { .. } => {
                to.cloned().map(RouterAction::Reset)
            }
            RouterCommand::Pop | RouterCommand::PopTo { .. } | RouterCommand::PopToRoot => None,
        }
    }

    fn infer_block_reason(
        &self,
        command: &RouterCommand,
        from: Option<&Route>,
        to: Option<&Route>,
    ) -> Option<RouterBlockReason> {
        match command {
            RouterCommand::Back { .. }
            | RouterCommand::Forward { .. }
            | RouterCommand::Pop
            | RouterCommand::PopTo { .. }
            | RouterCommand::PopToRoot => {
                if from == to {
                    Some(RouterBlockReason::NoHistory)
                } else {
                    None
                }
            }
            RouterCommand::GoToRoute { .. }
            | RouterCommand::ReplaceRoute { .. }
            | RouterCommand::Push { .. }
            | RouterCommand::GoToPath { .. }
            | RouterCommand::ReplacePath { .. }
            | RouterCommand::Reset { .. }
            | RouterCommand::SetStack { .. } => {
                if from == to {
                    Some(RouterBlockReason::RouteMissing)
                } else {
                    None
                }
            }
        }
    }

    pub fn dispatch(&mut self, cx: &mut Cx, command: RouterCommand) -> RouterDispatchResult {
        let from = self.router.current_route().cloned();
        self.last_blocked_reason = None;

        let changed = match &command {
            RouterCommand::GoToRoute {
                route_id,
                transition,
            } => match transition {
                Some(t) => self.navigate_with_transition(cx, *route_id, *t),
                None => self.navigate(cx, *route_id),
            },
            RouterCommand::GoToPath { path } => self.navigate_by_path(cx, path),
            RouterCommand::ReplaceRoute {
                route_id,
                transition,
            } => match transition {
                Some(t) => self.replace_with_transition(cx, *route_id, *t),
                None => self.replace(cx, *route_id),
            },
            RouterCommand::ReplacePath { path, clear_extras } => {
                self.replace_by_path(cx, path, *clear_extras)
            }
            RouterCommand::Back { transition } => match transition {
                Some(t) => self.back_with_transition(cx, *t),
                None => self.back(cx),
            },
            RouterCommand::Forward { transition } => match transition {
                Some(t) => self.forward_with_transition(cx, *t),
                None => self.forward(cx),
            },
            RouterCommand::Reset { route } => self.reset(cx, route.clone()),
            RouterCommand::Push {
                route_id,
                transition,
            } => match transition {
                Some(t) => self.navigate_with_transition(cx, *route_id, *t),
                None => self.push(cx, *route_id),
            },
            RouterCommand::Pop => self.pop(cx),
            RouterCommand::PopTo { route_id } => self.pop_to(cx, *route_id),
            RouterCommand::PopToRoot => self.pop_to_root(cx),
            RouterCommand::SetStack { stack } => self.set_stack(cx, stack.clone()),
        };

        let to = self.router.current_route().cloned();
        let blocked_reason = if changed {
            None
        } else {
            self.last_blocked_reason
                .clone()
                .or_else(|| self.infer_block_reason(&command, from.as_ref(), to.as_ref()))
        };

        RouterDispatchResult {
            changed,
            from,
            action: if changed {
                Self::primary_action_for_command(&command, to.as_ref())
            } else {
                None
            },
            to,
            blocked_reason,
        }
    }
}

impl RouterWidgetRef {
    pub fn go_to_route(&self, cx: &mut Cx, route_id: LiveId) -> bool {
        self.dispatch(
            cx,
            RouterCommand::GoToRoute {
                route_id,
                transition: None,
            },
        )
        .changed
    }

    pub fn go_to_path(&self, cx: &mut Cx, path: impl Into<String>) -> bool {
        self.dispatch(cx, RouterCommand::GoToPath { path: path.into() })
            .changed
    }

    pub fn replace_route(&self, cx: &mut Cx, route_id: LiveId) -> bool {
        self.dispatch(
            cx,
            RouterCommand::ReplaceRoute {
                route_id,
                transition: None,
            },
        )
        .changed
    }

    pub fn replace_path(&self, cx: &mut Cx, path: impl Into<String>) -> bool {
        self.dispatch(
            cx,
            RouterCommand::ReplacePath {
                path: path.into(),
                clear_extras: true,
            },
        )
        .changed
    }

    pub fn back(&self, cx: &mut Cx) -> bool {
        self.dispatch(cx, RouterCommand::Back { transition: None })
            .changed
    }

    pub fn forward(&self, cx: &mut Cx) -> bool {
        self.dispatch(cx, RouterCommand::Forward { transition: None })
            .changed
    }

    pub fn dispatch(&self, cx: &mut Cx, command: RouterCommand) -> RouterDispatchResult {
        if let Some(mut inner) = self.borrow_mut() {
            inner.dispatch(cx, command)
        } else {
            RouterDispatchResult::blocked(None, None, RouterBlockReason::NoHistory)
        }
    }

    pub fn apply_config(&self, config: RouterConfig) -> Result<(), String> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.apply_config(config);
            Ok(())
        } else {
            Err("Cannot borrow router widget".to_string())
        }
    }
}
