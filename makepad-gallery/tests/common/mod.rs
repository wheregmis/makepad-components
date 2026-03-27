use makepad_test::{KeyCode, Selector, TestApp, TestConfig, run_with_config};

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
        .type_text(query);
    app.press_key(KeyCode::ReturnKey);
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
pub fn run_gallery_test<F>(module_path: &str, test_name: &str, initial_route: &str, test: F)
where
    F: FnOnce(TestApp),
{
    let mut config = TestConfig::current_package(
        env!("CARGO_MANIFEST_DIR"),
        env!("CARGO_PKG_NAME"),
        format!("{module_path}::{test_name}"),
    )
    .expect("create makepad test config");
    config.env.insert(
        "MAKEPAD_GALLERY_INITIAL_ROUTE".to_string(),
        initial_route.to_string(),
    );
    run_with_config(config, test).expect("run makepad gallery test");
}
