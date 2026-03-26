use makepad_test::{Selector, TestApp};

pub fn open_command_palette(app: &TestApp) {
    app.locator(Selector::id("command_palette_trigger"))
        .wait_visible()
        .click();
    app.locator(Selector::id("search_label"))
        .wait_visible()
        .wait_text("Search gallery components");
    app.locator(Selector::widget_type("TextInput"))
        .wait_visible();
}

pub fn open_button_page_from_sidebar(app: &TestApp) {
    open_sidebar_page(app, "sidebar_button", "Button");
}

pub fn open_sidebar_page(app: &TestApp, sidebar_id: &str, page_title: &str) {
    app.locator(Selector::id(sidebar_id)).wait_visible().click();
    app.locator(Selector::id("page_label"))
        .wait_text(page_title);
}

pub fn assert_page_shell_and_code(app: &TestApp, marker: &str) {
    app.locator(Selector::widget_type("CodeView").text_contains(marker))
        .wait_visible();
}
