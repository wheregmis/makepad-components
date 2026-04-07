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
    assert!(
        source.contains("PopupMenuPosition::BelowInput"),
        "select should open its popup below the trigger by default"
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

#[test]
fn dialog_modal_surfaces_force_new_batch() {
    let source = include_str!("../src/dialog.rs");
    assert!(
        source.matches("new_batch: true").count() >= 3,
        "dialog modal surfaces should force a new batch so overlay text and controls redraw independently"
    );
}

#[test]
fn sheet_surface_forces_new_batch() {
    let source = include_str!("../src/sheet.rs");
    assert!(
        source.contains("mod.widgets.ShadSheetFrame = mod.widgets.ShadSurfacePanel{\n        new_batch: true"),
        "sheet frame should force a new batch so the sliding panel redraw stays isolated from the modal backdrop"
    );
}

#[test]
fn sonner_toast_slots_force_new_batch() {
    let source = include_str!("../src/sonner.rs");
    assert!(
        source.contains("let ToastSlotPanel = RoundedView {\n        visible: false\n        new_batch: true"),
        "toast slots should force a new batch so their text and progress updates stay isolated from the notification overlay"
    );
}
