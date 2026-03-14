use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::calendar::{ShadCalendarWidgetExt, ShadDate};
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryCalendarPage,
    page: calendar_page,
    title: "Calendar",
    subtitle: "A lightweight single-date calendar with controlled month navigation. Read selections with `changed(actions)` and drive the visible month or selected value from external page state when needed.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Controlled date" }

        calendar_demo := ShadCalendar{
            value: "2026-03-13"
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            calendar_prev_btn := ShadButtonOutline{
                text: "Previous month"
            }

            calendar_today_btn := ShadButton{
                text: "Jump to March 13"
            }

            calendar_next_btn := ShadButtonOutline{
                text: "Next month"
            }

            calendar_clear_btn := ShadButtonGhost{
                text: "Clear"
            }
        }

        calendar_status := ShadFieldDescription{
            text: "Selected: 2026-03-13. Visible month: 2026-03."
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Use `changed(actions)` when the page cares about the chosen date."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use `set_value(cx, Some(date))` to restore a saved date or select one from another control."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `prev_month(cx)`, `next_month(cx)`, or `set_month(cx, year, month)` when outer buttons or shortcuts should steer the calendar."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use `clear(cx)` when the field above or the surrounding form should return to an unset state."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryCalendarPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryCalendarPage {
    fn sync_status(&self, cx: &mut Cx) {
        let calendar = self.view.shad_calendar(cx, ids!(calendar_demo));
        let selected = calendar
            .value()
            .map(|value| value.format_iso())
            .unwrap_or_else(|| "None".to_string());
        let visible = calendar
            .visible_month()
            .map(|(year, month)| format!("{year:04}-{month:02}"))
            .unwrap_or_else(|| "Unknown".to_string());
        self.view.label(cx, ids!(calendar_status)).set_text(
            cx,
            &format!("Selected: {selected}. Visible month: {visible}."),
        );
    }
}

impl Widget for GalleryCalendarPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let calendar = self.view.shad_calendar(cx, ids!(calendar_demo));

            if self
                .view
                .button(cx, ids!(calendar_prev_btn))
                .clicked(actions)
            {
                calendar.prev_month(cx);
                self.sync_status(cx);
                return;
            }
            if self
                .view
                .button(cx, ids!(calendar_today_btn))
                .clicked(actions)
            {
                calendar.set_value(
                    cx,
                    Some(ShadDate {
                        year: 2026,
                        month: 3,
                        day: 13,
                    }),
                );
                self.sync_status(cx);
                return;
            }
            if self
                .view
                .button(cx, ids!(calendar_next_btn))
                .clicked(actions)
            {
                calendar.next_month(cx);
                self.sync_status(cx);
                return;
            }
            if self
                .view
                .button(cx, ids!(calendar_clear_btn))
                .clicked(actions)
            {
                calendar.clear(cx);
                self.sync_status(cx);
                return;
            }

            if calendar.changed(actions).is_some() {
                self.sync_status(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
