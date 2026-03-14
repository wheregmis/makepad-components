use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::pagination::ShadPaginationWidgetExt;

gallery_stateful_page_shell! {
    widget: GalleryPaginationPage,
    page: pagination_page,
    title: "Pagination",
    subtitle: "Stateful page controls with previous/next actions, compact ranges, and page-change events. Keep the selected page in app state, then drive data fetching or visible rows from that value.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Controlled pagination" }

        pagination_demo := ShadPagination{
            current_page: 5
            page_count: 12
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            prev_external_btn := ShadButtonOutline{
                text: "Previous"
            }

            next_external_btn := ShadButtonOutline{
                text: "Next"
            }

            jump_last_btn := ShadButtonGhost{
                text: "Jump to last"
            }
        }

        pagination_status := ShadFieldDescription{
            text: "Current page: 5 of 12"
        }

        ShadHr{}

        ShadSectionHeader{ text: "Compact range" }

        pagination_compact := ShadPagination{
            current_page: 21
            page_count: 42
            max_visible_pages: 5
        }

        pagination_compact_status := ShadFieldDescription{
            text: "Current page: 21 of 42"
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Treat the current page as app or page state. `ShadPagination` only emits page changes, it does not own your data source."}
        mod.widgets.GalleryActionFlowStep{text: "2. Listen to `changed(actions)` when a user clicks a number, Previous, or Next."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `set_page(cx, ...)`, `next(cx)`, or `prev(cx)` when external buttons or keyboard shortcuts should drive the control."}
        mod.widgets.GalleryActionFlowStep{text: "4. Recompute the visible rows or fetch the next slice from your backend using the selected page."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryPaginationPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryPaginationPage {
    fn sync_status_labels(&self, cx: &mut Cx) {
        let pagination = self.view.shad_pagination(cx, ids!(pagination_demo));
        self.view.label(cx, ids!(pagination_status)).set_text(
            cx,
            &format!(
                "Current page: {} of {}",
                pagination.page(),
                pagination.page_count()
            ),
        );

        let compact = self.view.shad_pagination(cx, ids!(pagination_compact));
        self.view
            .label(cx, ids!(pagination_compact_status))
            .set_text(
                cx,
                &format!(
                    "Current page: {} of {}",
                    compact.page(),
                    compact.page_count()
                ),
            );
    }
}

impl Widget for GalleryPaginationPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let pagination = self.view.shad_pagination(cx, ids!(pagination_demo));

            if self
                .view
                .button(cx, ids!(prev_external_btn))
                .clicked(actions)
            {
                pagination.prev(cx);
                self.sync_status_labels(cx);
                return;
            }
            if self
                .view
                .button(cx, ids!(next_external_btn))
                .clicked(actions)
            {
                pagination.next(cx);
                self.sync_status_labels(cx);
                return;
            }
            if self.view.button(cx, ids!(jump_last_btn)).clicked(actions) {
                pagination.set_page(cx, pagination.page_count());
                self.sync_status_labels(cx);
                return;
            }

            if pagination.changed(actions).is_some()
                || self
                    .view
                    .shad_pagination(cx, ids!(pagination_compact))
                    .changed(actions)
                    .is_some()
            {
                self.sync_status_labels(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
