use crate::internal::actions::{emit_open_changed_action, open_changed_action};
use crate::internal::overlay::{
    compute_anchor_overlay_pos, overlay_hover_zone_contains_abs, reclaim_overlay_pointer_down,
    should_dismiss_overlay_on_pointer_up, AnchorOverlayLayout,
};
use crate::internal::script_args::bool_arg;
use crate::internal::touch::is_primary_tap;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadPopoverContent = mod.widgets.ShadSurfacePopover{
        width: 320
        height: Fit
        spacing: 8.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
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
        open_on_hover: false

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

fn clamp_overlay_width(requested_width: f64, pass_width: f64, viewport_padding: f64) -> f64 {
    requested_width.min((pass_width - viewport_padding * 2.0).max(0.0))
}

fn responsive_popup_walk(mut walk: Walk, pass_width: f64, viewport_padding: f64) -> Walk {
    if walk.width.is_fill() {
        walk.width = Size::Fixed((pass_width - viewport_padding * 2.0).max(0.0));
        return walk;
    }

    if let Some(width) = walk.width.to_fixed() {
        walk.width = Size::Fixed(clamp_overlay_width(width, pass_width, viewport_padding));
    }

    walk
}

#[derive(Clone, Debug, Default)]
pub enum ShadPopoverAction {
    OpenChanged(bool),
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
    #[live(false)]
    open_on_hover: bool,

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

        vm.with_cx_mut(|cx| self.redraw_overlay(cx, false));
    }
}

impl ShadPopover {
    fn should_redraw_overlay_content(&self) -> bool {
        self.open && !self.popup_content.is_empty()
    }

    fn draw_overlay_content(&mut self, cx: &mut Cx2d, scope: &mut Scope, popup_pos: Vec2d) {
        let Some(draw_list) = self.draw_list.as_mut() else {
            return;
        };
        draw_list.begin_overlay_reuse(cx);

        let pass_size = cx.current_pass_size();
        cx.begin_root_turtle(pass_size, Layout::flow_down());
        self.draw_bg
            .begin(cx, Walk::new(Size::fill(), Size::fill()), Layout::default());

        let popup_walk = responsive_popup_walk(
            self.popup_content.walk(cx),
            pass_size.x,
            self.viewport_padding,
        )
        .with_abs_pos(popup_pos);
        self.popup_content.draw_walk_all(cx, scope, popup_walk);

        self.draw_bg.end(cx);
        cx.end_pass_sized_turtle();
        draw_list.end(cx);
    }

    fn redraw_overlay(&mut self, cx: &mut Cx, force_draw_list: bool) {
        if force_draw_list || self.should_redraw_overlay_content() {
            if let Some(draw_list) = self.draw_list.as_ref() {
                draw_list.redraw(cx);
            }
        }
        if !self.should_redraw_overlay_content() {
            return;
        }
        self.draw_bg.redraw(cx);
        self.popup_content.redraw(cx);
    }

    fn trigger_rect(&self, cx: &Cx) -> Rect {
        let trigger = self.view.widget(cx, ids!(trigger));
        if trigger.is_empty() {
            self.view.area().rect(cx)
        } else {
            trigger.area().rect(cx)
        }
    }

