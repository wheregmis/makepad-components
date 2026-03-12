use makepad_router::{
    Route, RouterAction, RouterAsyncDecision, RouterCommand, RouterGuardDecision, RouterNavContext,
    RouterRedirect, RouterRedirectTarget, RouterWidgetWidgetRefExt,
};
use makepad_widgets::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.AdvHomePage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Advanced Router Example" draw_text.text_style.font_size: 28 }
        Label { text: "Focus: guards (sync + async) and stack commands." }
        auth_status_label := Label { text: "Auth: disabled" }

        View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 8
            toggle_auth_btn := Button { text: "Toggle Auth" }
            go_protected_btn := Button { text: "Open Protected (sync guard)" }
            go_async_btn := Button { text: "Open Async Protected" }
            go_stack_btn := Button { text: "Open Stack Demo" }
            go_broken_btn := Button { text: "Broken Path" }
        }
    }

    mod.widgets.ProtectedPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Protected" draw_text.text_style.font_size: 28 }
        Label { text: "Reached only when auth is enabled." }
        protected_home_btn := Button { text: "Back to Home" }
    }

    mod.widgets.AsyncProtectedPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Async Protected" draw_text.text_style.font_size: 28 }
        Label { text: "This route is checked by an async guard." }
        async_home_btn := Button { text: "Back to Home" }
    }

    mod.widgets.StackDemoPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Stack Commands" draw_text.text_style.font_size: 28 }
        Label { text: "SetStack, Pop, PopTo, PopToRoot, ReplaceRoute" }

        View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 8
            set_stack_btn := Button { text: "SetStack: home > protected > stack_demo" }
            pop_btn := Button { text: "Pop" }
            pop_to_home_btn := Button { text: "PopTo(home)" }
            pop_to_root_btn := Button { text: "PopToRoot" }
            replace_home_btn := Button { text: "Replace -> home" }
        }

        stack_home_btn := Button { text: "Back to Home" }
    }

    mod.widgets.BlockedPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Blocked" draw_text.text_style.font_size: 28 }
        Label { text: "Guard redirected here because auth is disabled." }
        blocked_home_btn := Button { text: "Back to Home" }
    }

    mod.widgets.AdvNotFoundPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "404" draw_text.text_style.font_size: 42 }
        Label { text: "No route matched." }
        adv_not_found_home_btn := Button { text: "Back to Home" }
    }

    startup() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    spacing: 12
                    padding: 20

                    nav_bar := View {
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8

                        home_btn := Button { text: "Home" }
                        protected_btn := Button { text: "Protected" }
                        stack_btn := Button { text: "Stack" }

                        filler := View { width: Fill height: Fit }
                        status_label := Label { text: "" }
                        back_btn := Button { text: "Back" }
                    }

                    router := mod.widgets.RouterWidget {
                        width: Fill
                        height: Fill
                        default_route: @home
                        not_found_route: @not_found
                        cap_guards_sync: true
                        cap_guards_async: true

                        home := mod.widgets.RouterRoute {
                            route_pattern: "/"
                            mod.widgets.AdvHomePage {}
                        }
                        protected := mod.widgets.RouterRoute {
                            route_pattern: "/protected"
                            mod.widgets.ProtectedPage {}
                        }
                        async_protected := mod.widgets.RouterRoute {
                            route_pattern: "/async-protected"
                            mod.widgets.AsyncProtectedPage {}
                        }
                        stack_demo := mod.widgets.RouterRoute {
                            route_pattern: "/stack"
                            mod.widgets.StackDemoPage {}
                        }
                        blocked := mod.widgets.RouterRoute {
                            route_pattern: "/blocked"
                            mod.widgets.BlockedPage {}
                        }
                        not_found := mod.widgets.RouterRoute {
                            mod.widgets.AdvNotFoundPage {}
                        }
                    }
                }
            }
        }
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        makepad_widgets::script_mod(vm);
        makepad_router::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }

    fn install_guards_if_needed(&mut self, router: &makepad_router::RouterWidgetRef) {
        if self.guards_installed {
            return;
        }

        let auth_sync = self.auth_enabled.clone();
        let _ = router.add_route_guard(move |_cx: &mut Cx, nav: &RouterNavContext| {
            let is_protected = nav.to.as_ref().map(|r| r.id) == Some(live_id!(protected));
            if is_protected && !auth_sync.load(Ordering::SeqCst) {
                return RouterGuardDecision::Redirect(RouterRedirect {
                    target: RouterRedirectTarget::Route(live_id!(blocked)),
                    replace: true,
                });
            }
            RouterGuardDecision::Allow
        });

        let auth_async = self.auth_enabled.clone();
        let _ = router.add_route_guard_async(move |cx: &mut Cx, nav: &RouterNavContext| {
            let is_async_target = nav.to.as_ref().map(|r| r.id) == Some(live_id!(async_protected));
            if !is_async_target {
                return RouterAsyncDecision::Immediate(RouterGuardDecision::Allow);
            }

            let rx: ToUIReceiver<RouterGuardDecision> = ToUIReceiver::default();
            let tx = rx.sender();
            let auth = auth_async.clone();
            cx.spawn_thread(move || {
                std::thread::sleep(Duration::from_millis(120));
                let decision = if auth.load(Ordering::SeqCst) {
                    RouterGuardDecision::Allow
                } else {
                    RouterGuardDecision::Redirect(RouterRedirect {
                        target: RouterRedirectTarget::Route(live_id!(blocked)),
                        replace: true,
                    })
                };
                let _ = tx.send(decision);
            });

            RouterAsyncDecision::Pending(rx)
        });

        self.guards_installed = true;
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    auth_enabled: Arc<AtomicBool>,
    #[rust]
    guards_installed: bool,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(cx, ids!(router));
        self.install_guards_if_needed(&router);

        for action in actions.filter_widget_actions(router.widget_uid()) {
            if let Some(RouterAction::RouteChanged { from, to }) = action.action.downcast_ref() {
                log!("Route changed: {:?} -> {:?}", from, to);
            }
        }

        if self.ui.button(cx, ids!(nav_bar.home_btn)).clicked(actions) {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToRoute {
                    route_id: live_id!(home),
                    transition: None,
                },
            );
        }
        if self
            .ui
            .button(cx, ids!(nav_bar.protected_btn))
            .clicked(actions)
        {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToRoute {
                    route_id: live_id!(protected),
                    transition: None,
                },
            );
        }
        if self.ui.button(cx, ids!(nav_bar.stack_btn)).clicked(actions) {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToRoute {
                    route_id: live_id!(stack_demo),
                    transition: None,
                },
            );
        }
        if self.ui.button(cx, ids!(nav_bar.back_btn)).clicked(actions) {
            let _ = router.dispatch(cx, RouterCommand::Back { transition: None });
        }

        match router.current_route_id() {
            Some(id) if id == live_id!(home) => {
                let Some((toggle_auth, to_protected, to_async, to_stack, to_broken)) = router
                    .with_active_route_widget(|w| {
                        (
                            w.button(cx, &[live_id!(toggle_auth_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(go_protected_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(go_async_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(go_stack_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(go_broken_btn)]).clicked(actions),
                        )
                    })
                else {
                    return;
                };

                if toggle_auth {
                    let next = !self.auth_enabled.load(Ordering::SeqCst);
                    self.auth_enabled.store(next, Ordering::SeqCst);
                }

                let status_text = if self.auth_enabled.load(Ordering::SeqCst) {
                    "Auth: enabled"
                } else {
                    "Auth: disabled"
                };
                router.with_active_route_widget(|w| {
                    w.label(cx, &[live_id!(auth_status_label)])
                        .set_text(cx, status_text);
                });

                if to_protected {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(protected),
                            transition: None,
                        },
                    );
                }
                if to_async {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(async_protected),
                            transition: None,
                        },
                    );
                }
                if to_stack {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(stack_demo),
                            transition: None,
                        },
                    );
                }
                if to_broken {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToPath {
                            path: "/this/path/does/not/exist".to_string(),
                        },
                    );
                }
            }
            Some(id) if id == live_id!(protected) => {
                let Some(to_home) = router.with_active_route_widget(|w| {
                    w.button(cx, &[live_id!(protected_home_btn)])
                        .clicked(actions)
                }) else {
                    return;
                };
                if to_home {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(home),
                            transition: None,
                        },
                    );
                }
            }
            Some(id) if id == live_id!(async_protected) => {
                let Some(to_home) = router.with_active_route_widget(|w| {
                    w.button(cx, &[live_id!(async_home_btn)]).clicked(actions)
                }) else {
                    return;
                };
                if to_home {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(home),
                            transition: None,
                        },
                    );
                }
            }
            Some(id) if id == live_id!(stack_demo) => {
                let Some((set_stack, pop, pop_to_home, pop_to_root, replace_home, to_home)) =
                    router.with_active_route_widget(|w| {
                        (
                            w.button(cx, &[live_id!(set_stack_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(pop_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(pop_to_home_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(pop_to_root_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(replace_home_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(stack_home_btn)]).clicked(actions),
                        )
                    })
                else {
                    return;
                };

                if set_stack {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::SetStack {
                            stack: vec![
                                Route::new(live_id!(home)),
                                Route::new(live_id!(protected)),
                                Route::new(live_id!(stack_demo)),
                            ],
                        },
                    );
                }
                if pop {
                    let _ = router.dispatch(cx, RouterCommand::Pop);
                }
                if pop_to_home {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::PopTo {
                            route_id: live_id!(home),
                        },
                    );
                }
                if pop_to_root {
                    let _ = router.dispatch(cx, RouterCommand::PopToRoot);
                }
                if replace_home {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::ReplaceRoute {
                            route_id: live_id!(home),
                            transition: None,
                        },
                    );
                }
                if to_home {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(home),
                            transition: None,
                        },
                    );
                }
            }
            Some(id) if id == live_id!(blocked) => {
                let Some(to_home) = router.with_active_route_widget(|w| {
                    w.button(cx, &[live_id!(blocked_home_btn)]).clicked(actions)
                }) else {
                    return;
                };
                if to_home {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(home),
                            transition: None,
                        },
                    );
                }
            }
            Some(id) if id == live_id!(not_found) => {
                let Some(to_home) = router.with_active_route_widget(|w| {
                    w.button(cx, &[live_id!(adv_not_found_home_btn)])
                        .clicked(actions)
                }) else {
                    return;
                };
                if to_home {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(home),
                            transition: None,
                        },
                    );
                }
            }
            _ => {}
        }

        self.ui
            .button(cx, ids!(nav_bar.back_btn))
            .set_enabled(cx, router.can_go_back());

        let status = format!(
            "{}  {}",
            router
                .current_route_id()
                .map(|id: LiveId| id.to_string())
                .unwrap_or_else(|| "(none)".to_string()),
            router
                .current_url()
                .unwrap_or_else(|| "(no url)".to_string())
        );
        self.ui
            .label(cx, ids!(nav_bar.status_label))
            .set_text(cx, &status);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
