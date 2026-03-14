use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::calendar::ShadDate;
use makepad_components::date_picker::ShadDatePickerWidgetExt;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryDatePickerPage,
    page: date_picker_page,
    title: "Date Picker",
    subtitle: "A field-like date control built from `ShadPopover` and `ShadCalendar`. Keep the chosen date in page state, react to `changed(actions)`, and use `set_open` or `set_value` when other UI should control it.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Controlled picker" }

        date_picker_demo := ShadDatePicker{
            value: "2026-03-13"
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            date_picker_open_btn := ShadButtonOutline{
                text: "Open"
            }

            date_picker_deadline_btn := ShadButton{
                text: "Set deadline"
            }

            date_picker_clear_btn := ShadButtonGhost{
                text: "Clear"
            }
        }

        date_picker_status := ShadFieldDescription{
            text: "Selected: 2026-03-13. Picker is closed."
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Listen to `changed(actions)` when the selected date should update filters, due dates, or query params."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use `open_changed(actions)` when the surrounding page needs to react to picker visibility."}
        mod.widgets.GalleryActionFlowStep{text: "3. Drive the field from outside with `set_value(cx, Some(date))`, or reset it with `clear(cx)`."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use `set_open(cx, true)` when keyboard shortcuts or secondary buttons should open the calendar directly."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryDatePickerPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryDatePickerPage {
    fn sync_status(&self, cx: &mut Cx) {
        let picker = self.view.shad_date_picker(cx, ids!(date_picker_demo));
        let selected = picker
            .value()
            .map(|value| value.format_iso())
            .unwrap_or_else(|| "None".to_string());
        let open_state = if picker.is_open() { "open" } else { "closed" };
        self.view.label(cx, ids!(date_picker_status)).set_text(
            cx,
            &format!("Selected: {selected}. Picker is {open_state}."),
        );
    }
}

impl Widget for GalleryDatePickerPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let picker = self.view.shad_date_picker(cx, ids!(date_picker_demo));

            if self
                .view
                .button(cx, ids!(date_picker_open_btn))
                .clicked(actions)
            {
                picker.set_open(cx, true);
                self.sync_status(cx);
                return;
            }
            if self
                .view
                .button(cx, ids!(date_picker_deadline_btn))
                .clicked(actions)
            {
                picker.set_value(
                    cx,
                    Some(ShadDate {
                        year: 2026,
                        month: 4,
                        day: 1,
                    }),
                );
                self.sync_status(cx);
                return;
            }
            if self
                .view
                .button(cx, ids!(date_picker_clear_btn))
                .clicked(actions)
            {
                picker.clear(cx);
                self.sync_status(cx);
                return;
            }

            if picker.changed(actions).is_some() || picker.open_changed(actions).is_some() {
                self.sync_status(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
