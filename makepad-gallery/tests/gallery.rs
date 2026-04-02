use makepad_test::{makepad_test, run_with_config, Selector, TestApp, TestConfig};

fn go_to_button_page(app: &TestApp) {
    app.locator(Selector::id("sidebar_button"))
        .wait_visible()
        .click();
    app.locator(Selector::id("page_label")).wait_text("Button");
    app.locator(Selector::id("content_flip"))
        .wait_text("button_page");
}

#[allow(dead_code)]
fn go_to_input_page(app: &TestApp) {
    app.locator(Selector::id("sidebar_input"))
        .wait_visible()
        .click();
    app.locator(Selector::id("page_label")).wait_text("Input");
}

#[allow(dead_code)]
fn go_to_dialog_page(app: &TestApp) {
    app.locator(Selector::id("sidebar_dialog"))
        .wait_visible()
        .click();
    app.locator(Selector::id("page_label")).wait_text("Dialog");
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

#[makepad_test]
fn gallery_table_virtual_window_pages_forward_and_back(app: TestApp) {
    app.locator(Selector::id("sidebar"))
        .wait_visible()
        .scroll(0.0, 1800.0);
    app.locator(Selector::id("sidebar_table"))
        .wait_visible()
        .click();
    app.locator(Selector::id("content_flip"))
        .wait_text("table_page");
    app.locator(Selector::id("page_root"))
        .wait_visible()
        .scroll(0.0, 700.0);

    app.locator(Selector::id("table_virtual_btn"))
        .wait_visible()
        .click();
    app.locator(Selector::id("table_status"))
        .wait_text("Showing virtual jobs 0..31 of 10000. Selected row: none.");
    app.locator(Selector::all().text_contains("JOB-00000"))
        .wait_visible();

    app.locator(Selector::id("table_next_btn"))
        .wait_visible()
        .click();
    app.locator(Selector::id("table_status"))
        .wait_text("Showing virtual jobs 32..63 of 10000. Selected row: none.");
    app.locator(Selector::all().text_contains("JOB-00032"))
        .wait_visible();

    app.locator(Selector::id("table_prev_btn"))
        .wait_visible()
        .click();
    app.locator(Selector::id("table_status"))
        .wait_text("Showing virtual jobs 0..31 of 10000. Selected row: none.");
    app.locator(Selector::all().text_contains("JOB-00000"))
        .wait_visible();
}

fn run_gallery_route_test(
    route: &str,
    test_name: &str,
    test: impl FnOnce(TestApp),
) {
    let mut config = TestConfig::current_package(
        env!("CARGO_MANIFEST_DIR"),
        "makepad-gallery",
        test_name,
    )
    .unwrap();
    config.env.insert(
        "MAKEPAD_GALLERY_INITIAL_ROUTE".to_string(),
        route.to_string(),
    );
    run_with_config(config, test).unwrap();
}

#[test]
fn gallery_input_page_allows_typing_into_primary_field() {
    run_gallery_route_test("/input", "gallery_input_page_allows_typing_into_primary_field", |app| {
        app.locator(Selector::id("page_label")).wait_text("Input");
        app.locator(Selector::id("email_input"))
            .wait_visible()
            .fill("hello@example.com")
            .wait_value("hello@example.com");
        app.locator(Selector::id("workspace_slug_input"))
            .wait_visible()
            .assert_value("northwind-revamp");
    });
}

#[test]
fn gallery_dialog_page_opens_and_closes_rename_dialog() {
    run_gallery_route_test("/dialog", "gallery_dialog_page_opens_and_closes_rename_dialog", |app| {
        app.locator(Selector::id("page_label")).wait_text("Dialog");
        app.locator(Selector::id("open_rename_dialog_btn"))
            .wait_visible()
            .click();
        app.locator(Selector::id("rename_project_input"))
            .wait_visible()
            .fill("Northwind v2")
            .wait_value("Northwind v2");
        app.locator(Selector::id("confirm"))
            .wait_visible()
            .click();
        app.locator(Selector::id("open_rename_dialog_btn"))
            .wait_visible();
    });
}
