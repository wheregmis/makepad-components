use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::sheet::ShadSheetWidgetExt;

gallery_stateful_page_shell! {
    root: ShadScrollArea,
    shell: {
        draw_bg.color: (shad_theme.color_background)
        flow: Overlay
    },
    widget: GallerySheetPage,
    page: sheet_page,
    title: "Sheet",
    subtitle: "Modal sheet overlays for contextual editing and supporting flows. Use ShadSheetRef::open/close plus `open_changed(actions)` for page-level control.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                ShadSectionHeader{ text: "Sides" }
                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 12.0

                    open_right_sheet_btn := ShadButton{text: "Right"}
                    open_left_sheet_btn := ShadButtonOutline{text: "Left"}
                    open_top_sheet_btn := ShadButtonOutline{text: "Top"}
                    open_bottom_sheet_btn := ShadButtonOutline{text: "Bottom"}
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep one ShadSheetRef per sheet variant the page controls, such as right, left, top, or bottom."}
        mod.widgets.GalleryActionFlowStep{text: "2. Trigger open(cx) and close(cx) from page buttons or row actions instead of reaching into the sheet internals."}
        mod.widgets.GalleryActionFlowStep{text: "3. Listen to `open_changed(actions)` if the surrounding page needs to react to visibility changes."}
        mod.widgets.GalleryActionFlowStep{text: "4. Internal dismiss controls and backdrop behavior remain inside the sheet component."}
    },
    after_root: {
        right_sheet := ShadSheet{
            side: "right"
            sheet_size: 360.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Edit workspace"}
                            description +: {text: "Quick edits that should not take you away from the current screen."}
                        }
                        body +: {
                            ShadField{
                                ShadFieldLabel{text: "Workspace name"}
                                ShadInput{empty_text: "Northwind"}
                            }
                            close_right_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }

        left_sheet := ShadSheet{
            side: "left"
            sheet_size: 360.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Project navigation"}
                            description +: {text: "Use left sheets for secondary navigation and drill-in menus."}
                        }
                        body +: {
                            ShadSidebarItem{text: "Roadmap"}
                            ShadSidebarItem{text: "Backlog"}
                            ShadSidebarItem{text: "Releases"}
                            close_left_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }

        top_sheet := ShadSheet{
            side: "top"
            sheet_size: 220.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Filter panel"}
                            description +: {text: "Top sheets work well for lightweight filters or global controls."}
                        }
                        body +: {
                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 12.0

                                ShadSelect{labels: ["All teams" "Design" "Engineering" "Ops"]}
                                ShadSelect{labels: ["Any status" "Open" "Blocked" "Done"]}
                            }
                            close_top_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }

        bottom_sheet := ShadSheet{
            side: "bottom"
            sheet_size: 220.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Activity feed"}
                            description +: {text: "Bottom sheets suit notifications and short-lived supporting context."}
                        }
                        body +: {
                            ShadFieldDescription{text: "Lucas mentioned your team in release notes."}
                            ShadFieldDescription{text: "A new build finished successfully 3 minutes ago."}
                            close_bottom_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GallerySheetPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GallerySheetPage {
    fn set_sheet_open(&mut self, cx: &mut Cx, path: &[LiveId], open: bool) {
        let sheet = self.view.shad_sheet(cx, path);
        if open {
            sheet.open(cx);
        } else {
            sheet.close(cx);
        }
    }
}

impl Widget for GallerySheetPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self
                .view
                .button(cx, ids!(open_right_sheet_btn))
                .clicked(actions)
            {
                self.set_sheet_open(cx, ids!(right_sheet), true);
            }
            if self
                .view
                .button(cx, ids!(open_left_sheet_btn))
                .clicked(actions)
            {
                self.set_sheet_open(cx, ids!(left_sheet), true);
            }
            if self
                .view
                .button(cx, ids!(open_top_sheet_btn))
                .clicked(actions)
            {
                self.set_sheet_open(cx, ids!(top_sheet), true);
            }
            if self
                .view
                .button(cx, ids!(open_bottom_sheet_btn))
                .clicked(actions)
            {
                self.set_sheet_open(cx, ids!(bottom_sheet), true);
            }
            for (path, button) in [
                (ids!(right_sheet), ids!(close_right_sheet_btn)),
                (ids!(left_sheet), ids!(close_left_sheet_btn)),
                (ids!(top_sheet), ids!(close_top_sheet_btn)),
                (ids!(bottom_sheet), ids!(close_bottom_sheet_btn)),
            ] {
                if self.view.button(cx, button).clicked(actions) {
                    self.set_sheet_open(cx, path, false);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
