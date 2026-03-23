pub const CHART_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0
    line_chart := ShadLineChart{}
    area_chart := ShadAreaChart{}
    bar_chart := ShadBarChart{}
}"#;
pub const TABLE_PREVIEW_CODE: &str = r#"tasks_table := ShadTable{
    caption: "Tasks"
    headers: ["Name" "Owner" "Status"]
    rows: []
}"#;
