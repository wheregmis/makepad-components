use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_router::widget::RouterWidgetWidgetExt;

gallery_stateful_page_shell! {
    root: ShadScrollArea,
    widget: GalleryTabsPage,
    page: tabs_page,
    title: "Tabs",
    subtitle: "Composable tab styling that now behaves better on narrow widths: the trigger row scrolls horizontally, keeps touch targets larger, and still lets the page own tab state.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        tabs_preview_shell := ShadPanel{
            width: 340
            flow: Down
            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}

            ShadTabs{
                tabs_row := ShadTabsList{
                    overview_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        tabs_overview_trigger := ShadTabsTrigger{text: "Overview & Activity"}
                        tabs_overview_indicator := ShadTabsIndicator{}
                    }

                    usage_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        tabs_usage_trigger := ShadTabsTrigger{text: "Implementation Notes"}
                        tabs_usage_indicator := ShadTabsIndicator{
                            visible: false
                        }
                    }

                    settings_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        tabs_settings_trigger := ShadTabsTrigger{text: "Settings & Access"}
                        tabs_settings_indicator := ShadTabsIndicator{
                            visible: false
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
                        ShadFieldDescription{text: "Keep related content grouped, even when the trigger row overflows on mobile or inside compact panels."}
                    }

                    usage_page := mod.widgets.RouterRoute{
                        route_pattern: "/usage"
                        ShadSectionHeader{text: "Usage"}
                        ShadFieldDescription{text: "Pair `ShadTabsTrigger` with `PageFlip`, `RouterWidget`, or any app-owned state holder. The trigger row is visual; the page still owns the selected tab."}
                    }

                    settings_page := mod.widgets.RouterRoute{
                        route_pattern: "/settings"
                        ShadSectionHeader{text: "Settings"}
                        ShadFieldDescription{text: "The compact shell here intentionally forces horizontal overflow so the new tab-row scrolling stays visible in the demo."}
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. The selected tab is page-owned state, not app-shell glue and not hidden inside the visual trigger widgets."}
        mod.widgets.GalleryActionFlowStep{text: "2. The trigger row can now overflow horizontally, so narrow mobile panels do not need custom fallback layouts just to keep tabs usable."}
        mod.widgets.GalleryActionFlowStep{text: "3. The page controller still listens to trigger clicks locally and updates one source of truth for the active tab."}
        mod.widgets.GalleryActionFlowStep{text: "4. That selected value drives both the RouterWidget content and the active indicator visibility."}
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
