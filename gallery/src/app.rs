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
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
