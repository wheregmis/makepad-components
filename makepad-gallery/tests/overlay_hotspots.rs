#[test]
fn select_page_keeps_full_width_select_examples_and_limit_note() {
    let source = include_str!("../src/ui/select_page.rs");

    assert!(
        source.contains("status_select := ShadSelect{\n                        width: Fill"),
        "select page should keep the full-width status select example"
    );
    assert!(
        source.contains("city_select := ShadSelect{\n                        width: Fill"),
        "select page should keep the full-width city select example"
    );
    assert!(
        source.contains("Known limitation: popup-style selects can still be unreliable inside the current gallery PageFlip shell."),
        "select page should keep the current popup/PageFlip limitation note until the hotspot is resolved"
    );
    assert!(
        source.contains("select_status := ShadFieldDescription"),
        "select page should expose a live status label for the controlled select demo"
    );
    assert!(
        source.contains("select_reset_btn := ShadButtonGhost"),
        "select page should keep an external reset button for the controlled select demo"
    );
}

#[test]
fn sheet_page_keeps_selects_inside_sheet_overlay_examples() {
    let source = include_str!("../src/ui/sheet_page.rs");

    assert!(
        source.contains("ShadFieldLabel{text: \"Default team\"}\n                                ShadSelect{labels: [\"Design\" \"Engineering\" \"Ops\"]}"),
        "right sheet should continue covering an in-sheet select"
    );
    assert!(
        source.contains("ShadFieldLabel{text: \"Team\"}\n                                    ShadSelect{labels: [\"All teams\" \"Design\" \"Engineering\" \"Ops\"]}"),
        "top sheet should continue covering the team filter select"
    );
    assert!(
        source.contains("ShadFieldLabel{text: \"Status\"}\n                                    ShadSelect{labels: [\"Any status\" \"Open\" \"Blocked\" \"Done\"]}"),
        "top sheet should continue covering the status filter select"
    );
}
