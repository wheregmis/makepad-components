//! Router widget implementation and subsystem wiring.

use crate::{
    guards::{
        RouterAsyncDecision, RouterBeforeLeaveDecision, RouterGuardDecision, RouterNavContext,
    },
    route::Route,
    router::{Router, RouterAction},
    state::RouterState,
};
use makepad_draw::draw_list_2d::DrawListExt;
use makepad_widgets::*;
use std::collections::HashSet;

mod actions;
mod api;
mod browser_sync;
mod callbacks;
mod commands;
mod engine;
mod features;
mod fields;
mod guard_flow;
mod guards;
mod inspector;
mod live_apply;
mod nested;
mod path_nav;
mod persistence;
mod route_defs;
mod route_render;
mod route_widgets;
mod transitions;
mod url_cache;
mod url_state;

pub use commands::{
    RouterBlockReason, RouterCapabilities, RouterCommand, RouterConfig, RouterDispatchResult,
};
use fields::{
    PointerCleanup, RouterCaches, RouterCallbacks, RouterDrawLists, RouterGuards, RouterRouteMaps,
    TransitionRuntime,
};
use guard_flow::PendingNavigation;
use transitions::{RouterActionKind, RouterTransitionDirection, RouterTransitionState};
pub use transitions::{RouterTransitionPreset, RouterTransitionSpec};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    let BrowserUrlMode = set_type_default() do #(BrowserUrlMode::script_api(vm))
    mod.widgets.BrowserUrlMode = BrowserUrlMode

    set_type_default() do #(DrawInspectorRect::script_shader(vm)){
        ..mod.draw.DrawQuad
    }

    mod.widgets.DrawInspectorRect = {
        pixel: fn() {
            return self.color
        }
    }

    mod.widgets.RouterWidgetBase = #(RouterWidget::register_widget(vm)) {
        flow: Overlay
        clip_x: true
        clip_y: true

        // Phase 3: transitions/animations (default off).
        push_transition: @none
        pop_transition: @none
        replace_transition: @none
        transition_duration: 0.25
        debug_inspector: false
        inspector_bg +: {draw_depth: 10.0, color: #x00000012}
        inspector_text +: {
            text_style: theme.font_regular{font_size: 9}
            color: #xFFFFFFFF
            draw_depth: 11.0
        }

        cap_guards_sync: false
        cap_guards_async: false
        cap_transitions: false
        cap_nested: false
        cap_persistence: false
        sync_browser_url: false
        browser_url_mode: BrowserUrlMode.CleanPath
        browser_base_path: ""
    }

    mod.widgets.RouterWidget = mod.widgets.RouterWidgetBase {
        width: Fill
        height: Fill
    }

    mod.widgets.RouterRouteBase = #(RouterRoute::register_widget(vm)) {
        width: Fill
        height: Fill
    }

    mod.widgets.RouterRoute = mod.widgets.RouterRouteBase {}
}

#[derive(Script, ScriptHook, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserUrlMode {
    #[pick]
    CleanPath,
    HashPath,
}

impl Default for BrowserUrlMode {
    fn default() -> Self {
        Self::CleanPath
    }
}

#[derive(Clone, Debug)]
enum RouterNavRequest {
    Navigate {
        route_id: LiveId,
    },
    NavigateWithTransition {
        route_id: LiveId,
        transition: RouterTransitionSpec,
    },
    Replace {
        route_id: LiveId,
    },
    ReplaceWithTransition {
        route_id: LiveId,
        transition: RouterTransitionSpec,
    },
    NavigateByPath {
        path: String,
    },
    ReplaceByPath {
        path: String,
        clear_extras: bool,
    },
    Back {
        transition: Option<RouterTransitionSpec>,
    },
    Forward {
        transition: Option<RouterTransitionSpec>,
    },
    Reset {
        route: Route,
    },
    SetStack {
        stack: Vec<Route>,
    },
    Pop,
    PopTo {
        route_id: LiveId,
    },
    PopToRoot,
}

#[derive(Clone, Debug)]
pub(super) enum ResolvedPathKind {
    FullMatch,
    NestedPrefix { tail: String },
    NotFoundFallback,
}

#[derive(Clone, Debug)]
pub(super) struct ResolvedPathIntent {
    pub path: String,
    pub route: Route,
    pub kind: ResolvedPathKind,
    pub clear_extras: bool,
    pub replace: bool,
}

