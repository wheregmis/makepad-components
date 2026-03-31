use makepad_test::{Selector, TestApp};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn open_button_page_from_sidebar(app: &TestApp) {
    open_sidebar_page(app, "sidebar_button", "Button");
}

#[allow(dead_code)]
pub fn select_page_from_command_palette(app: &TestApp, query: &str) {
    open_command_palette(app);
    app.locator(Selector::widget_type("TextInput"))
        .wait_visible()
        .click()
        .type_text(query);
    app.locator(Selector::widget_type("ShadCommandPaletteRowButton"))
        .wait_visible()
        .click();
}

#[allow(dead_code)]
pub fn open_sidebar_page(app: &TestApp, sidebar_id: &str, page_title: &str) {
    app.locator(Selector::id(sidebar_id)).wait_visible().click();
    app.locator(Selector::id("page_label"))
        .wait_text(page_title);
}

#[allow(dead_code)]
pub fn assert_page_shell_and_code(app: &TestApp, marker: &str) {
    app.locator(Selector::widget_type("CodeView").text_contains(marker))
        .wait_visible();
}

#[allow(dead_code)]
pub fn assert_theme_toggle_round_trip(app: &TestApp) {
    let initial = app
        .locator(Selector::id("theme_toggle"))
        .wait_visible()
        .snapshot()
        .text
        .expect("theme toggle should expose text");

    let next_fragment = if initial.contains("Dark") {
        "Light"
    } else if initial.contains("Light") {
        "Dark"
    } else {
        panic!("unexpected theme toggle label: {initial}");
    };

    app.locator(Selector::id("theme_toggle")).click();
    app.locator(Selector::id("theme_toggle").text_contains(next_fragment))
        .wait_visible();

    app.locator(Selector::id("theme_toggle")).click();
    app.locator(Selector::id("theme_toggle")).wait_text(initial);
}
