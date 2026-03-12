use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSheetTitle = mod.widgets.ShadAlertTitle{}
    mod.widgets.ShadSheetDescription = mod.widgets.ShadAlertDescription{}

    mod.widgets.ShadSheetBase = #(ShadSheet::register_widget(vm))

    mod.widgets.ShadSheet = set_type_default() do mod.widgets.ShadSheetBase{
        width: Fill
        height: Fit
        open: false
        side: "right"
        sheet_size: 420.0

        overlay: Modal{
            align: Align{x: 1.0, y: 0.0}
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

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

                        title := ShadAlertTitle{
                            text: "Edit profile"
                        }

                        description := ShadAlertDescription{
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
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadSheet {
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

    #[rust]
    last_side: String,
    #[rust]
    is_side_initialized: bool,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadSheet {
    fn sync_side_layout(&mut self, cx: &mut Cx) {
        let current_side = self.side.as_ref();

        // Optimization: only reapply script evaluation if the side has changed or on first run
        if self.is_side_initialized && current_side == self.last_side.as_str() {
            return;
        }

        // Optimization: avoid repeated allocation in layout sync loop by reusing string capacity
        // Previously: allocated a new String using `current_side.to_string()`
        // Now: reuse `self.last_side` buffer, avoiding a heap allocation
        self.last_side.clear();
        self.last_side.push_str(current_side);
        self.is_side_initialized = true;

        let (align, content_width, content_height) = match current_side {
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

        // Optimization: avoid cloning the `WidgetRef`
        // Previously: created a new cloned reference using `self.overlay.clone()`
        // Now: apply directly to `self.overlay`, eliminating clone overhead
        script_apply_eval!(cx, self.overlay, {
            align: #(align)
        });

        let mut content = self.overlay.widget(cx, ids!(content));
        script_apply_eval!(cx, content, {
            width: #(content_width)
            height: #(content_height)
        });

        let mut sheet_frame = self.overlay.widget(cx, ids!(content.sheet_frame));
        script_apply_eval!(cx, sheet_frame, {
            width: #(Size::fill())
            height: #(Size::fill())
        });
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

impl Widget for ShadSheet {
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
