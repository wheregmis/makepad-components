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
        old_route: &Option<Route>,
        new_route: &Route,
    ) {
        if self.callbacks.route_change.is_empty() {
            return;
        }
        for callback in &self.callbacks.route_change {
            callback(cx, old_route.clone(), new_route.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use makepad_widgets::LiveId;

    #[test]
    fn clone_cost_measurement() {
        // Since we are optimizing away clones when there are no callbacks, let's verify
        // the theoretical savings. Route cloning allocates memory for its path segments and queries.
        let mut route = Route::new(LiveId::from_str("home"))
            .param(LiveId::from_str("path_1"), LiveId::from_str("test1"))
            .param(LiveId::from_str("path_2"), LiveId::from_str("test2"));
        route
            .query
            .data
            .insert("foo".to_string(), "bar".to_string());
        route.query.data.insert("num".to_string(), "42".to_string());

        let iterations = 100_000;
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let _cloned = route.clone();
            let _cloned_opt = Some(route.clone());
        }
        let duration_with_clones = start.elapsed();

        let start_refs = std::time::Instant::now();
        let opt_route = Some(route.clone());
        for _ in 0..iterations {
            let _ref = &route;
            let _ref_opt = &opt_route;
        }
        let duration_with_refs = start_refs.elapsed();

        println!(
            "dispatch_route_change cost over {} iterations:\nWith clones: {:?}\nWith refs: {:?}",
            iterations, duration_with_clones, duration_with_refs
        );
        // By changing the function signature, we save all the allocation time
        // measured in `duration_with_clones` when the callbacks vector is empty.
    }
}
