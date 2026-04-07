#[test]
fn select_popup_uses_popover_theme_tokens() {
    let source = include_str!("../src/select.rs");
    assert!(
        source.contains("color: (shad_theme.color_surface_popover)"),
        "select popup should use the popover surface token"
    );
    assert!(
        source.contains("border_radius: (shad_theme.radius)"),
        "select popup items should derive radius from shad_theme.radius"
    );
    assert!(
        source.contains("popup_menu: mod.widgets.ShadSelectPopupMenu{}"),
        "select should keep routing through its themed popup menu widget"
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
fn shared_surface_panels_use_new_batch_for_nested_content() {
    let source = include_str!("../src/surface.rs");
    assert!(
        source.contains("mod.widgets.ShadSurfacePanel = mod.widgets.ShadSurface{\n        width: Fill\n        height: Fit\n        new_batch: true"),
        "surface panels should isolate their draw list before nested card/sidebar/menu content"
    );
}

#[test]
fn hoverable_navigation_and_group_containers_use_new_batch() {
    let sidebar = include_str!("../src/sidebar.rs");
    assert!(
        sidebar.contains("mod.widgets.ShadSidebarItem = mod.widgets.ButtonFlat{\n        width: Fill\n        height: 40\n        new_batch: true"),
        "sidebar items should isolate hover redraws from sibling labels"
    );

    let menubar = include_str!("../src/menubar.rs");
    assert!(
        menubar.contains("mod.widgets.ShadMenubarTrigger = ButtonFlat{\n        height: 30\n        new_batch: true"),
        "menubar triggers should isolate hover redraws from surrounding text"
    );
    assert!(
        menubar.contains("mod.widgets.ShadMenubarItem = set_type_default() do mod.widgets.ShadNavButtonBase{\n        width: Fill\n        height: 32\n        new_batch: true"),
        "menubar items should isolate hover redraws from surrounding labels"
    );

    let button_group = include_str!("../src/button_group.rs");
    assert!(
        button_group.contains("mod.widgets.ShadButtonGroupItem = set_type_default() do mod.widgets.ShadNavButtonBase{\n        width: Fit\n        height: 36\n        new_batch: true"),
        "button-group items should isolate hover redraws from adjacent segments"
    );

    let navigation_menu = include_str!("../src/navigation_menu.rs");
    assert!(
        navigation_menu.contains("mod.widgets.ShadNavigationMenuList = View{\n        width: Fit\n        height: Fit\n        new_batch: true"),
        "navigation menu lists should isolate their background from trigger labels"
    );
    assert!(
        navigation_menu.contains("mod.widgets.ShadNavigationMenuTrigger = ButtonFlat{\n        height: 36\n        new_batch: true"),
        "navigation menu triggers should isolate hover redraws from sibling content"
    );
}

#[test]
fn carousel_and_otp_text_containers_use_new_batch() {
    let carousel = include_str!("../src/carousel.rs");
    assert_eq!(
        carousel.matches("surface := RoundedView{\n                        width: Fill\n                        height: Fill\n                        new_batch: true").count(),
        3,
        "each carousel slide surface should isolate its background from nested text and media"
    );
    assert!(
        carousel.contains("mod.widgets.ShadCarouselPrevBtn = mod.widgets.IconButtonChevronLeft{\n        width: 32\n        height: 32\n        new_batch: true"),
        "carousel prev button should isolate hover redraws from sibling controls"
    );
    assert!(
        carousel.contains("mod.widgets.ShadCarouselNextBtn = mod.widgets.IconButtonChevronRight{\n        width: 32\n        height: 32\n        new_batch: true"),
        "carousel next button should isolate hover redraws from sibling controls"
    );

    let input_otp = include_str!("../src/input_otp.rs");
    assert!(
        input_otp.contains("mod.widgets.ShadInputOtpSlot = RoundedView{\n        width: 48\n        height: 56\n        new_batch: true"),
        "OTP slots should isolate their digit labels from slot backgrounds"
    );
}
