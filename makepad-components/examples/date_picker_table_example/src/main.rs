use makepad_components::calendar::ShadDate;
use makepad_components::date_picker::ShadDatePickerWidgetRefExt;
use makepad_components::table::ShadTableWidgetRefExt;
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(980, 720)
                window.title: "Date Picker + Table Example"
                pass +: {
                    clear_color: (shad_theme.color_background)
                }
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    event_order: Down
                    spacing: 16.0
                    padding: Inset{left: 24.0, right: 24.0, top: 24.0, bottom: 24.0}

                    title := Label{
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 28.0
                        text: "Date Picker + Table Example"
                    }

                    subtitle := ShadFieldDescription{
                        width: Fill
                        text: "Standalone repro showing ShadDatePicker driving ShadTable through set_rows(cx, ...)."
                    }

                    controls := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        button_row := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 12.0
                            align: Align{y: 0.5}

                            deadline_picker := ShadDatePicker{
                                value: "2026-03-13"
                            }

                            set_march_btn := ShadButton{
                                text: "Set 2026-03-13"
                            }

                            set_april_btn := ShadButtonOutline{
                                text: "Set 2026-04-01"
                            }

                            clear_btn := ShadButtonGhost{
                                text: "Clear"
                            }
                        }

                        status := ShadFieldDescription{
                            width: Fill
                            text: "Waiting for initial sync..."
                        }
                    }

                    tasks_table := ShadTable{
                        width: Fill
                        height: Fit
                        caption: "Tasks"
                        empty_message: "Pick a date to load schedule rows."
                        headers: ["Time" "Task" "Status"]
                        rows: []
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl App {
    const DEFAULT_DATE: ShadDate = ShadDate {
        year: 2026,
        month: 3,
        day: 13,
    };
    const SECONDARY_DATE: ShadDate = ShadDate {
        year: 2026,
        month: 4,
        day: 1,
    };

    fn headers() -> Vec<String> {
        vec!["Time".to_string(), "Task".to_string(), "Status".to_string()]
    }

    fn caption_for_date(date: Option<ShadDate>) -> String {
        date.map(|value| format!("Tasks for {}", value.format_iso()))
            .unwrap_or_else(|| "Tasks".to_string())
    }

    fn rows_for_date(date: Option<ShadDate>) -> Vec<Vec<String>> {
        match date {
            Some(Self::DEFAULT_DATE) => vec![
                row("09:00", "Reproduce picker -> table refresh", "Open"),
                row("13:30", "Verify set_rows applies new rows", "Ready"),
                row("16:00", "Review final behavior", "Scheduled"),
            ],
            Some(Self::SECONDARY_DATE) => vec![
                row("08:45", "Ops handoff", "Queued"),
                row("11:15", "Regression pass", "Running"),
                row("15:30", "Ship sample fix", "Pending"),
            ],
            Some(value) => {
                let iso = value.format_iso();
                vec![
                    row("10:00", &format!("Follow up for {iso}"), "Queued"),
                    row("14:30", "Verify set_rows redraw", "Pending"),
                ]
            }
            None => Vec::new(),
        }
    }

    fn sync_table_for_picker(&self, cx: &mut Cx) {
        let picker = self.ui.shad_date_picker(cx, ids!(deadline_picker));
        let table = self.ui.shad_table(cx, ids!(tasks_table));
        let selected = picker.value();

        table.set_headers(cx, Self::headers());
        table.set_caption(cx, Self::caption_for_date(selected));
        table.set_rows(cx, Self::rows_for_date(selected));
        table.set_selected_row(cx, None);

        self.sync_status(cx);
    }

    fn sync_status(&self, cx: &mut Cx) {
        let picker = self.ui.shad_date_picker(cx, ids!(deadline_picker));
        let table = self.ui.shad_table(cx, ids!(tasks_table));
        let selected = picker.value();
        let rows = Self::rows_for_date(selected);
        let selected_task = table
            .selected_row()
            .and_then(|index| rows.get(index))
            .and_then(|row| row.get(1))
            .cloned()
            .unwrap_or_else(|| "none".to_string());
        let date_text = selected
            .map(|value| value.format_iso())
            .unwrap_or_else(|| "none".to_string());
        let open_text = if picker.is_open() { "open" } else { "closed" };

        self.ui.label(cx, ids!(status)).set_text(
            cx,
            &format!(
                "Date: {date_text}. Picker: {open_text}. Rows via set_rows: {}. Selected task: {selected_task}.",
                rows.len()
            ),
        );
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let picker = self.ui.shad_date_picker(cx, ids!(deadline_picker));
        if self.ui.button(cx, ids!(set_march_btn)).clicked(actions) {
            picker.set_value(cx, Some(Self::DEFAULT_DATE));
            self.sync_table_for_picker(cx);
            return;
        }
        if self.ui.button(cx, ids!(set_april_btn)).clicked(actions) {
            picker.set_value(cx, Some(Self::SECONDARY_DATE));
            self.sync_table_for_picker(cx);
            return;
        }
        if self.ui.button(cx, ids!(clear_btn)).clicked(actions) {
            picker.clear(cx);
            self.sync_table_for_picker(cx);
            return;
        }

        let table = self.ui.shad_table(cx, ids!(tasks_table));
        if picker.changed(actions).is_some() {
            self.sync_table_for_picker(cx);
        } else if picker.open_changed(actions).is_some()
            || table.row_clicked(actions).is_some()
            || table.selection_changed(actions).is_some()
        {
            self.sync_status(cx);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_components::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if matches!(event, Event::Startup) {
            self.sync_table_for_picker(cx);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn row(time: &str, task: &str, status: &str) -> Vec<String> {
    vec![time.to_string(), task.to_string(), status.to_string()]
}
