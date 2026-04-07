use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

pub(crate) fn emit_widget_action<T>(
    cx: &mut Cx,
    action_data: &WidgetActionData,
    uid: WidgetUid,
    action: T,
) where
    T: WidgetActionTrait,
{
    cx.widget_action_with_data(action_data, uid, action);
}

pub(crate) fn emit_open_changed_action<T, F>(
    cx: &mut Cx,
    action_data: &WidgetActionData,
    uid: WidgetUid,
    open: bool,
    map: F,
) where
    T: WidgetActionTrait,
    F: FnOnce(bool) -> T,
{
    emit_widget_action(cx, action_data, uid, map(open));
}

pub(crate) fn widget_action_map<T, U, F>(actions: &Actions, uid: WidgetUid, map: F) -> Option<U>
where
    T: WidgetActionTrait + Default + Clone,
    F: FnMut(T) -> Option<U>,
{
    actions.filter_widget_actions_cast::<T>(uid).find_map(map)
}

pub(crate) fn open_changed_action<T, F>(actions: &Actions, uid: WidgetUid, map: F) -> Option<bool>
where
    T: WidgetActionTrait + Default + Clone,
    F: FnMut(T) -> Option<bool>,
{
    widget_action_map::<T, _, _>(actions, uid, map)
}
