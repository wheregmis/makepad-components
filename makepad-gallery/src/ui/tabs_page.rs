use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_router::widget::RouterWidgetWidgetExt;

gallery_stateful_page_shell! {
    root: ShadScrollArea,
    widget: GalleryTabsPage,
    page: tabs_page,
    title: "Tabs",
    subtitle: "Composable trigger and content styles for app-level tab state. This gallery page keeps tab clicks local and routes content through a RouterWidget.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
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
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. The selected tab is page-owned state, not app-shell glue and not hidden inside the visual trigger widgets."}
        mod.widgets.GalleryActionFlowStep{text: "2. The page controller listens to trigger clicks locally and updates one source of truth for the active tab."}
        mod.widgets.GalleryActionFlowStep{text: "3. That selected value drives both the RouterWidget content and the active indicator visibility."}
        mod.widgets.GalleryActionFlowStep{text: "4. This keeps ShadTabs primitives composable while the page decides how content switching behaves."}
    },
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
