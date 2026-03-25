use makepad_widgets::popup_menu::{PopupMenu, PopupMenuAction};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::{cell::RefCell, rc::Rc};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSelectDrawLabelTextBase = #(ShadSelectDrawLabelText::script_component(vm))
    set_type_default() do #(ShadSelectDrawLabelText::script_shader(vm)){
        ..mod.draw.DrawText
    }

    mod.widgets.ShadSelectBase = #(ShadSelect::register_widget(vm))

    mod.widgets.ShadSelectItem = mod.widgets.PopupMenuItem{
        width: Fill
        height: Fit
        align: Align{y: 0.5}
        padding: Inset{left: 24, right: 12, top: 8, bottom: 8}

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
            mark_color_active: (shad_theme.color_primary)
            mark_color_disabled: (shad_theme.color_muted_foreground)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelectPopupMenu = mod.widgets.PopupMenu{
        width: 220
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}
        menu_item: mod.widgets.ShadSelectItem{}

        draw_bg +: {
            border_size: 1.0
            border_radius: (shad_theme.radius)
            color: (shad_theme.color_background)
            border_color: (shad_theme.color_outline_border)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelect = set_type_default() do mod.widgets.ShadSelectBase{
        width: 220
        height: 36
        align: Align{x: 0.0, y: 0.5}
        popup_menu: mod.widgets.ShadSelectPopupMenu{}
        selected_item: 0

        padding: Inset{left: 12, right: 28, top: 0, bottom: 0}

        draw_text +: {
            disabled: instance(0.0)
            down: instance(0.0)

            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11
        }

        draw_bg +: {
            hover: instance(0.0)
            focus: instance(0.0)
            down: instance(0.0)
            active: instance(0.0)
            disabled: instance(0.0)

            border_radius: (shad_theme.radius)
            border_size: 1.0
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_focus: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_active: (shad_theme.color_ghost_hover)
            color_disabled: #0000
            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
            border_color_down: (shad_theme.color_primary)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)
            arrow_color: (shad_theme.color_primary)
            arrow_color_hover: (shad_theme.color_primary)
            arrow_color_focus: (shad_theme.color_primary)
            arrow_color_down: (shad_theme.color_primary)
            arrow_color_disabled: (shad_theme.color_muted_foreground)
        }

        animator : Animator{
            disabled: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward{duration: 0.}}
                    apply: {
                        draw_bg: {disabled: 0.0}
                        draw_text: {disabled: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward{duration: 0.2}}
                    apply: {
                        draw_bg: {disabled: 1.0}
                        draw_text: {disabled: 1.0}
                    }
                }
            }
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward{duration: 0.1}}
                    apply: {
                        draw_bg: {down: 0.0, hover: 0.0}
                        draw_text: {down: 0.0, hover: 0.0}
                    }
                }

                on: AnimatorState{
                    from: {
                        all: Forward{duration: 0.1}
                        down: Forward{duration: 0.01}
                    }
                    apply: {
                        draw_bg: {down: 0.0, hover: [{time: 0.0, value: 1.0}]}
                        draw_text: {down: 0.0, hover: [{time: 0.0, value: 1.0}]}
                    }
                }

                down: AnimatorState{
                    from: {all: Forward{duration: 0.2}}
                    apply: {
                        draw_bg: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                        draw_text: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                    }
                }
            }
            focus: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward{duration: 0.2}}
                    apply: {
                        draw_bg: {focus: 0.0}
                        draw_text: {focus: 0.0}
                    }
                }
                on: AnimatorState{
                    cursor: MouseCursor.Arrow
                    from: {all: Forward{duration: 0.0}}
                    apply: {
                        draw_bg: {focus: 1.0}
                        draw_text: {focus: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Default, Clone)]
struct ShadSelectPopupMenuGlobal {
    map: Rc<RefCell<ComponentMap<ScriptValue, PopupMenu>>>,
}

#[derive(Script, ScriptHook)]
#[repr(C)]
struct ShadSelectDrawLabelText {
    #[deref]
    draw_super: DrawText,
    #[live]
    focus: f32,
    #[live]
    hover: f32,
}

#[derive(Script, Widget, Animator)]
pub struct ShadSelect {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_text: ShadSelectDrawLabelText,

    #[walk]
    walk: Walk,

    #[live]
    bind: String,
    #[live]
    bind_enum: String,

    #[live]
    popup_menu: ScriptValue,

    #[live]
    labels: Vec<String>,

    #[live(PopupMenuPosition::BelowInput)]
    popup_menu_position: PopupMenuPosition,

    #[rust]
    is_active: bool,

    #[live]
    selected_item: usize,

    #[layout]
    layout: Layout,

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
        if self.popup_menu.is_nil() {
            return;
        }

        vm.with_cx_mut(|cx| {
            let global = cx.global::<ShadSelectPopupMenuGlobal>().clone();
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

impl ShadSelect {
    pub fn set_active(&mut self, cx: &mut Cx) {
        self.is_active = true;
        self.draw_bg.redraw(cx);
        let global = cx.global::<ShadSelectPopupMenuGlobal>().clone();
        let mut map = global.map.borrow_mut();
        let popup_menu = map.get_mut(&self.popup_menu).unwrap();
        let node_id = LiveId(self.selected_item as u64).into();
        popup_menu.init_select_item(node_id);
        cx.sweep_lock(self.draw_bg.area());
    }

    pub fn set_closed(&mut self, cx: &mut Cx) {
        self.is_active = false;
        self.draw_bg.redraw(cx);
        cx.sweep_unlock(self.draw_bg.area());
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DropDownAction::Select(index) = item.cast() {
                return Some(index);
            }
        }
        None
    }

    pub fn changed_label(&self, actions: &Actions) -> Option<String> {
        self.changed(actions)
            .and_then(|index| self.labels.get(index).cloned())
    }

    pub fn set_selected_item(&mut self, cx: &mut Cx, item: usize) {
        let new_selected = item.min(self.labels.len().max(1) - 1);
        if new_selected != self.selected_item {
            self.selected_item = new_selected;
            self.draw_bg.redraw(cx);
        }
    }

    pub fn selected_item(&self) -> usize {
        self.selected_item
    }

    pub fn selected_label(&self) -> String {
        self.labels
            .get(self.selected_item)
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_selected_by_label(&mut self, label: &str, cx: &mut Cx) {
        if let Some(index) = self.labels.iter().position(|value| value == label) {
            if self.selected_item != index {
                self.selected_item = index;
                self.draw_bg.redraw(cx);
            }
        }
    }

    pub fn set_labels(&mut self, cx: &mut Cx, labels: Vec<String>) {
        self.labels = labels;
        if self.selected_item >= self.labels.len() {
            self.selected_item = self.labels.len().saturating_sub(1);
        }
        self.draw_bg.redraw(cx);
    }

    fn draw_select(&mut self, cx: &mut Cx2d, walk: Walk) {
        self.draw_bg.begin(cx, walk, self.layout);

        if let Some(value) = self.labels.get(self.selected_item) {
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), value);
        } else {
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), " ");
        }
        self.draw_bg.end(cx);

        cx.add_nav_stop(self.draw_bg.area(), NavRole::DropDown, Inset::default());

        if self.is_active && !self.popup_menu.is_nil() {
            let global = cx.global::<ShadSelectPopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup_menu).unwrap();
            let area = self.draw_bg.area().rect(cx);

            script_apply_eval!(cx, popup_menu, {
                width: #(area.size.x)
            });

            popup_menu.begin(cx);

            match self.popup_menu_position {
                PopupMenuPosition::OnSelected => {
                    let mut item_pos = None;
                    for (index, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(index as u64).into();
                        if index == self.selected_item {
                            item_pos = Some(cx.turtle().pos());
                        }
                        popup_menu.draw_item(cx, node_id, item);
                    }

                    popup_menu.end(
                        cx,
                        self.draw_bg.area(),
                        -item_pos.unwrap_or(dvec2(0.0, 0.0)),
                    );
                }
                PopupMenuPosition::BelowInput => {
                    for (index, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(index as u64).into();
                        popup_menu.draw_item(cx, node_id, item);
                    }

                    popup_menu.end(
                        cx,
                        self.draw_bg.area(),
                        dvec2(0.0, area.size.y),
                    );
                }
            }
        }
    }
}

