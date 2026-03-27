mod common;

use common::{
    assert_page_shell_and_code, open_command_palette, select_page_from_command_palette,
};
use makepad_test::{makepad_test, Selector, TestApp};

#[ignore = "requires MAKEPAD_TEST_VISIBLE=1 and a running Studio remote session"]
#[makepad_test]
fn gallery_visible_smoke_suite(app: TestApp) {
    app.locator(Selector::id("page_label")).wait_text("Accordion");
    assert_page_shell_and_code(&app, "ShadAccordion");

    app.locator(Selector::id("theme_toggle"))
        .wait_visible()
        .wait_text("Light")
        .click()
        .wait_text("Dark")
        .click()
        .wait_text("Light");

    open_command_palette(&app);
    app.locator(Selector::id("clear_search_btn"))
        .wait_visible()
        .wait_text("Close")
        .click();
    app.locator(Selector::id("command_palette_trigger"))
        .wait_visible()
        .wait_text("Search components");
}

#[ignore = "requires MAKEPAD_TEST_VISIBLE=1 and a running Studio remote session"]
#[makepad_test]
fn gallery_visible_table_smoke_suite(app: TestApp) {
    select_page_from_command_palette(&app, "Table");
    app.locator(Selector::id("table_status"))
        .wait_text("Showing team roster. Selected row: none.");
    app.locator(Selector::id("table_ops_btn"))
        .wait_visible()
        .click();
    app.locator(Selector::id("table_status"))
        .wait_text("Showing ops queue. Selected row: none.");
}

#[ignore = "requires MAKEPAD_TEST_VISIBLE=1 and a running Studio remote session"]
#[makepad_test]
fn gallery_visible_sonner_smoke_suite(app: TestApp) {
    select_page_from_command_palette(&app, "Sonner");
    app.locator(Selector::id("page_label")).wait_text("Sonner");
    app.locator(Selector::id("toast_event_btn"))
        .wait_visible()
        .click();
    app.locator(Selector::widget_type("IconButtonX"))
        .wait_visible()
        .click();
    app.locator(Selector::widget_type("IconButtonX"))
        .wait_hidden();
}
