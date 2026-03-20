pub use makepad_code_editor;
pub use makepad_widgets;

mod ui;

use crate::ui::catalog;
use crate::ui::command_palette::GalleryCommandPalette;
use crate::ui::command_palette_page::GalleryCommandPalettePageWidgetRefExt;
use makepad_components::makepad_widgets::*;
use makepad_router::RouterWidgetWidgetRefExt;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: mod.widgets.GalleryAppUi{}
    }
}

impl App {
    const SMALL_SCREEN_WIDTH: f64 = 900.0;

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

    fn set_current_page(&mut self, cx: &mut Cx, page: LiveId) {
        self.current_page = page;
        self.ui
            .router_widget(cx, ids!(content_flip))
            .go_to_route(cx, page);
        self.sync_page_metadata(cx);
        if self.is_small_screen {
            self.sidebar_open = false;
            self.apply_responsive_visibility(cx);
        }
    }

    fn toggle_command_palette(&mut self, cx: &mut Cx) {
        if let Some(mut palette) = self
            .ui
            .widget_flood(cx, ids!(command_palette))
            .borrow_mut::<GalleryCommandPalette>()
        {
            palette.toggle(cx);
        }
    }

    fn open_command_palette(&mut self, cx: &mut Cx) {
        if let Some(mut palette) = self
            .ui
            .widget_flood(cx, ids!(command_palette))
            .borrow_mut::<GalleryCommandPalette>()
        {
            palette.open(cx);
        }
    }

    fn sync_mobile_sidebar_button(&self, cx: &mut Cx) {
        let show_close = self.is_small_screen && self.sidebar_open;
        self.ui
            .button(cx, ids!(mobile_sidebar_menu_button))
            .set_visible(cx, !show_close);
        self.ui
            .button(cx, ids!(mobile_sidebar_close_button))
            .set_visible(cx, show_close);
    }

    fn sync_theme_toggle_copy(&self, cx: &mut Cx) {
        let desktop_label = if self.is_light_theme {
            "Dark theme"
        } else {
            "Light theme"
        };
        self.ui
            .button(cx, ids!(desktop_theme_toggle))
            .set_text(cx, desktop_label);
    }

    fn sync_page_metadata(&self, cx: &mut Cx) {
        if let Some(entry) = catalog::entry_for_page(self.current_page) {
            self.ui
                .label(cx, ids!(desktop_page_label))
                .set_text(cx, entry.title);
            self.ui
                .label(cx, ids!(mobile_page_label))
                .set_text(cx, entry.title);
        }
        self.sync_sidebar_focus_behavior(cx);
        self.sync_sidebar_selection(cx);
    }

    fn sync_sidebar_focus_behavior(&self, cx: &mut Cx) {
        let allow_sidebar_focus = !self.is_small_screen;

        for entry in catalog::entries() {
            let mut item = self.ui.button(cx, &[entry.sidebar_id]);
            script_apply_eval!(cx, item, {
                grab_key_focus: #(allow_sidebar_focus)
            });
        }
    }

