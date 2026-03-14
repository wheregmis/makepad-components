use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::popover::ShadPopoverWidgetExt;

gallery_stateful_page_shell! {
    widget: GalleryPopoverPage,
    page: popover_page,
    title: "Popover",
    subtitle: "Anchored overlays for compact, contextual UI. Use `set_open` / `open` / `close` on the widget ref, and reach into the popup content through `content_widget()` when the popup body has its own controls.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Basic" }

        profile_popover := ShadPopover{
            side: "bottom"
            align: "start"

            trigger := ShadButtonOutline{
                text: "Open profile editor"
            }

            content: ShadPopoverContent{
                title := ShadSectionHeader{
                    text: "Edit profile"
                }

                description := ShadFieldDescription{
                    text: "Quick edits belong in a popover when the current page context should remain visible."
                }

                footer := View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 8.0
                    margin: Inset{top: 8}

                    popover_close_btn := ShadButtonGhost{
                        text: "Cancel"
                    }

                    popover_apply_btn := ShadButton{
                        text: "Save"
                    }
                }
            }
        }

        popover_status := ShadFieldDescription{
            text: "Popover is closed."
        }

        ShadHr{}

        ShadSectionHeader{ text: "Top / End aligned" }

        help_popover := ShadPopover{
            side: "top"
            align: "end"

            trigger := ShadButtonGhost{
                text: "Open top-end help"
            }

            content: ShadPopoverContent{
                width: 280

                title := ShadSectionHeader{
                    text: "Keyboard shortcuts"
                }

                description := ShadFieldDescription{
                    text: "Use popovers for compact help, profile cards, or lightweight editing flows that should stay attached to a trigger."
                }
            }
        }

        help_popover_status := ShadFieldDescription{
            text: "Help popover is closed."
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep the trigger in normal layout, and let `ShadPopover` render the popup body into an overlay draw list when open."}
        mod.widgets.GalleryActionFlowStep{text: "2. The widget flips top/bottom or left/right automatically when there is not enough room on the preferred side."}
        mod.widgets.GalleryActionFlowStep{text: "3. Reach into popup controls with `content_widget()` if buttons or fields inside the popover should update page state."}
        mod.widgets.GalleryActionFlowStep{text: "4. Close on outside click, Escape, back gesture, or explicitly from a button inside the popup."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryPopoverPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryPopoverPage {
    fn sync_status_labels(&self, cx: &mut Cx) {
        let profile = self.view.shad_popover(cx, ids!(profile_popover));
        self.view.label(cx, ids!(popover_status)).set_text(
            cx,
            if profile.is_open() {
                "Popover is open."
            } else {
                "Popover is closed."
            },
        );

        let help = self.view.shad_popover(cx, ids!(help_popover));
        self.view.label(cx, ids!(help_popover_status)).set_text(
            cx,
            if help.is_open() {
                "Help popover is open."
            } else {
                "Help popover is closed."
            },
        );
    }
}

impl Widget for GalleryPopoverPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let profile = self.view.shad_popover(cx, ids!(profile_popover));
            let profile_content = profile.content_widget();

            if profile_content
                .button(cx, ids!(popover_close_btn))
                .clicked(actions)
            {
                profile.close(cx);
                self.sync_status_labels(cx);
                return;
            }

            if profile_content
                .button(cx, ids!(popover_apply_btn))
                .clicked(actions)
            {
                profile.close(cx);
                self.view
                    .label(cx, ids!(popover_status))
                    .set_text(cx, "Saved changes and closed the popover.");
                return;
            }

            let help = self.view.shad_popover(cx, ids!(help_popover));
            if profile.open_changed(actions).is_some() || help.open_changed(actions).is_some() {
                self.sync_status_labels(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
