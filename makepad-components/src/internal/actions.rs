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

pub(crate) fn first_widget_action<T>(actions: &Actions, uid: WidgetUid) -> Option<T>
where
    T: WidgetActionTrait + Default + Clone,
{
    actions.filter_widget_actions_cast::<T>(uid).next()
}

pub(crate) fn widget_action_map<T, U, F>(actions: &Actions, uid: WidgetUid, map: F) -> Option<U>
where
    T: WidgetActionTrait + Default + Clone,
    F: FnMut(T) -> Option<U>,
{
    actions.filter_widget_actions_cast::<T>(uid).find_map(map)
}
