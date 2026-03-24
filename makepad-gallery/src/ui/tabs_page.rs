use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::tabs::{ShadTabSpec, ShadTabsController};
use makepad_router::widget::RouterWidgetWidgetExt;

gallery_stateful_page_shell! {
    root: ShadScrollArea,
    widget: GalleryTabsPage,
    page: tabs_page,
    title: "Tabs",
    subtitle: "Composable tab primitives plus an optional `ShadTabsController` for keeping trigger state and content routing in sync.",
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
                    tabs_overview_indicator := ShadTabsIndicator{}
                }

                usage_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 4.0

                    tabs_usage_trigger := ShadTabsTrigger{text: "Usage"}
                    tabs_usage_indicator := ShadTabsIndicator{
                        visible: false
                    }
                }

                settings_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 4.0

                    tabs_settings_trigger := ShadTabsTrigger{text: "Settings"}
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
                    ShadFieldDescription{text: "Keep the page shell compact while switching between related content areas."}
                }

                usage_page := mod.widgets.RouterRoute{
                    route_pattern: "/usage"
                    ShadSectionHeader{text: "Usage"}
                    ShadFieldDescription{text: "Pair `ShadTabsController` with `PageFlip`, RouterWidget, or another app-owned content switcher."}
                }

                settings_page := mod.widgets.RouterRoute{
                    route_pattern: "/settings"
                    ShadSectionHeader{text: "Settings"}
                    ShadFieldDescription{text: "The controller is optional: keep composition when you want it, or centralize selection syncing when the page needs it."}
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep the selected tab in page-owned state and initialize a `ShadTabsController` with trigger and indicator ids."}
        mod.widgets.GalleryActionFlowStep{text: "2. Call controller.changed(cx, &view, actions) to update one source of truth for the active tab."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use controller.selected() to drive RouterWidget, PageFlip, or any other content switcher."}
        mod.widgets.GalleryActionFlowStep{text: "4. This keeps the visual tab primitives composable while still offering a reusable state-sync layer."}
    },
}

#[derive(Script, Widget)]
pub struct GalleryTabsPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    tabs: ShadTabsController,
}

impl GalleryTabsPage {
    fn make_tabs_controller() -> ShadTabsController {
        ShadTabsController::new(
            live_id!(overview_page),
            vec![
                ShadTabSpec::new(
                    live_id!(overview_page),
                    ids!(tabs_overview_trigger),
                    ids!(tabs_overview_indicator),
                ),
                ShadTabSpec::new(
                    live_id!(usage_page),
                    ids!(tabs_usage_trigger),
                    ids!(tabs_usage_indicator),
                ),
                ShadTabSpec::new(
                    live_id!(settings_page),
                    ids!(tabs_settings_trigger),
                    ids!(tabs_settings_indicator),
                ),
            ],
        )
    }

    fn set_selected_tab(&mut self, cx: &mut Cx, page: LiveId) {
        let root = self.view.widget(cx, &[]);
        self.tabs.set_selected(cx, &root, page);
        self.view
            .router_widget(cx, ids!(tabs_content_flip))
            .go_to_route(cx, page);
    }
}

impl ScriptHook for GalleryTabsPage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        if self.tabs.is_empty() {
            self.tabs = Self::make_tabs_controller();
        }
        vm.with_cx_mut(|cx| self.set_selected_tab(cx, self.tabs.selected()));
    }
}

impl Widget for GalleryTabsPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let root = self.view.widget(cx, &[]);
            if let Some(page) = self.tabs.changed(cx, &root, actions) {
                self.set_selected_tab(cx, page);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
