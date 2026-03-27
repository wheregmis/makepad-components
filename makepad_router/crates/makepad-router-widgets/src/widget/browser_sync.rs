use std::borrow::Cow;

use crate::{router::RouterAction, url::RouterUrl};
use makepad_widgets::*;

use super::{BrowserUrlMode, RouterWidget};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BrowserSync {
    Push,
    Replace,
    History(i32),
}

impl RouterWidget {
    fn browser_sync_enabled(&self, cx: &Cx) -> bool {
        self.sync_browser_url && cx.os_type().is_web()
    }

    fn normalized_browser_base_path_cow(base_path: &str) -> Cow<'_, str> {
        let trimmed = base_path.trim();
        if trimmed.is_empty() || trimmed == "/" {
            return Cow::Borrowed("");
        }

        let normalized_end = trimmed.trim_end_matches('/').len();
        if normalized_end == 0 {
            return Cow::Borrowed("");
        }

        // Optimization: browser base paths are typically already normalized.
        // Borrowing avoids rebuilding the same String during every sync event.
        if trimmed.starts_with('/') && normalized_end == trimmed.len() {
            return Cow::Borrowed(trimmed);
        }

        let trimmed = &trimmed[..normalized_end];
        let mut normalized = String::with_capacity(trimmed.len() + 1);
        if !trimmed.starts_with('/') {
            normalized.push('/');
        }
        normalized.push_str(trimmed);
        Cow::Owned(normalized)
    }

    fn normalized_browser_base_path(base_path: &str) -> String {
        Self::normalized_browser_base_path_cow(base_path).into_owned()
    }

    fn normalized_browser_path_cow(pathname: &str) -> Cow<'_, str> {
        let trimmed = pathname.trim();
        if trimmed.is_empty() {
            return Cow::Borrowed("/");
        }

        // Optimization: most browser pathnames are already absolute, so borrow them directly.
        if trimmed.starts_with('/') {
            Cow::Borrowed(trimmed)
        } else {
            let mut normalized = String::with_capacity(trimmed.len() + 1);
            normalized.push('/');
            normalized.push_str(trimmed);
            Cow::Owned(normalized)
        }
    }

    fn normalized_browser_path(pathname: &str) -> String {
        Self::normalized_browser_path_cow(pathname).into_owned()
    }

    fn strip_browser_base_path(pathname: &str, base_path: &str) -> String {
        let normalized_path = Self::normalized_browser_path_cow(pathname);
        let normalized_base = Self::normalized_browser_base_path_cow(base_path);

        if normalized_base.is_empty() {
            return normalized_path.into_owned();
        }

        let normalized_path_ref = normalized_path.as_ref();
        let normalized_base_ref = normalized_base.as_ref();
        if normalized_path_ref == normalized_base_ref
            || normalized_path_ref
                .strip_suffix('/')
                .is_some_and(|path| path == normalized_base_ref)
        {
            return "/".to_string();
        }

        if normalized_path_ref.len() > normalized_base_ref.len()
            && normalized_path_ref.starts_with(normalized_base_ref)
            && normalized_path_ref.as_bytes()[normalized_base_ref.len()] == b'/'
        {
            let stripped = normalized_path_ref[normalized_base_ref.len()..].trim_start_matches('/');
            if stripped.is_empty() {
                return "/".to_string();
            }

            let mut normalized = String::with_capacity(stripped.len() + 1);
            normalized.push('/');
            normalized.push_str(stripped);
            return normalized;
        }

        normalized_path.into_owned()
    }

    fn prefix_clean_browser_base_path_parts(
        path: &str,
        query: &str,
        hash: &str,
        base_path: &str,
    ) -> String {
        let normalized_base = Self::normalized_browser_base_path_cow(base_path);
        if normalized_base.is_empty() {
            let mut out = String::with_capacity(path.len() + query.len() + hash.len());
            out.push_str(path);
            out.push_str(query);
            out.push_str(hash);
            return out;
        }

        let path = if path.is_empty() { "/" } else { path };
        let mut out = String::with_capacity(
            normalized_base.len() + path.len() + query.len() + hash.len() + 1,
        );
        out.push_str(normalized_base.as_ref());
        if path == "/" {
            out.push('/');
        } else {
            out.push_str(path);
        }
        out.push_str(query);
        out.push_str(hash);
        out
    }

    fn prefix_clean_browser_base_path(route_url: &str, base_path: &str) -> String {
        let parsed = RouterUrl::parse(route_url);
        Self::prefix_clean_browser_base_path_parts(
            &parsed.path,
            &parsed.query,
            &parsed.hash,
            base_path,
        )
    }

    fn prefix_hash_browser_base_path(route_url: &str, base_path: &str) -> String {
        let route_url = route_url.trim();
        let route_url = if route_url.is_empty() { "/" } else { route_url };
        let normalized_base = Self::normalized_browser_base_path_cow(base_path);

        let extra = if normalized_base.is_empty() { 2 } else { 3 };
        let mut out = String::with_capacity(normalized_base.len() + route_url.len() + extra);
        if normalized_base.is_empty() {
            out.push_str("/#");
        } else {
            out.push_str(normalized_base.as_ref());
            out.push_str("/#");
        }
        out.push_str(route_url);
        out
    }

    fn configured_browser_base_path(&self) -> String {
        Self::normalized_browser_base_path(self.browser_base_path.as_ref())
    }

    fn effective_browser_base_path(&self) -> &str {
        if self.browser_base_path.as_ref().trim().is_empty() {
            self.inferred_browser_base_path.as_str()
        } else {
            self.browser_base_path.as_ref()
        }
    }

    fn has_real_route_match(&mut self, path: &str) -> bool {
        let parsed = self.parse_url_cached(path);
        let normalized_path = parsed.path;

        self.router
            .route_registry
            .resolve_path(&normalized_path)
            .is_some_and(|route| self.routes.templates.contains_key(&route.id))
            || self
                .resolve_nested_prefix(&normalized_path)
                .is_some_and(|(route_id, _, _, _)| self.routes.templates.contains_key(&route_id))
    }

    fn infer_browser_base_path(&mut self, pathname: &str) -> String {
        let normalized_path = Self::normalized_browser_path(pathname);
        if self.has_real_route_match(&normalized_path) {
            return String::new();
        }

        let trimmed = normalized_path.trim_start_matches('/');
        if trimmed.is_empty() {
            return String::new();
        }

        let mut candidate = String::new();
        let mut offset = normalized_path.len() - trimmed.len();
        for segment in trimmed.split('/') {
            if segment.is_empty() {
                offset += 1;
                continue;
            }
            candidate.push('/');
            candidate.push_str(segment);

            // Only infer bases that match the browser pathname byte-for-byte up to this
            // offset. If redundant slashes appeared earlier in the candidate prefix, later
            // stripping would reject that base anyway, so skip probing it here.
            // Optimization: this scan already walks a normalized `pathname` in order.
            // Slice the remaining suffix directly instead of rebuilding a stripped String
            // for every candidate prefix we probe during base-path inference. Normalize
            // duplicate leading slashes on the suffix so clean-path inference keeps the
            // same behavior as `strip_browser_base_path` for redundant-slash inputs.
            offset += segment.len();
            if normalized_path.get(..offset) == Some(candidate.as_str()) {
                let stripped = if offset >= normalized_path.len() {
                    std::borrow::Cow::Borrowed("/")
                } else {
                    let stripped = &normalized_path[offset..];
                    if stripped.starts_with("//") {
                        let trimmed = stripped.trim_start_matches('/');
                        if trimmed.is_empty() {
                            std::borrow::Cow::Borrowed("/")
                        } else {
                            let mut normalized = String::with_capacity(trimmed.len() + 1);
                            normalized.push('/');
                            normalized.push_str(trimmed);
                            std::borrow::Cow::Owned(normalized)
                        }
                    } else {
                        std::borrow::Cow::Borrowed(stripped)
                    }
                };
                if self.has_real_route_match(stripped.as_ref()) {
                    return candidate;
                }
            }

            offset += 1;
        }

        String::new()
    }

    fn refresh_inferred_browser_base_path(&mut self, pathname: &str) {
        if !self.configured_browser_base_path().is_empty() {
            self.inferred_browser_base_path.clear();
            return;
        }

        self.inferred_browser_base_path = self.infer_browser_base_path(pathname);
    }

    fn route_url_from_clean_browser_parts(
        pathname: &str,
        base_path: &str,
        search: &str,
        hash: &str,
    ) -> String {
        // Optimization: browser sync already allocates the stripped route path.
        // Reuse that buffer for query/hash suffixes instead of formatting a second String.
        let mut route_url = Self::strip_browser_base_path(pathname, base_path);
        route_url.reserve(search.len() + hash.len());
        route_url.push_str(search);
        route_url.push_str(hash);
        route_url
    }

    fn route_url_from_hash_fragment(hash: &str) -> String {
        let route = hash.trim().trim_start_matches('#');
        if route.is_empty() {
            return "/".to_string();
        }
        if route.starts_with('/') {
            return route.to_string();
        }

        let mut normalized = String::with_capacity(route.len() + 1);
        normalized.push('/');
        normalized.push_str(route);
        normalized
    }

    fn browser_url_from_os(&mut self, cx: &Cx) -> Option<String> {
        let OsType::Web(params) = cx.os_type() else {
            return None;
        };

        let pathname = Self::normalized_browser_path_cow(&params.pathname);
        self.refresh_inferred_browser_base_path(pathname.as_ref());

        match self.browser_url_mode {
            BrowserUrlMode::CleanPath => Some(Self::route_url_from_clean_browser_parts(
                pathname.as_ref(),
                self.effective_browser_base_path(),
                &params.search,
                &params.hash,
            )),
            BrowserUrlMode::HashPath => Some(Self::route_url_from_hash_fragment(&params.hash)),
        }
    }

    fn route_url_to_browser_url(&self, route_url: &str) -> String {
        match self.browser_url_mode {
            BrowserUrlMode::CleanPath => {
                Self::prefix_clean_browser_base_path(route_url, self.effective_browser_base_path())
            }
            BrowserUrlMode::HashPath => {
                Self::prefix_hash_browser_base_path(route_url, self.effective_browser_base_path())
            }
        }
    }

    fn browser_sync_url(&self) -> String {
        match self.browser_url_mode {
            BrowserUrlMode::CleanPath => {
                let base_path = self.effective_browser_base_path();
                if let Some(route) = self.router.current_route() {
                    let path = self.current_path_for_route(route);
                    let query = if route.query.data.is_empty() {
                        String::new()
                    } else {
                        // Optimization: browser sync already has the current route parts.
                        // Previously: built a full route URL, reparsed it, then appended the base path.
                        // Now: append the base path directly to the path/query/hash pieces and skip that extra work.
                        route.query_string()
                    };
                    Self::prefix_clean_browser_base_path_parts(
                        &path,
                        &query,
                        &route.hash,
                        base_path,
                    )
                } else {
                    Self::prefix_clean_browser_base_path_parts("/", "", "", base_path)
                }
            }
            BrowserUrlMode::HashPath => self.route_url_to_browser_url(&self.current_url()),
        }
    }

    fn sync_browser(&mut self, cx: &mut Cx, sync: BrowserSync) {
        if !self.browser_sync_enabled(cx) || self.browser_sync_inbound {
            return;
        }

        if self.browser_sync_suppress_once {
            self.browser_sync_suppress_once = false;
            return;
        }

        match sync {
            BrowserSync::Push => cx.browser_update_url(&self.browser_sync_url(), false),
            BrowserSync::Replace => cx.browser_update_url(&self.browser_sync_url(), true),
            BrowserSync::History(delta) => {
                if delta != 0 {
                    cx.browser_history_go(delta);
                }
            }
        }
    }

    pub(super) fn sync_browser_with_action(&mut self, cx: &mut Cx, action: &RouterAction) {
        match action {
            RouterAction::Navigate(_) => self.sync_browser(cx, BrowserSync::Push),
            RouterAction::Replace(_) | RouterAction::Reset(_) => {
                self.sync_browser(cx, BrowserSync::Replace)
            }
            RouterAction::Back => self.sync_browser(cx, BrowserSync::History(-1)),
            RouterAction::Forward => self.sync_browser(cx, BrowserSync::History(1)),
            RouterAction::RouteChanged { .. } => {}
        }
    }

    pub(super) fn sync_browser_after_pop(&mut self, cx: &mut Cx, steps: usize) {
        if steps == 0 {
            return;
        }
        self.sync_browser(cx, BrowserSync::History(-(steps as i32)));
    }

    pub(super) fn bootstrap_from_browser_url(&mut self, cx: &mut Cx) {
        if !self.browser_sync_enabled(cx) {
            return;
        }

        let Some(route_url) = self.browser_url_from_os(cx) else {
            return;
        };

        if route_url == self.current_url() {
            return;
        }

        self.browser_sync_inbound = true;
        self.browser_sync_suppress_once = true;
        let changed = self.replace_by_path_internal(cx, &route_url, true);
        self.browser_sync_inbound = false;

        if !changed {
            self.browser_sync_suppress_once = false;
            self.sync_browser(cx, BrowserSync::Replace);
        }
    }

    pub(super) fn handle_browser_location_signal(&mut self, cx: &mut Cx) {
        if !self.browser_sync_enabled(cx) {
            return;
        }

        let Some(route_url) = self.browser_url_from_os(cx) else {
            return;
        };

        if route_url == self.current_url() {
            return;
        }

        self.browser_sync_inbound = true;
        self.browser_sync_suppress_once = true;
        let handled = if self.preview_back_url().as_deref() == Some(route_url.as_str()) {
            self.back(cx)
        } else if self.preview_forward_url().as_deref() == Some(route_url.as_str()) {
            self.forward(cx)
        } else {
            self.replace_by_path_internal(cx, &route_url, true)
        };
        self.browser_sync_inbound = false;

        if !handled {
            self.browser_sync_suppress_once = false;
            self.sync_browser(cx, BrowserSync::Replace);
        }
    }

    pub(super) fn handle_descendant_router_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.browser_sync_enabled(cx) || self.browser_sync_inbound {
            return;
        }

        let mut saw_route_change = false;

        for action in actions {
            let Some(widget_action) = action.downcast_ref::<WidgetAction>() else {
                continue;
            };
            let Some(router_action) = widget_action.action.downcast_ref::<RouterAction>() else {
                continue;
            };

            match router_action {
                RouterAction::RouteChanged { .. } => saw_route_change = true,
                _ => {
                    self.sync_browser_with_action(cx, router_action);
                    return;
                }
            }
        }

        if saw_route_change {
            self.sync_browser(cx, BrowserSync::Replace);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RouterWidget;
    use std::hint::black_box;
    use std::time::Instant;

    fn old_infer_browser_base_path(pathname: &str, match_path: &str) -> String {
        let normalized_path = RouterWidget::normalized_browser_path(pathname);
        let trimmed = normalized_path.trim_start_matches('/');
        let mut candidate = String::new();
        for segment in trimmed.split('/') {
            if segment.is_empty() {
                continue;
            }
            candidate.push('/');
            candidate.push_str(segment);

            let stripped = RouterWidget::strip_browser_base_path(&normalized_path, &candidate);
            if stripped == match_path {
                return candidate;
            }
        }
        String::new()
    }

    fn new_infer_browser_base_path(pathname: &str, match_path: &str) -> String {
        let normalized_path = RouterWidget::normalized_browser_path(pathname);
        let trimmed = normalized_path.trim_start_matches('/');
        let mut candidate = String::new();
        let mut offset = normalized_path.len() - trimmed.len();
        for segment in trimmed.split('/') {
            if segment.is_empty() {
                offset += 1;
                continue;
            }
            candidate.push('/');
            candidate.push_str(segment);
            offset += segment.len();
            if normalized_path.get(..offset) == Some(candidate.as_str()) {
                let stripped = if offset >= normalized_path.len() {
                    std::borrow::Cow::Borrowed("/")
                } else {
                    let stripped = &normalized_path[offset..];
                    if stripped.starts_with("//") {
                        let trimmed = stripped.trim_start_matches('/');
                        if trimmed.is_empty() {
                            std::borrow::Cow::Borrowed("/")
                        } else {
                            let mut normalized = String::with_capacity(trimmed.len() + 1);
                            normalized.push('/');
                            normalized.push_str(trimmed);
                            std::borrow::Cow::Owned(normalized)
                        }
                    } else {
                        std::borrow::Cow::Borrowed(stripped)
                    }
                };
                if stripped.as_ref() == match_path {
                    return candidate;
                }
            }
            offset += 1;
        }
        String::new()
    }

    fn old_strip_browser_base_path(pathname: &str, base_path: &str) -> String {
        fn old_normalized_browser_base_path(base_path: &str) -> String {
            let trimmed = base_path.trim();
            if trimmed.is_empty() || trimmed == "/" {
                return String::new();
            }

            let mut normalized = trimmed.to_string();
            if !normalized.starts_with('/') {
                normalized.insert(0, '/');
            }
            while normalized.len() > 1 && normalized.ends_with('/') {
                normalized.pop();
            }

            if normalized == "/" {
                String::new()
            } else {
                normalized
            }
        }

        fn old_normalized_browser_path(pathname: &str) -> String {
            let trimmed = pathname.trim();
            if trimmed.is_empty() {
                "/".to_string()
            } else if trimmed.starts_with('/') {
                trimmed.to_string()
            } else {
                format!("/{}", trimmed)
            }
        }

        let normalized_path = old_normalized_browser_path(pathname);
        let normalized_base = old_normalized_browser_base_path(base_path);

        if normalized_base.is_empty() {
            return normalized_path;
        }

        if normalized_path == normalized_base || normalized_path == format!("{}/", normalized_base)
        {
            return "/".to_string();
        }

        if let Some(stripped) = normalized_path.strip_prefix(&(normalized_base.clone() + "/")) {
            return format!("/{}", stripped.trim_start_matches('/'));
        }

        normalized_path
    }

    #[test]
    fn normalizes_browser_base_path() {
        assert_eq!(RouterWidget::normalized_browser_base_path(""), "");
        assert_eq!(RouterWidget::normalized_browser_base_path("/"), "");
        assert_eq!(
            RouterWidget::normalized_browser_base_path("makepad-components/"),
            "/makepad-components"
        );
    }

    #[test]
    fn strips_browser_base_path_from_clean_urls() {
        assert_eq!(
            RouterWidget::strip_browser_base_path(
                "/makepad-components/alert",
                "/makepad-components"
            ),
            "/alert"
        );
        assert_eq!(
            RouterWidget::strip_browser_base_path("/makepad-components/", "/makepad-components"),
            "/"
        );
        assert_eq!(
            RouterWidget::strip_browser_base_path(
                "/makepad-components//alert",
                "/makepad-components"
            ),
            "/alert"
        );
        assert_eq!(
            RouterWidget::strip_browser_base_path("/makepad-components///", "/makepad-components"),
            "/"
        );
    }

    #[test]
    fn prefixes_clean_urls_with_base_path() {
        assert_eq!(
            RouterWidget::prefix_clean_browser_base_path("/", "/makepad-components"),
            "/makepad-components/"
        );
        assert_eq!(
            RouterWidget::prefix_clean_browser_base_path(
                "/alert?tab=api#hash",
                "/makepad-components"
            ),
            "/makepad-components/alert?tab=api#hash"
        );
        assert_eq!(
            RouterWidget::prefix_clean_browser_base_path(
                " alert?tab=api#hash ",
                "/makepad-components"
            ),
            "/makepad-components/alert?tab=api#hash"
        );
    }

    #[test]
    fn prefixes_hash_urls_with_base_path() {
        assert_eq!(
            RouterWidget::prefix_hash_browser_base_path("/", "/makepad-components"),
            "/makepad-components/#/"
        );
        assert_eq!(
            RouterWidget::prefix_hash_browser_base_path("/alert", "/makepad-components"),
            "/makepad-components/#/alert"
        );
    }

    #[test]
    #[ignore = "micro-benchmark; run explicitly in release mode for stable numbers"]
    fn prefix_clean_browser_base_path_direct_append_benchmark() {
        fn old_prefix_clean_browser_base_path(route_url: &str, base_path: &str) -> String {
            let normalized_base = RouterWidget::normalized_browser_base_path_cow(base_path);
            if normalized_base.is_empty() {
                return route_url.to_string();
            }

            let parsed = RouterUrl::parse(route_url);
            let mut out = String::with_capacity(
                normalized_base.len()
                    + parsed.path.len()
                    + parsed.query.len()
                    + parsed.hash.len()
                    + 1,
            );
            out.push_str(normalized_base.as_ref());
            if parsed.path == "/" {
                out.push('/');
            } else {
                out.push_str(&parsed.path);
            }
            out.push_str(&parsed.query);
            out.push_str(&parsed.hash);
            out
        }

        const BENCHMARK_ITERATIONS: usize = 200_000;
        const ROUTE_URL: &str = "/examples/router/alert/details?tab=active#job-42";
        const BASE_PATH: &str = "/makepad-components";

        let old_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(old_prefix_clean_browser_base_path(ROUTE_URL, BASE_PATH));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(RouterWidget::prefix_clean_browser_base_path(
                ROUTE_URL, BASE_PATH,
            ));
        }
        let new_elapsed = new_start.elapsed();

        println!(
            "prefix_clean_browser_base_path benchmark: old={old_elapsed:?}, new={new_elapsed:?}"
        );
    }

    #[test]
    fn route_url_from_hash_fragment_normalizes_hash_paths() {
        assert_eq!(RouterWidget::route_url_from_hash_fragment(""), "/");
        assert_eq!(RouterWidget::route_url_from_hash_fragment("#"), "/");
        assert_eq!(
            RouterWidget::route_url_from_hash_fragment("#reports/detail"),
            "/reports/detail"
        );
        assert_eq!(
            RouterWidget::route_url_from_hash_fragment("#/reports/detail"),
            "/reports/detail"
        );
    }

    #[test]
    fn infer_browser_base_path_normalizes_duplicate_slash_suffixes() {
        assert_eq!(
            new_infer_browser_base_path(
                "/makepad-components//examples/router/alert/details",
                "/examples/router/alert/details"
            ),
            "/makepad-components"
        );
    }

    #[test]
    fn infer_browser_base_path_rejects_malformed_prefixes() {
        assert_eq!(
            new_infer_browser_base_path("//makepad-components/alert", "/alert"),
            ""
        );
        assert_eq!(
            new_infer_browser_base_path("/makepad-components//alert/details", "/details"),
            ""
        );
    }

    #[test]
    fn strip_browser_base_path_performance_comparison() {
        // Performance comparison helper: it exercises a real browser-sync hot path without
        // asserting on absolute timings, which would be flaky in CI.
        const BENCHMARK_ITERATIONS: usize = 200_000;
        const PATHNAME: &str = "/makepad-components/examples/router/alert/details";
        const BASE_PATH: &str = "/makepad-components";

        let old_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(old_strip_browser_base_path(PATHNAME, BASE_PATH));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(RouterWidget::strip_browser_base_path(PATHNAME, BASE_PATH));
        }
        let new_elapsed = new_start.elapsed();

        println!("strip_browser_base_path benchmark: old={old_elapsed:?}, new={new_elapsed:?}");
    }

    #[test]
    #[ignore = "micro-benchmark; run explicitly in release mode for stable numbers"]
    fn route_url_from_browser_parts_direct_append_benchmark() {
        fn old_browser_route_url(
            pathname: &str,
            base_path: &str,
            search: &str,
            hash: &str,
        ) -> String {
            let route_path = RouterWidget::strip_browser_base_path(pathname, base_path);
            format!("{}{}{}", route_path, search, hash)
        }

        const BENCHMARK_ITERATIONS: usize = 200_000;
        const PATHNAME: &str = "/makepad-components/examples/router/alert/details";
        const BASE_PATH: &str = "/makepad-components";
        const SEARCH: &str = "?tab=active&region=ca";
        const HASH: &str = "#job-42";

        let old_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(old_browser_route_url(PATHNAME, BASE_PATH, SEARCH, HASH));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(RouterWidget::route_url_from_clean_browser_parts(
                PATHNAME, BASE_PATH, SEARCH, HASH,
            ));
        }
        let new_elapsed = new_start.elapsed();

        println!(
            "route_url_from_browser_parts benchmark: old={old_elapsed:?}, new={new_elapsed:?}"
        );
    }

    #[test]
    #[ignore = "micro-benchmark; run explicitly in release mode for stable numbers"]
    fn prefix_clean_browser_base_path_parts_avoids_reparse_benchmark() {
        fn old_clean_browser_url(path: &str, query: &str, hash: &str, base_path: &str) -> String {
            let route_url = format!("{path}{query}{hash}");
            RouterWidget::prefix_clean_browser_base_path(&route_url, base_path)
        }

        const BENCHMARK_ITERATIONS: usize = 200_000;
        const PATH: &str = "/examples/router/alert/details";
        const QUERY: &str = "?tab=active&region=ca";
        const HASH: &str = "#job-42";
        const BASE_PATH: &str = "/makepad-components";

        let old_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(old_clean_browser_url(PATH, QUERY, HASH, BASE_PATH));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(RouterWidget::prefix_clean_browser_base_path_parts(
                PATH, QUERY, HASH, BASE_PATH,
            ));
        }
        let new_elapsed = new_start.elapsed();

        println!(
            "prefix_clean_browser_base_path_parts benchmark: old={old_elapsed:?}, new={new_elapsed:?}"
        );
    }

    #[test]
    fn infer_browser_base_path_segment_scan_performance_comparison() {
        // Performance comparison helper: base-path inference probes several prefixes while
        // the browser URL is being synchronized, so avoiding per-probe String rebuilds helps.
        const BENCHMARK_ITERATIONS: usize = 200_000;
        const PATHNAME: &str = "/makepad-components/examples/router/alert/details";
        const MATCH_PATH: &str = "/examples/router/alert/details";

        let old_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(old_infer_browser_base_path(PATHNAME, MATCH_PATH));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            black_box(new_infer_browser_base_path(PATHNAME, MATCH_PATH));
        }
        let new_elapsed = new_start.elapsed();

        println!("infer_browser_base_path benchmark: old={old_elapsed:?}, new={new_elapsed:?}");
    }
}
