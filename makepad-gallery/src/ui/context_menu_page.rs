use crate::ui::snippets::CONTEXT_MENU_PREVIEW_CODE;
use makepad_components::context_menu::ShadContextMenuWidgetExt;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryContextMenuPageBase = #(GalleryContextMenuPage::register_widget(vm))

    mod.widgets.GalleryContextMenuPage = set_type_default() do mod.widgets.GalleryContextMenuPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Context Menu"
            }

            ShadPageSubtitle{
                text: "Right click or long press a trigger area to reveal contextual actions."
            }

            ShadHr{}

            context_menu_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                context_menu_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    context_menu_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        context_menu_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        context_menu_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    context_menu_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        context_menu_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        context_menu_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                context_menu_preview_panel := mod.widgets.ShadPanel{
                    context_menu_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Basic" }

                            context_menu_basic := ShadContextMenu{
                                labels: ["Open" "Duplicate" "Share" "Delete"]

                                RoundedView{
                                    width: 360
                                    height: Fit
                                    flow: Down
                                    spacing: 6.0
                                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                    draw_bg +: {
                                        color: (shad_theme.color_secondary)
                                        border_size: 1.0
                                        border_radius: (shad_theme.radius)
                                        border_color: (shad_theme.color_outline_border)
                                    }

                                    ShadLabel{text: "Project brief.md"}
                                    ShadFieldDescription{text: "Right click this card to open the menu."}
                                }
                            }

                            context_menu_status := ShadFieldDescription{
                                text: "No action selected yet."
                            }
                        }

                        code_page +: {
                            body +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            GalleryCodeSnippet{
                                code_view +: { text: #(CONTEXT_MENU_PREVIEW_CODE) }
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
pub struct GalleryContextMenuPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryContextMenuPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if let Some(index) = self
                .view
                .shad_context_menu(cx, ids!(context_menu_basic))
                .selected(actions)
            {
                let label = match index {
                    0 => "Open",
                    1 => "Duplicate",
                    2 => "Share",
                    3 => "Delete",
                    _ => "Unknown",
                };
                self.view
                    .label(cx, ids!(context_menu_status))
                    .set_text(cx, &format!("Selected: {}", label));
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
