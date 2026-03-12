use super::RouterWidget;

impl RouterWidget {
    pub(super) fn clear_url_extras(&mut self) {
        self.url_path_override = None;
    }

    fn join_paths(base: &str, tail: &str) -> String {
        let base = base.trim();
        let tail = tail.trim();

        let base = if base.is_empty() { "/" } else { base };
        let base_trim = base.trim_end_matches('/');
        let tail_trim = tail.trim_start_matches('/');

        if tail_trim.is_empty() {
            if base_trim.is_empty() || base_trim == "/" {
                "/".to_string()
            } else {
                base_trim.to_string()
            }
        } else if base_trim.is_empty() || base_trim == "/" {
            format!("/{}", tail_trim)
        } else {
            format!("{}/{}", base_trim, tail_trim)
        }
    }

    fn current_path_for_route(&self, route: &crate::route::Route) -> String {
        // Keep unknown path visible while showing the configured not-found route.
        if self.not_found_route.0 != 0 && route.id == self.not_found_route {
            if let Some(path) = &self.url_path_override {
                let mut p = path.trim().to_string();
                if p.is_empty() {
                    p = "/".to_string();
                } else if !p.starts_with('/') {
                    p.insert(0, '/');
                }
                return p;
            }
        }

        let pattern = route
            .pattern
            .as_ref()
            .or_else(|| self.router.route_registry.get_pattern(route.id));

        let base = if let Some(pattern) = pattern {
            pattern
                .format_path(&route.params)
                .unwrap_or_else(|| pattern.format_base_path(&route.params))
        } else {
            let s = route.id.to_string();
            if s.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", s)
            }
        };

        let Some(child_router) = self.child_routers.get(&route.id) else {
            return base;
        };
        let Some(child) = child_router.borrow() else {
            return base;
        };
        let tail = child.current_path();
        Self::join_paths(&base, &tail)
    }

    pub(super) fn url_for_route(&self, route: &crate::route::Route) -> String {
        format!(
            "{}{}{}",
            self.current_path_for_route(route),
            route.query_string(),
            route.hash
        )
    }

    fn current_path(&self) -> String {
        let Some(route) = self.router.current_route() else {
            return "/".to_string();
        };
        self.current_path_for_route(route)
    }

    pub(super) fn preview_back_url(&self) -> Option<String> {
        self.router.preview_back_route().map(|route| self.url_for_route(route))
    }

    pub(super) fn preview_forward_url(&self) -> Option<String> {
        self.router
            .preview_forward_route()
            .map(|route| self.url_for_route(route))
    }

    pub fn current_url(&self) -> String {
        let Some(route) = self.router.current_route() else {
            return self.current_path();
        };
        self.url_for_route(route)
    }
}
