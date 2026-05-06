pub const CHART_PREVIEW_CODE: &str = r#"// Dataset-switching buttons
View{
    width: Fit
    height: Fit
    flow: Right
    spacing: 8.0

    chart_growth_btn := ShadButton{
        text: "Growth data"
    }

    chart_ops_btn := ShadButtonOutline{
        text: "Ops data"
    }

    chart_revenue_btn := ShadButtonOutline{
        text: "Revenue data"
    }
}

chart_status := ShadFieldDescription{
    text: "Showing growth dataset."
}

ShadSectionHeader{ text: "Line" }
line_chart_demo := ShadLineChart{}

ShadSectionHeader{ text: "Area" }
area_chart_demo := ShadAreaChart{}

ShadSectionHeader{ text: "Bar" }
bar_chart_demo := ShadBarChart{}

// Controller example (Rust):
// use makepad_components::chart::{AreaChart, BarChart, DataPoint, LineChart};
//
// fn apply_dataset(&mut self, cx: &mut Cx) {
//     let points = vec![
//         DataPoint { x: 0.0, y: 18.0 },
//         DataPoint { x: 1.0, y: 26.0 },
//         DataPoint { x: 2.0, y: 31.0 },
//         DataPoint { x: 3.0, y: 42.0 },
//         DataPoint { x: 4.0, y: 54.0 },
//         DataPoint { x: 5.0, y: 61.0 },
//     ];
//
//     if let Some(mut chart) = self.view.widget_flood(cx, ids!(line_chart_demo)).borrow_mut::<LineChart>() {
//         chart.set_data(points.clone());
//     }
//     if let Some(mut chart) = self.view.widget_flood(cx, ids!(area_chart_demo)).borrow_mut::<AreaChart>() {
//         chart.set_data(points.clone());
//     }
//     if let Some(mut chart) = self.view.widget_flood(cx, ids!(bar_chart_demo)).borrow_mut::<BarChart>() {
//         chart.set_data(points);
//     }
//
//     self.view.label(cx, ids!(chart_status))
//         .set_text(cx, "Showing growth dataset.");
//     self.view.redraw(cx);
// }
//
// if self.view.button(cx, ids!(chart_growth_btn)).clicked(actions) {
//     self.dataset_index = 0;
//     self.apply_dataset(cx);
// }
// if self.view.button(cx, ids!(chart_ops_btn)).clicked(actions) {
//     self.dataset_index = 1;
//     self.apply_dataset(cx);
// }
// if self.view.button(cx, ids!(chart_revenue_btn)).clicked(actions) {
//     self.dataset_index = 2;
//     self.apply_dataset(cx);
// }"#;
pub const TABLE_PREVIEW_CODE: &str = r#"// Control bar
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

// Controller example (Rust):
// use makepad_components::table::ShadTableWidgetExt;
//
// let table = self.view.shad_table(cx, ids!(table_demo));
// table.set_headers(cx, vec!["Name".into(), "Role".into(), "Location".into(), "Status".into()]);
// table.set_rows(cx, vec![
//     vec!["Alice".into(), "Designer".into(), "Toronto".into(), "Active".into()],
//     vec!["Bob".into(), "Engineer".into(), "Remote".into(), "Active".into()],
// ]);
// table.set_selected_row(cx, None);
//
// // Virtualized 10k dataset:
// table.set_virtual_total_rows(cx, 10_000);
// table.set_virtual_window(cx, 0, vec![
//     vec!["JOB-00000".into(), "Batch".into(), "Remote".into(), "Running".into()],
//     vec!["JOB-00001".into(), "Realtime".into(), "Toronto".into(), "Running".into()],
// ]);
//
// if self.view.button(cx, ids!(table_clear_btn)).clicked(actions) {
//     self.view.shad_table(cx, ids!(table_demo)).set_selected_row(cx, None);
// }
//
// if let Some(row_index) = self.view.shad_table(cx, ids!(table_demo)).row_clicked(actions) {
//     self.view.label(cx, ids!(table_status))
//         .set_text(cx, &format!("Selected row: {}", row_index));
// }"#;
