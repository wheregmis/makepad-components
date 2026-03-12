use crate::ui::snippets::TABS_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;
use makepad_router::widget::RouterWidgetWidgetExt;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryTabsPageBase = #(GalleryTabsPage::register_widget(vm))

    mod.widgets.GalleryTabsPage = set_type_default() do mod.widgets.GalleryTabsPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollArea{
            width: Fill
            height: Fill

            ShadPageTitle{
                text: "Tabs"
            }

            ShadPageSubtitle{
                text: "Composable trigger and content styles for app-level tab state. This gallery page keeps tab clicks local and routes content through a RouterWidget."
            }

            ShadSeparator{}

            tabs_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                tabs_demo_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    tabs_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        tabs_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        tabs_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    tabs_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        tabs_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        tabs_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                tabs_preview_panel := mod.widgets.ShadPanel{
                    tabs_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadTabs{
                                tabs_row := ShadTabsList{
                                    overview_group := View{
                                        width: Fit
                                        height: Fit
                                        flow: Down
                                        spacing: 4.0

                                        tabs_overview_trigger := ShadTabsTrigger{text: "Overview"}
                                        tabs_overview_indicator := SolidView{
                                            width: Fill
                                            height: 2
                                            draw_bg.color: (shad_theme.color_primary)
                                        }
                                    }

                                    usage_group := View{
                                        width: Fit
                                        height: Fit
                                        flow: Down
                                        spacing: 4.0

                                        tabs_usage_trigger := ShadTabsTrigger{text: "Usage"}
                                        tabs_usage_indicator := SolidView{
                                            width: Fill
                                            height: 2
                                            visible: false
                                            draw_bg.color: (shad_theme.color_primary)
                                        }
                                    }

                                    settings_group := View{
                                        width: Fit
                                        height: Fit
                                        flow: Down
                                        spacing: 4.0

                                        tabs_settings_trigger := ShadTabsTrigger{text: "Settings"}
                                        tabs_settings_indicator := SolidView{
                                            width: Fill
                                            height: 2
                                            visible: false
                                            draw_bg.color: (shad_theme.color_primary)
                                        }
                                    }
                                }

                                tabs_content_flip := mod.widgets.RouterWidget{
                                    width: Fill
                                    height: Fit
                                    default_route: @overview_page
                                    not_found_route: @overview_page

                                    overview_page := mod.widgets.RouterRoute{
                                        route_pattern: "/"
                                        ShadSectionHeader{text: "Overview"}
                                        ShadFieldDescription{text: "Keep the page shell compact while switching between related content areas."}
                                    }

                                    usage_page := mod.widgets.RouterRoute{
                                        route_pattern: "/usage"
                                        ShadSectionHeader{text: "Usage"}
                                        ShadFieldDescription{text: "Pair `ShadTabsTrigger` with `PageFlip` or another state holder in app code."}
                                    }

                                    settings_page := mod.widgets.RouterRoute{
                                        route_pattern: "/settings"
                                        ShadSectionHeader{text: "Settings"}
                                        ShadFieldDescription{text: "This first pass focuses on composition and styling, not a fully stateful tab controller."}
                                    }
                                }
                            }

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. The selected tab is page-owned state, not app-shell glue and not hidden inside the visual trigger widgets."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. The page controller listens to trigger clicks locally and updates one source of truth for the active tab."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. That selected value drives both the RouterWidget content and the active indicator visibility."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. This keeps ShadTabs primitives composable while the page decides how content switching behaves."}
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            GalleryCodeSnippet{
                                code_view +: { text: #(TABS_PREVIEW_CODE) }
                            }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryTabsPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryTabsPage {
    fn set_selected_tab(&mut self, cx: &mut Cx, page: LiveId) {
        self.view
            .router_widget(cx, ids!(tabs_content_flip))
            .go_to_route(cx, page);
        self.view
            .view(cx, ids!(tabs_overview_indicator))
            .set_visible(cx, page == live_id!(overview_page));
        self.view
            .view(cx, ids!(tabs_usage_indicator))
            .set_visible(cx, page == live_id!(usage_page));
        self.view
            .view(cx, ids!(tabs_settings_indicator))
            .set_visible(cx, page == live_id!(settings_page));
    }
}

impl Widget for GalleryTabsPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self
                .view
                .button(cx, ids!(tabs_overview_trigger))
                .clicked(actions)
            {
                self.set_selected_tab(cx, live_id!(overview_page));
            }
            if self
                .view
                .button(cx, ids!(tabs_usage_trigger))
                .clicked(actions)
            {
                self.set_selected_tab(cx, live_id!(usage_page));
            }
            if self
                .view
                .button(cx, ids!(tabs_settings_trigger))
                .clicked(actions)
            {
                self.set_selected_tab(cx, live_id!(settings_page));
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
