pub const OVERVIEW_PREVIEW_CODE: &str = r#"
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 16.0

    ShadLabel{
        text: "Welcome to Makepad Components Gallery!"
        draw_text.text_style.font_size: 16.0
    }
}
"#;

pub const CATALOG_PREVIEW_CODE: &str = r#"// The catalog page is a visual index of the gallery.
// It groups representative widgets from each component family so the
// sidebar docs and the code tab stay aligned as the library grows.
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 16.0

    ShadSectionHeader{text: "Surfaces & Media"}
    ShadSurface{
        width: 240
        height: Fit
        flow: Down
        spacing: 6.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
    }
}
"#;
