mod common;

use common::{assert_page_shell_and_code, run_gallery_test};
use makepad_test::{Selector, TestApp};

fn open_sonner_page(app: &TestApp) {
    app.locator(Selector::id("toast_event_btn")).wait_visible();
}

#[test]
fn gallery_sonner_page_routes_and_renders_code() {
    run_gallery_test(
        module_path!(),
        "gallery_sonner_page_routes_and_renders_code",
        "/sonner",
        |app| {
            open_sonner_page(&app);
            assert_page_shell_and_code(&app, "ShadSonner");
        },
    );
}
