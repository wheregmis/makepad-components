use crate::internal::actions::emit_widget_action;
use makepad_widgets::event::TouchState;
use makepad_widgets::popup_menu::PopupMenu;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

pub(crate) struct AnchorOverlayLayout<'a> {
    pub side: &'a str,
    pub align: &'a str,
    pub side_offset: f64,
    pub viewport_padding: f64,
}

pub(crate) fn resolve_anchor_side<'a>(
    layout: &'a AnchorOverlayLayout<'a>,
    trigger_rect: Rect,
    pass_size: Vec2d,
    content_size: Vec2d,
) -> &'a str {
    let top_space = trigger_rect.pos.y - layout.side_offset - layout.viewport_padding;
    let bottom_space = pass_size.y
        - (trigger_rect.pos.y + trigger_rect.size.y)
        - layout.side_offset
        - layout.viewport_padding;
    let left_space = trigger_rect.pos.x - layout.side_offset - layout.viewport_padding;
    let right_space = pass_size.x
        - (trigger_rect.pos.x + trigger_rect.size.x)
        - layout.side_offset
        - layout.viewport_padding;

    match layout.side {
        "top" if content_size.y > top_space && bottom_space > top_space => "bottom",
        "bottom" if content_size.y > bottom_space && top_space > bottom_space => "top",
        "left" if content_size.x > left_space && right_space > left_space => "right",
        "right" if content_size.x > right_space && left_space > right_space => "left",
        _ => layout.side,
    }
}

pub(crate) fn compute_anchor_overlay_pos(
    layout: &AnchorOverlayLayout<'_>,
    trigger_rect: Rect,
    pass_size: Vec2d,
    content_size: Vec2d,
) -> Vec2d {
    let side = resolve_anchor_side(layout, trigger_rect, pass_size, content_size);

    let cross_x = match layout.align {
        "end" => trigger_rect.pos.x + trigger_rect.size.x - content_size.x,
        "center" => trigger_rect.pos.x + (trigger_rect.size.x - content_size.x) * 0.5,
        _ => trigger_rect.pos.x,
    };
    let cross_y = match layout.align {
        "end" => trigger_rect.pos.y + trigger_rect.size.y - content_size.y,
        "center" => trigger_rect.pos.y + (trigger_rect.size.y - content_size.y) * 0.5,
        _ => trigger_rect.pos.y,
    };

    let mut pos = match side {
        "top" => dvec2(
            cross_x,
            trigger_rect.pos.y - content_size.y - layout.side_offset,
        ),
        "left" => dvec2(cross_x - content_size.x - layout.side_offset, cross_y),
        "right" => dvec2(
            trigger_rect.pos.x + trigger_rect.size.x + layout.side_offset,
            cross_y,
        ),
        _ => dvec2(
            cross_x,
            trigger_rect.pos.y + trigger_rect.size.y + layout.side_offset,
        ),
    };

    let max_x =
        (pass_size.x - content_size.x - layout.viewport_padding).max(layout.viewport_padding);
    let max_y =
        (pass_size.y - content_size.y - layout.viewport_padding).max(layout.viewport_padding);
    pos.x = pos.x.clamp(layout.viewport_padding, max_x);
    pos.y = pos.y.clamp(layout.viewport_padding, max_y);
    pos
}

pub(crate) fn overlay_hover_bridge_rect(trigger_rect: Rect, content_rect: Rect) -> Option<Rect> {
    let padding = 12.0;
    let trigger_right = trigger_rect.pos.x + trigger_rect.size.x;
    let trigger_bottom = trigger_rect.pos.y + trigger_rect.size.y;
    let content_right = content_rect.pos.x + content_rect.size.x;
    let content_bottom = content_rect.pos.y + content_rect.size.y;

    if content_rect.pos.y >= trigger_bottom {
        let height = content_rect.pos.y - trigger_bottom;
        if height > 0.0 {
            return Some(Rect {
                pos: dvec2(trigger_rect.pos.x - padding, trigger_bottom),
                size: dvec2(trigger_rect.size.x + padding * 2.0, height),
            });
        }
    }

    if trigger_rect.pos.y >= content_bottom {
        let height = trigger_rect.pos.y - content_bottom;
        if height > 0.0 {
            return Some(Rect {
                pos: dvec2(trigger_rect.pos.x - padding, content_bottom),
                size: dvec2(trigger_rect.size.x + padding * 2.0, height),
            });
        }
    }

    if content_rect.pos.x >= trigger_right {
        let width = content_rect.pos.x - trigger_right;
        if width > 0.0 {
            return Some(Rect {
                pos: dvec2(trigger_right, trigger_rect.pos.y - padding),
                size: dvec2(width, trigger_rect.size.y + padding * 2.0),
            });
        }
    }

    if trigger_rect.pos.x >= content_right {
        let width = trigger_rect.pos.x - content_right;
        if width > 0.0 {
            return Some(Rect {
                pos: dvec2(content_right, trigger_rect.pos.y - padding),
                size: dvec2(width, trigger_rect.size.y + padding * 2.0),
            });
        }
    }

    None
}

