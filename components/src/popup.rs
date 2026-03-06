use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadPopupTitle = mod.widgets.ShadAlertTitle{}
    mod.widgets.ShadPopupDescription = mod.widgets.ShadAlertDescription{}

    mod.widgets.ShadPopupBase = #(ShadPopup::register_widget(vm))

    mod.widgets.ShadPopup = set_type_default() do mod.widgets.ShadPopupBase {
        width: Fill
        height: Fit
        open: false

        // Sheet-specific layout properties
        side: "none"
        sheet_size: 420.0

        overlay: Modal {
            align: Align{x: 0.5, y: 0.5}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: Fit
                height: Fit
            }
        }
    }

    // Sheet (defaults to right slide-in style via alignment and dimensions)
    mod.widgets.ShadSheet = set_type_default() do mod.widgets.ShadPopup {
        side: "right"
        overlay +: {
            align: Align{x: 1.0, y: 0.0}
            content +: {
                width: 420
                height: Fill
                flow: Down

                sheet_frame := RoundedView{
                    width: Fill
                    height: Fill
                    flow: Down

                    draw_bg +: {
                        color: (shad_theme.color_background)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    header := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 6.0
                        padding: Inset{left: 20, right: 20, top: 20, bottom: 12}

                        title := ShadPopupTitle{
                            text: "Edit profile"
                        }

                        description := ShadPopupDescription{
                            text: "Make changes to your profile here."
                        }
                    }

                    body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 16}
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
                        align: Align{x: 1.0, y: 0.5}
                    }
                }
            }
        }
    }

    // Drawer (defaults to bottom slide-up style)
    mod.widgets.ShadDrawer = set_type_default() do mod.widgets.ShadPopup {
        overlay +: {
            align: Align{x: 0.5, y: 0.0}
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

                        title_label := ShadPopupTitle{
                            text: "Edit profile"
                        }
                        description_label := ShadPopupDescription{
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

    // Dialog (generic modal popup)
    mod.widgets.ShadDialog = set_type_default() do mod.widgets.ShadPopup {
        overlay +: {
            align: Align{x: 0.5, y: 0.5}
            content +: {
                width: 400
                height: Fit

                body := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 20}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    title_label := ShadPopupTitle{
                        text: "Dialog Title"
                    }

                    desc_label := ShadPopupDescription{
                        text: "Dialog description goes here."
                    }
                }
            }
        }
    }

    mod.widgets.ShadPopupAlert = set_type_default() do mod.widgets.ShadPopup{
        overlay +: {
            align: Align{x: 0.5, y: 0.5}
            content +: {
                width: 400
                height: Fit

                body := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 20}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    dialog_panel := RoundedView{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        draw_bg +: {
                            color: #0000
                            border_size: 0.0
                        }

                        dialog_body := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0
                            title_label := ShadPopupTitle{
                                text: "Are you absolutely sure?"
                            }
                            description_label := ShadPopupDescription{
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
    }

    mod.widgets.ShadPopupAlertDestructive = set_type_default() do mod.widgets.ShadPopupAlert{
        overlay +: {
            content +: {
                body +: {
                    dialog_panel +: {
                        footer +: {
                            confirm +: {
                                text: "Delete"
                                draw_bg +: {
                                    color: (shad_theme.color_destructive)
                                    color_hover: (shad_theme.color_destructive_hover)
                                }
                                draw_text.color: (shad_theme.color_destructive_foreground)
                            }
                        }
                    }
                }
            }
        }
    }

    // Sonner / Toast
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

    mod.widgets.ShadSonner = set_type_default() do mod.widgets.ShadPopup {
        overlay +: {
            align: Align{x: 1.0, y: 0.0}

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

    mod.widgets.ShadPopupWithDescription = set_type_default() do mod.widgets.ShadSonner {
        overlay +: {
            content +: {
                toast_panel +: {
                    body +: {
                        title_label +: {
                            text: "Toast with description"
                        }
                        description_label +: {
                            text: "Your changes have been saved."
                            visible: true
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadPopup {
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
    #[live]
    side: ArcStringMut,
    #[live]
    sheet_size: f64,

    #[rust]
    is_synced_open: bool,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadPopup {
    fn sync_side_layout(&mut self, cx: &mut Cx) {
        let side = self.side.as_ref();
        if side.is_empty() || side == "none" {
            // Not acting as a directional sheet, skip override
            return;
        }

        let (align, content_width, content_height) = match side {
            "left" => (
                Align { x: 0.0, y: 0.0 },
                Size::Fixed(self.sheet_size),
                Size::fill(),
            ),
            "top" => (
                Align { x: 0.0, y: 0.0 },
                Size::fill(),
                Size::Fixed(self.sheet_size),
            ),
            "bottom" => (
                Align { x: 0.0, y: 1.0 },
                Size::fill(),
                Size::Fixed(self.sheet_size),
            ),
            _ => (
                Align { x: 1.0, y: 0.0 },
                Size::Fixed(self.sheet_size),
                Size::fill(),
            ),
        };

        let mut overlay = self.overlay.clone();
        script_apply_eval!(cx, overlay, {
            align: #(align)
        });

        let mut content = self.overlay.widget(cx, ids!(content));
        if !content.is_empty() {
            script_apply_eval!(cx, content, {
                width: #(content_width)
                height: #(content_height)
            });

            let mut sheet_frame = self.overlay.widget(cx, ids!(content.sheet_frame));
            if !sheet_frame.is_empty() {
                script_apply_eval!(cx, sheet_frame, {
                    width: #(Size::fill())
                    height: #(Size::fill())
                });
            }
        }
    }

    fn sync_open_state(&mut self, cx: &mut Cx) {
        self.sync_side_layout(cx);

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

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Widget for ShadPopup {
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
            if let Event::Actions(actions) = event {
                let content = self.overlay.widget(cx, ids!(content));
                if actions
                    .find_widget_action(content.widget_uid())
                    .is_some_and(|a| matches!(a.cast(), ModalAction::Dismissed))
                {
                    self.open = false;
                    self.sync_open_state(cx);
                }

                // Check for cancel/confirm actions from common dialogs/drawers
                // They both use footer -> cancel or confirm
                let cancel_btn_dialog = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(body),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(cancel),
                    ],
                );
                let confirm_btn_dialog = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(body),
                        live_id!(dialog_panel),
                        live_id!(footer),
                        live_id!(confirm),
                    ],
                );
                let cancel_btn_drawer = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(drawer_panel),
                        live_id!(footer),
                        live_id!(cancel),
                    ],
                );
                let confirm_btn_drawer = self.overlay.widget(
                    cx,
                    &[
                        live_id!(content),
                        live_id!(drawer_panel),
                        live_id!(footer),
                        live_id!(confirm),
                    ],
                );

                if !cancel_btn_dialog.is_empty()
                    && actions
                        .find_widget_action(cancel_btn_dialog.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.open = false;
                    self.sync_open_state(cx);
                }
                if !confirm_btn_dialog.is_empty()
                    && actions
                        .find_widget_action(confirm_btn_dialog.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.open = false;
                    self.sync_open_state(cx);
                }
                if !cancel_btn_drawer.is_empty()
                    && actions
                        .find_widget_action(cancel_btn_drawer.widget_uid())
                        .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
                {
                    self.open = false;
                    self.sync_open_state(cx);
                }
                if !confirm_btn_drawer.is_empty()
                    && actions
                        .find_widget_action(confirm_btn_drawer.widget_uid())
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
