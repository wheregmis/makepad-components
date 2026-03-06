use makepad_components::makepad_widgets::*;
use makepad_components::drawer::ShadDrawer;
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
    fn set_page(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        sidebar_button: &[LiveId],
        page: LiveId,
    ) {
        if self.ui.button(cx, sidebar_button).clicked(actions) {
            // Avoid cx.set_key_focus(Area::Empty) here - can trigger KeyFocusLost feedback loop
            // when TextInput or DropDown popup had focus (makepad-platform issue).
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, page);
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
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Sidebar → page mappings. When adding a new component:
        // 1) Add a ShadSidebarItem in GallerySidebar (sidebar::<name>)
        // 2) Add a matching page in GalleryContentFlip (<name>_page)
        // 3) Add a set_page call here with the same IDs.
        self.set_page(
            cx,
            actions,
            ids!(sidebar_accordion),
            live_id!(accordion_page),
        );
        self.set_page(cx, actions, ids!(sidebar_alert), live_id!(alert_page));
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
        );
        self.set_page(cx, actions, ids!(sidebar_avatar), live_id!(avatar_page));
        self.set_page(cx, actions, ids!(sidebar_badge), live_id!(badge_page));
        self.set_page(
            cx,
            actions,
            ids!(sidebar_breadcrumb),
            live_id!(breadcrumb_page),
        );
        self.set_page(cx, actions, ids!(sidebar_button), live_id!(button_page));
        self.set_page(
            cx,
            actions,
            ids!(sidebar_button_group),
            live_id!(button_group_page),
        );
        self.set_page(cx, actions, ids!(sidebar_card), live_id!(card_page));
        self.set_page(cx, actions, ids!(sidebar_carousel), live_id!(carousel_page));
        self.set_page(cx, actions, ids!(sidebar_checkbox), live_id!(checkbox_page));
        self.set_page(
            cx,
            actions,
            ids!(sidebar_collapsible),
            live_id!(collapsible_page),
        );
        self.set_page(cx, actions, ids!(sidebar_dialog), live_id!(dialog_page));
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
        self.set_page(cx, actions, ids!(sidebar_drawer), live_id!(drawer_page));
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
        );
                self.set_page(
            cx,
            actions,
            ids!(sidebar_hover_card),
            live_id!(hover_card_page),
        );
        self.set_page(cx, actions, ids!(sidebar_skeleton), live_id!(skeleton_page));
        self.set_page(cx, actions, ids!(sidebar_switch), live_id!(switch_page));
        self.set_page(cx, actions, ids!(sidebar_kbd), live_id!(kbd_page));
        self.set_page(cx, actions, ids!(sidebar_label), live_id!(label_page));
        self.set_page(cx, actions, ids!(sidebar_progress), live_id!(progress_page));
        self.set_page(cx, actions, ids!(sidebar_sidebar), live_id!(sidebar_page));
        self.set_page(cx, actions, ids!(sidebar_slider), live_id!(slider_page));
        self.set_page(cx, actions, ids!(sidebar_sonner), live_id!(sonner_page));
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
        self.set_page(cx, actions, ids!(sidebar_spinner), live_id!(spinner_page));

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
        let tooltip_ref = self.ui.widget_flood(cx, ids!(hover_card_tooltip));
        if self.ui.button(cx, ids!(hover_card_trigger)).clicked(actions) && !tooltip_ref.is_empty() {
            self.hover_card_open = !self.hover_card_open;
            if let Some(mut ct) = tooltip_ref.borrow_mut::<CalloutTooltip>() {
                if self.hover_card_open {
                    let trigger = self.ui.view(cx, ids!(hover_card_trigger));
                    let rect = trigger.area().rect(cx);
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
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
