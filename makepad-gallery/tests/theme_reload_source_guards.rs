#[test]
fn theme_switch_rebuilds_script_modules() {
    let source = include_str!("../src/main.rs");

    assert!(
        source.contains("let value = Self::build_script_mod(vm, self.is_light_theme);"),
        "theme reload should rebuild the script modules in this pinned Makepad checkout"
    );
    assert!(
        !source.contains("let value: ScriptValue = self.source.clone().into();"),
        "in-place source reapply is not reliable here until desktop script reapply support lands upstream"
    );
}

#[test]
fn startup_uses_dark_theme_script_root() {
    let source = include_str!("../src/main.rs");

    assert!(
        source.contains("fn script_mod(vm: &mut ScriptVm) -> ScriptValue {\n        Self::build_script_mod(vm, false)"),
        "app startup should continue to build the dark theme root before any user toggle"
    );
}
