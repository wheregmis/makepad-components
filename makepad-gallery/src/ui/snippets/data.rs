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
// if let Some(mut line) = self.ui.widget_flood(cx, ids!(line_chart)).borrow_mut::<LineChart>() {
//     line.set_data(vec![
//         DataPoint{x: 0.0, y: 18.0},
//         DataPoint{x: 1.0, y: 26.0},
//         DataPoint{x: 2.0, y: 31.0},
//     ]);
// }"#;
pub const TABLE_PREVIEW_CODE: &str = r#"tasks_table := ShadTable{
    caption: "Tasks"
    headers: ["Name" "Owner" "Status"]
    rows: []
}

// Controller example (Rust):
// let table = self.ui.shad_table(cx, ids!(tasks_table));
// table.set_rows(cx, vec![
//     vec!["API-204".into(), "Maya".into(), "Investigating".into()],
//     vec!["OBS-118".into(), "Noah".into(), "Scheduled".into()],
// ]);
//
// if let Some(row) = table.selection_changed(actions) {
//     self.selected_task = Some(row);
// }"#;
