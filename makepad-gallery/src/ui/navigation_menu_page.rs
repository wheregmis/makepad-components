use crate::ui::snippets::NAVIGATION_MENU_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;
use makepad_components::popover::ShadPopoverWidgetExt;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryNavigationMenuPageBase = #(GalleryNavigationMenuPage::register_widget(vm))

    mod.widgets.GalleryNavigationMenuPage = set_type_default() do mod.widgets.GalleryNavigationMenuPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Navigation Menu"
            }

            ShadPageSubtitle{
                text: "Top-level site or dashboard navigation with richer flyout content than a menubar. `ShadNavigationMenuItem` is also popover-backed, but the default content surface is wider and tuned for callouts, grouped links, and launch panels."
            }

            ShadHr{}

            navigation_menu_preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_flip +: {
                        root_view +: {
                            preview_content +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 16.0

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
                                                            products_sdk_btn := ShadButtonGhost{
                                                                width: Fill
                                                                align: Align{x: 0.0, y: 0.5}
                                                                text: "SDKs"
                                                            }
                                                            ShadFieldDescription{text: "Type-safe clients for web, desktop, and native apps."}
                                                        }

                                                        dashboard_panel := ShadNavigationMenuPanel{
                                                            products_dashboard_btn := ShadButtonGhost{
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
                                                guides_btn := ShadButtonGhost{
                                                    width: Fill
                                                    align: Align{x: 0.0, y: 0.5}
                                                    text: "Guides"
                                                }
                                                ShadFieldDescription{text: "Architecture notes, implementation walkthroughs, and migration recipes."}
                                                examples_btn := ShadButtonGhost{
                                                    width: Fill
                                                    align: Align{x: 0.0, y: 0.5}
                                                    text: "Examples"
                                                }
                                                ShadFieldDescription{text: "Reference compositions for onboarding, billing, and analytics flows."}
                                            }
                                        }

                                        pricing_link := ShadButtonGhost{
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
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Use `ShadNavigationMenuList` for the visible trigger row, then place one `ShadNavigationMenuItem` per flyout."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. The wider `ShadNavigationMenuContent` surface is meant for grouped links, feature callouts, and multi-column summaries."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Because each item reuses `ShadPopover`, you can query `opened(actions)` / `closed(actions)` and close the flyout after a button or link fires."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Keep route changes in app code. These primitives own layout and popup presentation, not browser/history behavior."}
                                    }
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(NAVIGATION_MENU_PREVIEW_CODE)
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
            if self.view.button(cx, ids!(pricing_link)).clicked(actions) {
                self.set_selection_status(cx, "Selected destination: Pricing");
                return;
            }

            let products = self.view.shad_popover(cx, ids!(products_menu));
            let products_content = products.content_widget();
            if products_content
                .button(cx, ids!(products_trial_btn))
                .clicked(actions)
            {
                products.close(cx);
                self.set_selection_status(cx, "Selected destination: Start trial");
                self.sync_open_status(cx);
                return;
            }
            if products_content
                .button(cx, ids!(products_sdk_btn))
                .clicked(actions)
            {
                products.close(cx);
                self.set_selection_status(cx, "Selected destination: SDKs");
                self.sync_open_status(cx);
                return;
            }
            if products_content
                .button(cx, ids!(products_dashboard_btn))
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
                .button(cx, ids!(guides_btn))
                .clicked(actions)
            {
                resources.close(cx);
                self.set_selection_status(cx, "Selected destination: Guides");
                self.sync_open_status(cx);
                return;
            }
            if resources_content
                .button(cx, ids!(examples_btn))
                .clicked(actions)
            {
                resources.close(cx);
                self.set_selection_status(cx, "Selected destination: Examples");
                self.sync_open_status(cx);
                return;
            }

            if products.opened(actions)
                || products.closed(actions)
                || resources.opened(actions)
                || resources.closed(actions)
            {
                self.sync_open_status(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
