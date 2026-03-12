use crate::router::RouterAction;
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

    fn browser_url_from_os(&self, cx: &Cx) -> Option<String> {
        let OsType::Web(params) = cx.os_type() else {
            return None;
        };

        let mut pathname = params.pathname.trim().to_string();
        if pathname.is_empty() {
            pathname = "/".to_string();
        } else if !pathname.starts_with('/') {
            pathname.insert(0, '/');
        }

        match self.browser_url_mode {
            BrowserUrlMode::CleanPath => Some(format!("{}{}{}", pathname, params.search, params.hash)),
            BrowserUrlMode::HashPath => {
                let hash = params.hash.trim();
                if hash.is_empty() || hash == "#" {
                    Some("/".to_string())
                } else {
                    let route = hash.trim_start_matches('#');
                    if route.starts_with('/') {
                        Some(route.to_string())
                    } else {
                        Some(format!("/{}", route))
                    }
                }
            }
        }
    }

    fn route_url_to_browser_url(&self, route_url: &str) -> String {
        match self.browser_url_mode {
            BrowserUrlMode::CleanPath => route_url.to_string(),
            BrowserUrlMode::HashPath => {
                let route_url = route_url.trim();
                let route_url = if route_url.is_empty() { "/" } else { route_url };
                format!("/#{}", route_url)
            }
        }
    }

    fn browser_sync_url(&self) -> String {
        self.route_url_to_browser_url(&self.current_url())
    }

    fn preview_back_browser_url(&self) -> Option<String> {
        self.preview_back_url()
            .map(|route_url| self.route_url_to_browser_url(&route_url))
    }

    fn preview_forward_browser_url(&self) -> Option<String> {
        self.preview_forward_url()
            .map(|route_url| self.route_url_to_browser_url(&route_url))
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

        let Some(browser_url) = self.browser_url_from_os(cx) else {
            return;
        };

        if browser_url == self.current_url() {
            return;
        }

        self.browser_sync_inbound = true;
        self.browser_sync_suppress_once = true;
        let changed = self.replace_by_path_internal(cx, &browser_url, true);
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

        let Some(browser_url) = self.browser_url_from_os(cx) else {
            return;
        };

        if browser_url == self.current_url() {
            return;
        }

        self.browser_sync_inbound = true;
        self.browser_sync_suppress_once = true;
        let handled = if self.preview_back_browser_url().as_deref() == Some(browser_url.as_str()) {
            self.back(cx)
        } else if self.preview_forward_browser_url().as_deref() == Some(browser_url.as_str()) {
            self.forward(cx)
        } else {
            self.replace_by_path_internal(cx, &browser_url, true)
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
