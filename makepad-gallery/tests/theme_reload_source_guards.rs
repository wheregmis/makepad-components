#[test]
fn theme_switch_reapplies_existing_app_source() {
    let source = include_str!("../src/main.rs");

    assert!(
        source.contains("#[source]\n    source: ScriptObjectRef"),
        "app should keep a rooted source object so theme changes can reapply the existing tree"
    );
    assert!(
        source.contains("let value: ScriptValue = self.source.clone().into();"),
        "theme reload should reapply the existing app source instead of rebuilding script modules"
    );
    assert!(
        !source.contains("let value = Self::build_script_mod(vm, self.is_light_theme);"),
        "theme reload should no longer rebuild the full script module graph on each toggle"
    );
}

#[test]
fn light_theme_survives_live_edit_reload() {
    let source = include_str!("../src/main.rs");

    assert!(
        source.contains("Event::LiveEdit => {\n                if self.is_light_theme {\n                    self.reload_ui_for_theme(cx);"),
        "live edit should reapply the active light theme after the framework reload path resets module defaults"
    );
}