/// Route entry wrapper that carries route metadata plus a page widget child.
#[derive(Script, ScriptHook, Widget)]
pub struct RouterRoute {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    route_pattern: String,
    #[live]
    route_transition: LiveId,
    #[live(0.0)]
    route_transition_duration: f64,
}

impl Widget for RouterRoute {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

/// Router widget for managing navigation between pages
#[derive(Script, WidgetRef, WidgetSet, WidgetRegister)]
pub struct RouterWidget {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[rust]
    area: Area,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    active_route: LiveId,
    #[live]
    default_route: LiveId,
    #[live]
    not_found_route: LiveId,
    #[live(false)]
    persist_state: bool,
    /// Default transition used for push/navigate.
    #[live]
    push_transition: LiveId,
    /// Default transition used for back/pop.
    #[live]
    pop_transition: LiveId,
    /// Default transition used for replace/reset/set_stack.
    #[live]
    replace_transition: LiveId,
    /// Default transition duration (seconds).
    #[live(0.25)]
    transition_duration: f64,
    /// Shows a small debug overlay with current route/stack/params (dev tool).
    #[live(false)]
    debug_inspector: bool,
    #[live(false)]
    cap_guards_sync: bool,
    #[live(false)]
    cap_guards_async: bool,
    #[live(false)]
    cap_transitions: bool,
    #[live(false)]
    cap_nested: bool,
    #[live(false)]
    cap_persistence: bool,
    #[live(false)]
    sync_browser_url: bool,
    #[live]
    browser_url_mode: BrowserUrlMode,
    #[live]
    browser_base_path: ArcStringMut,
    #[rust]
    router: Router,
    #[rust]
    draw_state: DrawStateWrap<Walk>,
    #[rust]
    child_routers: ComponentMap<LiveId, RouterWidgetRef>,
    #[rust]
    child_router_scan_misses: HashSet<LiveId>,
    #[rust]
    routes: RouterRouteMaps,
    #[rust]
    callbacks: RouterCallbacks,
    #[rust]
    guards: RouterGuards,
    #[rust]
    pending_navigation: Option<PendingNavigation>,
    #[rust]
    guard_bypass: bool,
    #[rust]
    pending_actions: Vec<RouterAction>,
    #[rust]
    browser_sync_inbound: bool,
    #[rust]
    browser_sync_suppress_once: bool,
    #[rust]
    inferred_browser_base_path: String,
    #[rust]
    url_path_override: Option<String>,
    #[rust]
    caches: RouterCaches,
    #[rust]
    pointer_cleanup: PointerCleanup,
    #[new]
    draw_lists: RouterDrawLists,
    #[live]
    inspector_bg: DrawInspectorRect,
    #[live]
    inspector_text: DrawText,
    #[rust]
    transition_rt: TransitionRuntime,
    #[rust]
    last_blocked_reason: Option<RouterBlockReason>,
}

impl RouterWidget {
    fn is_pointer_cleanup_event(event: &Event) -> bool {
        matches!(
            event,
            Event::MouseDown(_)
                | Event::MouseMove(_)
                | Event::MouseUp(_)
                | Event::MouseLeave(_)
                | Event::TouchUpdate(_)
                | Event::LongPress(_)
                | Event::Scroll(_)
                | Event::Drag(_)
                | Event::Drop(_)
                | Event::DragEnd
        )
    }

    /// Register a child router
    pub fn register_child_router(&mut self, route_id: LiveId, child: RouterWidgetRef) {
        if !self.nested_enabled() {
            self.last_blocked_reason = Some(RouterBlockReason::CapabilityDisabled);
            return;
        }
        self.child_routers.insert(route_id, child);
    }

    /// Register a route pattern
    pub fn register_route_pattern(
        &mut self,
        pattern: &str,
        route_id: LiveId,
    ) -> Result<(), String> {
        self.router.register_route_pattern(pattern, route_id)?;
        self.routes.patterns.insert(route_id, pattern.to_string());
        self.caches.route_registry_epoch = self.caches.route_registry_epoch.wrapping_add(1);
        self.caches.nested_prefix_cache_epoch = 0;
        self.caches.nested_prefix_cache_path.clear();
        self.caches.nested_prefix_cache_result = None;
        Ok(())
    }

