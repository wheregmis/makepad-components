pub use makepad_code_editor;
pub use makepad_widgets;

mod ui;

use crate::ui::command_palette::GalleryCommandPalette;
use crate::ui::command_palette_page::GalleryCommandPalettePageWidgetRefExt;
use makepad_components::makepad_widgets::*;
use makepad_router::RouterWidgetWidgetRefExt;

const SIDEBAR_ROUTES: &[(LiveId, LiveId)] = &[
    (live_id!(sidebar_accordion), live_id!(accordion_page)),
    (live_id!(sidebar_alert), live_id!(alert_page)),
    (live_id!(sidebar_aspect_ratio), live_id!(aspect_ratio_page)),
    (live_id!(sidebar_avatar), live_id!(avatar_page)),
    (live_id!(sidebar_badge), live_id!(badge_page)),
    (live_id!(sidebar_breadcrumb), live_id!(breadcrumb_page)),
    (live_id!(sidebar_button), live_id!(button_page)),
    (live_id!(sidebar_button_group), live_id!(button_group_page)),
    (live_id!(sidebar_card), live_id!(card_page)),
    (live_id!(sidebar_carousel), live_id!(carousel_page)),
    (live_id!(sidebar_checkbox), live_id!(checkbox_page)),
    (live_id!(sidebar_collapsible), live_id!(collapsible_page)),
    (
        live_id!(sidebar_command_palette),
        live_id!(command_palette_page),
    ),
    (live_id!(sidebar_context_menu), live_id!(context_menu_page)),
    (live_id!(sidebar_dialog), live_id!(dialog_page)),
    (live_id!(sidebar_input), live_id!(input_page)),
    (live_id!(sidebar_input_otp), live_id!(input_otp_page)),
    (live_id!(sidebar_menubar), live_id!(menubar_page)),
    (
        live_id!(sidebar_navigation_menu),
        live_id!(navigation_menu_page),
    ),
    (live_id!(sidebar_pagination), live_id!(pagination_page)),
    (live_id!(sidebar_popover), live_id!(popover_page)),
    (live_id!(sidebar_radio_group), live_id!(radio_group_page)),
    (live_id!(sidebar_resizable), live_id!(resizable_page)),
    (live_id!(sidebar_scroll_area), live_id!(scroll_area_page)),
    (live_id!(sidebar_select), live_id!(select_page)),
    (live_id!(sidebar_separator), live_id!(separator_page)),
    (live_id!(sidebar_sheet), live_id!(sheet_page)),
    (live_id!(sidebar_skeleton), live_id!(skeleton_page)),
    (live_id!(sidebar_switch), live_id!(switch_page)),
    (live_id!(sidebar_tabs), live_id!(tabs_page)),
    (live_id!(sidebar_textarea), live_id!(textarea_page)),
    (live_id!(sidebar_toggle), live_id!(toggle_page)),
    (live_id!(sidebar_kbd), live_id!(kbd_page)),
    (live_id!(sidebar_label), live_id!(label_page)),
    (live_id!(sidebar_progress), live_id!(progress_page)),
    (live_id!(sidebar_sidebar), live_id!(sidebar_page)),
    (live_id!(sidebar_slider), live_id!(slider_page)),
    (live_id!(sidebar_sonner), live_id!(sonner_page)),
    (live_id!(sidebar_spinner), live_id!(spinner_page)),
];

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(App::script_component(vm)){
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
        self.ui.button(cx, ids!(mobile_sidebar_button)).set_text(
            cx,
            if self.is_small_screen && self.sidebar_open {
                "X"
            } else {
                "☰"
            },
        );
    }

    fn sync_theme_toggle_labels(&self, cx: &mut Cx) {
        let button_text = if self.is_light_theme {
            "Dark mode"
        } else {
            "Light mode"
        };
        self.ui
            .button(cx, ids!(sidebar_theme_toggle))
            .set_text(cx, button_text);
        self.ui
            .button(cx, ids!(mobile_theme_toggle))
            .set_text(cx, button_text);
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
        self.sync_theme_toggle_labels(cx);
        self.apply_responsive_visibility(cx);
        self.set_current_page(cx, self.current_page);
        self.ui.redraw(cx);
    }

    fn set_theme(&mut self, cx: &mut Cx, is_light_theme: bool) {
        self.is_light_theme = is_light_theme;
        self.reload_ui_for_theme(cx);
    }

    fn apply_responsive_visibility(&mut self, cx: &mut Cx) {
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
        for (button_id, page_id) in SIDEBAR_ROUTES {
            if self.ui.button(cx, &[*button_id]).clicked(actions) {
                self.set_current_page(cx, *page_id);
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
        if self
            .ui
            .button(cx, ids!(mobile_sidebar_button))
            .clicked(actions)
            && self.is_small_screen
        {
            self.sidebar_open = !self.sidebar_open;
            self.apply_responsive_visibility(cx);
        }
        if self
            .ui
            .button(cx, ids!(sidebar_theme_toggle))
            .clicked(actions)
            || self
                .ui
                .button(cx, ids!(mobile_theme_toggle))
                .clicked(actions)
        {
            self.set_theme(cx, !self.is_light_theme);
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
                self.current_page = live_id!(accordion_page);
                self.is_light_theme = false;
                self.sync_theme_toggle_labels(cx);
                self.apply_responsive_visibility(cx);
                self.set_current_page(cx, self.current_page);
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
