use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::table::ShadTableWidgetExt;

gallery_stateful_page_shell! {
    widget: GalleryTablePage,
    page: table_page,
    title: "Table",
    subtitle: "A reusable app-owned table shell with a styled header, scrollable rows, and typed row-selection events. Keep rows in page state and push them into the table whenever filters, sorting, or tabs change.",
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
        mod.widgets.GalleryActionFlowStep{text: "1. Keep headers and rows in app state, then call `set_headers(cx, ...)` and `set_rows(cx, ...)` when the dataset changes."}
        mod.widgets.GalleryActionFlowStep{text: "2. Listen to `row_clicked(actions)` or `selection_changed(actions)` when surrounding details panes need to react."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `set_selected_row(cx, ...)` when other controls should move the current selection."}
        mod.widgets.GalleryActionFlowStep{text: "4. Sorting, filtering, and pagination stay in app code; this table only renders the rows you provide."}
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
    current_rows: Vec<Vec<String>>,
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
    fn apply_dataset(&mut self, cx: &mut Cx) {
        let (title, headers, rows) = table_dataset(self.dataset_index);
        self.current_rows = rows.clone();

        let table = self.view.shad_table(cx, ids!(table_demo));
        table.set_headers(cx, headers);
        table.set_rows(cx, rows);
        table.set_selected_row(cx, None);

        self.view
            .label(cx, ids!(table_status))
            .set_text(cx, &format!("Showing {title}. Selected row: none."));
        self.view.redraw(cx);
    }

    fn sync_status(&self, cx: &mut Cx, source: Option<usize>) {
        let table = self.view.shad_table(cx, ids!(table_demo));
        let title = if self.dataset_index == 0 {
            "team roster"
        } else {
            "ops queue"
        };
        let selected_text = table
            .selected_row()
            .and_then(|index| {
                self.current_rows
                    .get(index)
                    .and_then(|row| row.first())
                    .cloned()
            })
            .unwrap_or_else(|| "none".to_string());
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
            if self.view.button(cx, ids!(table_clear_btn)).clicked(actions) {
                self.view
                    .shad_table(cx, ids!(table_demo))
                    .set_selected_row(cx, None);
                self.sync_status(cx, None);
                return;
            }

            let table = self.view.shad_table(cx, ids!(table_demo));
            if let Some(index) = table.row_clicked(actions) {
                self.sync_status(cx, Some(index));
            } else if table.selection_changed(actions).is_some() {
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