pub(crate) fn overlay_pair_contains_abs(
    trigger_rect: Rect,
    content_rect: Rect,
    abs: Vec2d,
) -> bool {
    trigger_rect.contains(abs) || content_rect.contains(abs)
}

pub(crate) fn overlay_hover_zone_contains_abs(
    trigger_rect: Rect,
    content_rect: Rect,
    abs: Vec2d,
) -> bool {
    overlay_pair_contains_abs(trigger_rect, content_rect, abs)
        || overlay_hover_bridge_rect(trigger_rect, content_rect)
            .is_some_and(|bridge| bridge.contains(abs))
}

pub(crate) fn reclaim_overlay_pointer_down(
    cx: &mut Cx,
    event: &Event,
    trigger_rect: Rect,
    content_rect: Rect,
) {
    match event {
        Event::MouseDown(fe) => {
            let handled_area = fe.handled.get();
            if !handled_area.is_empty()
                && overlay_pair_contains_abs(trigger_rect, content_rect, fe.abs)
            {
                event.unhandle(cx, &handled_area);
            }
        }
        Event::TouchUpdate(te) => {
            for touch in &te.touches {
                if !matches!(touch.state, TouchState::Start) {
                    continue;
                }
                let handled_area = touch.handled.get();
                if !handled_area.is_empty()
                    && overlay_pair_contains_abs(trigger_rect, content_rect, touch.abs)
                {
                    event.unhandle(cx, &handled_area);
                    break;
                }
            }
        }
        _ => {}
    }
}

pub(crate) fn should_dismiss_overlay_on_pointer_up(
    trigger_rect: Rect,
    content_rect: Rect,
    abs: Vec2d,
) -> bool {
    !overlay_pair_contains_abs(trigger_rect, content_rect, abs)
}

pub(crate) fn sync_modal_open_state(
    cx: &mut Cx,
    overlay: &mut WidgetRef,
    is_synced_open: &mut bool,
    open: bool,
) {
    if *is_synced_open == open {
        return;
    }

    if let Some(mut modal) = overlay.borrow_mut::<Modal>() {
        if open {
            modal.open(cx);
        } else {
            modal.close(cx);
        }
    }

    *is_synced_open = open;
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn set_modal_widget_open<T, F>(
    cx: &mut Cx,
    overlay: &mut WidgetRef,
    open_state: &mut bool,
    is_synced_open: &mut bool,
    action_data: &WidgetActionData,
    uid: WidgetUid,
    open: bool,
    map_action: F,
) where
    T: WidgetActionTrait,
    F: FnOnce(bool) -> T,
{
    if *open_state == open {
        sync_modal_open_state(cx, overlay, is_synced_open, open);
        return;
    }

    *open_state = open;
    sync_modal_open_state(cx, overlay, is_synced_open, open);
    overlay.redraw(cx);
    emit_widget_action(cx, action_data, uid, map_action(open));
}

pub(crate) fn draw_modal_overlay(
    cx: &mut Cx2d,
    scope: &mut Scope,
    walk: Walk,
    layout: Layout,
    open: bool,
    overlay: &mut WidgetRef,
) -> DrawStep {
    if !open {
        return DrawStep::done();
    }

    cx.begin_turtle(walk, layout);
    let step = overlay.draw_walk(cx, scope, Walk::new(Size::fill(), Size::fill()));
    cx.end_turtle();
    step
}

pub(crate) fn modal_dismissed(overlay: &WidgetRef, cx: &Cx, actions: &Actions) -> bool {
    let content = overlay.widget(cx, &[live_id!(content)]);
    !content.is_empty()
        && actions
            .find_widget_action(content.widget_uid())
            .is_some_and(|action| matches!(action.cast(), ModalAction::Dismissed))
}

pub(crate) fn button_clicked(
    overlay: &WidgetRef,
    cx: &Cx,
    path: &[LiveId],
    actions: &Actions,
) -> bool {
    let button = overlay.widget(cx, path);
    !button.is_empty()
        && actions
            .find_widget_action(button.widget_uid())
            .is_some_and(|action| matches!(action.cast(), ButtonAction::Clicked(_)))
}

pub(crate) fn sync_popup_menu_state(
    vm: &mut ScriptVm,
    popup_menu_value: ScriptValue,
    popup_menu_state: &mut Option<PopupMenu>,
    popup_menu_state_key: &mut ScriptValue,
) {
    if popup_menu_value.is_nil() {
        *popup_menu_state = None;
        *popup_menu_state_key = ScriptValue::default();
        return;
    }
    if popup_menu_state.is_some() && *popup_menu_state_key == popup_menu_value {
        return;
    }
    *popup_menu_state = Some(PopupMenu::script_from_value(vm, popup_menu_value));
    *popup_menu_state_key = popup_menu_value;
}
