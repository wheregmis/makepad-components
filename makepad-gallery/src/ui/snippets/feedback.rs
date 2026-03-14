pub const ALERT_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    mod.widgets.ShadAlert{\n        width: Fill\n        icon := mod.widgets.ShadAlertIcon{}\n        content := mod.widgets.ShadAlertContent{\n            title := mod.widgets.ShadAlertTitle{text: \"Heads up!\"}\n            description := mod.widgets.ShadAlertDescription{text: \"Action complete.\"}\n        }\n    }\n    mod.widgets.ShadAlertDestructive{\n        width: Fill\n        icon := mod.widgets.ShadAlertDestructiveIcon{}\n        content := mod.widgets.ShadAlertContent{\n            title := mod.widgets.ShadAlertDestructiveTitle{text: \"Error\"}\n            description := mod.widgets.ShadAlertDescription{text: \"Your session expired. Please sign in again.\"}\n        }\n    }\n}";
pub const PROGRESS_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    ShadProgress33{}\n    ShadProgress66{}\n    ShadProgressFull{}\n    ShadProgressIndeterminate{}\n}";
pub const SKELETON_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Right\n    spacing: 12.0\n    align: Align{x: 0.0, y: 0.5}\n\n    ShadSkeleton{\n        width: 48\n        height: 48\n        draw_bg.border_radius: 24.0\n    }\n\n    View{\n        width: Fit\n        height: Fit\n        flow: Down\n        spacing: 8.0\n\n        ShadSkeleton{\n            width: 200\n            height: 16\n        }\n        ShadSkeleton{\n            width: 150\n            height: 16\n        }\n    }\n}";
pub const SPINNER_PREVIEW_CODE: &str =
    "ShadSpinner{}\n\n// 24×24 circular loading indicator. Use for async/loading states.";
pub const SONNER_PREVIEW_CODE: &str = r#"// Basic toast
View{
    flow: Right
    spacing: 8.0
    ShadButton{text: "Event created"}
    ShadButton{text: "Toast with description"}
}
ShadSonner{open: false}
ShadSonnerWithDescription{open: false}

// Toast with check icon + close button
ShadButton{text: "Show toast with close"}
ShadSonnerWithClose{open: false}

// Controller example (Rust):
// let toast = self.ui.shad_sonner(cx, ids!(toast_event));
//
// if self.ui.button(cx, ids!(toast_event_btn)).clicked(actions) {
//     toast.open(cx);
// }
//
// if let Some(is_open) = toast.open_changed(actions) {
//     log!("Toast open: {}", is_open);
// }"#;
