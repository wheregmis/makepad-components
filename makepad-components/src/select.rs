use crate::internal::overlay::sync_popup_menu_state;
use makepad_widgets::popup_menu::{PopupMenu, PopupMenuAction};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSelectItem = mod.widgets.PopupMenuItem{
        width: Fill
        height: Fit
        align: Align{y: 0.5}
        padding: Inset{left: 24, right: 12, top: 8, bottom: 8}

        draw_text +: {
            color: (shad_theme.color_text)
            color_hover: (shad_theme.color_text)
            color_active: (shad_theme.color_text)
            color_disabled: (shad_theme.color_text_muted)
            text_style.font_size: (shad_theme.control_font_size_md)
        }

        draw_bg +: {
            border_size: 0.0
            border_radius: (shad_theme.radius)
            color: #0000
            color_hover: (shad_theme.color_secondary)
            color_active: (shad_theme.color_secondary_hover)
            color_disabled: #0000
            border_color: #0000
            border_color_hover: #0000
            border_color_active: #0000
            border_color_disabled: #0000
            mark_color: #0000
            mark_color_active: (shad_theme.color_text)
            mark_color_disabled: (shad_theme.color_text_muted)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelectPopupMenu = mod.widgets.PopupMenu{
        width: 220
        padding: Inset{left: 6, right: 6, top: 6, bottom: 6}
        menu_item: mod.widgets.ShadSelectItem{}

        draw_bg +: {
            border_size: 1.0
            border_radius: (shad_theme.radius)
            color: (shad_theme.color_surface_popover)
            border_color: (shad_theme.color_border)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelectBase = #(ShadSelect::register_widget(vm))

    mod.widgets.ShadSelect = set_type_default() do mod.widgets.ShadSelectBase{
        width: 220
        height: Fit
        labels: ["Select option"]
        selected_item: 0
        popup_menu: mod.widgets.ShadSelectPopupMenu{}

        trigger := ShadButtonOutline{
            width: Fill
            height: (shad_theme.control_height_md)
            align: Align{x: 0.0, y: 0.5}
            text: "Select option"
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadSelectAction {
    Changed(usize),
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadSelect {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    labels: Vec<String>,
    #[live]
    popup_menu: ScriptValue,
    #[live]
    selected_item: usize,

    #[rust]
    popup_menu_state: Option<PopupMenu>,
    #[rust]
    popup_menu_state_key: ScriptValue,
    #[rust]
    is_open: bool,
    #[rust]
    trigger_text_cache: String,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadSelect {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _obj: ScriptValue,
    ) {
        sync_popup_menu_state(
            vm,
            self.popup_menu,
            &mut self.popup_menu_state,
            &mut self.popup_menu_state_key,
        );

        vm.with_cx_mut(|cx| {
            self.selected_item = self
                .selected_item
                .min(self.labels.len().saturating_sub(1));
            self.sync_trigger_text(cx);
        });
    }
}

impl ShadSelect {
    fn selected_label_value(&self) -> String {
        self.labels
            .get(self.selected_item)
            .cloned()
            .unwrap_or_default()
    }

    fn trigger_ref(&self, cx: &Cx) -> ButtonRef {
        self.view.button(cx, ids!(trigger))
    }

    fn trigger_area(&self, cx: &Cx) -> Area {
        let trigger = self.trigger_ref(cx);
        if trigger.is_empty() {
            self.view.area()
        } else {
            trigger.area()
        }
    }

    fn trigger_rect(&self, cx: &Cx) -> Rect {
        self.trigger_area(cx).rect(cx)
    }

    fn sync_trigger_text(&mut self, cx: &mut Cx) {
        let next_text = self.selected_label_value();
        if self.trigger_text_cache == next_text {
            return;
        }
        self.trigger_ref(cx).set_text(cx, &next_text);
        self.trigger_text_cache = next_text;
    }

    fn open(&mut self, cx: &mut Cx) {
        if self.is_open || self.popup_menu.is_nil() {
            return;
        }
        if let Some(menu) = self.popup_menu_state.as_mut() {
            let node_id = LiveId(self.selected_item as u64).into();
            menu.init_select_item(node_id);
        }
        self.is_open = true;
        self.view.redraw(cx);
        cx.sweep_lock(self.trigger_area(cx));
    }

    fn close(&mut self, cx: &mut Cx) {
        if !self.is_open {
            return;
        }
        self.is_open = false;
        self.view.redraw(cx);
        cx.sweep_unlock(self.trigger_area(cx));
    }

    fn set_selected_item_inner(
        &mut self,
        cx: &mut Cx,
        item: usize,
        emit_action: bool,
    ) -> bool {
        let next = item.min(self.labels.len().saturating_sub(1));
        if next == self.selected_item {
            return false;
        }
        self.selected_item = next;
        self.sync_trigger_text(cx);
        self.view.redraw(cx);
        if emit_action {
            cx.widget_action_with_data(
                &self.action_data,
                self.widget_uid(),
                ShadSelectAction::Changed(self.selected_item),
            );
        }
        true
    }

    pub fn set_selected_item(&mut self, cx: &mut Cx, item: usize) {
        let _ = self.set_selected_item_inner(cx, item, false);
    }

    pub fn set_selected_by_label(&mut self, label: &str, cx: &mut Cx) {
        if let Some(index) = self.labels.iter().position(|value| value == label) {
            let _ = self.set_selected_item_inner(cx, index, false);
        }
    }

    pub fn selected_item(&self) -> usize {
        self.selected_item
    }

    pub fn selected_label(&self) -> String {
        self.selected_label_value()
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadSelectAction::Changed(index) = item.cast() {
                return Some(index);
            }
        }
        None
    }

    pub fn changed_label(&self, actions: &Actions) -> Option<String> {
        self.changed(actions)
            .and_then(|index| self.labels.get(index).cloned())
    }

}

impl Widget for ShadSelect {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self.trigger_ref(cx).clicked(actions) {
                if self.is_open {
                    self.close(cx);
                } else {
                    self.open(cx);
                }
                return;
            }
        }

        if self.is_open {
            let trigger_rect = self.trigger_rect(cx);
            let trigger_area = self.trigger_area(cx);
            if let Some(menu) = self.popup_menu_state.as_ref() {
                match event {
                    Event::MouseDown(fe) => {
                        let handled_area = fe.handled.get();
                        if !handled_area.is_empty()
                            && (trigger_rect.contains(fe.abs) || menu.menu_contains_pos(cx, fe.abs))
                        {
                            event.unhandle(cx, &handled_area);
                        }
                    }
                    Event::TouchUpdate(te) => {
                        for touch in &te.touches {
                            let handled_area = touch.handled.get();
                            if !handled_area.is_empty()
                                && (trigger_rect.contains(touch.abs)
                                    || menu.menu_contains_pos(cx, touch.abs))
                            {
                                event.unhandle(cx, &handled_area);
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
            let Some(menu) = self.popup_menu_state.as_mut() else {
                self.close(cx);
                return;
            };

            let mut selected = None;
            menu.handle_event_with(cx, event, trigger_area, &mut |_, action| {
                if let PopupMenuAction::WasSelected(node_id) = action {
                    selected = Some(node_id.0 .0 as usize);
                }
            });

            if let Some(index) = selected {
                let _ = self.set_selected_item_inner(cx, index, true);
                self.close(cx);
                return;
            }

            if let Event::MouseDown(e) = event {
                if !trigger_rect.contains(e.abs) && !menu.menu_contains_pos(cx, e.abs) {
                    self.close(cx);
                    return;
                }
            }

            if matches!(event, Event::BackPressed { .. }) {
                self.close(cx);
                return;
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)?;

        if self.is_open {
            let trigger_area = self.trigger_area(cx);
            let trigger_rect = trigger_area.rect(cx);
            let Some(menu) = self.popup_menu_state.as_mut() else {
                self.is_open = false;
                return DrawStep::done();
            };

            menu.begin(cx);
            for (index, label) in self.labels.iter().enumerate() {
                let node_id = LiveId(index as u64).into();
                menu.draw_item(cx, node_id, label);
            }
            menu.end(cx, trigger_area, dvec2(0.0, trigger_rect.size.y));
        }

        DrawStep::done()
    }
}

impl ShadSelectRef {
    pub fn set_selected_item(&self, cx: &mut Cx, item: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected_item(cx, item);
        }
    }

    pub fn selected_item(&self) -> usize {
        self.borrow().map_or(0, |inner| inner.selected_item())
    }

    pub fn selected_label(&self) -> String {
        self.borrow()
            .map_or_else(String::new, |inner| inner.selected_label())
    }

    pub fn set_selected_by_label(&self, label: &str, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected_by_label(label, cx);
        }
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }

    pub fn changed_label(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.changed_label(actions))
    }
}
