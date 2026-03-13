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
                text: "Pressed-state buttons for formatting, filtering, and grouped controls. Keep grouped selection in the page/controller and update each item's active state from that source of truth."
            }

            ShadHr{}

            toggle_preview_section := mod.widgets.GalleryPreviewSection{
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

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Treat pressed and selected values as page/controller state, especially for grouped toggles."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. On user click, update that state in the page rather than trying to manage each toggle from the app shell."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Re-render the matching toggle or toggle-group item with active: true from the current state."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. This is the same controller pattern used by the Tabs page: local state in the page, visuals derived from that state."}
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
                                    code: #(TOGGLE_PREVIEW_CODE)
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
