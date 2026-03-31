pub use makepad_code_editor;
pub use makepad_widgets;

mod ui;

use crate::ui::catalog;
use crate::ui::command_palette_page::GalleryCommandPalettePageWidgetRefExt;
use makepad_components::button::ShadButtonWidgetExt;
use makepad_components::command_palette::ShadCommandPaletteWidgetRefExt;
use makepad_components::makepad_widgets::*;
use makepad_components::sidebar::ShadSidebarItemRef;
use makepad_router::RouterWidgetWidgetRefExt;

app_main!(App);

const GALLERY_GITHUB_URL: &str = "https://github.com/wheregmis/makepad-components";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum PendingPaletteAction {
    Open,
    Toggle,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum PendingHeaderSync {
    SyncState,
    #[default]
    None,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: mod.widgets.GalleryAppUi{}
    }
}

impl App {
    const SMALL_SCREEN_WIDTH: f64 = 900.0;

    fn initial_page_from_env() -> LiveId {
        std::env::var("MAKEPAD_GALLERY_INITIAL_ROUTE")
            .ok()
            .and_then(|route| catalog::entry_for_route(&route).map(|entry| entry.page))
            .unwrap_or_else(catalog::default_page)
    }

    fn build_script_mod(vm: &mut ScriptVm, is_light_theme: bool) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        makepad_components::theme::script_mod(vm);
        if is_light_theme {
            script_eval!(vm, {
                mod.widgets.shad_theme = mod.widgets.shad_themes.light
            });
        } else {
            script_eval!(vm, {
                mod.widgets.shad_theme = mod.widgets.shad_themes.dark
            });
        }
        makepad_components::script_mod_without_theme(vm);
        makepad_code_editor::script_mod(vm);
        makepad_router::script_mod(vm);
        crate::ui::script_mod(vm);
        self::script_mod(vm)
    }

    fn sync_content_route(&self, cx: &mut Cx) {
        let router = self.ui.router_widget(cx, ids!(content_flip));
        if router.current_route_id() != Some(self.current_page) {
            router.go_to_route(cx, self.current_page);
        }
    }

    fn active_page(&self, cx: &mut Cx) -> LiveId {
        self.ui
            .router_widget(cx, ids!(content_flip))
            .current_route_id()
            .unwrap_or(self.current_page)
    }

    fn configure_header_adaptive_view(&self, cx: &mut Cx) {
        self.ui.adaptive_view(cx, ids!(header)).set_variant_selector(
            |_cx, parent_size| {
                if parent_size.x < Self::SMALL_SCREEN_WIDTH {
                    live_id!(Mobile)
                } else {
                    live_id!(Desktop)
                }
            },
        );
    }

    fn queue_header_state_sync(&mut self, cx: &mut Cx) {
        self.pending_header_sync = PendingHeaderSync::SyncState;
        self.header_sync_next_frame = cx.new_next_frame();
    }

    fn set_current_page(&mut self, cx: &mut Cx, page: LiveId) {
        self.current_page = page;
        self.sync_content_route(cx);
        self.sync_page_metadata(cx);
        if self.is_small_screen {
            self.sidebar_open = false;
            self.apply_responsive_visibility(cx);
        }
    }

    fn toggle_command_palette(&mut self, cx: &mut Cx) {
        let palette = self.ui.shad_command_palette(cx, ids!(command_palette));
        palette.set_items(cx, crate::ui::command_palette::catalog_command_items());
        palette.toggle(cx);
    }

    fn open_command_palette(&mut self, cx: &mut Cx) {
        let palette = self.ui.shad_command_palette(cx, ids!(command_palette));
        palette.set_items(cx, crate::ui::command_palette::catalog_command_items());
        palette.open(cx);
    }

    fn queue_open_command_palette(&mut self, cx: &mut Cx) {
        self.pending_palette_action = PendingPaletteAction::Open;
        self.palette_action_next_frame = cx.new_next_frame();
    }

    fn queue_toggle_command_palette(&mut self, cx: &mut Cx) {
        self.pending_palette_action = PendingPaletteAction::Toggle;
        self.palette_action_next_frame = cx.new_next_frame();
    }

    fn sync_mobile_sidebar_button(&self, cx: &mut Cx) {
        let show_close = self.is_small_screen && self.sidebar_open;
        self.ui
            .shad_button(cx, ids!(mobile_sidebar_menu_button))
            .set_visible(cx, !show_close);
        self.ui
            .shad_button(cx, ids!(mobile_sidebar_close_button))
            .set_visible(cx, show_close);
    }

    fn sync_theme_toggle_copy(&self, cx: &mut Cx) {
        let label = if self.is_small_screen {
            if self.is_light_theme { "Dark" } else { "Light" }
        } else if self.is_light_theme {
            "Dark theme"
        } else {
            "Light theme"
        };
        self.ui.shad_button(cx, ids!(theme_toggle)).set_text(cx, label);
    }

    fn sync_page_metadata(&self, cx: &mut Cx) {
        let page = self.active_page(cx);

        if let Some(entry) = catalog::entry_for_page(page) {
            self.ui.label(cx, ids!(page_label)).set_text(cx, entry.title);
        }
        self.sync_sidebar_focus_behavior(cx);
        self.sync_sidebar_selection(cx);
    }

    fn sync_sidebar_focus_behavior(&self, cx: &mut Cx) {
        let allow_sidebar_focus = !self.is_small_screen;

        for entry in catalog::entries() {
            let mut item = self.ui.widget(cx, &[entry.sidebar_id]);
            script_apply_eval!(cx, item, {
                grab_key_focus: #(allow_sidebar_focus)
            });
        }
    }

    fn sync_sidebar_selection(&self, cx: &mut Cx) {
        let page = self.active_page(cx);
        for entry in catalog::entries() {
            ShadSidebarItemRef(self.ui.widget(cx, &[entry.sidebar_id]))
                .set_active(cx, entry.page == page);
        }
    }

    fn reload_ui_for_theme(&mut self, cx: &mut Cx) {
        cx.with_vm(|vm| {
            let value = Self::build_script_mod(vm, self.is_light_theme);
            <Self as ScriptApply>::script_apply(
                self,
                vm,
                &Apply::Reload,
                &mut Scope::empty(),
                value,
            );
        });
        self.configure_header_adaptive_view(cx);
        self.apply_responsive_visibility(cx);
        self.sync_theme_toggle_copy(cx);
        self.set_current_page(cx, self.current_page);
        self.queue_header_state_sync(cx);
        self.ui.redraw(cx);
    }

    fn set_theme(&mut self, cx: &mut Cx, is_light_theme: bool) {
        self.is_light_theme = is_light_theme;
        self.reload_ui_for_theme(cx);
    }

    fn queue_theme_change(&mut self, cx: &mut Cx, is_light_theme: bool) {
        self.pending_theme = Some(is_light_theme);
        self.theme_reload_next_frame = cx.new_next_frame();
    }

    fn apply_responsive_visibility(&mut self, cx: &mut Cx) {
        self.ui
            .view(cx, ids!(sidebar))
            .set_visible(cx, !self.is_small_screen || self.sidebar_open);
        self.ui
            .view(cx, ids!(main_content))
            .set_visible(cx, !self.is_small_screen || !self.sidebar_open);
        self.sync_mobile_sidebar_button(cx);
        self.sync_theme_toggle_copy(cx);
        self.sync_sidebar_focus_behavior(cx);
        self.sync_content_route(cx);
    }

    fn update_screen_mode(&mut self, cx: &mut Cx, window_width: f64) {
        let is_small_screen = window_width < Self::SMALL_SCREEN_WIDTH;
        if self.is_small_screen != is_small_screen {
            self.is_small_screen = is_small_screen;
            self.sidebar_open = !is_small_screen;
            self.queue_header_state_sync(cx);
        }
        self.apply_responsive_visibility(cx);
    }

    fn handle_sidebar_navigation(&mut self, cx: &mut Cx, actions: &Actions) {
        for entry in catalog::entries() {
            if ShadSidebarItemRef(self.ui.widget(cx, &[entry.sidebar_id])).clicked(actions) {
                self.set_current_page(cx, entry.page);
                break;
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    is_small_screen: bool,
    #[rust]
    sidebar_open: bool,
    #[rust]
    is_light_theme: bool,
    #[rust]
    current_page: LiveId,
    #[rust]
    pending_theme: Option<bool>,
    #[rust]
    theme_reload_next_frame: NextFrame,
    #[rust]
    pending_palette_action: PendingPaletteAction,
    #[rust]
    palette_action_next_frame: NextFrame,
    #[rust]
    pending_header_sync: PendingHeaderSync,
    #[rust]
    header_sync_next_frame: NextFrame,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(cx, ids!(content_flip));

        let palette = self.ui.shad_command_palette(cx, ids!(command_palette));
        if let Some(index) = palette.selected(actions) {
            if let Some(entry) = catalog::entries().get(index) {
                self.set_current_page(cx, entry.page);
            }
        }
        if self.is_small_screen
            && (self
                .ui
                .shad_button(cx, ids!(mobile_sidebar_menu_button))
                .clicked(actions)
                || self
                    .ui
                    .shad_button(cx, ids!(mobile_sidebar_close_button))
                    .clicked(actions))
        {
            self.sidebar_open = !self.sidebar_open;
            self.apply_responsive_visibility(cx);
        }
        if self
            .ui
            .shad_button(cx, ids!(theme_toggle))
            .clicked(actions)
        {
            self.queue_theme_change(cx, !self.is_light_theme);
        }
        if self
            .ui
            .shad_button(cx, ids!(command_palette_trigger))
            .clicked(actions)
        {
            self.queue_open_command_palette(cx);
        }
        if self
            .ui
            .shad_button(cx, ids!(github_button))
            .clicked(actions)
        {
            cx.open_url(GALLERY_GITHUB_URL, OpenUrlInPlace::No);
        }

        self.handle_sidebar_navigation(cx, actions);
        if self
            .ui
            .gallery_command_palette_page(cx, ids!(command_palette_page))
            .open_requested(actions)
        {
            self.queue_open_command_palette(cx);
        }

        if let Some(page) = router.current_route_id() {
            if self.current_page != page {
                self.current_page = page;
            }
        }
        self.sync_page_metadata(cx);
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        Self::build_script_mod(vm, false)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.sidebar_open = true;
                self.current_page = Self::initial_page_from_env();
                self.is_light_theme = false;
                self.pending_theme = None;
                self.theme_reload_next_frame = NextFrame::default();
                self.pending_palette_action = PendingPaletteAction::None;
                self.palette_action_next_frame = NextFrame::default();
                self.pending_header_sync = PendingHeaderSync::None;
                self.header_sync_next_frame = NextFrame::default();
                self.configure_header_adaptive_view(cx);
                self.apply_responsive_visibility(cx);
                self.sync_theme_toggle_copy(cx);
                self.set_current_page(cx, self.current_page);
                self.queue_header_state_sync(cx);
            }
            Event::NextFrame(_) => {
                if self.theme_reload_next_frame.is_event(event).is_some() {
                    self.theme_reload_next_frame = NextFrame::default();
                    if let Some(is_light_theme) = self.pending_theme.take() {
                        self.set_theme(cx, is_light_theme);
                    }
                }
                if self.palette_action_next_frame.is_event(event).is_some() {
                    self.palette_action_next_frame = NextFrame::default();
                    match std::mem::take(&mut self.pending_palette_action) {
                        PendingPaletteAction::Open => self.open_command_palette(cx),
                        PendingPaletteAction::Toggle => self.toggle_command_palette(cx),
                        PendingPaletteAction::None => {}
                    }
                }
                if self.header_sync_next_frame.is_event(event).is_some() {
                    self.header_sync_next_frame = NextFrame::default();
                    match std::mem::take(&mut self.pending_header_sync) {
                        PendingHeaderSync::SyncState => {
                            self.sync_theme_toggle_copy(cx);
                            self.sync_page_metadata(cx);
                        }
                        PendingHeaderSync::None => {}
                    }
                }
            }
            Event::MacosMenuCommand(command) => {
                if *command == live_id!(command_palette_menu) {
                    self.queue_open_command_palette(cx);
                }
            }
            Event::WindowGeomChange(geom) => {
                self.update_screen_mode(cx, geom.new_geom.inner_size.x)
            }
            Event::KeyDown(key_event) => {
                if key_event.key_code == KeyCode::KeyK
                    && (key_event.modifiers.logo || key_event.modifiers.control)
                {
                    // Keep forwarding the shortcut event through the UI tree so the
                    // palette's overlay state settles in the same dispatch cycle as
                    // button-driven opens.
                    self.queue_toggle_command_palette(cx);
                }
            }
            _ => {}
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
