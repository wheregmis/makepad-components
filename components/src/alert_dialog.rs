use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadAlertDialogBase = #(ShadAlertDialog::register_widget(vm))

    mod.widgets.ShadAlertDialog = set_type_default() do mod.widgets.ShadAlertDialogBase{
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

    mod.widgets.ShadAlertDialogDestructive = set_type_default() do mod.widgets.ShadAlertDialogBase{
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

#[derive(Script, ScriptHook, Widget)]
pub struct ShadAlertDialog {
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

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadAlertDialog {
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Widget for ShadAlertDialog {
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
                    self.open = open;
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
        if self.open {
            if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
                modal.open(cx);
            }
            self.overlay.handle_event(cx, event, scope);
            // Close when Cancel/Confirm is clicked or modal is dismissed (backdrop/Escape)
            if let Event::Actions(actions) = event {
                let content = self.overlay.widget(cx, ids!(content));
                if actions
                    .find_widget_action(content.widget_uid())
                    .is_some_and(|a| matches!(a.cast(), ModalAction::Dismissed))
                {
                    self.open = false;
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
                    self.open = false;
                }
                if !confirm_btn.is_empty()
                    && actions
                        .find_widget_action(confirm_btn.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.open = false;
                }
            }
        } else {
            if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
                modal.close(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.open {
            if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
                modal.close(cx);
            }
            return DrawStep::done();
        }
        if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
            modal.open(cx);
        }
        cx.begin_turtle(walk, self.layout);
        let step = self
            .overlay
            .draw_walk(cx, scope, Walk::new(Size::fill(), Size::fill()));
        cx.end_turtle();
        step
    }
}

