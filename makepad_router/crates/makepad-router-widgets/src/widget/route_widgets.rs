use makepad_widgets::*;

use super::RouterWidget;

impl RouterWidget {
    pub(super) fn new_route_widget_from_template(
        cx: &mut Cx,
        template: ScriptObjectRef,
    ) -> WidgetRef {
        let value: ScriptValue = template.as_object().into();
        cx.with_vm(|vm| WidgetRef::script_from_value(vm, value))
    }

    pub(super) fn ensure_route_widget(&mut self, cx: &mut Cx, route_id: LiveId) {
        if self.routes.widgets.contains_key(&route_id) {
            return;
        }
        let Some(template) = self.routes.templates.get(&route_id).cloned() else {
            return;
        };
        self.routes.widgets.get_or_insert(cx, route_id, |cx| {
            Self::new_route_widget_from_template(cx, template)
        });
        self.caches.child_router_scan_widget_count = 0;
    }
}
