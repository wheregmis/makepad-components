use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadDrawerBase = #(ShadDrawer::register_widget(vm))

    mod.widgets.ShadDrawer = set_type_default() do mod.widgets.ShadDrawerBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            align: Align{x: 0.5, y: 0.0}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 640
                height: Fit
                margin: Inset{top: 0, right: 0, bottom: 0, left: 0}

                drawer_panel := RoundedView{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 0.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    drawer_body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        padding: Inset{left: 20, right: 20, top: 20, bottom: 16}

                        drawer_handle := View{
                            width: Fill
                            height: Fit
                            align: Center
                            handle := RoundedView{
                                width: 40
                                height: 4
                                draw_bg +: {
                                    color: (shad_theme.color_outline_border)
                                    border_radius: 2.0
                                }
                            }
                        }

                        title_label := ShadAlertTitle{
                            text: "Edit profile"
                        }
                        description_label := ShadAlertDescription{
                            text: "Make changes to your profile here. Click submit when you're done."
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}

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
                            text: "Submit"
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
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadDrawer {
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

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadDrawer {
    fn sync_open_state(&mut self, cx: &mut Cx) {
        if self.is_synced_open == self.open {
            return;
        }

        if self.open {
            if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
                modal.open(cx);
            }
        } else if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
            modal.close(cx);
        }

        self.is_synced_open = self.open;
    }

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Widget for ShadDrawer {
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
                    vm.with_cx_mut(|cx| {
                        self.open = open;
                        self.sync_open_state(cx);
                    });
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
            // Close when Cancel/Confirm is clicked or modal is dismissed (backdrop/Escape)
            if let Event::Actions(actions) = event {
                let content = self.overlay.widget(cx, ids!(content));
                if actions
                    .find_widget_action(content.widget_uid())
                    .is_some_and(|a| matches!(a.cast(), ModalAction::Dismissed))
                {
                    self.open = false;
                    self.sync_open_state(cx);
                }
                let cancel_btn = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(drawer_panel),
                        live_id!(footer),
                        live_id!(cancel),
                    ],
                );
                let confirm_btn = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(drawer_panel),
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
                    self.sync_open_state(cx);
                }
                if !confirm_btn.is_empty()
                    && actions
                        .find_widget_action(confirm_btn.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.open = false;
                    self.sync_open_state(cx);
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