    fn begin_overlay_layout(&mut self, cx: &mut Cx2d, walk: Walk) {
        let active_route = self.active_route;
        if let Some(route_widget) = self.routes.widgets.get_mut(&active_route) {
            if self
                .draw_state
                .begin_with(cx, &(), |cx, _| route_widget.walk(cx))
            {
                let mut layout = self.layout;
                layout.flow = Flow::Overlay;
                layout.clip_x = true;
                layout.clip_y = true;
                cx.begin_turtle(walk, layout);
            }
        } else {
            let mut layout = self.layout;
            layout.flow = Flow::Overlay;
            layout.clip_x = true;
            layout.clip_y = true;
            cx.begin_turtle(walk, layout);
        }
    }

    fn draw_active_routes(&mut self, cx: &mut Cx2d, scope: &mut Scope) -> DrawStep {
        let active_route = self.active_route;
        if self.transition_rt.state.is_some() {
            let rect = cx.turtle().inner_rect();
            self.draw_routes_with_transition(cx, scope, rect);
            self.draw_debug_inspector(cx, rect);
        } else if let Some(route_widget) = self.routes.widgets.get_mut(&active_route) {
            if let Some(route_walk) = self.draw_state.get() {
                route_widget.draw_walk(cx, scope, route_walk)?;
            }
            let rect = cx.turtle().inner_rect();
            self.draw_debug_inspector(cx, rect);
        } else {
            let rect = cx.turtle().inner_rect();
            self.draw_debug_inspector(cx, rect);
        }
        DrawStep::done()
    }

    fn handle_active_route_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(active) = self.routes.widgets.get_mut(&self.active_route) {
            let actions = cx.capture_actions(|cx| active.handle_event(cx, event, scope));
            self.handle_descendant_router_actions(cx, &actions);
            cx.extend_actions(actions);
        }
    }

    fn redraw_transition_frame(&mut self, cx: &mut Cx) {
        // Transition frames only update draw-list transforms/progress. Keep route draw-lists
        // clean so draw-list reuse remains effective across animation ticks.
        self.draw_lists.inspector.redraw(cx);
        self.area.redraw(cx);
    }
}

impl WidgetNode for RouterWidget {
    fn widget_uid(&self) -> WidgetUid {
        self.uid
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn area(&self) -> Area {
        self.area
    }

    fn children(&self, visit: &mut dyn FnMut(LiveId, WidgetRef)) {
        for (route_id, widget) in self.routes.widgets.iter() {
            visit(*route_id, widget.clone());
        }
        for (route_id, child_router) in self.child_routers.iter() {
            visit(*route_id, child_router.0.clone());
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_lists.from.redraw(cx);
        self.draw_lists.to.redraw(cx);
        self.draw_lists.inspector.redraw(cx);
        self.area.redraw(cx);
    }
}

impl Widget for RouterWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(ne) = self.transition_rt.next_frame.is_event(event) {
            self.update_transition(cx, ne.time);
        }
        self.flush_router_actions(cx, scope);
        if matches!(event, Event::Signal) {
            self.handle_browser_location_signal(cx);
        }

        if event.requires_visibility() {
            self.handle_active_route_event(cx, event, scope);

            // Keep a short pointer-only grace window for the previous route so pressed/hover state
            // can settle after route swaps without exposing hidden routes to keyboard/focus traffic.
            if self.pointer_cleanup.budget > 0 && Self::is_pointer_cleanup_event(event) {
                if let Some(route_id) = self.pointer_cleanup.route {
                    if route_id != self.active_route {
                        if let Some(prev) = self.routes.widgets.get_mut(&route_id) {
                            prev.handle_event(cx, event, scope);
                        }
                    }
                }
            }
        } else {
            // Keep hidden pages cold for non-visibility events; dispatch only to the active page.
            self.handle_active_route_event(cx, event, scope);
        }

        if self.pointer_cleanup.budget > 0 {
            self.pointer_cleanup.budget = self.pointer_cleanup.budget.saturating_sub(1);
            if self.pointer_cleanup.budget == 0 {
                self.pointer_cleanup.budget = 0;
                self.pointer_cleanup.route = None;
            }
        }

        self.poll_pending_navigation(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.begin_overlay_layout(cx, walk);
        self.draw_active_routes(cx, scope)?;
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

#[derive(Script, ScriptHook)]
#[repr(C)]
pub struct DrawInspectorRect {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    color: Vec4f,
}

impl RouterWidgetRef {
    pub fn with_active_route_widget<R>(&self, f: impl FnOnce(&WidgetRef) -> R) -> Option<R> {
        let inner = self.borrow()?;
        let active_route = inner.active_route;
        let route_widget = inner.routes.widgets.get(&active_route)?;
        Some(f(route_widget))
    }

    pub fn can_go_back(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.can_go_back()
        } else {
            false
        }
    }

    pub fn can_go_forward(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.can_go_forward()
        } else {
            false
        }
    }

    pub fn depth(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.depth()
        } else {
            0
        }
    }

