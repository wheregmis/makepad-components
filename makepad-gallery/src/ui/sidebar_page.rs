use crate::ui::snippets::SIDEBAR_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySidebarPageBase = #(GallerySidebarPage::register_widget(vm))

    mod.widgets.GallerySidebarPage = set_type_default() do mod.widgets.GallerySidebarPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Sidebar"
            }

            ShadPageSubtitle{
                text: "Sidebar primitives are navigation-flavored button actions. Name the items you care about, then route or swap page state from their clicks."
            }

            ShadHr{}

            sidebar_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                sidebar_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    sidebar_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        sidebar_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        sidebar_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    sidebar_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        sidebar_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        sidebar_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                sidebar_preview_panel := mod.widgets.ShadPanel{
                    sidebar_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 12.0
                                align: Align{y: 0.0}

                                ShadSidebar{
                                    width: 300
                                    height: 320
                                    ShadLabel{
                                        text: "Acme Inc"
                                        draw_text.text_style.font_size: 12
                                    }
                                    ShadSidebarSectionLabel{text: "Platform"}
                                    ShadSidebarItem{text: "Playground"}
                                    ShadSidebarItem{text: "History"}
                                    ShadSidebarItem{text: "Settings"}
                                }

                                View{
                                    width: Fill
                                    height: 320
                                    draw_bg.color: #0000
                                    draw_bg.border_size: 1.0
                                    draw_bg.border_color: (shad_theme.color_outline_border)
                                    draw_bg.border_radius: (shad_theme.radius)
                                }
                            }

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. Treat each ShadSidebarItem like a named button action with sidebar styling."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. Read clicks with ui.button(cx, ids!(nav_playground)).clicked(actions), then route or swap the selected page."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. Keep the active route in page or app state, not inside the sidebar primitive."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. Render the active item from that route state so sidebar, router, and content stay in sync."}
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
                                    code_view +: { text: #(SIDEBAR_PREVIEW_CODE) }
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
pub struct GallerySidebarPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySidebarPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
