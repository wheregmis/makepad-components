pub const ASPECT_RATIO_PREVIEW_CODE: &str = r#"// Use ShadAspectRatio as the frame, then let the child media fill it.
ShadAspectRatio{
    width: 320
    ratio: 1.7777777778

    ShadMediaFrame{
        width: Fill
        height: Fill
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        ShadImage{
            width: Fill
            height: Fill
            fit: ImageFit.Biggest
            src: crate_resource("self://resources/aspect-ratio/royal-esplanade.jpg")
        }
    }
}"#;
pub const RESIZABLE_PREVIEW_CODE: &str = r#"horizontal_split := ShadResizable{
    axis: SplitterAxis.Horizontal
    align: SplitterAlign.FromA(180.0)
    a: View{width: Fill height: Fill}
    b: View{width: Fill height: Fill}
}"#;
pub const SCROLL_AREA_PREVIEW_CODE: &str = "ShadScrollArea{\n    width: Fill\n    height: 220\n    View{\n        width: Fill\n        height: Fit\n        flow: Down\n        spacing: 8.0\n        ShadLabel{text: \"Row 1\"}\n        ShadLabel{text: \"Row 2\"}\n        ShadLabel{text: \"Row 3\"}\n    }\n}";
pub const SEPARATOR_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    ShadLabel{text: \"Account\"}\n    ShadSeparator{}\n    ShadLabel{text: \"Billing\"}\n}";
