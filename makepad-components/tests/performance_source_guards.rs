#[test]
fn surface_popover_forces_a_new_batch() {
    let source = include_str!("../src/surface.rs");
    assert!(
        source.contains("mod.widgets.ShadSurfacePopover = mod.widgets.ShadSurfacePanel{")
            && source.contains("new_batch: true"),
        "popover surface should keep its own batch so overlay text draws above the shared background"
    );
}

#[test]
fn overlay_content_widgets_reuse_the_popover_surface_primitive() {
    let popover = include_str!("../src/popover.rs");
    let menubar = include_str!("../src/menubar.rs");
    let navigation_menu = include_str!("../src/navigation_menu.rs");

    assert!(
        popover.contains("mod.widgets.ShadPopoverContent = mod.widgets.ShadSurfacePopover{"),
        "popover content should inherit the shared popover surface batching/styling"
    );
    assert!(
        menubar.contains("mod.widgets.ShadMenubarContent = mod.widgets.ShadSurfacePopover{"),
        "menubar content should inherit the shared popover surface batching/styling"
    );
    assert!(
        navigation_menu
            .contains("mod.widgets.ShadNavigationMenuContent = mod.widgets.ShadSurfacePopover{"),
        "navigation menu content should inherit the shared popover surface batching/styling"
    );
}

#[test]
fn table_keeps_virtualized_and_cached_rendering_guards() {
    let source = include_str!("../src/table.rs");

    assert!(
        source.contains("list := PortalList{")
            && source.contains("const VIRTUAL_WINDOW_PRELOAD_MARGIN: usize = 8;"),
        "table should stay virtualized and preload adjacent windows before the viewport outruns the current slice"
    );
    assert!(
        source.contains("cached_content_widths: Vec<f64>,")
            && source.contains("fn apply_content_width_if_changed(&mut self, cx: &mut Cx, width: f64)"),
        "table should keep width-application caching so steady-state scrolling avoids repeated script_apply work"
    );
    assert!(
        source.contains("text_x_offsets: Vec<f64>,")
            && source.contains("fn sync_text_x_offsets<T: AsRef<str>>("),
        "table rows and headers should keep cached text offsets instead of recomputing layout every draw"
    );
}
