const SUITE_PATH: &str = "tests/ui_visible.splash";

#[ignore = "requires MAKEPAD_TEST_VISIBLE=1 and a running Studio remote session"]
#[test]
fn splash_suite_visible() {
    let previous_route = std::env::var("MAKEPAD_GALLERY_INITIAL_ROUTE").ok();
    std::env::set_var("MAKEPAD_GALLERY_INITIAL_ROUTE", "/button");

    let result = makepad_test::run_splash_suite(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_MANIFEST_DIR"),
        module_path!(),
        SUITE_PATH,
    );

    if let Some(route) = previous_route {
        std::env::set_var("MAKEPAD_GALLERY_INITIAL_ROUTE", route);
    } else {
        std::env::remove_var("MAKEPAD_GALLERY_INITIAL_ROUTE");
    }

    result.unwrap();
}
