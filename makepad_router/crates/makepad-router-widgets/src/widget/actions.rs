use crate::router::RouterAction;
use makepad_widgets::*;

use super::RouterWidget;

impl RouterWidget {
    pub(super) fn queue_route_actions(
        &mut self,
        primary_action: Option<RouterAction>,
        old_route_id: Option<LiveId>,
        new_route_id: LiveId,
    ) {
        if let Some(primary_action) = primary_action {
            self.pending_actions.push(primary_action);
        }
        self.pending_actions.push(RouterAction::RouteChanged {
            from: old_route_id,
            to: new_route_id,
        });
    }

    pub(super) fn flush_router_actions(&mut self, cx: &mut Cx, _scope: &mut Scope) {
        if self.pending_actions.is_empty() {
            return;
        }
        let uid = self.widget_uid();
        for action in self.pending_actions.drain(..) {
            cx.widget_action(uid, action);
        }
    }
}
