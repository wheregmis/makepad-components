use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadToast = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
        flow: Down
        spacing: 4.0

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadToastTitle = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 12
    }

    mod.widgets.ShadToastDescription = mod.widgets.ShadAlertDescription{
        width: Fit
        height: Fit
    }

    mod.widgets.ShadSonnerBase = #(ShadSonner::register_widget(vm))

    mod.widgets.ShadSonner = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            align: Align{x: 1.0 y: 0.0}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.0)
            }

            content +: {
                width: Fit
                height: Fit
                margin: Inset{top: 16, right: 16}

                toast_panel := RoundedView{
                    width: Fit
                    height: Fit
                    padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
                    flow: Down
                    spacing: 4.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    body := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        title_label := mod.widgets.ShadToastTitle{
                            text: "Event created"
                        }
                        description_label := mod.widgets.ShadToastDescription{
                            text: ""
                            visible: false
                        }
                    }
                }
            }
        }
    }

    mod.widgets.ShadSonnerWithDescription = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            align: Align{x: 1.0 y: 0.0}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.0)
            }

            content +: {
                width: Fit
                height: Fit
                margin: Inset{top: 16, right: 16}

                toast_panel := RoundedView{
                    width: Fit
                    height: Fit
                    padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
                    flow: Down
                    spacing: 4.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    body := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        title_label := mod.widgets.ShadToastTitle{
                            text: "Toast with description"
                        }
                        description_label := mod.widgets.ShadToastDescription{
                            text: "Your changes have been saved."
                        }
                    }
                }
            }
        }
    }

    // Toast with a leading check icon and a close (X) button.
    // The close button dismisses the toast when clicked.
    mod.widgets.ShadSonnerWithClose = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            align: Align{x: 1.0 y: 0.0}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.0)
            }

            content +: {
                width: Fit
                height: Fit
                margin: Inset{top: 16, right: 16}

                toast_panel := RoundedView{
                    width: 260
                    height: Fit
                    padding: Inset{left: 14, right: 8, top: 10, bottom: 10}
                    flow: Down
                    spacing: 4.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    header_row := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        spacing: 8.0

                        check_icon := mod.widgets.IconCheck{
                            icon_walk: Walk{width: 14, height: 14}
                            draw_icon.color: (shad_theme.color_primary)
                        }

                        title_label := mod.widgets.ShadToastTitle{
                            width: Fill
                            text: "Event created"
                        }

                        close_btn := mod.widgets.IconButtonX{
                            width: 24
                            height: 24
                            draw_bg +: {
                                color: #0000
                                color_hover: (shad_theme.color_ghost_hover)
                                color_down: (shad_theme.color_ghost_down)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                            }
                            draw_icon.color: (shad_theme.color_muted_foreground)
                        }
                    }

                    description_label := mod.widgets.ShadToastDescription{
                        visible: false
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadSonner {
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

impl ShadSonner {
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Widget for ShadSonner {
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
            if let Event::Actions(actions) = event {
                let content = self.overlay.widget(cx, ids!(content));
                if actions
                    .find_widget_action(content.widget_uid())
                    .is_some_and(|a| matches!(a.cast(), ModalAction::Dismissed))
                {
                    self.open = false;
                }
                // Handle close button click (ShadSonnerWithClose variant)
                let close_btn = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(toast_panel),
                        live_id!(header_row),
                        live_id!(close_btn),
                    ],
                );
                if !close_btn.is_empty()
                    && actions
                        .find_widget_action(close_btn.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.open = false;
                }
            }
        } else if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
            modal.close(cx);
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
