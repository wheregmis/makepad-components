use makepad_test::{makepad_test, Selector, TestApp};

fn go_to_button_page(app: &TestApp) {
    app.locator(Selector::id("sidebar_button"))
        .wait_visible()
        .click();
    app.locator(Selector::id("page_label")).wait_text("Button");
    app.locator(Selector::id("content_flip"))
        .wait_text("button_page");
}

#[ignore = "requires MAKEPAD_TEST_VISIBLE=1 and a running Studio remote session"]
#[makepad_test]
fn gallery_visible_button_page_smoke(app: TestApp) {
    go_to_button_page(&app);
    app.locator(Selector::widget_type("CodeView").text_contains("ShadButton"))
        .wait_visible();
    app.locator(Selector::id("theme_toggle"))
        .wait_visible()
        .click();
    app.locator(Selector::id("theme_toggle"))
        .wait_visible();
}
