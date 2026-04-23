#[test]
fn icon_gallery_generator_emits_metadata_not_preview_templates() {
    let source = include_str!("../build.rs");

    assert!(
        source.contains("const GENERATED_DATA_FILENAME: &str = \"icon_gallery_data.rs\";"),
        "icon gallery build output should use the compact metadata file"
    );
    assert!(
        !source.contains("icon_preview_rows.rs"),
        "icon gallery generator should no longer emit per-icon preview row scripts"
    );
    assert!(
        source.contains("svg_markup: include_str!("),
        "icon gallery metadata should embed SVG markup for runtime row rendering"
    );
}

#[test]
fn icon_gallery_page_uses_one_runtime_row_template() {
    let source = include_str!("../src/ui/icon_gallery_page.rs");

    assert!(
        source.contains("draw_svg.load_from_str(entry.svg_markup);"),
        "icon gallery rows should load SVG markup at runtime instead of relying on generated icon-specific templates"
    );
    assert!(
        source.contains("icon_results_list := PortalList{"),
        "icon gallery should render results through a reusable portal list row"
    );
    assert!(
        !source.contains("set_custom_row_templates"),
        "icon gallery should not drive one template id per icon anymore"
    );
}
