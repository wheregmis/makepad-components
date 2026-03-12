use makepad_router::{RouterAction, RouterCommand, RouterWidgetWidgetRefExt};
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HomePage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Home" draw_text.text_style.font_size: 28 }
        Label { text: "Start here. Use buttons to navigate to detail, settings, or a broken path." }

        View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 8
            open_detail_btn := Button { text: "Open Detail (/detail/42)" }
            open_settings_btn := Button { text: "Open Settings" }
            open_broken_btn := Button { text: "Broken Path" }
        }
    }

    mod.widgets.DetailPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Detail" draw_text.text_style.font_size: 28 }
        detail_id_label := Label { text: "Detail ID: (dynamic)" }

        View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 8
            detail_to_1_btn := Button { text: "Go /detail/1" }
            detail_to_42_btn := Button { text: "Go /detail/42" }
            detail_home_btn := Button { text: "Back to Home" }
        }
    }

    mod.widgets.SettingsOverviewPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 8
        padding: 12

        Label { text: "Overview" draw_text.text_style.font_size: 18 }
        Label { text: "Nested route: /settings/overview" }
    }

    mod.widgets.SettingsProfilePage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 8
        padding: 12

        Label { text: "Profile" draw_text.text_style.font_size: 18 }
        Label { text: "Nested route: /settings/profile" }
    }

    mod.widgets.SettingsPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "Settings" draw_text.text_style.font_size: 28 }
        Label { text: "This page owns a small nested router." }

        View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 8
            settings_overview_btn := Button { text: "Overview" }
            settings_profile_btn := Button { text: "Profile" }
            settings_home_btn := Button { text: "Back to Home" }
        }

        settings_router := mod.widgets.RouterWidget {
            width: Fill
            height: 220
            default_route: @settings_overview
            not_found_route: @settings_overview

            settings_overview := mod.widgets.RouterRoute {
                route_pattern: "/overview"
                mod.widgets.SettingsOverviewPage {}
            }
            settings_profile := mod.widgets.RouterRoute {
                route_pattern: "/profile"
                mod.widgets.SettingsProfilePage {}
            }
        }
    }

    mod.widgets.NotFoundPage = View {
        width: Fill
        height: Fill
        flow: Down
        spacing: 12
        padding: 20

        Label { text: "404" draw_text.text_style.font_size: 42 }
        Label { text: "That route does not exist." }
        not_found_home_btn := Button { text: "Back to Home" }
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
                        settings_btn := Button { text: "Settings" }
                        detail_btn := Button { text: "Detail" }

                        filler := View { width: Fill height: Fit }
                        status_label := Label { text: "" }
                        back_btn := Button { text: "Back" }
                    }

                    router := mod.widgets.RouterWidget {
                        width: Fill
                        height: Fill
                        default_route: @home
                        not_found_route: @not_found
                        cap_nested: true

                        home := mod.widgets.RouterRoute {
                            route_pattern: "/"
                            mod.widgets.HomePage {}
                        }
                        settings := mod.widgets.RouterRoute {
                            route_pattern: "/settings/*"
                            mod.widgets.SettingsPage {}
                        }
                        detail := mod.widgets.RouterRoute {
                            route_pattern: "/detail/:id"
                            mod.widgets.DetailPage {}
                        }
                        not_found := mod.widgets.RouterRoute {
                            mod.widgets.NotFoundPage {}
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
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(cx, ids!(router));

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
            .button(cx, ids!(nav_bar.settings_btn))
            .clicked(actions)
        {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToRoute {
                    route_id: live_id!(settings),
                    transition: None,
                },
            );
        }
        if self
            .ui
            .button(cx, ids!(nav_bar.detail_btn))
            .clicked(actions)
        {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToPath {
                    path: "/detail/42".to_string(),
                },
            );
        }
        if self.ui.button(cx, ids!(nav_bar.back_btn)).clicked(actions) {
            let _ = router.dispatch(cx, RouterCommand::Back { transition: None });
        }

        match router.current_route_id() {
            Some(id) if id == live_id!(home) => {
                let Some((to_detail, to_settings, to_broken)) =
                    router.with_active_route_widget(|w| {
                        (
                            w.button(cx, &[live_id!(open_detail_btn)]).clicked(actions),
                            w.button(cx, &[live_id!(open_settings_btn)])
                                .clicked(actions),
                            w.button(cx, &[live_id!(open_broken_btn)]).clicked(actions),
                        )
                    })
                else {
                    return;
                };

                if to_detail {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToPath {
                            path: "/detail/42".to_string(),
                        },
                    );
                }
                if to_settings {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToRoute {
                            route_id: live_id!(settings),
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
            Some(id) if id == live_id!(detail) => {
                let _ = router.bind_param_to_label(cx, "id", live_id!(detail_id_label), |id| {
                    format!("Detail ID: {}", id)
                });

                let Some((to_1, to_42, to_home)) = router.with_active_route_widget(|w| {
                    (
                        w.button(cx, &[live_id!(detail_to_1_btn)]).clicked(actions),
                        w.button(cx, &[live_id!(detail_to_42_btn)]).clicked(actions),
                        w.button(cx, &[live_id!(detail_home_btn)]).clicked(actions),
                    )
                }) else {
                    return;
                };

                if to_1 {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToPath {
                            path: "/detail/1".to_string(),
                        },
                    );
                }
                if to_42 {
                    let _ = router.dispatch(
                        cx,
                        RouterCommand::GoToPath {
                            path: "/detail/42".to_string(),
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
            Some(id) if id == live_id!(settings) => {
                let Some((to_overview, to_profile, to_home, settings_router)) = router
                    .with_active_route_widget(|w| {
                        (
                            w.button(cx, &[live_id!(settings_overview_btn)])
                                .clicked(actions),
                            w.button(cx, &[live_id!(settings_profile_btn)])
                                .clicked(actions),
                            w.button(cx, &[live_id!(settings_home_btn)])
                                .clicked(actions),
                            w.widget(cx, &[live_id!(settings_router)])
                                .as_router_widget(),
                        )
                    })
                else {
                    return;
                };

                if !settings_router.is_empty() {
                    if to_overview {
                        let _ = settings_router.dispatch(
                            cx,
                            RouterCommand::GoToPath {
                                path: "/overview".to_string(),
                            },
                        );
                    }
                    if to_profile {
                        let _ = settings_router.dispatch(
                            cx,
                            RouterCommand::GoToPath {
                                path: "/profile".to_string(),
                            },
                        );
                    }
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
            Some(id) if id == live_id!(not_found) => {
                let Some(to_home) = router.with_active_route_widget(|w| {
                    w.button(cx, &[live_id!(not_found_home_btn)])
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
