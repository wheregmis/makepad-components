# Makepad Router

A standalone routing package for Makepad applications, providing navigation, deep links, and page management for both widget-based and headless use.

## Features

- **LiveId routes + script DSL integration** (define routes directly in `script_mod!`)
- **Path patterns** with params and wildcards (`/user/:id`, `/admin/*`, `/docs/**`)
- **Navigation history** with back/forward semantics
- **Query + hash support** per history entry
- **Nested routers** for sub-navigation
- **Transitions** (opt-in)
- **Guards + before-leave hooks** (sync + async)
- **State persistence** via SerRon/DeRon
- **Debug inspector overlay** for dev diagnostics

## RouterWidget Toggles

These script properties control optional subsystems.
Advanced subsystems are opt-in and disabled by default.

- `persist_state` (bool): serialize/restore router state via `RouterState`.
- `debug_inspector` (bool): show a small overlay with route/stack/params.
- `push_transition`, `pop_transition`, `replace_transition`, `transition_duration`: configure route transitions.
- `cap_guards_sync`, `cap_guards_async`: enable sync/async guards.
- `cap_transitions`: enable transition runtime.
- `cap_nested`: enable nested-router behavior.
- `cap_persistence`: enable `get_state` / `set_state`.
- `sync_browser_url` (bool): mirror top-level route changes into the browser URL on web.
- `browser_url_mode` (`BrowserUrlMode`): controls how synced browser URLs are written/read on web. Default is `BrowserUrlMode.CleanPath`.

Route metadata is configured on `RouterRoute` entries (`route_pattern`, `route_transition`, `route_transition_duration`), not directly on page widgets.

When `sync_browser_url: true`, Makepad Router uses clean path URLs by default on web, so routes like `/scroll-area` and `/detail/42` appear directly in the address bar. Hosts must serve the app shell for unknown extensionless paths to support direct loads and refreshes.

## Command API

Use `dispatch` as the single mutation entrypoint:

```rust
let result = router.dispatch(cx, RouterCommand::GoToRoute {
    route_id: live_id!(settings),
    transition: None,
});

if !result.changed {
    log!("blocked: {:?}", result.blocked_reason);
}
```

## Quick Start

```rust
use makepad_widgets::*;
use makepad_router::{RouterAction, RouterCommand, RouterWidgetWidgetRefExt};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                router := RouterWidget{
                    width: Fill
                    height: Fill
                    default_route: @home
                    not_found_route: @not_found
                    cap_nested: true
                    cap_transitions: true

                    home := RouterRoute{
                        route_pattern: "/"
                        HomePage{}
                    }
                    settings := RouterRoute{
                        route_pattern: "/settings/*"
                        SettingsPage{}
                    }
                    detail := RouterRoute{
                        route_pattern: "/detail/:id"
                        DetailPage{}
                    }
                    not_found := RouterRoute{
                        NotFoundPage{}
                    }
                }
            }
        }
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(cx, ids!(router));

        if self.ui.button(cx, ids!(home_btn)).clicked(actions) {
            let _ = router.dispatch(cx, RouterCommand::GoToRoute {
                route_id: live_id!(home),
                transition: None,
            });
        }

        if self.ui.button(cx, ids!(back_btn)).clicked(actions) {
            let _ = router.dispatch(cx, RouterCommand::Back { transition: None });
        }

        for action in actions.filter_widget_actions(router.widget_uid()) {
            if let Some(RouterAction::RouteChanged { from, to }) = action.action.downcast_ref() {
                log!("Route changed: {:?} -> {:?}", from, to);
            }
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_router::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

## Path Parameters + Query

```rust
use makepad_router::prelude::*;

// Navigate by path (matches registered patterns)
router.dispatch(cx, RouterCommand::GoToPath {
    path: "/detail/42?tab=posts".to_string(),
});

// Read params/query from current route
if let Some(route) = router.current_route() {
    if let Some(id) = route.get_param_string(live_id!(id)) {
        log!("id={}", id);
    }
    if let Some(tab) = route.query_get("tab") {
        log!("tab={}", tab);
    }
}
```

## Guards + Before-Leave Hooks

```rust
use makepad_router::{RouterGuardDecision, RouterRedirect, RouterRedirectTarget, RouterBeforeLeaveDecision};

let _ = router.add_route_guard(|_cx, nav| {
    if nav.to.as_ref().map(|r| r.id) == Some(live_id!(admin)) {
        return RouterGuardDecision::Redirect(RouterRedirect {
            target: RouterRedirectTarget::Route(live_id!(login)),
            replace: true,
        });
    }
    RouterGuardDecision::Allow
});

let _ = router.add_before_leave_hook(|_cx, nav| {
    if nav.from.as_ref().map(|r| r.id) == Some(live_id!(settings)) {
        return RouterBeforeLeaveDecision::Block;
    }
    RouterBeforeLeaveDecision::Allow
});
```

## Example Apps

Start here (simple, dead-end-free):

```
cargo run -p router_example
```

Simple example routes:
- `/` (home)
- `/settings/*` (small nested router)
- `/detail/:id` (param route)
- `not_found` (catch-all)

Next step (advanced guards + stack operations):

```
cargo run -p router_advanced_example
```

Advanced example focus:
- sync/async guards (allow/block/redirect)
- stack commands (`SetStack`, `Pop`, `PopTo`, `PopToRoot`, `ReplaceRoute`)

## Architecture

Key components in `src/`:

1. **route.rs** – Route + pattern parsing, params, and query helpers
2. **navigation.rs** – History stack and back/forward logic
3. **router.rs** – Core router state + pattern registry
4. **widget.rs** – RouterWidget implementation and DSL integration
5. **guards.rs** – Guard/before-leave types
6. **url.rs** – URL parsing + query helpers
7. **state.rs** – Serializable router state

## License

MIT OR Apache-2.0
