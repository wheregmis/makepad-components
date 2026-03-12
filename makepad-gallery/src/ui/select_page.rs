use crate::ui::snippets::SELECT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySelectPageBase = #(GallerySelectPage::register_widget(vm))

    mod.widgets.GallerySelectPage = set_type_default() do mod.widgets.GallerySelectPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollArea{
            ShadPageTitle{
                text: "Select"
            }

            ShadPageSubtitle{
                text: "Select uses the dropdown ref API: read changed(actions) or changed_label(actions), then store the chosen index or label in page state."
            }

            ShadSeparator{}

            select_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                select_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    select_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        select_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        select_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    select_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        select_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        select_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                select_preview_panel := mod.widgets.ShadPanel{
                    select_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadPanel{
                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12.0

                                    View{
                                        width: Fit
                                        height: Fit
                                        flow: Right
                                        spacing: 12.0

                                        ShadSelect{labels: ["Pending" "In Progress" "Done"]}
                                        ShadSelect{labels: ["Toronto" "Montreal" "Vancouver" "Calgary"]}
                                    }

                                    ShadFieldDescription{
                                        text: "Known limitation: popup-style selects can still be unreliable inside the current gallery PageFlip shell. The splash app remains the best place to verify interaction."
                                    }
                                }
                            }

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. Give the select an id, then get the dropdown ref with view.drop_down(cx, ids!(status_select))."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. Use changed(actions) when you want the selected index, or changed_label(actions) when the label is enough."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. Persist the chosen item in page state, then restore it with set_selected_item(cx, ...) or set_selected_by_label(..., cx)."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. The popup interaction stays inside the component; the page only reacts to the semantic selection result."}
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
                                    code_view +: { text: #(SELECT_PREVIEW_CODE) }
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
pub struct GallerySelectPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySelectPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
