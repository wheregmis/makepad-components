use crate::ui::snippets::TOGGLE_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryTogglePageBase = #(GalleryTogglePage::register_widget(vm))

    mod.widgets.GalleryTogglePage = set_type_default() do mod.widgets.GalleryTogglePageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Toggle"
            }

            ShadPageSubtitle{
                text: "Pressed-state buttons for formatting, filtering, and grouped controls."
            }

            ShadHr{}

            toggle_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                toggle_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    toggle_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        toggle_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        toggle_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    toggle_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        toggle_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        toggle_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                toggle_preview_panel := mod.widgets.ShadPanel{
                    toggle_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Standalone toggles" }
                            View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 8.0

                                ShadToggle{text: "Bold"}
                                ShadToggle{text: "Italic" active: true}
                                ShadToggle{text: "Underline"}
                            }

                            ShadHr{}

                            ShadSectionHeader{ text: "Toggle group" }
                            ShadToggleGroup{
                                ShadToggleGroupItem{text: "Left"}
                                ShadToggleGroupItem{text: "Center" active: true}
                                ShadToggleGroupItem{text: "Right"}
                            }

                            ShadHr{}

                            ShadSectionHeader{ text: "Sizes" }
                            View{
                                width: Fit
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                ShadToggleGroup{
                                    ShadToggleGroupItemSm{text: "Sm"}
                                    ShadToggleGroupItemSm{text: "Active" active: true}
                                }

                                ShadToggleGroup{
                                    ShadToggleGroupItem{text: "Default"}
                                    ShadToggleGroupItem{text: "Active" active: true}
                                }

                                ShadToggleGroup{
                                    ShadToggleGroupItemLg{text: "Large"}
                                    ShadToggleGroupItemLg{text: "Active" active: true}
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
                                code_view +: { text: #(TOGGLE_PREVIEW_CODE) }
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
pub struct GalleryTogglePage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryTogglePage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
