use makepad_components::makepad_widgets::*;
use makepad_components::{ShadAlertDialog, ShadCarousel, ShadSonner};

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
        self.set_page(
            cx,
            actions,
            ids!(sidebar_alert_dialog),
            live_id!(alert_dialog_page),
        );
        if self.ui.button(cx, ids!(open_default_btn)).clicked(actions) {
            if let Some(mut d) = self
                .ui
                .widget_flood(cx, ids!(default_dialog))
                .borrow_mut::<ShadAlertDialog>()
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
                .borrow_mut::<ShadAlertDialog>()
            {
                d.set_open(true);
            }
        }
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
        self.set_page(cx, actions, ids!(sidebar_skeleton), live_id!(skeleton_page));
        self.set_page(cx, actions, ids!(sidebar_switch), live_id!(switch_page));
        self.set_page(cx, actions, ids!(sidebar_input), live_id!(input_page));
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
            ids!(accordion_demo_tab),
            ids!(accordion_code_tab),
            ids!(accordion_preview_flip),
            ids!(accordion_demo_indicator),
            ids!(accordion_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(alert_demo_tab),
            ids!(alert_code_tab),
            ids!(alert_preview_flip),
            ids!(alert_demo_indicator),
            ids!(alert_code_indicator),
        );
        Self::handle_preview_tabs(
            &self.ui,
            cx,
            actions,
            ids!(alert_dialog_demo_tab),
            ids!(alert_dialog_code_tab),
            ids!(alert_dialog_preview_flip),
            ids!(alert_dialog_demo_indicator),
            ids!(alert_dialog_code_indicator),
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
