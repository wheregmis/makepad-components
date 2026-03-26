mod common;

use common::{assert_page_shell_and_code, open_command_palette};
use makepad_test::{makepad_test, Selector, TestApp};

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