    pub fn current_route_id(&self) -> Option<LiveId> {
        if let Some(inner) = self.borrow() {
            inner.current_route_id()
        } else {
            None
        }
    }

    pub fn current_url(&self) -> Option<String> {
        let inner = self.borrow()?;
        Some(inner.current_url())
    }

    pub fn current_route(&self) -> Option<Route> {
        if let Some(inner) = self.borrow() {
            inner.router.current_route().cloned()
        } else {
            None
        }
    }

    pub fn get_query_string(&self, key: &str) -> Option<String> {
        self.current_route()?.query_get_string(key)
    }

    pub fn get_query_i64(&self, key: &str) -> Option<i64> {
        self.current_route()?.query_get_i64(key)
    }

    pub fn get_query_u64(&self, key: &str) -> Option<u64> {
        self.current_route()?.query_get_u64(key)
    }

    pub fn get_query_bool(&self, key: &str) -> Option<bool> {
        self.current_route()?.query_get_bool(key)
    }

    pub fn get_query_f64(&self, key: &str) -> Option<f64> {
        self.current_route()?.query_get_f64(key)
    }

    pub fn get_state(&self) -> Option<RouterState> {
        Some(self.borrow()?.get_state())
    }

    pub fn set_state(&self, cx: &mut Cx, state: RouterState) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_state(cx, state)
        } else {
            false
        }
    }

    /// Get a route parameter as a string
    /// Returns None if the parameter doesn't exist or the route is not active
    pub fn get_param_string(&self, param_name: &str) -> Option<String> {
        if let Some(route) = self.current_route() {
            if let Some(param_value) = route.get_param(LiveId::from_str(param_name)) {
                return param_value.as_string(|id_str| id_str.map(|s| s.to_string()));
            }
        }
        None
    }

    /// Bind a route parameter to a label widget
    /// The formatter function is called with the parameter value to generate the label text
    pub fn bind_param_to_label<F>(
        &self,
        cx: &mut Cx,
        param_name: &str,
        label_id: LiveId,
        formatter: F,
    ) -> bool
    where
        F: Fn(&str) -> String,
    {
        if let Some(param_value) = self.get_param_string(param_name) {
            let formatted_text = formatter(&param_value);
            self.with_active_route_widget(|route_widget| {
                let label = route_widget.widget(cx, &[label_id]);
                if label.is_empty() {
                    return false;
                }
                label.set_text(cx, &formatted_text);
                true
            })
            .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn register_child_router(&self, route_id: LiveId, child: RouterWidgetRef) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.register_child_router(route_id, child);
        }
    }

    /// Register a route change callback
    pub fn on_route_change<F>(&self, callback: F)
    where
        F: Fn(&mut Cx, Option<Route>, Route) + Send + Sync + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.on_route_change(callback);
        }
    }

    pub fn add_route_guard<F>(&self, guard: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterGuardDecision + Send + Sync + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_route_guard(guard)
        } else {
            Err(RouterBlockReason::CapabilityDisabled)
        }
    }

    pub fn add_route_guard_async<F>(&self, guard: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterGuardDecision>
            + Send
            + Sync
            + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_route_guard_async(guard)
        } else {
            Err(RouterBlockReason::CapabilityDisabled)
        }
    }

    pub fn add_before_leave_hook<F>(&self, hook: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterBeforeLeaveDecision + Send + Sync + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_before_leave_hook(hook)
        } else {
            Err(RouterBlockReason::CapabilityDisabled)
        }
    }

    pub fn add_before_leave_hook_async<F>(&self, hook: F) -> Result<(), RouterBlockReason>
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterBeforeLeaveDecision>
            + Send
            + Sync
            + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_before_leave_hook_async(hook)
        } else {
            Err(RouterBlockReason::CapabilityDisabled)
        }
    }

    pub fn register_route_pattern(&self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.register_route_pattern(pattern, route_id)
        } else {
            Err("Cannot borrow router widget".to_string())
        }
    }
}
