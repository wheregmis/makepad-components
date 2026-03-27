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
            let mut joined = String::with_capacity(tail_trim.len() + 1);
            // Optimization: nested router path joins run during current/preview URL resolution.
            // Build the path directly into one pre-sized buffer instead of going through `format!`.
            joined.push('/');
            joined.push_str(tail_trim);
            joined
        } else {
            let mut joined = String::with_capacity(base_trim.len() + tail_trim.len() + 1);
            joined.push_str(base_trim);
            joined.push('/');
            joined.push_str(tail_trim);
            joined
        }
    }

    pub(super) fn current_path_for_route(&self, route: &crate::route::Route) -> String {
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
        // Optimization: `current_path_for_route` already allocates the base URL path.
        // Reuse that buffer for query/hash suffixes instead of formatting a second String.
        let mut url = self.current_path_for_route(route);
        if !route.query.data.is_empty() {
            url.push_str(&route.query_string());
        }
        if !route.hash.is_empty() {
            url.push_str(&route.hash);
        }
        url
    }

    fn current_path(&self) -> String {
        let Some(route) = self.router.current_route() else {
            return "/".to_string();
        };
        self.current_path_for_route(route)
    }

    pub(super) fn preview_back_url(&self) -> Option<String> {
        self.router
            .preview_back_route()
            .map(|route| self.url_for_route(route))
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

#[cfg(test)]
mod tests {
    use super::RouterWidget;
    use std::hint::black_box;
    use std::time::Instant;

    #[test]
    fn join_paths_handles_root_and_nested_segments() {
        assert_eq!(RouterWidget::join_paths("/", "/child"), "/child");
        assert_eq!(
            RouterWidget::join_paths("/admin/", "reports"),
            "/admin/reports"
        );
        assert_eq!(
            RouterWidget::join_paths(" /admin ", " /reports/2026 "),
            "/admin/reports/2026"
        );
    }

    #[test]
    #[ignore = "micro-benchmark; run explicitly in release mode for stable numbers"]
    fn bench_join_paths_direct_buffer_build() {
        fn legacy_join_paths(base: &str, tail: &str) -> String {
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

        const ITERATIONS: usize = 200_000;
        let base = "/team/alpha/member/beta";
        let tail = "/settings/profile";

        let old_start = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(legacy_join_paths(base, tail));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(RouterWidget::join_paths(base, tail));
        }
        let new_elapsed = new_start.elapsed();

        println!(
            "legacy_join_paths={:?} new_join_paths={:?} improvement={:.2}%",
            old_elapsed,
            new_elapsed,
            (1.0 - (new_elapsed.as_secs_f64() / old_elapsed.as_secs_f64())) * 100.0
        );
    }
}
