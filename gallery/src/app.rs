use makepad_components::makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: mod.widgets.GalleryAppUi{}
    }
}

impl App {
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
        if self.ui.button(cx, ids!(sidebar_accordion)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(accordion_page));
        }

        if self.ui.button(cx, ids!(sidebar_alert)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(alert_page));
        }

        if self
            .ui
            .button(cx, ids!(sidebar_aspect_ratio))
            .clicked(actions)
        {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(aspect_ratio_page));
        }

        if self.ui.button(cx, ids!(sidebar_avatar)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(avatar_page));
        }

        if self.ui.button(cx, ids!(sidebar_badge)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(badge_page));
        }

        if self
            .ui
            .button(cx, ids!(sidebar_breadcrumb))
            .clicked(actions)
        {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(breadcrumb_page));
        }

        if self.ui.button(cx, ids!(sidebar_button)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(button_page));
        }

        if self
            .ui
            .button(cx, ids!(sidebar_button_group))
            .clicked(actions)
        {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(button_group_page));
        }

        if self.ui.button(cx, ids!(sidebar_checkbox)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(checkbox_page));
        }

        if self
            .ui
            .button(cx, ids!(sidebar_collapsible))
            .clicked(actions)
        {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(collapsible_page));
        }

        if self.ui.button(cx, ids!(sidebar_skeleton)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(skeleton_page));
        }

        if self.ui.button(cx, ids!(sidebar_switch)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(switch_page));
        }

        if self.ui.button(cx, ids!(sidebar_input)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(input_page));
        }

        if self.ui.button(cx, ids!(sidebar_label)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(label_page));
        }

        if self.ui.button(cx, ids!(sidebar_sidebar)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(sidebar_page));
        }

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
            ids!(sidebar_demo_tab),
            ids!(sidebar_code_tab),
            ids!(sidebar_preview_flip),
            ids!(sidebar_demo_indicator),
            ids!(sidebar_code_indicator),
        );
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
