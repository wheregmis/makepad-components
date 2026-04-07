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

    fn is_mobile_width(width: f64) -> bool {
        width.is_finite() && width > 0.0 && width < Self::SMALL_SCREEN_WIDTH
    }

    fn is_mobile_layout(&self, cx: &Cx) -> bool {
        Self::is_mobile_width(cx.display_context.screen_size.x)
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
        self.ui
            .router_widget(cx, ids!(content_flip))
            .go_to_route(cx, self.current_page);
    }

    fn configure_adaptive_views(&self, cx: &mut Cx) {
        self.ui
            .adaptive_view(cx, ids!(responsive_header))
            .set_variant_selector(|cx, _parent_size| {
                if Self::is_mobile_width(cx.display_context.screen_size.x) {
                    live_id!(Mobile)
                } else {
                    live_id!(Desktop)
                }
            });
        self.ui
            .adaptive_view(cx, ids!(responsive_sidebar))
            .set_variant_selector(|cx, _parent_size| {
                if Self::is_mobile_width(cx.display_context.screen_size.x) {
                    live_id!(Mobile)
                } else {
                    live_id!(Desktop)
                }
            });
    }

    fn set_current_page(&mut self, cx: &mut Cx, page: LiveId) {
        self.current_page = page;
        self.sync_content_route(cx);
        self.sync_page_metadata(cx);
        if self.is_mobile_layout(cx) && self.sidebar_open {
            self.set_mobile_sidebar_open(cx, false);
        } else {
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

    fn sync_mobile_sidebar_button_for(&self, cx: &mut Cx, is_mobile: bool) {
        let show_close = is_mobile && self.sidebar_open;
        self.ui
            .view(
                cx,
                &[
                    live_id!(responsive_header),
                    live_id!(Mobile),
                    live_id!(mobile_sidebar_menu_button),
                ],
            )
            .set_visible(cx, !show_close);
        self.ui
            .view(
                cx,
                &[
                    live_id!(mobile_sidebar_panel),
                    live_id!(sidebar_mobile),
                    live_id!(mobile_sidebar_close_button),
                ],
            )
            .set_visible(cx, show_close);
    }

    fn apply_sidebar_layout_for(&self, cx: &mut Cx, is_mobile: bool) {
        self.ui
            .view(cx, ids!(mobile_sidebar_backdrop))
            .set_visible(cx, is_mobile && self.sidebar_open);
        self.ui.view(cx, ids!(main_content)).set_visible(cx, true);
    }

    fn sync_theme_toggle_copy(&self, cx: &mut Cx) {
        if self.is_mobile_layout(cx) {
            self.ui
                .view(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Mobile),
                        live_id!(mobile_theme_toggle_sun),
                    ],
                )
                .set_visible(cx, self.is_light_theme);
            self.ui
                .view(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Mobile),
                        live_id!(mobile_theme_toggle_moon),
                    ],
                )
                .set_visible(cx, !self.is_light_theme);
        } else {
            self.ui
                .view(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Desktop),
                        live_id!(desktop_theme_toggle_sun),
                    ],
                )
                .set_visible(cx, self.is_light_theme);
            self.ui
                .view(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Desktop),
                        live_id!(desktop_theme_toggle_moon),
                    ],
                )
                .set_visible(cx, !self.is_light_theme);
        }
    }

    fn sync_page_metadata(&self, cx: &mut Cx) {
        if let Some(entry) = catalog::entry_for_page(self.current_page) {
            if self.is_mobile_layout(cx) {
                self.ui
                    .label(
                        cx,
                        &[
                            live_id!(responsive_header),
                            live_id!(Mobile),
                            live_id!(mobile_page_label),
                        ],
                    )
                    .set_text(cx, entry.title);
            } else {
                self.ui
                    .label(
                        cx,
                        &[
                            live_id!(responsive_header),
                            live_id!(Desktop),
                            live_id!(desktop_page_label),
                        ],
                    )
                    .set_text(cx, entry.title);
            }
        }
        // Optimization: prevent redundant script evaluations on page navigation
        // Previously: `self.sync_sidebar_focus_behavior(cx)` re-evaluated scripts for ~50 sidebar items on every page click
        // Now: focus behavior is strictly managed by screen size changes (`apply_responsive_visibility`), eliminating 50 macro evals per click
        self.sync_sidebar_selection(cx);
    }

    fn sync_sidebar_focus_behavior_for(&self, cx: &mut Cx, is_mobile: bool) {
        let allow_sidebar_focus = !is_mobile || self.sidebar_open;

        for entry in catalog::entries() {
            if is_mobile {
                let mut mobile_item = self.ui.button(
                    cx,
                    &[
                        live_id!(mobile_sidebar_panel),
                        live_id!(sidebar_mobile),
                        entry.sidebar_id,
                    ],
                );
                script_apply_eval!(cx, mobile_item, {
                    grab_key_focus: #(allow_sidebar_focus)
                });
            } else {
                let mut desktop_item = self.ui.button(
                    cx,
                    &[
                        live_id!(responsive_sidebar),
                        live_id!(Desktop),
                        live_id!(sidebar_shell),
                        live_id!(sidebar_desktop),
                        entry.sidebar_id,
                    ],
                );
                script_apply_eval!(cx, desktop_item, {
                    grab_key_focus: #(true)
                });
            }
        }
    }

    fn clear_mobile_sidebar_focus(&self, cx: &mut Cx) {
        let close_button_has_focus = self
            .ui
            .button(
                cx,
                &[
                    live_id!(mobile_sidebar_panel),
                    live_id!(sidebar_mobile),
                    live_id!(mobile_sidebar_close_button),
                    live_id!(button),
                ],
            )
            .key_focus(cx);
        let sidebar_item_has_focus = catalog::entries().iter().any(|entry| {
            self.ui
                .widget(
                    cx,
                    &[
                        live_id!(mobile_sidebar_panel),
                        live_id!(sidebar_mobile),
                        entry.sidebar_id,
                    ],
                )
                .key_focus(cx)
        });

        if close_button_has_focus || sidebar_item_has_focus {
            cx.set_key_focus(Area::Empty);
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
            if self.is_mobile_layout(cx) {
                let mut mobile_item = self.ui.widget(
                    cx,
                    &[
                        live_id!(mobile_sidebar_panel),
                        live_id!(sidebar_mobile),
                        entry.sidebar_id,
                    ],
                );
                script_apply_eval!(cx, mobile_item, {
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
                mobile_item.redraw(cx);
            } else {
                let mut desktop_item = self.ui.widget(
                    cx,
                    &[
                        live_id!(responsive_sidebar),
                        live_id!(Desktop),
                        live_id!(sidebar_shell),
                        live_id!(sidebar_desktop),
                        entry.sidebar_id,
                    ],
                );
                script_apply_eval!(cx, desktop_item, {
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
                desktop_item.redraw(cx);
            }
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
        self.configure_adaptive_views(cx);
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

    fn sync_safe_area_padding_for(&self, cx: &mut Cx, is_mobile: bool) {
        let _ = (cx, is_mobile);
    }

    fn apply_responsive_visibility(&mut self, cx: &mut Cx) {
        self.apply_responsive_visibility_for(cx, self.is_mobile_layout(cx));
    }

    fn apply_responsive_visibility_for(&mut self, cx: &mut Cx, is_mobile: bool) {
        self.configure_adaptive_views(cx);
        self.sync_safe_area_padding_for(cx, is_mobile);
        self.apply_sidebar_layout_for(cx, is_mobile);
        self.sync_mobile_sidebar_button_for(cx, is_mobile);
        self.sync_sidebar_focus_behavior_for(cx, is_mobile);
        self.sync_page_metadata(cx);
        self.sync_content_route(cx);
    }

    fn set_mobile_sidebar_open(&mut self, cx: &mut Cx, open: bool) {
        self.sidebar_open = open;
        if !open {
            self.clear_mobile_sidebar_focus(cx);
        }
        let panel = self.ui.slide_panel(cx, ids!(mobile_sidebar_panel));
        if open {
            panel.open(cx);
        } else {
            panel.close(cx);
        }
        self.apply_responsive_visibility(cx);
    }

    fn handle_window_geom_change(&mut self, cx: &mut Cx, old_width: f64, new_width: f64) {
        let was_mobile = Self::is_mobile_width(old_width);
        let is_mobile = Self::is_mobile_width(new_width);
        if was_mobile != is_mobile {
            self.sidebar_open = false;
            self.ui
                .slide_panel(cx, ids!(mobile_sidebar_panel))
                .close(cx);
            self.responsive_layout_next_frame = cx.new_next_frame();
        } else if is_mobile {
            self.sync_mobile_sidebar_button_for(cx, true);
        }
    }

    fn handle_sidebar_navigation(&mut self, cx: &mut Cx, actions: &Actions) {
        for entry in catalog::entries() {
            let desktop_clicked = self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(responsive_sidebar),
                        live_id!(Desktop),
                        live_id!(sidebar_shell),
                        live_id!(sidebar_desktop),
                        entry.sidebar_id,
                    ],
                )
                .clicked(actions);
            let mobile_clicked = self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(mobile_sidebar_panel),
                        live_id!(sidebar_mobile),
                        entry.sidebar_id,
                    ],
                )
                .clicked(actions);

            if desktop_clicked || mobile_clicked {
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
    responsive_layout_next_frame: NextFrame,
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
        if self.is_mobile_layout(cx)
            && (self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Mobile),
                        live_id!(mobile_sidebar_menu_button),
                        live_id!(button),
                    ],
                )
                .clicked(actions)
                || self
                    .ui
                    .button(
                        cx,
                        &[
                            live_id!(mobile_sidebar_panel),
                            live_id!(sidebar_mobile),
                            live_id!(mobile_sidebar_close_button),
                            live_id!(button),
                        ],
                    )
                    .clicked(actions)
                || self
                    .ui
                    .button(cx, ids!(mobile_sidebar_backdrop))
                    .clicked(actions))
        {
            self.set_mobile_sidebar_open(cx, !self.sidebar_open);
        }
        if self
            .ui
            .button(
                cx,
                &[
                    live_id!(responsive_header),
                    live_id!(Desktop),
                    live_id!(desktop_theme_toggle_sun),
                    live_id!(button),
                ],
            )
            .clicked(actions)
            || self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Desktop),
                        live_id!(desktop_theme_toggle_moon),
                        live_id!(button),
                    ],
                )
                .clicked(actions)
            || self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Mobile),
                        live_id!(mobile_theme_toggle_sun),
                        live_id!(button),
                    ],
                )
                .clicked(actions)
            || self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Mobile),
                        live_id!(mobile_theme_toggle_moon),
                        live_id!(button),
                    ],
                )
                .clicked(actions)
        {
            self.queue_theme_change(cx, !self.is_light_theme);
        }
        if self
            .ui
            .button(
                cx,
                &[
                    live_id!(responsive_header),
                    live_id!(Desktop),
                    live_id!(desktop_command_palette_trigger),
                ],
            )
            .clicked(actions)
            || self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(responsive_header),
                        live_id!(Mobile),
                        live_id!(mobile_command_palette_trigger),
                    ],
                )
                .clicked(actions)
            || self
                .ui
                .button(
                    cx,
                    &[
                        live_id!(mobile_sidebar_panel),
                        live_id!(sidebar_mobile),
                        live_id!(mobile_sidebar_search),
                    ],
                )
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
                self.sidebar_open = false;
                self.current_page = catalog::default_page();
                self.is_light_theme = false;
                self.pending_theme = None;
                self.theme_reload_next_frame = NextFrame::default();
                self.responsive_layout_next_frame = NextFrame::default();
                self.apply_responsive_visibility(cx);
                self.sync_theme_toggle_copy(cx);
                self.set_current_page(cx, self.current_page);
            }
            Event::NextFrame(_) => {
                if self.responsive_layout_next_frame.is_event(event).is_some() {
                    self.responsive_layout_next_frame = NextFrame::default();
                    self.apply_responsive_visibility(cx);
                }
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
                }
            }
            Event::WindowGeomChange(geom) => self.handle_window_geom_change(
                cx,
                geom.old_geom.inner_size.x,
                geom.new_geom.inner_size.x,
            ),
            Event::KeyDown(key_event) => {
                if key_event.key_code == KeyCode::KeyK
                    && (key_event.modifiers.logo || key_event.modifiers.control)
                {
                    // Keep forwarding the shortcut event through the UI tree so the
                    // palette's overlay state settles in the same dispatch cycle as
                    // button-driven opens.
                    self.toggle_command_palette(cx);
                }
            }
            _ => {}
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
