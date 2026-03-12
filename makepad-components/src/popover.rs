use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadPopoverContent = RoundedView{
        width: 320
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

        draw_bg +: {
            color: (shad_theme.color_popover)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadPopoverBase = #(ShadPopover::register_widget(vm))

    mod.widgets.ShadPopover = set_type_default() do mod.widgets.ShadPopoverBase{
        width: Fit
        height: Fit

        open: false
        side: "bottom"
        align: "center"
        side_offset: 8.0
        viewport_padding: 12.0
        can_dismiss: true

        draw_bg +: {
            pixel: fn() {
                return vec4(0.0, 0.0, 0.0, 0.0)
            }
        }

        trigger := mod.widgets.ShadButtonOutline{
            text: "Open popover"
        }

        content: mod.widgets.ShadPopoverContent{
            title := ShadSectionHeader{
                text: "Popover"
            }

            description := ShadFieldDescription{
                text: "Anchored content that opens next to a trigger."
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadPopoverAction {
    Opened,
    Closed,
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadPopover {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    content: ScriptValue,
    #[live]
    open: bool,
    #[live]
    side: ArcStringMut,
    #[live]
    align: ArcStringMut,
    #[live]
    side_offset: f64,
    #[live]
    viewport_padding: f64,
    #[live(true)]
    can_dismiss: bool,

    #[live]
    draw_bg: DrawQuad,

    #[rust]
    draw_list: Option<DrawList2d>,
    #[rust]
    popup_content: WidgetRef,
    #[rust]
    content_size: Vec2d,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadPopover {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        self.draw_list = Some(DrawList2d::script_new(vm));
    }

    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        scope: &mut Scope,
        _value: ScriptValue,
    ) {
        if !self.content.is_nil() && WidgetRef::value_is_newable_widget(vm, self.content) {
            self.popup_content
                .script_apply(vm, apply, scope, self.content);
        } else {
            self.popup_content = WidgetRef::empty();
        }

        vm.with_cx_mut(|cx| self.redraw_overlay(cx));
    }
}

impl ShadPopover {
    fn redraw_overlay(&mut self, cx: &mut Cx) {
        if let Some(draw_list) = &self.draw_list {
            draw_list.redraw(cx);
        }
        self.draw_bg.redraw(cx);
        self.popup_content.redraw(cx);
    }

    fn trigger_rect(&self, cx: &Cx) -> Rect {
        let trigger = self.view.widget(cx, ids!(trigger));
        let rect = if trigger.is_empty() {
            self.view.area().rect(cx)
        } else {
            trigger.area().rect(cx)
        };
        rect
    }

    fn resolve_side<'a>(
        &'a self,
        trigger_rect: Rect,
        pass_size: Vec2d,
        content_size: Vec2d,
    ) -> &'a str {
        let side = self.side.as_ref();
        let top_space = trigger_rect.pos.y - self.side_offset - self.viewport_padding;
        let bottom_space = pass_size.y
            - (trigger_rect.pos.y + trigger_rect.size.y)
            - self.side_offset
            - self.viewport_padding;
        let left_space = trigger_rect.pos.x - self.side_offset - self.viewport_padding;
        let right_space = pass_size.x
            - (trigger_rect.pos.x + trigger_rect.size.x)
            - self.side_offset
            - self.viewport_padding;

        match side {
            "top" if content_size.y > top_space && bottom_space > top_space => "bottom",
            "bottom" if content_size.y > bottom_space && top_space > bottom_space => "top",
            "left" if content_size.x > left_space && right_space > left_space => "right",
            "right" if content_size.x > right_space && left_space > right_space => "left",
            _ => side,
        }
    }

    fn compute_popup_pos(&self, cx: &Cx2d) -> Vec2d {
        let trigger_rect = self.trigger_rect(cx);
        let pass_size = cx.current_pass_size();
        let content_size = self.content_size;
        let side = self.resolve_side(trigger_rect, pass_size, content_size);

        let align = self.align.as_ref();
        let cross_x = match align {
            "end" => trigger_rect.pos.x + trigger_rect.size.x - content_size.x,
            "center" => trigger_rect.pos.x + (trigger_rect.size.x - content_size.x) * 0.5,
            _ => trigger_rect.pos.x,
        };
        let cross_y = match align {
            "end" => trigger_rect.pos.y + trigger_rect.size.y - content_size.y,
            "center" => trigger_rect.pos.y + (trigger_rect.size.y - content_size.y) * 0.5,
            _ => trigger_rect.pos.y,
        };

        let mut pos = match side {
            "top" => dvec2(
                cross_x,
                trigger_rect.pos.y - content_size.y - self.side_offset,
            ),
            "left" => dvec2(cross_x - content_size.x - self.side_offset, cross_y),
            "right" => dvec2(
                trigger_rect.pos.x + trigger_rect.size.x + self.side_offset,
                cross_y,
            ),
            _ => dvec2(
                cross_x,
                trigger_rect.pos.y + trigger_rect.size.y + self.side_offset,
            ),
        };

        let max_x =
            (pass_size.x - content_size.x - self.viewport_padding).max(self.viewport_padding);
        let max_y =
            (pass_size.y - content_size.y - self.viewport_padding).max(self.viewport_padding);
        pos.x = pos.x.clamp(self.viewport_padding, max_x);
        pos.y = pos.y.clamp(self.viewport_padding, max_y);
        pos
    }

    fn emit_open_state(&self, cx: &mut Cx, open: bool) {
        cx.widget_action_with_data(
            &self.action_data,
            self.widget_uid(),
            if open {
                ShadPopoverAction::Opened
            } else {
                ShadPopoverAction::Closed
            },
        );
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if self.open == open {
            self.redraw_overlay(cx);
            return;
        }

        self.open = open;
        self.redraw_overlay(cx);
        self.emit_open_state(cx, open);
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

    pub fn content_widget(&self) -> WidgetRef {
        self.popup_content.clone()
    }

    pub fn opened(&self, actions: &Actions) -> bool {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            matches!(
                action.cast::<ShadPopoverAction>(),
                ShadPopoverAction::Opened
            )
        } else {
            false
        }
    }

    pub fn closed(&self, actions: &Actions) -> bool {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            matches!(
                action.cast::<ShadPopoverAction>(),
                ShadPopoverAction::Closed
            )
        } else {
            false
        }
    }
}

impl Widget for ShadPopover {
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
        self.view.handle_event(cx, event, scope);

        let trigger_area = {
            let trigger = self.view.widget(cx, ids!(trigger));
            if trigger.is_empty() {
                self.view.area()
            } else {
                trigger.area()
            }
        };

        match event.hits(cx, trigger_area) {
            Hit::FingerUp(fe) if fe.is_primary_hit() => {
                self.set_open(cx, !self.open);
                return;
            }
            Hit::KeyDown(ke) if matches!(ke.key_code, KeyCode::ReturnKey | KeyCode::Space) => {
                self.set_open(cx, !self.open);
                return;
            }
            _ => {}
        }

        if !self.open {
            return;
        }

        self.popup_content.handle_event(cx, event, scope);

        if let Event::KeyDown(ke) = event {
            if ke.key_code == KeyCode::Escape && self.can_dismiss {
                self.close(cx);
                return;
            }
        }

        if event.back_pressed() && self.can_dismiss {
            self.close(cx);
            return;
        }

        let overlay_hit = event.hits(cx, self.draw_bg.area());
        if self.can_dismiss {
            if let Hit::FingerUp(fe) = overlay_hit {
                let content_rect = self.popup_content.area().rect(cx);
                let trigger_rect = self.trigger_rect(cx);
                if !content_rect.contains(fe.abs) && !trigger_rect.contains(fe.abs) {
                    self.close(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)?;

        if !self.open || self.popup_content.is_empty() {
            return DrawStep::done();
        }

        let popup_pos = self.compute_popup_pos(cx);
        let draw_list = self.draw_list.as_mut().unwrap();
        draw_list.begin_overlay_reuse(cx);

        let pass_size = cx.current_pass_size();
        cx.begin_root_turtle(pass_size, Layout::flow_down());
        self.draw_bg
            .begin(cx, Walk::new(Size::fill(), Size::fill()), Layout::default());

        let popup_walk = self.popup_content.walk(cx).with_abs_pos(popup_pos);
        self.popup_content.draw_walk_all(cx, scope, popup_walk);

        self.draw_bg.end(cx);
        cx.end_pass_sized_turtle();
        draw_list.end(cx);

        let measured_rect = self.popup_content.area().rect(cx);
        if measured_rect.size != self.content_size
            && measured_rect.size.x > 0.0
            && measured_rect.size.y > 0.0
        {
            self.content_size = measured_rect.size;
            self.redraw_overlay(cx);
        }

        DrawStep::done()
    }
}

impl ShadPopoverRef {
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

    pub fn content_widget(&self) -> WidgetRef {
        self.borrow()
            .map_or_else(WidgetRef::empty, |inner| inner.content_widget())
    }
}
