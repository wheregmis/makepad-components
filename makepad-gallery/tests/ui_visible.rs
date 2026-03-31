const SUITE_PATH: &str = "tests/ui_visible.splash";

#[ignore = "requires MAKEPAD_TEST_VISIBLE=1 and a running Studio remote session"]
#[test]
fn splash_suite_visible() {
    makepad_test::run_splash_suite(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_MANIFEST_DIR"),
        module_path!(),
        SUITE_PATH,
    )
    .unwrap();
}
