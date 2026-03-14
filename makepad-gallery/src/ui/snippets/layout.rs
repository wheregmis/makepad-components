pub const ASPECT_RATIO_PREVIEW_CODE: &str = "ShadAspectRatio{\n    width: Fill\n    ratio: 1.7777777778\n    RoundedView{\n        width: Fill\n        height: Fill\n        draw_bg.color: (shad_theme.color_secondary)\n    }\n}";
pub const RESIZABLE_PREVIEW_CODE: &str = r#"horizontal_split := ShadResizable{
    axis: SplitterAxis.Horizontal
    align: SplitterAlign.FromA(180.0)
    a: View{width: Fill height: Fill}
    b: View{width: Fill height: Fill}
}

// Controller example (Rust):
// let split = self.view.splitter(cx, ids!(horizontal_split));
//
// if let Some((axis, align)) = split.changed(actions) {
//     self.saved_split = Some((axis, align));
// }
//
// if let Some(align) = self.saved_split.map(|(_, align)| align) {
//     split.set_align(cx, align);
// }"#;
pub const SCROLL_AREA_PREVIEW_CODE: &str = "ShadScrollArea{\n    width: Fill\n    height: 220\n    View{\n        width: Fill\n        height: Fit\n        flow: Down\n        spacing: 8.0\n        ShadLabel{text: \"Row 1\"}\n        ShadLabel{text: \"Row 2\"}\n        ShadLabel{text: \"Row 3\"}\n    }\n}";
pub const SEPARATOR_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    ShadLabel{text: \"Account\"}\n    ShadSeparator{}\n    ShadLabel{text: \"Billing\"}\n}";
