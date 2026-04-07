#[test]
fn kbd_page_keeps_wrapped_shortcut_rows_for_narrow_layouts() {
    let source = include_str!("../src/ui/kbd_page.rs");

    assert!(
        source.contains("text: \"Duplicate current row\""),
        "kbd page should keep the duplicate-row shortcut example"
    );
    assert!(
        source.contains("flow: Right{wrap: true}\n                            spacing: 16.0"),
        "kbd page should let the second shortcut row wrap on narrow widths"
    );
    assert!(
        source.contains("flow: Right{wrap: true}\n                                spacing: 6.0"),
        "kbd page should let the multi-key shortcut chip group wrap on narrow widths"
    );
}

#[test]
fn command_palette_page_keeps_wrapped_shortcut_hints() {
    let source = include_str!("../src/ui/command_palette_page.rs");

    assert!(
        source.contains("open_command_palette_btn := ShadButton{text: \"Open Command Palette\"}"),
        "command palette page should keep the trigger button example"
    );
    assert!(
        source.contains("width: Fill\n                    height: Fit\n                    flow: Right{wrap: true}"),
        "command palette page should let the trigger row wrap across narrow gallery widths"
    );
    assert!(
        source.contains("flow: Right{wrap: true}\n                        spacing: 6.0"),
        "command palette page should keep each shortcut hint in its own wrap-friendly chip group"
    );
}
