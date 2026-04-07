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

#[test]
fn badge_uses_theme_radius_and_new_batch() {
    let source = include_str!("../src/badge.rs");
    assert!(
        source.contains("new_batch: true"),
        "badge should force a new batch so text draws above its background"
    );
    assert!(
        source.contains("border_radius: (shad_theme.radius)"),
        "badge should derive its radius from the shared theme token"
    );
}

#[test]
fn kbd_uses_theme_radius_and_new_batch() {
    let source = include_str!("../src/kbd.rs");
    assert!(
        source.contains("new_batch: true"),
        "kbd should force a new batch so key labels draw above the background"
    );
    assert!(
        source.contains("border_radius: (shad_theme.radius)"),
        "kbd should derive its radius from the shared theme token"
    );
}
