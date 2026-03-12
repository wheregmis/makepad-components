use crate::route::Route;
use makepad_widgets::Cx;

use super::RouterWidget;

impl RouterWidget {
    pub fn on_route_change<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cx, Option<Route>, Route) + Send + Sync + 'static,
    {
        self.callbacks.route_change.push(Box::new(callback));
    }

    pub(super) fn dispatch_route_change(
        &self,
        cx: &mut Cx,
        old_route: Option<Route>,
        new_route: Route,
    ) {
        for callback in &self.callbacks.route_change {
            callback(cx, old_route.clone(), new_route.clone());
        }
    }
}
