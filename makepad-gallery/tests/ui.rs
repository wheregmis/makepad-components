mod common;

use common::{
    assert_page_shell_and_code, open_button_page_from_sidebar, open_command_palette,
    open_sidebar_page,
};
use makepad_test::{makepad_test, Selector, TestApp};

#[makepad_test]
fn gallery_default_accordion_page_renders(app: TestApp) {
    app.locator(Selector::id("desktop_page_label"))
        .wait_text("Accordion");
    assert_page_shell_and_code(&app, "ShadAccordion");
}

#[makepad_test]
fn gallery_theme_toggle_round_trips(app: TestApp) {
    app.locator(Selector::id("desktop_theme_toggle"))
        .wait_visible()
        .wait_text("Light theme")
        .click()
        .wait_text("Dark theme")
        .click()
        .wait_text("Light theme");
}

#[makepad_test]
fn gallery_command_palette_opens_and_closes(app: TestApp) {
    open_command_palette(&app);
    app.locator(Selector::id("clear_search_btn"))
        .wait_visible()
        .wait_text("Close")
        .click();
    app.locator(Selector::id("desktop_command_palette_trigger"))
        .wait_visible()
        .wait_text("Search components");
}

#[makepad_test]
fn gallery_button_page_routes_and_renders_preview(app: TestApp) {
    open_button_page_from_sidebar(&app);
    assert_page_shell_and_code(&app, "ShadButton");
}

#[makepad_test]
fn gallery_badge_page_routes_and_renders_code(app: TestApp) {
    open_sidebar_page(&app, "sidebar_badge", "Badge");
    assert_page_shell_and_code(&app, "ShadBadge");
}

#[makepad_test]
fn gallery_breadcrumb_page_routes_and_renders_code(app: TestApp) {
    open_sidebar_page(&app, "sidebar_breadcrumb", "Breadcrumb");
    assert_page_shell_and_code(&app, "ShadBreadcrumb");
}

#[makepad_test]
fn gallery_button_group_page_routes_and_renders_code(app: TestApp) {
    open_sidebar_page(&app, "sidebar_button_group", "Button Group");
    assert_page_shell_and_code(&app, "ShadButtonGroup");
}

#[makepad_test]
fn gallery_calendar_page_routes_and_renders_code(app: TestApp) {
    open_sidebar_page(&app, "sidebar_calendar", "Calendar");
    assert_page_shell_and_code(&app, "ShadCalendar");
}

#[makepad_test]
fn gallery_card_page_routes_and_renders_code(app: TestApp) {
    open_sidebar_page(&app, "sidebar_card", "Card");
    assert_page_shell_and_code(&app, "ShadCard");
}

#[makepad_test]
fn gallery_checkbox_page_routes_and_renders_code(app: TestApp) {
    open_sidebar_page(&app, "sidebar_checkbox", "Checkbox");
    assert_page_shell_and_code(&app, "ShadCheckbox");
}

#[makepad_test]
fn gallery_collapsible_page_routes_and_renders_code(app: TestApp) {
    open_sidebar_page(&app, "sidebar_collapsible", "Collapsible");
    assert_page_shell_and_code(&app, "ShadCollapsible");
}
