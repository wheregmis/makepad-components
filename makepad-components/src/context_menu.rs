use makepad_widgets::popup_menu::{PopupMenu, PopupMenuAction};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::{cell::RefCell, rc::Rc};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadContextMenuItem = mod.widgets.PopupMenuItem{
        width: Fill
        height: Fit
        align: Align{y: 0.5}
        padding: Inset{left: 12, right: 12, top: 8, bottom: 8}

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11
        }

        draw_bg +: {
            border_size: 0.0
            border_radius: 6.0
            color: #0000
            color_hover: (shad_theme.color_secondary)
            color_active: (shad_theme.color_secondary_hover)
            color_disabled: #0000
            border_color: #0000
            border_color_hover: #0000
            border_color_active: #0000
            border_color_disabled: #0000
            mark_color: #0000
            mark_color_active: #0000
            mark_color_disabled: #0000
            color_dither: 0.0
        }
    }

    mod.widgets.ShadContextMenuContent = mod.widgets.PopupMenu{
        width: 200
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}
        menu_item: mod.widgets.ShadContextMenuItem{}

        draw_bg +: {
            border_size: 1.0
            border_radius: (shad_theme.radius)
            color: (shad_theme.color_background)
            border_color: (shad_theme.color_outline_border)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadContextMenuBase = #(ShadContextMenu::register_widget(vm))

    mod.widgets.ShadContextMenu = set_type_default() do mod.widgets.ShadContextMenuBase{
        width: Fit
        height: Fit
        flow: Overlay
        labels: ["Back" "Forward" "Reload"]
        popup_menu: mod.widgets.ShadContextMenuContent{}
    }
}

#[derive(Default, Clone)]
struct PopupMenuGlobal {
    map: Rc<RefCell<ComponentMap<ScriptValue, PopupMenu>>>,
}

#[derive(Clone, Debug, Default)]
pub enum ShadContextMenuAction {
    Changed(usize),
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadContextMenu {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    labels: Vec<String>,
    #[live]
    popup_menu: ScriptValue,

    #[rust]
    is_active: bool,
    #[rust]
    open_shift: Vec2d,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadContextMenu {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _obj: ScriptValue,
    ) {
        if self.popup_menu.is_nil() {
            return;
        }
        vm.with_cx_mut(|cx| {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let Ok(mut map) = global.map.try_borrow_mut() else {
                return;
            };

            let popup_menu_val = self.popup_menu;
            map.get_or_insert(cx, popup_menu_val, |cx| {
                cx.with_vm(|vm| PopupMenu::script_from_value(vm, popup_menu_val))
            });
        });
    }
}

impl ShadContextMenu {
    fn open_at_abs(&mut self, cx: &mut Cx, abs: Vec2d) {
        self.is_active = true;
        let rect = self.view.area().rect(cx);
        self.open_shift = abs - rect.pos;
        self.redraw(cx);
        cx.sweep_lock(self.view.area());
    }

    fn close(&mut self, cx: &mut Cx) {
        self.is_active = false;
        self.redraw(cx);
        cx.sweep_unlock(self.view.area());
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadContextMenuAction::Changed(index) = item.cast() {
                return Some(index);
            }
        }
        None
    }
}

impl Widget for ShadContextMenu {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        let uid = self.widget_uid();

        if self.is_active && !self.popup_menu.is_nil() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let menu = map.get_mut(&self.popup_menu).unwrap();
            let mut close = false;
            menu.handle_event_with(
                cx,
                event,
                self.view.area(),
                &mut |cx, action| match action {
                    PopupMenuAction::WasSweeped(_node_id) => {}
                    PopupMenuAction::WasSelected(node_id) => {
                        let index = node_id.0 .0 as usize;
                        cx.widget_action_with_data(
                            &self.action_data,
                            uid,
                            ShadContextMenuAction::Changed(index),
                        );
                        close = true;
                    }
                    _ => (),
                },
            );
            if close {
                self.close(cx);
            }

            match event {
                Event::MouseDown(e) => {
                    if !menu.menu_contains_pos(cx, e.abs)
                        && !self.view.area().rect(cx).contains(e.abs)
                    {
                        self.close(cx);
                        return;
                    }
                }
                Event::BackPressed { .. } => {
                    self.close(cx);
                    return;
                }
                _ => {}
            }
        }

        match event.hits_with_sweep_area(cx, self.view.area(), self.view.area()) {
            Hit::FingerDown(fe) if fe.mouse_button().is_some_and(|mb| mb.is_secondary()) => {
                self.open_at_abs(cx, fe.abs);
            }
            Hit::FingerLongPress(lp) => {
                self.open_at_abs(cx, lp.abs);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)?;

        if self.is_active && !self.popup_menu.is_nil() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup_menu).unwrap();

            popup_menu.begin(cx);
            for (i, item) in self.labels.iter().enumerate() {
                let node_id = LiveId(i as u64).into();
                popup_menu.draw_item(cx, node_id, item);
            }
            popup_menu.end(cx, self.view.area(), self.open_shift);
        }

        DrawStep::done()
    }
}

impl ShadContextMenuRef {
    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }
}
