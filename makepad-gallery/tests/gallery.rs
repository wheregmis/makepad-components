use makepad_test::{makepad_test, Selector, TestApp};

fn go_to_button_page(app: &TestApp) {
    app.locator(Selector::id("sidebar_button"))
        .wait_visible()
        .click();
    app.locator(Selector::id("page_label")).wait_text("Button");
    app.locator(Selector::id("content_flip"))
        .wait_text("button_page");
}

#[makepad_test]
fn gallery_default_accordion_page_renders(app: TestApp) {
    app.locator(Selector::id("page_label"))
        .wait_text("Accordion");
    app.locator(Selector::id("content_flip"))
        .wait_text("accordion_page");
    app.locator(Selector::widget_type("CodeView").text_contains("ShadAccordion"))
        .wait_visible();
}

#[makepad_test]
fn gallery_button_page_renders_preview_and_code(app: TestApp) {
    go_to_button_page(&app);
    app.locator(Selector::widget_type("CodeView").text_contains("ShadButton"))
        .wait_visible();
}

#[makepad_test]
fn gallery_theme_toggle_smoke(app: TestApp) {
    app.locator(Selector::id("theme_toggle"))
        .wait_visible()
        .click();
    app.locator(Selector::id("theme_toggle"))
        .wait_visible();
}