impl Widget for ShadSelect {
    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.animator_toggle(
            cx,
            disabled,
            animator::Animate::Yes,
            ids!(disabled.on),
            ids!(disabled.off),
        );
    }

    fn disabled(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(disabled.on))
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        let uid = self.widget_uid();

        if self.is_active && !self.popup_menu.is_nil() {
            let global = cx.global::<ShadSelectPopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let menu = map.get_mut(&self.popup_menu).unwrap();
            let mut close = false;

            menu.handle_event_with(
                cx,
                event,
                self.draw_bg.area(),
                &mut |cx, action| match action {
                    PopupMenuAction::WasSweeped(_node_id) => {}
                    PopupMenuAction::WasSelected(node_id) => {
                        self.selected_item = node_id.0 .0 as usize;
                        cx.widget_action_with_data(
                            &self.action_data,
                            uid,
                            DropDownAction::Select(self.selected_item),
                        );
                        self.draw_bg.redraw(cx);
                        close = true;
                    }
                    _ => {}
                },
            );

            if close {
                self.set_closed(cx);
            }

            if let Event::MouseDown(e) = event {
                if !menu.menu_contains_pos(cx, e.abs) {
                    self.set_closed(cx);
                    self.animator_play(cx, ids!(hover.off));
                    return;
                }
            }
        }

        match event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area()) {
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, ids!(focus.off));
                self.set_closed(cx);
                self.animator_play(cx, ids!(hover.off));
                self.draw_bg.redraw(cx);
            }
            Hit::KeyFocus(_) => {
                self.animator_play(cx, ids!(focus.on));
            }
            Hit::KeyDown(ke) => match ke.key_code {
                KeyCode::ArrowUp => {
                    if self.selected_item > 0 {
                        self.selected_item -= 1;
                        cx.widget_action_with_data(
                            &self.action_data,
                            uid,
                            DropDownAction::Select(self.selected_item),
                        );
                        self.set_closed(cx);
                        self.draw_bg.redraw(cx);
                    }
                }
                KeyCode::ArrowDown => {
                    if !self.labels.is_empty() && self.selected_item < self.labels.len() - 1 {
                        self.selected_item += 1;
                        cx.widget_action_with_data(
                            &self.action_data,
                            uid,
                            DropDownAction::Select(self.selected_item),
                        );
                        self.set_closed(cx);
                        self.draw_bg.redraw(cx);
                    }
                }
                _ => {}
            },
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                if self.animator_in_state(cx, ids!(disabled.off)) {
                    cx.set_key_focus(self.draw_bg.area());
                    self.animator_play(cx, ids!(hover.down));
                    self.set_active(cx);
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) if fe.is_primary_hit() => {
                if fe.is_over {
                    if fe.device.has_hovers() {
                        self.animator_play(cx, ids!(hover.on));
                    }
                } else {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => {}
        };
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_select(cx, walk);
        DrawStep::done()
    }
}

impl ShadSelectRef {
    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }

    pub fn changed_label(&self, _cx: &Cx, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.changed_label(actions))
    }

    pub fn set_selected_item(&self, cx: &mut Cx, item: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected_item(cx, item);
        }
    }

    pub fn selected_item(&self, _cx: &Cx) -> usize {
        self.borrow().map_or(0, |inner| inner.selected_item())
    }

    pub fn selected_label(&self, _cx: &Cx) -> String {
        self.borrow()
            .map_or_else(String::new, |inner| inner.selected_label())
    }

    pub fn set_selected_by_label(&self, label: &str, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected_by_label(label, cx);
        }
    }

    pub fn set_labels(&self, cx: &mut Cx, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_labels(cx, labels);
        }
    }
}
