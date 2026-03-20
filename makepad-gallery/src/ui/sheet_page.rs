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
    subtitle: "Contextual overlay panels where the page owns open and close decisions, while ShadSheet owns backdrop dismissal, Escape handling, side layout, and the modal shell.",
    divider: { ShadSeparator{} },
    preview_spacing: 16.0,
    preview: {
        sheet_demo_shell := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            sheet_status := ShadFieldDescription{
                width: Fill
                text: "No sheet open. Compare right and left for wider in-context panels, and top and bottom for lighter supporting flows."
            }

            ShadSurface{
                width: Fill
                height: Fit
                flow: Down
                spacing: 14.0
                padding: Inset{left: 18, right: 18, top: 18, bottom: 18}
                draw_bg +: {
                    color: (shad_theme.color_secondary)
                    border_size: 1.0
                    border_color: (shad_theme.color_outline_border)
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 8.0

                    ShadSectionHeader{ text: "Compare sheet sides" }
                    ShadFieldDescription{
                        width: Fill
                        text: "Right and left sheets use 360px widths for editing and drill-in navigation. Top and bottom sheets use 220px heights for filters, activity, and lightweight supporting context."
                    }
                    ShadFieldDescription{
                        width: Fill
                        text: "The page only calls open(cx) or close(cx) on each owned ShadSheetRef. The component still handles backdrop dismissal and Escape."
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 12.0

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSurface{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 8.0
                            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                            draw_bg +: {
                                color: (shad_theme.color_background)
                                border_size: 1.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            ShadLabel{text: "Right / 360px"}
                            ShadFieldDescription{
                                width: Fill
                                text: "Use for contextual editing flows where the main page should remain visible while users update details."
                            }
                            open_right_sheet_btn := ShadButton{text: "Open editor"}
                        }

                        ShadSurface{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 8.0
                            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                            draw_bg +: {
                                color: (shad_theme.color_background)
                                border_size: 1.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            ShadLabel{text: "Left / 360px"}
                            ShadFieldDescription{
                                width: Fill
                                text: "Use for secondary navigation or drill-in menus that should slide in without replacing the working view."
                            }
                            open_left_sheet_btn := ShadButtonOutline{text: "Open navigation"}
                        }
                    }

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSurface{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 8.0
                            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                            draw_bg +: {
                                color: (shad_theme.color_background)
                                border_size: 1.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            ShadLabel{text: "Top / 220px"}
                            ShadFieldDescription{
                                width: Fill
                                text: "Use for lightweight filter bars or global controls that need width more than vertical space."
                            }
                            open_top_sheet_btn := ShadButtonOutline{text: "Open filters"}
                        }

                        ShadSurface{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 8.0
                            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                            draw_bg +: {
                                color: (shad_theme.color_background)
                                border_size: 1.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            ShadLabel{text: "Bottom / 220px"}
                            ShadFieldDescription{
                                width: Fill
                                text: "Use for activity, playback queues, or supporting context that should stay close to the active surface."
                            }
                            open_bottom_sheet_btn := ShadButtonOutline{text: "Open activity"}
                        }
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep one owned ShadSheetRef per sheet variant your page can open, such as right, left, top, or bottom."}
        mod.widgets.GalleryActionFlowStep{text: "2. Call open(cx) and close(cx) from page buttons, table rows, or command handlers instead of reaching into the sheet internals."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `open_changed(actions)` when surrounding page copy or state needs to react to the sheet lifecycle."}
        mod.widgets.GalleryActionFlowStep{text: "4. Configure layout declaratively with side and sheet_size; backdrop dismissal and Escape remain inside ShadSheet."}
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
                            description +: {text: "Keep editing in context without leaving the dashboard you are already working in."}
                        }
                        body +: {
                            ShadField{
                                ShadFieldLabel{text: "Workspace name"}
                                ShadInput{empty_text: "Northwind"}
                            }
                            ShadField{
                                ShadFieldLabel{text: "Default team"}
                                ShadSelect{labels: ["Design" "Engineering" "Ops"]}
                            }
                        }
                        footer +: {
                            close_right_sheet_btn := ShadButtonOutline{text: "Cancel"}
                            save_right_sheet_btn := ShadButton{text: "Save changes"}
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
                            description +: {text: "Use left sheets for drill-in navigation and supporting menus that should stay near the current screen."}
                        }
                        body +: {
                            ShadFieldDescription{text: "Switch between planning surfaces without losing the main work area behind the sheet."}
                            ShadSidebarItem{text: "Roadmap"}
                            ShadSidebarItem{text: "Backlog"}
                            ShadSidebarItem{text: "Releases"}
                            ShadSidebarItem{text: "Postmortems"}
                        }
                        footer +: {
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
                            description +: {text: "Top sheets work well for lightweight filters and global controls that need width more than depth."}
                        }
                        body +: {
                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                ShadField{
                                    width: Fill
                                    ShadFieldLabel{text: "Team"}
                                    ShadSelect{labels: ["All teams" "Design" "Engineering" "Ops"]}
                                }

                                ShadField{
                                    width: Fill
                                    ShadFieldLabel{text: "Status"}
                                    ShadSelect{labels: ["Any status" "Open" "Blocked" "Done"]}
                                }
                            }
                        }
                        footer +: {
                            close_top_sheet_btn := ShadButtonOutline{text: "Close"}
                            apply_top_sheet_btn := ShadButton{text: "Apply filters"}
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
                            description +: {text: "Bottom sheets suit notifications, activity, and short-lived supporting context close to the active surface."}
                        }
                        body +: {
                            ShadLabel{text: "Latest updates"}
                            ShadFieldDescription{text: "Lucas mentioned your team in release notes."}
                            ShadFieldDescription{text: "A new build finished successfully 3 minutes ago."}
                            ShadFieldDescription{text: "Design review notes were added to Project Atlas."}
                        }
                        footer +: {
                            close_bottom_sheet_btn := ShadButtonOutline{text: "Dismiss"}
                            mark_bottom_sheet_btn := ShadButton{text: "Mark all read"}
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
    fn set_status(&mut self, cx: &mut Cx, text: &str) {
        self.view.label(cx, ids!(sheet_status)).set_text(cx, text);
    }

    fn set_sheet_open(&mut self, cx: &mut Cx, path: &[LiveId], open: bool) {
        let sheet = self.view.shad_sheet(cx, path);
        if open {
            sheet.open(cx);
        } else {
            sheet.close(cx);
        }
    }

    fn sync_status_from_sheet_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for (path, open_text) in [
            (
                ids!(right_sheet),
                "Right sheet open. This is the best fit for contextual editing and multi-field forms.",
            ),
            (
                ids!(left_sheet),
                "Left sheet open. Use this side for supporting navigation and drill-in menus.",
            ),
            (
                ids!(top_sheet),
                "Top sheet open. This side works well for wide, lightweight filters and controls.",
            ),
            (
                ids!(bottom_sheet),
                "Bottom sheet open. This side is useful for activity, playback queues, and supporting context.",
            ),
        ] {
            if let Some(is_open) = self.view.shad_sheet(cx, path).open_changed(actions) {
                if is_open {
                    self.set_status(cx, open_text);
                } else {
                    self.set_status(
                        cx,
                        "No sheet open. Compare right and left for wider in-context panels, and top and bottom for lighter supporting flows.",
                    );
                }
            }
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
                self.set_status(
                    cx,
                    "Right sheet open. This is the best fit for contextual editing and multi-field forms.",
                );
            }
            if self
                .view
                .button(cx, ids!(open_left_sheet_btn))
                .clicked(actions)
            {
                self.set_sheet_open(cx, ids!(left_sheet), true);
                self.set_status(
                    cx,
                    "Left sheet open. Use this side for supporting navigation and drill-in menus.",
                );
            }
            if self
                .view
                .button(cx, ids!(open_top_sheet_btn))
                .clicked(actions)
            {
                self.set_sheet_open(cx, ids!(top_sheet), true);
                self.set_status(
                    cx,
                    "Top sheet open. This side works well for wide, lightweight filters and controls.",
                );
            }
            if self
                .view
                .button(cx, ids!(open_bottom_sheet_btn))
                .clicked(actions)
            {
                self.set_sheet_open(cx, ids!(bottom_sheet), true);
                self.set_status(
                    cx,
                    "Bottom sheet open. This side is useful for activity, playback queues, and supporting context.",
                );
            }
            for (path, button) in [
                (ids!(right_sheet), ids!(close_right_sheet_btn)),
                (ids!(right_sheet), ids!(save_right_sheet_btn)),
                (ids!(left_sheet), ids!(close_left_sheet_btn)),
                (ids!(top_sheet), ids!(close_top_sheet_btn)),
                (ids!(top_sheet), ids!(apply_top_sheet_btn)),
                (ids!(bottom_sheet), ids!(close_bottom_sheet_btn)),
                (ids!(bottom_sheet), ids!(mark_bottom_sheet_btn)),
            ] {
                if self.view.button(cx, button).clicked(actions) {
                    self.set_sheet_open(cx, path, false);
                    self.set_status(
                        cx,
                        "No sheet open. Compare right and left for wider in-context panels, and top and bottom for lighter supporting flows.",
                    );
                }
            }

            self.sync_status_from_sheet_actions(cx, actions);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
