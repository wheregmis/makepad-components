use crate::internal::actions::emit_widget_action;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

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
