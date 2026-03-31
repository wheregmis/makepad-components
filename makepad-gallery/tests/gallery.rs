const SUITE_PATH: &str = "tests/gallery.splash";

#[test]
fn splash_suite() {
    makepad_test::run_splash_suite(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_MANIFEST_DIR"),
        module_path!(),
        SUITE_PATH,
    )
    .unwrap();
}
