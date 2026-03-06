use makepad_components::makepad_widgets::*;
use makepad_components::drawer::ShadDrawer;
use makepad_components::sheet::ShadSheet;
use makepad_components::{ShadCarousel, ShadDialog, ShadSonner};

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

    fn apply_responsive_visibility(&mut self, cx: &mut Cx) {
        self.ui
            .view(cx, ids!(mobile_header))
            .set_visible(cx, self.is_small_screen);
        self.ui.view(cx, ids!(sidebar)).set_visible(
            cx,
            !self.is_small_screen || self.sidebar_open,
        );
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

    fn set_page(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        sidebar_button: &[LiveId],
        page: LiveId,
        content_flip: &PageFlipRef,
    ) {
        if self.ui.button(cx, sidebar_button).clicked(actions) {
            // Avoid cx.set_key_focus(Area::Empty) here - can trigger KeyFocusLost feedback loop
            // when TextInput or DropDown popup had focus (makepad-platform issue).
            content_flip.set_active_page(cx, page);
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, page);
            if self.is_small_screen {
                self.sidebar_open = false;
                self.apply_responsive_visibility(cx);
            }
        }
    }

    fn set_preview_mode(
        ui: &WidgetRef,
        cx: &mut Cx,
        flip: &[LiveId],
        demo_indicator: &[LiveId],
        code_indicator: &[LiveId],
        show_code: bool,
    ) {
        // Avoid cx.set_key_focus(Area::Empty) - can trigger KeyFocusLost feedback loop
        // when TextInput or DropDown had focus (makepad-platform issue).
        ui.page_flip(cx, flip).set_active_page(
            cx,
            if show_code {
                live_id!(code_page)
            } else {
                live_id!(demo_page)
            },
        );
        ui.view(cx, demo_indicator).set_visible(cx, !show_code);
        ui.view(cx, code_indicator).set_visible(cx, show_code);
    }

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
        if ui.button(cx, demo_button).clicked(actions) {
            Self::set_preview_mode(ui, cx, flip, demo_indicator, code_indicator, false);
        }

        if ui.button(cx, code_button).clicked(actions) {
            Self::set_preview_mode(ui, cx, flip, demo_indicator, code_indicator, true);
        }
    }

    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        makepad_code_editor::script_mod(vm);
        makepad_components::script_mod(vm);
        crate::ui::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    hover_card_open: bool,
    #[rust]
    is_small_screen: bool,
    #[rust]
    sidebar_open: bool,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let content_flip = self.ui.page_flip(cx, ids!(content_flip));
        if self.ui.button(cx, ids!(mobile_sidebar_button)).clicked(actions) && self.is_small_screen {
            self.sidebar_open = !self.sidebar_open;
            self.apply_responsive_visibility(cx);
        }

        // Sidebar → page mappings. When adding a new component:
        // 1) Add a ShadSidebarItem in GallerySidebar (sidebar::<name>)
        // 2) Add a matching page in GalleryContentFlip (<name>_page)
        // 3) Add a set_page call here with the same IDs.
        self.set_page(
            cx,
            actions,
            ids!(sidebar_accordion),
            live_id!(accordion_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_alert), live_id!(alert_page), &content_flip);
        if let Some(mut carousel) = self
            .ui
            .widget_flood(cx, ids!(carousel_demo))
            .borrow_mut::<ShadCarousel>()
        {
            carousel.handle_actions(cx, actions);
        }
        self.set_page(
            cx,
            actions,
            ids!(sidebar_aspect_ratio),
            live_id!(aspect_ratio_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_avatar), live_id!(avatar_page), &content_flip);
        self.set_page(cx, actions, ids!(sidebar_badge), live_id!(badge_page), &content_flip);
        self.set_page(
            cx,
            actions,
            ids!(sidebar_breadcrumb),
            live_id!(breadcrumb_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_button), live_id!(button_page), &content_flip);
        self.set_page(
            cx,
            actions,
            ids!(sidebar_button_group),
            live_id!(button_group_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_card), live_id!(card_page), &content_flip);
        self.set_page(cx, actions, ids!(sidebar_carousel), live_id!(carousel_page), &content_flip);
        self.set_page(cx, actions, ids!(sidebar_checkbox), live_id!(checkbox_page), &content_flip);
        self.set_page(
            cx,
            actions,
            ids!(sidebar_collapsible),
            live_id!(collapsible_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_dialog), live_id!(dialog_page), &content_flip);
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
        if self.ui.button(cx, ids!(open_destructive_btn)).clicked(actions) {
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
        self.set_page(cx, actions, ids!(sidebar_drawer), live_id!(drawer_page), &content_flip);
        if self.ui.button(cx, ids!(open_drawer_btn)).clicked(actions) {
            let drawer = self.ui.widget_flood(cx, ids!(drawer_demo));
            if let Some(mut d) = drawer.borrow_mut::<ShadDrawer>() {
                d.set_open(true);
            }
            drawer.redraw(cx);
        }
        self.set_page(
            cx,
            actions,
            ids!(sidebar_dropdown_menu),
            live_id!(dropdown_menu_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_hover_card),
            live_id!(hover_card_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_input), live_id!(input_page), &content_flip);
        self.set_page(
            cx,
            actions,
            ids!(sidebar_radio_group),
            live_id!(radio_group_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_resizable),
            live_id!(resizable_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_scroll_area),
            live_id!(scroll_area_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_select), live_id!(select_page), &content_flip);
        self.set_page(
            cx,
            actions,
            ids!(sidebar_separator),
            live_id!(separator_page),
            &content_flip,
        );
        self.set_page(cx, actions, ids!(sidebar_sheet), live_id!(sheet_page), &content_flip);
        self.set_page(
            cx,
            actions,
            ids!(sidebar_skeleton),
            live_id!(skeleton_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_switch),
            live_id!(switch_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_tabs),
            live_id!(tabs_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_tooltip),
            live_id!(tooltip_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_kbd),
            live_id!(kbd_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_label),
            live_id!(label_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_progress),
            live_id!(progress_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_sidebar),
            live_id!(sidebar_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_slider),
            live_id!(slider_page),
            &content_flip,
        );
        self.set_page(
            cx,
            actions,
            ids!(sidebar_sonner),
            live_id!(sonner_page),
            &content_flip,
        );
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
        self.set_page(cx, actions, ids!(sidebar_spinner), live_id!(spinner_page), &content_flip);

        if self.ui.button(cx, ids!(tooltip_basic_btn)).clicked(actions) {
            let trigger = self.ui.button(cx, ids!(tooltip_basic_btn));
            let content_rect = self.ui.view(cx, ids!(content_flip)).area().rect(cx);
            let trigger_rect = trigger.area().rect(cx);
            let pos = Vec2d {
                x: content_rect.pos.x + trigger_rect.pos.x,
                y: content_rect.pos.y + trigger_rect.pos.y + trigger_rect.size.y + 8.0,
            };
            self.ui
                .tooltip(cx, ids!(basic_tooltip))
                .show_with_options(cx, pos, "Helpful context for a nearby action.");
        }
        if self.ui.button(cx, ids!(tooltip_callout_btn)).clicked(actions) {
            let tooltip_ref = self.ui.widget_flood(cx, ids!(callout_tooltip));
            if !tooltip_ref.is_empty() {
                let trigger = self.ui.button(cx, ids!(tooltip_callout_btn));
                let content_rect = self.ui.view(cx, ids!(content_flip)).area().rect(cx);
                let trigger_rect = trigger.area().rect(cx);
                let rect = Rect {
                    pos: content_rect.pos + trigger_rect.pos,
                    size: trigger_rect.size,
                };
                if let Some(mut ct) = tooltip_ref.borrow_mut::<CalloutTooltip>() {
                    ct.show_with_options(
                        cx,
                        "Callout tooltips can point at the related control.",
                        rect,
                        CalloutTooltipOptions::default(),
                        false,
                    );
                }
            }
        }

        if self.ui.button(cx, ids!(tabs_overview_trigger)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(tabs_content_flip))
                .set_active_page(cx, live_id!(overview_page));
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
        if self.ui.button(cx, ids!(tabs_usage_trigger)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(tabs_content_flip))
                .set_active_page(cx, live_id!(usage_page));
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
        if self.ui.button(cx, ids!(tabs_settings_trigger)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(tabs_content_flip))
                .set_active_page(cx, live_id!(settings_page));
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

        if self.ui.button(cx, ids!(open_right_sheet_btn)).clicked(actions) {
            let sheet = self.ui.widget_flood(cx, ids!(right_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self.ui.button(cx, ids!(open_left_sheet_btn)).clicked(actions) {
            let sheet = self.ui.widget_flood(cx, ids!(left_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self.ui.button(cx, ids!(open_top_sheet_btn)).clicked(actions) {
            let sheet = self.ui.widget_flood(cx, ids!(top_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self.ui.button(cx, ids!(open_bottom_sheet_btn)).clicked(actions) {
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
            ids!(hover_card_demo_tab),
            ids!(hover_card_code_tab),
            ids!(hover_card_preview_flip),
            ids!(hover_card_demo_indicator),
            ids!(hover_card_code_indicator),
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
        let tooltip_ref = self.ui.widget_flood(cx, ids!(hover_card_tooltip));
        if self.ui.button(cx, ids!(hover_card_trigger)).clicked(actions) && !tooltip_ref.is_empty() {
            self.hover_card_open = !self.hover_card_open;
            if let Some(mut ct) = tooltip_ref.borrow_mut::<CalloutTooltip>() {
                if self.hover_card_open {
                    let trigger = self.ui.view(cx, ids!(hover_card_trigger));
                    let content_rect = self.ui.view(cx, ids!(content_flip)).area().rect(cx);
                    let trigger_rect = trigger.area().rect(cx);
                    let rect = Rect {
                        pos: content_rect.pos + trigger_rect.pos,
                        size: trigger_rect.size,
                    };
                    ct.show_with_options(
                        cx,
                        "Card-style tooltip shown on hover or click.",
                        rect,
                        CalloutTooltipOptions::default(),
                        false,
                    );
                } else {
                    ct.hide(cx);
                }
            }
        }
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

    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.sidebar_open = true;
                self.apply_responsive_visibility(cx);
            }
            Event::WindowGeomChange(geom) => self.update_screen_mode(cx, geom.new_geom.inner_size.x),
            _ => {}
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