    fn overlay_layout(&self) -> AnchorOverlayLayout<'_> {
        AnchorOverlayLayout {
            side: self.side.as_ref(),
            align: self.align.as_ref(),
            side_offset: self.side_offset,
            viewport_padding: self.viewport_padding,
        }
    }

    fn compute_popup_pos_with_content_size(&self, cx: &Cx2d, content_size: Vec2d) -> Vec2d {
        let trigger_rect = self.trigger_rect(cx);
        let pass_size = cx.current_pass_size();
        compute_anchor_overlay_pos(
            &self.overlay_layout(),
            trigger_rect,
            pass_size,
            content_size,
        )
    }

    fn compute_popup_pos(&self, cx: &Cx2d) -> Vec2d {
        self.compute_popup_pos_with_content_size(cx, self.content_size)
    }

    fn emit_open_state(&self, cx: &mut Cx, open: bool) {
        emit_open_changed_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            open,
            ShadPopoverAction::OpenChanged,
        );
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if self.open == open {
            return;
        }

        let was_open = self.open;
        self.open = open;
        self.redraw_overlay(cx, was_open || open);
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

    pub fn content_widget(&self) -> &WidgetRef {
        &self.popup_content
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        open_changed_action::<ShadPopoverAction, _>(actions, self.widget_uid(), |action| {
            if let ShadPopoverAction::OpenChanged(open) = action {
                Some(open)
            } else {
                None
            }
        })
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
            if let Some(open) = bool_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_open(cx, open));
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
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe)
                if self.open_on_hover && fe.device.has_hovers() =>
            {
                self.open(cx);
            }
            Hit::FingerUp(fe)
                if is_primary_tap(&fe) && (!self.open_on_hover || !fe.device.has_hovers()) =>
            {
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

        let trigger_rect = self.trigger_rect(cx);
        let content_rect = self.popup_content.area().rect(cx);

        // Popover bodies are drawn in an overlay draw list, but their widgets still live under
        // the trigger in the normal tree. If a later sibling handled mouse/touch-down first,
        // reclaim that hit here so the overlay body can capture and react to the interaction.
        reclaim_overlay_pointer_down(cx, event, trigger_rect, content_rect);
        self.popup_content.handle_event(cx, event, scope);

        // Consume overlay hits while open so pointer events do not fall through to widgets
        // behind the popover when the popup body is rendered in the overlay draw list.
        let overlay_hit = event.hits(cx, self.draw_bg.area());

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
        if self.open_on_hover {
            match overlay_hit {
                Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe)
                    if fe.device.has_hovers()
                        && !overlay_hover_zone_contains_abs(trigger_rect, content_rect, fe.abs) =>
                {
                    self.close(cx);
                    return;
                }
                Hit::FingerMove(fe)
                    if fe.device.has_hovers()
                        && !overlay_hover_zone_contains_abs(trigger_rect, content_rect, fe.abs) =>
                {
                    self.close(cx);
                    return;
                }
                _ => {}
            }
        }

        if self.can_dismiss {
            if let Hit::FingerUp(fe) = overlay_hit {
                if should_dismiss_overlay_on_pointer_up(trigger_rect, content_rect, fe.abs) {
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
        self.draw_overlay_content(cx, scope, popup_pos);

        let measured_rect = self.popup_content.area().rect(cx);
        if measured_rect.size != self.content_size
            && measured_rect.size.x > 0.0
            && measured_rect.size.y > 0.0
        {
            let next_popup_pos = self.compute_popup_pos_with_content_size(cx, measured_rect.size);
            self.content_size = measured_rect.size;
            let moved_x = (next_popup_pos.x - popup_pos.x).abs();
            let moved_y = (next_popup_pos.y - popup_pos.y).abs();
            if moved_x > 0.5 || moved_y > 0.5 {
                // Reposition immediately in this frame to avoid a guaranteed
                // follow-up frame redraw on first open for center/end alignment.
                self.draw_overlay_content(cx, scope, next_popup_pos);
            }
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

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }

    pub fn content_widget(&self) -> WidgetRef {
        self.borrow()
            .map_or_else(WidgetRef::empty, |inner| inner.content_widget().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::{clamp_overlay_width, responsive_popup_walk};
    use makepad_widgets::{Size, Walk};

    #[test]
    fn fixed_overlay_width_is_clamped_to_available_viewport() {
        assert_eq!(clamp_overlay_width(320.0, 280.0, 12.0), 256.0);
        assert_eq!(clamp_overlay_width(280.0, 640.0, 12.0), 280.0);
    }

    #[test]
    fn fill_overlay_width_becomes_a_fixed_viewport_width() {
        let walk = responsive_popup_walk(Walk::new(Size::fill(), Size::fit()), 300.0, 16.0);

        assert!(matches!(walk.width, Size::Fixed(width) if (width - 268.0).abs() < f64::EPSILON));
    }
}
