use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::table::ShadTableWidgetExt;

gallery_stateful_page_shell! {
    widget: GalleryTablePage,
    page: table_page,
    title: "Table",
    subtitle: "A reusable app-owned table shell with a styled header, scrollable rows, typed row-selection events, and virtual row-window APIs for huge datasets.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            table_team_btn := ShadButton{
                text: "Team roster"
            }

            table_ops_btn := ShadButtonOutline{
                text: "Ops queue"
            }

            table_virtual_btn := ShadButtonSecondary{
                text: "Virtualized 10k"
            }

            table_prev_btn := ShadButtonGhost{
                text: "Prev window"
            }

            table_next_btn := ShadButtonGhost{
                text: "Next window"
            }

            table_clear_btn := ShadButtonGhost{
                text: "Clear selection"
            }
        }

        table_status := ShadFieldDescription{
            text: "Showing team roster. Selected row: none."
        }

        table_demo := ShadTable{
            caption: "Team roster"
            headers: ["Name" "Role" "Location" "Status"]
            rows: []
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. For regular datasets, keep headers and rows in app state, then call `set_headers(cx, ...)` and `set_rows(cx, ...)` when data changes."}
        mod.widgets.GalleryActionFlowStep{text: "2. Listen to `row_clicked(actions)`, `selection_changed(actions)`, or `selection_cleared(actions)` when surrounding details panes need to react."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `set_selected_row(cx, ...)` when other controls should move or clear the current selection."}
        mod.widgets.GalleryActionFlowStep{text: "4. For huge data, call `set_virtual_total_rows(cx, ...)` and then `set_virtual_window(cx, start, rows)` to render only a loaded window."}
    },
}

#[derive(Script, Widget)]
pub struct GalleryTablePage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    dataset_index: usize,
    #[rust]
    virtual_mode: bool,
    #[rust]
    virtual_start: usize,
}

impl ScriptHook for GalleryTablePage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.dataset_index = 0;
            self.apply_dataset(cx);
        });
    }
}

impl GalleryTablePage {
    const VIRTUAL_TOTAL: usize = 10_000;
    const VIRTUAL_WINDOW_SIZE: usize = 32;

    fn make_virtual_rows(start: usize, count: usize) -> Vec<Vec<String>> {
        (start..start.saturating_add(count))
            .map(|index| {
                row(
                    &format!("JOB-{index:05}"),
                    if index & 1 == 0 { "Batch" } else { "Realtime" },
                    if index % 3 == 0 { "Remote" } else { "Toronto" },
                    if index % 5 == 0 {
                        "Investigating"
                    } else {
                        "Running"
                    },
                )
            })
            .collect()
    }

    fn apply_dataset(&mut self, cx: &mut Cx) {
        self.virtual_mode = false;
        let (title, headers, rows) = table_dataset(self.dataset_index);

        let table = self.view.shad_table(cx, ids!(table_demo));
        table.set_caption(cx, title.to_string());
        table.set_headers(cx, headers);
        table.set_rows(cx, rows);
        table.set_selected_row(cx, None);

        self.view
            .label(cx, ids!(table_status))
            .set_text(cx, &format!("Showing {title}. Selected row: none."));
        self.view.redraw(cx);
    }

    fn apply_virtual_dataset(&mut self, cx: &mut Cx) {
        self.virtual_mode = true;
        self.sync_virtual_window(cx, 0, true);
    }

    fn sync_virtual_window(&mut self, cx: &mut Cx, start: usize, clear_selection: bool) {
        self.virtual_start = start.min(Self::VIRTUAL_TOTAL.saturating_sub(1));
        let headers = vec![
            "Task".to_string(),
            "Queue".to_string(),
            "Region".to_string(),
            "Status".to_string(),
        ];
        let rows = Self::make_virtual_rows(self.virtual_start, Self::VIRTUAL_WINDOW_SIZE);

        let table = self.view.shad_table(cx, ids!(table_demo));
        table.set_caption(cx, "Virtualized 10k".to_string());
        table.set_headers(cx, headers);
        table.set_virtual_total_rows(cx, Self::VIRTUAL_TOTAL);
        table.set_virtual_window(cx, self.virtual_start, rows);
        if clear_selection {
            table.set_selected_row(cx, None);
        }
        let remaining = Self::VIRTUAL_TOTAL.saturating_sub(self.virtual_start);
        let window_len = remaining.min(Self::VIRTUAL_WINDOW_SIZE).max(1);
        let end = self
            .virtual_start
            .saturating_add(window_len.saturating_sub(1));
        self.view.label(cx, ids!(table_status)).set_text(
            cx,
            &format!(
                "Showing virtual jobs {start}..{end} of {}. Selected row: none.",
                Self::VIRTUAL_TOTAL,
                start = self.virtual_start,
                end = end
            ),
        );
        self.view.redraw(cx);
    }

