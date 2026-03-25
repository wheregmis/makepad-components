use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::button::ShadButtonWidgetExt;
use makepad_components::makepad_widgets::*;
use makepad_components::popover::ShadPopoverWidgetExt;

gallery_stateful_page_shell! {
    widget: GalleryNavigationMenuPage,
    page: navigation_menu_page,
    title: "Navigation Menu",
    subtitle: "Top-level site or dashboard navigation with richer flyout content than a menubar. `ShadNavigationMenuItem` is also popover-backed, but the default content surface is wider and tuned for callouts, grouped links, and launch panels.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Marketing / docs navigation" }

        navigation_menu_demo := ShadNavigationMenu{
            navigation_list := ShadNavigationMenuList{
                products_menu := ShadNavigationMenuItem{
                    trigger := ShadNavigationMenuTrigger{text: "Products"}

                    content: ShadNavigationMenuContent{
                        menu_row := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 12.0

                            ShadNavigationMenuCallout{
                                ShadSectionHeader{text: "Ship faster"}
                                ShadFieldDescription{text: "Launch with billing, auth, and analytics primitives that already fit the design system."}
                                products_trial_btn := ShadButton{text: "Start trial"}
                            }

                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 10.0

                                sdk_panel := ShadNavigationMenuPanel{
                                    products_sdk_btn := ShadButton{
                                        variant: ShadButtonVariant.Ghost
                                        width: Fill
                                        align: Align{x: 0.0, y: 0.5}
                                        text: "SDKs"
                                    }
                                    ShadFieldDescription{text: "Type-safe clients for web, desktop, and native apps."}
                                }

                                dashboard_panel := ShadNavigationMenuPanel{
                                    products_dashboard_btn := ShadButton{
                                        variant: ShadButtonVariant.Ghost
                                        width: Fill
                                        align: Align{x: 0.0, y: 0.5}
                                        text: "Dashboards"
                                    }
                                    ShadFieldDescription{text: "Operational views for metrics, queues, and release health."}
                                }
                            }
                        }
                    }
                }

                resources_menu := ShadNavigationMenuItem{
                    trigger := ShadNavigationMenuTrigger{text: "Resources"}

                    content: ShadNavigationMenuContent{
                        ShadNavigationMenuSectionLabel{text: "Learn"}
                        guides_btn := ShadButton{
                            variant: ShadButtonVariant.Ghost
                            width: Fill
                            align: Align{x: 0.0, y: 0.5}
                            text: "Guides"
                        }
                        ShadFieldDescription{text: "Architecture notes, implementation walkthroughs, and migration recipes."}
                        examples_btn := ShadButton{
                            variant: ShadButtonVariant.Ghost
                            width: Fill
                            align: Align{x: 0.0, y: 0.5}
                            text: "Examples"
                        }
                        ShadFieldDescription{text: "Reference compositions for onboarding, billing, and analytics flows."}
                    }
                }

                pricing_link := ShadButton{
                    variant: ShadButtonVariant.Ghost
                    text: "Pricing"
                }
            }
        }

        navigation_selection_status := ShadFieldDescription{
            text: "Selected destination: none yet."
        }

        navigation_open_status := ShadFieldDescription{
            text: "All navigation menus are closed."
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Use `ShadNavigationMenuList` for the visible trigger row, then place one hover-open `ShadNavigationMenuItem` per flyout."}
        mod.widgets.GalleryActionFlowStep{text: "2. The wider `ShadNavigationMenuContent` surface is meant for grouped links, feature callouts, and multi-column summaries."}
        mod.widgets.GalleryActionFlowStep{text: "3. Navigation menu flyouts share a hover group, so moving between top-level items closes siblings without page-level orchestration."}
        mod.widgets.GalleryActionFlowStep{text: "4. Keep route changes in app code. These primitives own layout and popup presentation, not browser/history behavior."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryNavigationMenuPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryNavigationMenuPage {
    fn set_selection_status(&self, cx: &mut Cx, text: &str) {
        self.view
            .label(cx, ids!(navigation_selection_status))
            .set_text(cx, text);
    }

    fn sync_open_status(&self, cx: &mut Cx) {
        let products = self.view.shad_popover(cx, ids!(products_menu));
        let resources = self.view.shad_popover(cx, ids!(resources_menu));

        let text = if products.is_open() {
            "Products menu is open."
        } else if resources.is_open() {
            "Resources menu is open."
        } else {
            "All navigation menus are closed."
        };

        self.view
            .label(cx, ids!(navigation_open_status))
            .set_text(cx, text);
    }
}

impl Widget for GalleryNavigationMenuPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self.view.shad_button(cx, ids!(pricing_link)).clicked(actions) {
                self.set_selection_status(cx, "Selected destination: Pricing");
                return;
            }

            let products = self.view.shad_popover(cx, ids!(products_menu));
            let products_content = products.content_widget();
            if products_content
                .shad_button(cx, ids!(products_trial_btn))
                .clicked(actions)
            {
                products.close(cx);
                self.set_selection_status(cx, "Selected destination: Start trial");
                self.sync_open_status(cx);
                return;
            }
            if products_content
                .shad_button(cx, ids!(products_sdk_btn))
                .clicked(actions)
            {
                products.close(cx);
                self.set_selection_status(cx, "Selected destination: SDKs");
                self.sync_open_status(cx);
                return;
            }
            if products_content
                .shad_button(cx, ids!(products_dashboard_btn))
                .clicked(actions)
            {
                products.close(cx);
                self.set_selection_status(cx, "Selected destination: Dashboards");
                self.sync_open_status(cx);
                return;
            }

            let resources = self.view.shad_popover(cx, ids!(resources_menu));
            let resources_content = resources.content_widget();
            if resources_content
                .shad_button(cx, ids!(guides_btn))
                .clicked(actions)
            {
                resources.close(cx);
                self.set_selection_status(cx, "Selected destination: Guides");
                self.sync_open_status(cx);
                return;
            }
            if resources_content
                .shad_button(cx, ids!(examples_btn))
                .clicked(actions)
            {
                resources.close(cx);
                self.set_selection_status(cx, "Selected destination: Examples");
                self.sync_open_status(cx);
                return;
            }

            if matches!(products.open_changed(actions), Some(true))
                || matches!(resources.open_changed(actions), Some(true))
            {
                self.sync_open_status(cx);
                return;
            }
            if products.open_changed(actions).is_some() || resources.open_changed(actions).is_some()
            {
                self.sync_open_status(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
