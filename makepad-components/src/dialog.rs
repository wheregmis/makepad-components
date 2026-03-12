use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadDialogBase = #(ShadDialog::register_widget(vm))

    mod.widgets.ShadDialog = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 360
                height: Fit

                body := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 16}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }
                }
            }
        }
    }

    mod.widgets.ShadDialogAlert = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 360
                height: Fit

                dialog_panel := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 16}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    dialog_body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        margin: Inset{left: 20, right: 20}
                        title_label := ShadAlertTitle{
                            text: "Are you absolutely sure?"
                        }
                        description_label := ShadAlertDescription{
                            text: "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        margin: Inset{top: 8}

                        cancel := ButtonFlat{
                            text: "Cancel"
                            height: 36
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
                            draw_bg +: {
                                color: #0000
                                color_hover: (shad_theme.color_ghost_hover)
                                color_down: (shad_theme.color_ghost_down)
                                border_size: 1.0
                                border_radius: (shad_theme.radius)
                                border_color: (shad_theme.color_outline_border)
                            }
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 11
                        }

                        confirm := ButtonFlat{
                            text: "Continue"
                            height: 36
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
                            draw_bg +: {
                                color: (shad_theme.color_primary_foreground)
                                color_hover: (shad_theme.color_secondary)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                            }
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 11
                        }
                    }
                }
            }
        }
    }

    mod.widgets.ShadDialogAlertDestructive = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 360
                height: Fit

                dialog_panel := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 16}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    dialog_body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        margin: Inset{left: 20, right: 20}
                        title_label := ShadAlertTitle{
                            text: "Are you absolutely sure?"
                        }
                        description_label := ShadAlertDescription{
                            text: "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        margin: Inset{top: 8}

                        cancel := ButtonFlat{
                            text: "Cancel"
                            height: 36
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
                            draw_bg +: {
                                color: #0000
                                color_hover: (shad_theme.color_ghost_hover)
                                color_down: (shad_theme.color_ghost_down)
                                border_size: 1.0
                                border_radius: (shad_theme.radius)
                                border_color: (shad_theme.color_outline_border)
                            }
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 11
                        }

                        confirm := ButtonFlat{
                            text: "Delete"
                            height: 36
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
                            draw_bg +: {
                                color: (shad_theme.color_destructive)
                                color_hover: (shad_theme.color_destructive_hover)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                            }
                            draw_text.color: (shad_theme.color_destructive_foreground)
                            draw_text.text_style.font_size: 11
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadDialogAction {
    Opened,
    Closed,
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadDialog {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,

    #[find]
    #[redraw]
    #[live]
    overlay: WidgetRef,

    #[live]
    open: bool,
    #[rust]
    is_synced_open: bool,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadDialog {
    fn sync_open_state(&mut self, cx: &mut Cx) {
        if self.is_synced_open == self.open {
            return;
        }

        if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
            if self.open {
                modal.open(cx);
            } else {
                modal.close(cx);
            }
        }

        self.is_synced_open = self.open;
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if self.open == open {
            self.sync_open_state(cx);
            return;
        }

        self.open = open;
        self.sync_open_state(cx);
        self.overlay.redraw(cx);
        cx.widget_action_with_data(
            &self.action_data,
            self.widget_uid(),
            if open {
                ShadDialogAction::Opened
            } else {
                ShadDialogAction::Closed
            },
        );
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.set_open(cx, true);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.set_open(cx, false);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn opened(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast::<ShadDialogAction>(), ShadDialogAction::Opened)
        } else {
            false
        }
    }

    pub fn closed(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast::<ShadDialogAction>(), ShadDialogAction::Closed)
        } else {
            false
        }
    }
}

impl Widget for ShadDialog {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_open) {
            if let Some(args_obj) = args.as_object() {
                let trap = vm.bx.threads.cur().trap.pass();
                let value = vm.bx.heap.vec_value(args_obj, 0, trap);
                if let Some(open) = value.as_bool() {
                    vm.with_cx_mut(|cx| self.set_open(cx, open));
                }
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(is_open) {
            return ScriptAsyncResult::Return(ScriptValue::from_bool(self.open));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_open_state(cx);

        if self.open {
            self.overlay.handle_event(cx, event, scope);
            // Close when modal is dismissed (backdrop/Escape) or when cancel/confirm clicked (alert variants)
            if let Event::Actions(actions) = event {
                let content = self.overlay.widget(cx, ids!(content));
                if actions
                    .find_widget_action(content.widget_uid())
                    .is_some_and(|a| matches!(a.cast(), ModalAction::Dismissed))
                {
                    self.close(cx);
                }
                let cancel_btn = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(cancel),
                    ],
                );
                let confirm_btn = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(confirm),
                    ],
                );
                if !cancel_btn.is_empty()
                    && actions
                        .find_widget_action(cancel_btn.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.close(cx);
                }
                if !confirm_btn.is_empty()
                    && actions
                        .find_widget_action(confirm_btn.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.close(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_open_state(cx);

        if !self.open {
            return DrawStep::done();
        }
        cx.begin_turtle(walk, self.layout);
        let step = self
            .overlay
            .draw_walk(cx, scope, Walk::new(Size::fill(), Size::fill()));
        cx.end_turtle();
        step
    }
}

impl ShadDialogRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }

    pub fn opened(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.opened(actions))
    }

    pub fn closed(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.closed(actions))
    }
}