    fn sync_sidebar_selection(&self, cx: &mut Cx) {
        let (
            active_bg,
            active_bg_hover,
            active_bg_down,
            active_text,
            inactive_bg_hover,
            inactive_bg_down,
            inactive_text,
            inactive_focus_bg,
            inactive_focus_text,
        ) = if self.is_light_theme {
            (
                Vec4f::from_u32(0xe4e4e7ff),
                Vec4f::from_u32(0xd4d4d8ff),
                Vec4f::from_u32(0xa1a1aaff),
                Vec4f::from_u32(0x09090bff),
                Vec4f::from_u32(0xf4f4f5ff),
                Vec4f::from_u32(0xe4e4e7ff),
                Vec4f::from_u32(0x71717aff),
                Vec4f::from_u32(0xf4f4f5ff),
                Vec4f::from_u32(0x09090bff),
            )
        } else {
            (
                Vec4f::from_u32(0x3f3f46ff),
                Vec4f::from_u32(0x52525bff),
                Vec4f::from_u32(0x71717aff),
                Vec4f::from_u32(0xfafafaff),
                Vec4f::from_u32(0x27272aff),
                Vec4f::from_u32(0x3f3f46ff),
                Vec4f::from_u32(0xfafafaff),
                Vec4f::from_u32(0x27272aff),
                Vec4f::from_u32(0xfafafaff),
            )
        };

        for entry in catalog::entries() {
            let is_active = entry.page == self.current_page;
            let mut item = self.ui.widget(cx, &[entry.sidebar_id]);
            script_apply_eval!(cx, item, {
                draw_bg +: {
                    color: #(if is_active { active_bg } else { Vec4f::all(0.0) })
                    color_hover: #(if is_active { active_bg_hover } else { inactive_bg_hover })
                    color_down: #(if is_active { active_bg_down } else { inactive_bg_down })
                    color_focus: #(if is_active { active_bg_hover } else { inactive_focus_bg })
                }
                draw_text +: {
                    color: #(if is_active { active_text } else { inactive_text })
                    color_hover: #(active_text)
                    color_down: #(active_text)
                    color_focus: #(if is_active { active_text } else { inactive_focus_text })
                }
            });
            item.redraw(cx);
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
        self.apply_responsive_visibility(cx);
        self.sync_theme_toggle_copy(cx);
        self.set_current_page(cx, self.current_page);
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
            .view(cx, ids!(desktop_header))
            .set_visible(cx, !self.is_small_screen);
        self.ui
            .view(cx, ids!(mobile_header))
            .set_visible(cx, self.is_small_screen);
        self.ui
            .view(cx, ids!(sidebar))
            .set_visible(cx, !self.is_small_screen || self.sidebar_open);
        self.ui
            .view(cx, ids!(main_content))
            .set_visible(cx, !self.is_small_screen || !self.sidebar_open);
        self.sync_mobile_sidebar_button(cx);
        self.sync_sidebar_focus_behavior(cx);
    }

    fn update_screen_mode(&mut self, cx: &mut Cx, window_width: f64) {
        let is_small_screen = window_width < Self::SMALL_SCREEN_WIDTH;
        if self.is_small_screen != is_small_screen {
            self.is_small_screen = is_small_screen;
            self.sidebar_open = !is_small_screen;
        }
        self.apply_responsive_visibility(cx);
    }

    fn handle_sidebar_navigation(&mut self, cx: &mut Cx, actions: &Actions) {
        for entry in catalog::entries() {
            if self.ui.button(cx, &[entry.sidebar_id]).clicked(actions) {
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
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let palette = self.ui.widget_flood(cx, ids!(command_palette));
        if let Some(page) = actions
            .find_widget_action(palette.widget_uid())
            .and_then(|action| match action.cast() {
                ui::command_palette::GalleryCommandPaletteAction::Selected(page) => Some(page),
                _ => None,
            })
        {
            self.set_current_page(cx, page);
        }
        if self.is_small_screen
            && (self
                .ui
                .button(cx, ids!(mobile_sidebar_menu_button))
                .clicked(actions)
                || self
                    .ui
                    .button(cx, ids!(mobile_sidebar_close_button))
                    .clicked(actions))
        {
            self.sidebar_open = !self.sidebar_open;
            self.apply_responsive_visibility(cx);
        }
        if self
            .ui
            .button(cx, ids!(desktop_theme_toggle))
            .clicked(actions)
            || self
                .ui
                .button(cx, ids!(mobile_theme_toggle))
                .clicked(actions)
        {
            self.queue_theme_change(cx, !self.is_light_theme);
        }
        if self
            .ui
            .button(cx, ids!(desktop_command_palette_trigger))
            .clicked(actions)
            || self
                .ui
                .button(cx, ids!(mobile_command_palette_trigger))
                .clicked(actions)
        {
            self.open_command_palette(cx);
        }

        self.handle_sidebar_navigation(cx, actions);
        if self
            .ui
            .gallery_command_palette_page(cx, ids!(command_palette_page))
            .open_requested(actions)
        {
            self.open_command_palette(cx);
        }
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
                self.current_page = catalog::default_page();
                self.is_light_theme = false;
                self.pending_theme = None;
                self.theme_reload_next_frame = NextFrame::default();
                self.apply_responsive_visibility(cx);
                self.sync_theme_toggle_copy(cx);
                self.set_current_page(cx, self.current_page);
            }
            Event::NextFrame(_) => {
                if self.theme_reload_next_frame.is_event(event).is_some() {
                    self.theme_reload_next_frame = NextFrame::default();
                    if let Some(is_light_theme) = self.pending_theme.take() {
                        self.set_theme(cx, is_light_theme);
                        return;
                    }
                }
            }
            Event::MacosMenuCommand(command) => {
                if *command == live_id!(command_palette_menu) {
                    self.open_command_palette(cx);
                    return;
                }
            }
            Event::WindowGeomChange(geom) => {
                self.update_screen_mode(cx, geom.new_geom.inner_size.x)
            }
            Event::KeyDown(key_event) => {
                if key_event.key_code == KeyCode::KeyK
                    && (key_event.modifiers.logo || key_event.modifiers.control)
                {
                    self.toggle_command_palette(cx);
                    return;
                }
            }
            _ => {}
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
