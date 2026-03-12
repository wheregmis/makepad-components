pub use makepad_widgets;

mod ui;

use crate::ui::command_palette::GalleryCommandPalette;
use makepad_components::context_menu::ShadContextMenu;
use makepad_components::input_otp::ShadInputOtp;
use makepad_components::makepad_widgets::*;
use makepad_components::sheet::ShadSheet;
use makepad_components::{ShadCarousel, ShadDialog, ShadSonner};
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
        makepad_router::script_mod(vm);
        crate::ui::script_mod(vm);
        self::script_mod(vm)
    }

    fn widget_button_clicked(ui: &WidgetRef, cx: &Cx, actions: &Actions, path: &[LiveId]) -> bool {
        let widget = ui.widget_flood(cx, path);
        actions
            .find_widget_action(widget.widget_uid())
            .is_some_and(|action| matches!(action.cast(), ButtonAction::Clicked(_)))
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

    fn set_preview_mode(
        ui: &WidgetRef,
        cx: &mut Cx,
        preview_flip: &[LiveId],
        demo_indicator: &[LiveId],
        code_indicator: &[LiveId],
        show_code: bool,
    ) {
        Self::set_gallery_flip_page(
            ui,
            cx,
            preview_flip,
            if show_code {
                live_id!(code_page)
            } else {
                live_id!(root_view)
            },
        );
        ui.view(cx, demo_indicator).set_visible(cx, !show_code);
        ui.view(cx, code_indicator).set_visible(cx, show_code);
    }

    fn set_gallery_flip_page(ui: &WidgetRef, cx: &mut Cx, flip: &[LiveId], page: LiveId) {
        let widget = ui.widget_flood(cx, flip);
        if let Some(mut page_flip) = widget.borrow_mut::<PageFlip>() {
            page_flip.set_active_page(cx, page);
            return;
        }
        ui.router_widget(cx, flip).go_to_route(cx, page);
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_preview_tabs(
        ui: &WidgetRef,
        cx: &mut Cx,
        actions: &Actions,
        demo_button: &[LiveId],
        code_button: &[LiveId],
        flip: &[LiveId],
        demo_indicator: &[LiveId],
        code_indicator: &[LiveId],
    ) {
        if Self::widget_button_clicked(ui, cx, actions, demo_button) {
            Self::set_preview_mode(ui, cx, flip, demo_indicator, code_indicator, false);
        }

        if Self::widget_button_clicked(ui, cx, actions, code_button) {
            Self::set_preview_mode(ui, cx, flip, demo_indicator, code_indicator, true);
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
        if let Some(mut carousel) = self
            .ui
            .widget_flood(cx, ids!(carousel_demo))
            .borrow_mut::<ShadCarousel>()
        {
            carousel.handle_actions(cx, actions);
        }
        if self
            .ui
            .button(cx, ids!(open_command_palette_btn))
            .clicked(actions)
        {
            self.open_command_palette(cx);
        }
        if self.ui.button(cx, ids!(open_dialog_btn)).clicked(actions) {
            if let Some(mut d) = self
                .ui
                .widget_flood(cx, ids!(default_dialog))
                .borrow_mut::<ShadDialog>()
            {
                d.set_open(true);
            }
        }
        if self.ui.button(cx, ids!(open_default_btn)).clicked(actions) {
            if let Some(mut d) = self
                .ui
                .widget_flood(cx, ids!(alert_default_dialog))
                .borrow_mut::<ShadDialog>()
            {
                d.set_open(true);
            }
        }
        if self
            .ui
            .button(cx, ids!(open_destructive_btn))
            .clicked(actions)
        {
            if let Some(mut d) = self
                .ui
                .widget_flood(cx, ids!(destructive_dialog))
                .borrow_mut::<ShadDialog>()
            {
                d.set_open(true);
            }
        }
        let dialog_ref = self.ui.widget_flood(cx, ids!(default_dialog));
        if !dialog_ref.is_empty() && dialog_ref.button(cx, ids!(close_btn)).clicked(actions) {
            if let Some(mut d) = dialog_ref.borrow_mut::<ShadDialog>() {
                d.set_open(false);
            }
        }
        if self.ui.button(cx, ids!(toast_event_btn)).clicked(actions) {
            if let Some(mut s) = self
                .ui
                .widget_flood(cx, ids!(toast_event))
                .borrow_mut::<ShadSonner>()
            {
                s.set_open(true);
            }
        }
        if self.ui.button(cx, ids!(toast_desc_btn)).clicked(actions) {
            if let Some(mut s) = self
                .ui
                .widget_flood(cx, ids!(toast_desc))
                .borrow_mut::<ShadSonner>()
            {
                s.set_open(true);
            }
        }
        if self.ui.button(cx, ids!(toast_close_btn)).clicked(actions) {
            if let Some(mut s) = self
                .ui
                .widget_flood(cx, ids!(toast_close))
                .borrow_mut::<ShadSonner>()
            {
                s.set_open(true);
            }
        }
        if let Some(index) = self
            .ui
            .widget_flood(cx, ids!(context_menu_basic))
            .borrow::<ShadContextMenu>()
            .and_then(|inner| inner.selected(actions))
        {
            let label = match index {
                0 => "Open",
                1 => "Duplicate",
                2 => "Share",
                3 => "Delete",
                _ => "Unknown",
            };
            self.ui
                .label(cx, ids!(context_menu_status))
                .set_text(cx, &format!("Selected: {}", label));
        }
        let otp_demo = self.ui.widget_flood(cx, ids!(otp_demo));
        let otp_state = otp_demo.borrow::<ShadInputOtp>().map(|inner| {
            (
                inner.completed(actions),
                inner.changed(actions),
                inner.value().to_string(),
            )
        });
        if let Some((completed, changed, current_value)) = otp_state {
            let status = if let Some(value) = completed {
                format!("Completed: {}", value)
            } else if let Some(value) = changed {
                format!("Current value: {}", value)
            } else if current_value.len() >= 6 {
                format!("Completed: {}", current_value)
            } else if !current_value.is_empty() {
                format!("Current value: {}", current_value)
            } else {
                "Waiting for input.".to_string()
            };
            self.ui.label(cx, ids!(otp_status)).set_text(cx, &status);
        }

        if self
            .ui
            .button(cx, ids!(tabs_overview_trigger))
            .clicked(actions)
        {
            Self::set_gallery_flip_page(
                &self.ui,
                cx,
                ids!(tabs_content_flip),
                live_id!(overview_page),
            );
            self.ui
                .view(cx, ids!(tabs_overview_indicator))
                .set_visible(cx, true);
            self.ui
                .view(cx, ids!(tabs_usage_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_settings_indicator))
                .set_visible(cx, false);
        }
        if self
            .ui
            .button(cx, ids!(tabs_usage_trigger))
            .clicked(actions)
        {
            Self::set_gallery_flip_page(
                &self.ui,
                cx,
                ids!(tabs_content_flip),
                live_id!(usage_page),
            );
            self.ui
                .view(cx, ids!(tabs_overview_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_usage_indicator))
                .set_visible(cx, true);
            self.ui
                .view(cx, ids!(tabs_settings_indicator))
                .set_visible(cx, false);
        }
        if self
            .ui
            .button(cx, ids!(tabs_settings_trigger))
            .clicked(actions)
        {
            Self::set_gallery_flip_page(
                &self.ui,
                cx,
                ids!(tabs_content_flip),
                live_id!(settings_page),
            );
            self.ui
                .view(cx, ids!(tabs_overview_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_usage_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_settings_indicator))
                .set_visible(cx, true);
        }

        if self
            .ui
            .button(cx, ids!(open_right_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(right_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self
            .ui
            .button(cx, ids!(open_left_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(left_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self
            .ui
            .button(cx, ids!(open_top_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(top_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self
            .ui
            .button(cx, ids!(open_bottom_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(bottom_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        for (path, button) in [
            (ids!(right_sheet), ids!(close_right_sheet_btn)),
            (ids!(left_sheet), ids!(close_left_sheet_btn)),
            (ids!(top_sheet), ids!(close_top_sheet_btn)),
            (ids!(bottom_sheet), ids!(close_bottom_sheet_btn)),
        ] {
            if self.ui.button(cx, button).clicked(actions) {
                let sheet = self.ui.widget_flood(cx, path);
                if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                    s.set_open(false);
                }
                sheet.redraw(cx);
            }
        }

        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(accordion_preview_section.demo_tab),
            ids!(accordion_preview_section.code_tab),
            ids!(accordion_preview_section.preview_flip),
            ids!(accordion_preview_section.demo_indicator),
            ids!(accordion_preview_section.code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(alert_preview_section.demo_tab),
            ids!(alert_preview_section.code_tab),
            ids!(alert_preview_section.preview_flip),
            ids!(alert_preview_section.demo_indicator),
            ids!(alert_preview_section.code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(aspect_ratio_demo_tab),
            ids!(aspect_ratio_code_tab),
            ids!(aspect_ratio_preview_flip),
            ids!(aspect_ratio_demo_indicator),
            ids!(aspect_ratio_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(avatar_demo_tab),
            ids!(avatar_code_tab),
            ids!(avatar_preview_flip),
            ids!(avatar_demo_indicator),
            ids!(avatar_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(badge_demo_tab),
            ids!(badge_code_tab),
            ids!(badge_preview_flip),
            ids!(badge_demo_indicator),
            ids!(badge_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(breadcrumb_demo_tab),
            ids!(breadcrumb_code_tab),
            ids!(breadcrumb_preview_flip),
            ids!(breadcrumb_demo_indicator),
            ids!(breadcrumb_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(button_demo_tab),
            ids!(button_code_tab),
            ids!(button_preview_flip),
            ids!(button_demo_indicator),
            ids!(button_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(button_group_demo_tab),
            ids!(button_group_code_tab),
            ids!(button_group_preview_flip),
            ids!(button_group_demo_indicator),
            ids!(button_group_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(card_demo_tab),
            ids!(card_code_tab),
            ids!(card_preview_flip),
            ids!(card_demo_indicator),
            ids!(card_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(carousel_demo_tab),
            ids!(carousel_code_tab),
            ids!(carousel_preview_flip),
            ids!(carousel_demo_indicator),
            ids!(carousel_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(checkbox_demo_tab),
            ids!(checkbox_code_tab),
            ids!(checkbox_preview_flip),
            ids!(checkbox_demo_indicator),
            ids!(checkbox_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(collapsible_demo_tab),
            ids!(collapsible_code_tab),
            ids!(collapsible_preview_flip),
            ids!(collapsible_demo_indicator),
            ids!(collapsible_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(command_palette_demo_tab),
            ids!(command_palette_code_tab),
            ids!(command_palette_preview_flip),
            ids!(command_palette_demo_indicator),
            ids!(command_palette_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(context_menu_demo_tab),
            ids!(context_menu_code_tab),
            ids!(context_menu_preview_flip),
            ids!(context_menu_demo_indicator),
            ids!(context_menu_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(dialog_demo_tab),
            ids!(dialog_code_tab),
            ids!(dialog_preview_flip),
            ids!(dialog_demo_indicator),
            ids!(dialog_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(input_demo_tab),
            ids!(input_code_tab),
            ids!(input_preview_flip),
            ids!(input_demo_indicator),
            ids!(input_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(input_otp_demo_tab),
            ids!(input_otp_code_tab),
            ids!(input_otp_preview_flip),
            ids!(input_otp_demo_indicator),
            ids!(input_otp_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(kbd_demo_tab),
            ids!(kbd_code_tab),
            ids!(kbd_preview_flip),
            ids!(kbd_demo_indicator),
            ids!(kbd_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(label_demo_tab),
            ids!(label_code_tab),
            ids!(label_preview_flip),
            ids!(label_demo_indicator),
            ids!(label_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(progress_demo_tab),
            ids!(progress_code_tab),
            ids!(progress_preview_flip),
            ids!(progress_demo_indicator),
            ids!(progress_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(radio_group_demo_tab),
            ids!(radio_group_code_tab),
            ids!(radio_group_preview_flip),
            ids!(radio_group_demo_indicator),
            ids!(radio_group_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(resizable_demo_tab),
            ids!(resizable_code_tab),
            ids!(resizable_preview_flip),
            ids!(resizable_demo_indicator),
            ids!(resizable_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(scroll_area_demo_tab),
            ids!(scroll_area_code_tab),
            ids!(scroll_area_preview_flip),
            ids!(scroll_area_demo_indicator),
            ids!(scroll_area_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(select_demo_tab),
            ids!(select_code_tab),
            ids!(select_preview_flip),
            ids!(select_demo_indicator),
            ids!(select_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(separator_demo_tab),
            ids!(separator_code_tab),
            ids!(separator_preview_flip),
            ids!(separator_demo_indicator),
            ids!(separator_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(sheet_demo_tab),
            ids!(sheet_code_tab),
            ids!(sheet_preview_flip),
            ids!(sheet_demo_indicator),
            ids!(sheet_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(skeleton_demo_tab),
            ids!(skeleton_code_tab),
            ids!(skeleton_preview_flip),
            ids!(skeleton_demo_indicator),
            ids!(skeleton_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(switch_demo_tab),
            ids!(switch_code_tab),
            ids!(switch_preview_flip),
            ids!(switch_demo_indicator),
            ids!(switch_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(sidebar_demo_tab),
            ids!(sidebar_code_tab),
            ids!(sidebar_preview_flip),
            ids!(sidebar_demo_indicator),
            ids!(sidebar_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(slider_demo_tab),
            ids!(slider_code_tab),
            ids!(slider_preview_flip),
            ids!(slider_demo_indicator),
            ids!(slider_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(sonner_demo_tab),
            ids!(sonner_code_tab),
            ids!(sonner_preview_flip),
            ids!(sonner_demo_indicator),
            ids!(sonner_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(spinner_demo_tab),
            ids!(spinner_code_tab),
            ids!(spinner_preview_flip),
            ids!(spinner_demo_indicator),
            ids!(spinner_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(tabs_demo_tab),
            ids!(tabs_code_tab),
            ids!(tabs_preview_flip),
            ids!(tabs_demo_indicator),
            ids!(tabs_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(textarea_demo_tab),
            ids!(textarea_code_tab),
            ids!(textarea_preview_flip),
            ids!(textarea_demo_indicator),
            ids!(textarea_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(toggle_demo_tab),
            ids!(toggle_code_tab),
            ids!(toggle_preview_flip),
            ids!(toggle_demo_indicator),
            ids!(toggle_code_indicator),
        );
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