    fn selected_primary_cell(&self, selected_row: Option<usize>) -> String {
        selected_row
            .and_then(|index| {
                if self.virtual_mode {
                    if index >= Self::VIRTUAL_TOTAL {
                        return None;
                    }
                    return Some(format!("JOB-{index:05}"));
                }

                let rows = table_dataset(self.dataset_index).2;
                rows.get(index).and_then(|row| row.first()).cloned()
            })
            .unwrap_or_else(|| "none".to_string())
    }

    fn sync_status(&self, cx: &mut Cx, source: Option<usize>) {
        let table = self.view.shad_table(cx, ids!(table_demo));
        let title = if self.virtual_mode {
            "virtual jobs"
        } else if self.dataset_index == 0 {
            "team roster"
        } else {
            "ops queue"
        };
        let selected_text = self.selected_primary_cell(table.selected_row());
        let prefix = if source.is_some() {
            "Clicked"
        } else {
            "Selected row"
        };
        self.view
            .label(cx, ids!(table_status))
            .set_text(cx, &format!("Showing {title}. {prefix}: {selected_text}."));
    }
}

impl Widget for GalleryTablePage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self.view.button(cx, ids!(table_team_btn)).clicked(actions) {
                self.dataset_index = 0;
                self.apply_dataset(cx);
                return;
            }
            if self.view.button(cx, ids!(table_ops_btn)).clicked(actions) {
                self.dataset_index = 1;
                self.apply_dataset(cx);
                return;
            }
            if self
                .view
                .button(cx, ids!(table_virtual_btn))
                .clicked(actions)
            {
                self.apply_virtual_dataset(cx);
                return;
            }
            if self.view.button(cx, ids!(table_prev_btn)).clicked(actions) {
                if self.virtual_mode {
                    let start = self.virtual_start.saturating_sub(Self::VIRTUAL_WINDOW_SIZE);
                    self.sync_virtual_window(cx, start, true);
                }
                return;
            }
            if self.view.button(cx, ids!(table_next_btn)).clicked(actions) {
                if self.virtual_mode {
                    let max_start = Self::VIRTUAL_TOTAL.saturating_sub(1);
                    let start = (self.virtual_start + Self::VIRTUAL_WINDOW_SIZE).min(max_start);
                    self.sync_virtual_window(cx, start, true);
                }
                return;
            }
            if self.view.button(cx, ids!(table_clear_btn)).clicked(actions) {
                self.view
                    .shad_table(cx, ids!(table_demo))
                    .set_selected_row(cx, None);
                self.sync_status(cx, None);
                return;
            }

            let table = self.view.shad_table(cx, ids!(table_demo));
            if let Some(start) = table.virtual_window_request(actions) {
                self.sync_virtual_window(cx, start, false);
                return;
            }
            if let Some(index) = table.row_clicked(actions) {
                self.sync_status(cx, Some(index));
            } else if table.selection_changed(actions).is_some() {
                self.sync_status(cx, None);
            } else if table.selection_cleared(actions) {
                self.sync_status(cx, None);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

fn table_dataset(index: usize) -> (&'static str, Vec<String>, Vec<Vec<String>>) {
    let headers = vec![
        "Name".to_string(),
        "Role".to_string(),
        "Location".to_string(),
        "Status".to_string(),
    ];
    match index {
        1 => (
            "ops queue",
            headers,
            vec![
                row("API-204", "Incident", "Montreal", "Investigating"),
                row("OBS-118", "Migration", "Remote", "Scheduled"),
                row("WEB-311", "Release", "Toronto", "Ready"),
                row("SEC-091", "Patch", "Remote", "Blocked"),
            ],
        ),
        _ => (
            "team roster",
            headers,
            vec![
                row("Maya Chen", "Design", "Toronto", "Online"),
                row("Noah Patel", "Frontend", "New York", "In review"),
                row("Ava Singh", "Platform", "Remote", "Deploying"),
                row("Luca Martin", "Support", "Berlin", "On call"),
            ],
        ),
    }
}

fn row(name: &str, role: &str, location: &str, status: &str) -> Vec<String> {
    vec![
        name.to_string(),
        role.to_string(),
        location.to_string(),
        status.to_string(),
    ]
}
