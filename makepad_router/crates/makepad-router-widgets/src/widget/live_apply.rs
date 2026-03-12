use crate::registry::RouteRegistry;
use crate::route::Route;
use makepad_widgets::*;

use super::route_defs::route_definition_from_template;
use super::RouterWidget;

impl ScriptHook for RouterWidget {
    fn on_before_apply(
        &mut self,
        _vm: &mut ScriptVm,
        apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        if apply.is_reload() {
            self.caches.route_registry_epoch = self.caches.route_registry_epoch.wrapping_add(1);
            self.caches.nested_prefix_cache_epoch = 0;
            self.caches.nested_prefix_cache_path.clear();
            self.caches.nested_prefix_cache_result = None;
            self.caches.url_parse_cache.clear();
            self.caches.child_router_scan_epoch = 0;
            self.caches.child_router_scan_widget_count = 0;
            self.routes.templates.clear();
            self.routes.patterns.clear();
            self.routes.transition_overrides.clear();
            self.routes.transition_duration_overrides.clear();
            self.child_routers.clear();
            self.router.route_registry = RouteRegistry::default();
            self.transition_rt.state = None;
        }
    }

    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        scope: &mut Scope,
        value: ScriptValue,
    ) {
        if !apply.is_eval() {
            if let Some(obj) = value.as_object() {
                vm.vec_with(obj, |vm, vec| {
                    for kv in vec {
                        let Some(route_id) = kv.key.as_id() else {
                            continue;
                        };
                        if !WidgetRef::value_is_newable_widget(vm, kv.value) {
                            continue;
                        }

                        let Some(template_obj) = kv.value.as_object() else {
                            continue;
                        };
                        let template_ref = vm.bx.heap.new_object_ref(template_obj);
                        self.routes.templates.insert(route_id, template_ref);

                        let route_def = route_definition_from_template(vm, template_obj);

                        if let Some(pattern) = route_def.pattern {
                            self.routes.patterns.insert(route_id, pattern.clone());
                            if let Err(err) = self.router.register_route_pattern(&pattern, route_id)
                            {
                                log!("Failed to register route pattern {}: {}", pattern, err);
                            } else {
                                self.caches.route_registry_epoch =
                                    self.caches.route_registry_epoch.wrapping_add(1);
                                self.caches.child_router_scan_widget_count = 0;
                            }
                        }

                        if let Some(transition) = route_def.transition {
                            if transition.0 != 0 {
                                self.routes
                                    .transition_overrides
                                    .insert(route_id, transition);
                            }
                        }

                        if let Some(duration) = route_def.transition_duration {
                            self.routes
                                .transition_duration_overrides
                                .insert(route_id, duration);
                        }

                        if let Some(route_widget) = self.routes.widgets.get_mut(&route_id) {
                            route_widget.script_apply(vm, apply, scope, kv.value);
                        }
                    }
                });
            }
        }

        if apply.is_new() || apply.is_reload() {
            if self.router.current_route().is_none() {
                let initial_route = if self.active_route.0 != 0 {
                    self.active_route
                } else {
                    self.default_route
                };

                if initial_route.0 != 0 {
                    self.router.persist_state = self.persist_state && self.persistence_enabled();
                    self.router.reset(Route::new(initial_route));
                    self.active_route = initial_route;
                }
            }

            vm.with_cx_mut(|cx| {
                // Performance-first: lazily instantiate only the active route.
                if self.active_route.0 != 0 {
                    self.ensure_route_widget(cx, self.active_route);
                }

                self.detect_child_routers(cx);
                self.bootstrap_from_browser_url(cx);
            });
        }
    }
}
