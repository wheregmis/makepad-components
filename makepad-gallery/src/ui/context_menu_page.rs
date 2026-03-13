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
                text: "Right click or long press a trigger area to reveal contextual actions. Read the selected item with ShadContextMenuRef::selected(actions)."
            }

            ShadHr{}

            context_menu_preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_flip +: {
                        root_view +: {
                            preview_content +: {
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

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Compose the trigger area as the child of ShadContextMenu; the component owns the popup internals."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Read selected(actions) from ShadContextMenuRef to get the chosen item index."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Translate that index into domain actions like Open, Duplicate, Share, or Delete inside the page/controller."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Update visible state, status text, or execute commands without touching popup menu internals."}
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
                                    code: #(CONTEXT_MENU_PREVIEW_CODE)
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
