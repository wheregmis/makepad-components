pub const CHART_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0
    line_chart := ShadLineChart{}
    area_chart := ShadAreaChart{}
    bar_chart := ShadBarChart{}
}

// Controller example (Rust):
// self.ui.shad_line_chart(cx, ids!(line_chart)).set_data(vec![
//     DataPoint{x: 0.0, y: 18.0},
//     DataPoint{x: 1.0, y: 26.0},
//     DataPoint{x: 2.0, y: 31.0},
// ]);"#;
pub const TABLE_PREVIEW_CODE: &str = r#"tasks_table := ShadTable{
    viewport_height: 240.0
    caption: "Tasks"
    headers: ["Name" "Owner" "Status"]
    rows: []
}

View{
    width: Fit
    height: Fit
    flow: Right
    spacing: 8.0

    tasks_prev_btn := ShadButton{text: "Prev"}
    tasks_next_btn := ShadButton{text: "Next"}
}

// Controller example (Rust):
// let table = self.ui.shad_table(cx, ids!(tasks_table));
// table.set_rows(cx, vec![
//     vec!["API-204".into(), "Maya".into(), "Investigating".into()],
//     vec!["OBS-118".into(), "Noah".into(), "Scheduled".into()],
// ]);
// For huge datasets (virtualized window):
// table.set_virtual_total_rows(cx, 10_000);
// table.set_virtual_window(cx, 960, vec![
//     vec!["JOB-00960".into(), "Batch".into(), "Running".into()],
//     vec!["JOB-00961".into(), "Realtime".into(), "Running".into()],
// ]);
//
// if let Some(row) = table.selection_changed(actions) {
//     self.selected_task = Some(row);
// } else if table.selection_cleared(actions) {
//     self.selected_task = None;
// }"#;
