#[test]
fn select_popup_uses_popover_theme_tokens() {
    let source = include_str!("../src/select.rs");
    assert!(
        source.contains("color: (shad_theme.color_popover)"),
        "select popup should use the popover surface token"
    );
    assert!(
        !source.contains("border_radius: 6.0"),
        "select popup items should derive radius from shad_theme.radius"
    );
}

#[test]
fn context_menu_uses_popover_theme_tokens() {
    let source = include_str!("../src/context_menu.rs");
    assert!(
        source.contains("color: (shad_theme.color_popover)"),
        "context menu surface should use the popover surface token"
    );
    assert!(
        !source.contains("border_radius: 6.0"),
        "context menu items should derive radius from shad_theme.radius"
    );
}
