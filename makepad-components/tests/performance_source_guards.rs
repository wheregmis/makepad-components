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

#[test]
fn horizontal_scroll_wrappers_do_not_remap_vertical_scroll_input() {
    let scroll = include_str!("../src/scroll.rs");
    let tabs = include_str!("../src/tabs.rs");

    assert!(
        !scroll.contains("use_vertical_finger_scroll: true"),
        "horizontal scroll wrappers should not remap vertical wheel/finger input into x-scroll, or parent vertical surfaces will jitter"
    );
    assert!(
        !tabs.contains("use_vertical_finger_scroll: true"),
        "tabs should rely on real horizontal deltas instead of leaking vertical gesture input into nested scroll areas"
    );
}

#[test]
fn resizable_defaults_leave_vertical_splitters_room_to_move() {
    let source = include_str!("../src/resizable.rs");
    assert!(
        source.contains("min_horizontal: 72.0") && source.contains("max_horizontal: 72.0"),
        "vertical splitters should not clamp both panes to 120px inside a 260px demo; leave real drag range to avoid jumps near center"
    );
    assert!(
        source.contains("min_vertical: 120.0") && source.contains("max_vertical: 120.0"),
        "horizontal splitters should keep the wider pane minimums that already feel stable in the gallery"
    );
}
