mod common;

use common::{assert_page_shell_and_code, run_gallery_test};
use makepad_test::{Selector, TestApp};

fn open_table_page(app: &TestApp) {
    app.locator(Selector::id("table_demo")).wait_visible();
}

fn reveal_table_controls(app: &TestApp) {
    app.locator(Selector::id("page_root")).scroll(0.0, 700.0);
    app.locator(Selector::id("table_ops_btn")).wait_visible();
}

#[test]
fn gallery_table_default_dataset_renders() {
    run_gallery_test(module_path!(), "gallery_table_default_dataset_renders", "/table", |app| {
        open_table_page(&app);
        reveal_table_controls(&app);

        app.locator(Selector::id("table_status"))
            .wait_text("Showing team roster. Selected row: none.");
        assert_page_shell_and_code(&app, "ShadTable");
    });
}

#[test]
fn gallery_table_switches_between_datasets() {
    run_gallery_test(module_path!(), "gallery_table_switches_between_datasets", "/table", |app| {
        open_table_page(&app);
        reveal_table_controls(&app);

        app.locator(Selector::id("table_ops_btn")).wait_visible().click();
        app.locator(Selector::id("table_status"))
            .wait_text("Showing ops queue. Selected row: none.");

        app.locator(Selector::id("table_team_btn"))
            .wait_visible()
            .click();
        app.locator(Selector::id("table_status"))
            .wait_text("Showing team roster. Selected row: none.");
    });
}

#[test]
fn gallery_table_virtual_window_pages_forward_and_back() {
    run_gallery_test(
        module_path!(),
        "gallery_table_virtual_window_pages_forward_and_back",
        "/table",
        |app| {
            open_table_page(&app);
            reveal_table_controls(&app);

            app.locator(Selector::id("table_virtual_btn"))
                .wait_visible()
                .click();
            app.locator(Selector::id("table_status"))
                .wait_text("Showing virtual jobs 0..31 of 10000. Selected row: none.");

            app.locator(Selector::id("table_next_btn"))
                .wait_visible()
                .click();
            app.locator(Selector::id("table_status"))
                .wait_text("Showing virtual jobs 32..63 of 10000. Selected row: none.");

            app.locator(Selector::id("table_prev_btn"))
                .wait_visible()
                .click();
            app.locator(Selector::id("table_status"))
                .wait_text("Showing virtual jobs 0..31 of 10000. Selected row: none.");
        },
    );
}
