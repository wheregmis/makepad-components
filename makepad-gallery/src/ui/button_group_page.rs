use crate::ui::snippets::BUTTON_GROUP_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryButtonGroupPageBase = #(GalleryButtonGroupPage::register_widget(vm))

    mod.widgets.GalleryButtonGroupPage = set_type_default() do mod.widgets.GalleryButtonGroupPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Button Group"
            }

            ShadPageSubtitle{
                text: "Button groups are grouped leaf actions: the container is presentational, while each named child button still emits its own clicked(actions) event."
            }

            ShadHr{}

            button_group_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                button_group_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    button_group_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        button_group_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}
                        button_group_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    button_group_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        button_group_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}
                        button_group_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                button_group_preview_panel := mod.widgets.ShadPanel{
                    button_group_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Default" }

                            View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 10.0
                                align: Align{y: 0.5}

                                IconButtonChevronLeft{
                                    draw_bg +: {
                                        color: (shad_theme.color_secondary)
                                        border_size: 1.0
                                        border_radius: (shad_theme.radius)
                                        border_color: (shad_theme.color_outline_border)
                                    }
                                    draw_icon.color: (shad_theme.color_primary)
                                }

                                ShadButtonGroup{
                                    ShadButtonGroupItem{text: "Archive"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItem{text: "Report"}
                                }

                                ShadButtonGroup{
                                    ShadButtonGroupItem{text: "Snooze"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItemIcon{text: "⋯"}
                                }
                            }

                            ShadSectionHeader{ text: "Sizes" }

                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 10.0

                                ShadButtonGroup{
                                    ShadButtonGroupItemSm{text: "Day"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItemSm{text: "Week"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItemSm{text: "Month"}
                                }

                                ShadButtonGroup{
                                    ShadButtonGroupItem{text: "Day"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItem{text: "Week"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItem{text: "Month"}
                                }

                                ShadButtonGroup{
                                    ShadButtonGroupItemLg{text: "Day"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItemLg{text: "Week"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItemLg{text: "Month"}
                                }
                            }

                            ShadSectionHeader{ text: "Toolbar" }

                            View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 10.0
                                align: Align{y: 0.5}

                                ShadButtonGroup{
                                    ShadButtonGroupItem{text: "Bold"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItem{text: "Italic"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItem{text: "Underline"}
                                }

                                ShadButtonGroup{
                                    ShadButtonGroupItemIcon{text: "A-"}
                                    ShadButtonGroupSeparator{}
                                    ShadButtonGroupItemIcon{text: "A+"}
                                }
                            }

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. Treat ShadButtonGroup as layout and visual grouping, not as a separate state machine."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. Name the child buttons you care about, then listen to each with ui.button(cx, ids!(archive_btn)).clicked(actions)."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. Keep the selected tool or toolbar mode in page state, then re-render whichever button should look active."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. This scales because business identity stays in your state model, not in the group container."}
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
                                    code_view +: { text: #(BUTTON_GROUP_PREVIEW_CODE) }
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
pub struct GalleryButtonGroupPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryButtonGroupPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
