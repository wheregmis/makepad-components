use crate::ui::snippets::RESIZABLE_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryResizablePageBase = #(GalleryResizablePage::register_widget(vm))

    mod.widgets.GalleryResizablePage = set_type_default() do mod.widgets.GalleryResizablePageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollArea{
            ShadPageTitle{
                text: "Resizable"
            }

            ShadPageSubtitle{
                text: "Resizable panes emit splitter alignment changes, so pages can persist and restore layout without reaching into child internals."
            }

            ShadSeparator{}

            resizable_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "Horizontal panes" }
                                ShadPanel{
                                    height: 240

                                    horizontal_resizable := ShadResizable{
                                        width: Fill
                                        height: Fill
                                        axis: SplitterAxis.Horizontal
                                        align: SplitterAlign.FromA(180.0)
                                        a: RoundedView{
                                            width: Fill
                                            height: Fill
                                            padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                            flow: Down
                                            spacing: 8.0
                                            draw_bg.color: (shad_theme.color_muted)
                                            draw_bg.border_radius: (shad_theme.radius)
                                            ShadSectionHeader{text: "Navigation"}
                                            ShadFieldDescription{text: "Keep filters, folders, or nav trees here."}
                                        }

                                        b: RoundedView{
                                            width: Fill
                                            height: Fill
                                            padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                            flow: Down
                                            spacing: 8.0
                                            draw_bg.color: #0000
                                            draw_bg.border_radius: (shad_theme.radius)
                                            ShadSectionHeader{text: "Content"}
                                            ShadFieldDescription{text: "Main editing or reading surface."}
                                        }
                                    }
                                }

                                ShadSectionHeader{ text: "Vertical panes" }
                                ShadPanel{
                                    height: 260

                                    vertical_resizable := ShadResizable{
                                        width: Fill
                                        height: Fill
                                        axis: SplitterAxis.Vertical
                                        align: SplitterAlign.FromA(120.0)
                                        a: RoundedView{
                                            width: Fill
                                            height: Fill
                                            padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                            flow: Down
                                            spacing: 8.0
                                            draw_bg.color: (shad_theme.color_muted)
                                            draw_bg.border_radius: (shad_theme.radius)
                                            ShadSectionHeader{text: "Metrics"}
                                            ShadFieldDescription{text: "Compact summary cards or charts."}
                                        }

                                        b: RoundedView{
                                            width: Fill
                                            height: Fill
                                            padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                            flow: Down
                                            spacing: 8.0
                                            draw_bg.color: #0000
                                            draw_bg.border_radius: (shad_theme.radius)
                                            ShadSectionHeader{text: "Details"}
                                            ShadFieldDescription{text: "Expanded logs, notes, or tables."}
                                        }
                                    }
                                }
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Name the splitter you want to persist, like horizontal_resizable or vertical_resizable."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Read layout changes with view.splitter(cx, ids!(horizontal_resizable)).changed(actions)."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. Persist the returned SplitterAlign in page state, local storage, or app settings."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. Restore the saved layout with set_align(cx, align) when the page is rebuilt."}
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
                                    code: #(RESIZABLE_PREVIEW_CODE)
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
pub struct GalleryResizablePage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryResizablePage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
