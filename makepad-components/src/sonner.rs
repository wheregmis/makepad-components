use crate::internal::actions::emit_widget_action;
use crate::internal::overlay::button_clicked;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

const MAX_VISIBLE_TOASTS: usize = 4;
const DEFAULT_TIMEOUT_SEC: f64 = 5.0; // 默认 5 秒关闭

// --- 新增数据结构 ---

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SonnerKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
    Close,
}

#[derive(Default, Debug, Clone)]
pub struct SonnerItem {
    pub title: String,
    pub description: Option<String>,
    pub kind: SonnerKind,
    pub duration: Option<f64>,
    pub show_close: bool,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadToastTitle = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 12
    }

    mod.widgets.ShadToastDescription = mod.widgets.ShadAlertDescription{
        width: Fill
        height: Fit
    }
    let CheckIcon = RoundedView{
        width: 28
        height: 28
        visible: false
        draw_bg +: {
            color: #0000
            border_size: 0.0
            border_radius: 4.0
        }
        icon := Icon{
            draw_icon.svg: crate_resource("self://resources/icons/checkmark.svg")
            draw_icon.color: #22c55e
            icon_walk: Walk{width: 24, height: 24}
        }
    }
    let InfoIcon = CheckIcon{
        icon +: {
            draw_icon.svg: crate_resource("self://resources/icons/info.svg")
            draw_icon.color: (shad_theme.color_primary)
            icon_walk: Walk{width: 24, height: 24}
        }
    }

    let ForbiddenIcon = CheckIcon{
        icon +: {
            draw_icon.svg: crate_resource("self://resources/icons/forbidden.svg")
            draw_icon.color: #ef4444
            icon_walk: Walk{width: 24, height: 24}
        }
    }

    let WarningIcon = CheckIcon{
        icon +: {
            draw_icon.svg: crate_resource("self://resources/icons/warning.svg")
            draw_icon.color: #f59e0b
            icon_walk: Walk{width: 24, height: 24}
        }
    }
    let CloseIcon = CheckIcon{
        icon +: {
            draw_icon.svg: crate_resource("self://resources/icons/x.svg")
            draw_icon.color: (shad_theme.color_muted_foreground)
            icon_walk: Walk{width: 24, height: 24}
        }
    }
    let ToastSlotPanel = RoundedView{
        visible: false
        width: 260
        height: Fit
        padding: Inset{left: 14, right: 8, top: 10, bottom: 10}
        spacing: 4.0

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        header_row := View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0
            info_icon := InfoIcon{
                visible: false
            }
            success_icon :=CheckIcon{
                visible: false
            }
            warning_icon := WarningIcon{
                visible: false
            }
            error_icon := ForbiddenIcon{
                visible: false
            }
            View {
                width: Fill
                height: Fit
                flow: Down
                spacing: 4.0
                title_label := mod.widgets.ShadToastTitle{
                    text: "Notification"
                }
                description_label := mod.widgets.ShadToastDescription{
                    text: ""
                    visible: false
                }
            }

            close_btn := mod.widgets.IconButtonX{
                width: 24
                height: 24
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    border_size: 0.0
                    border_radius: (shad_theme.radius)
                }
                draw_icon.color: (shad_theme.color_muted_foreground)
            }
        }


    }

    mod.widgets.ShadSonnerBase = #(ShadSonner::register_widget(vm))

    mod.widgets.ShadSonner = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false

        overlay: PopupNotification{
            content +: {
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                margin: Inset{top: 25, right: 12}

                toast_0 := ToastSlotPanel{}
                toast_1 := ToastSlotPanel{}
                toast_2 := ToastSlotPanel{}
                toast_3 := ToastSlotPanel{}
            }
        }
    }

    // 为了兼容性保留旧组件名，但内部逻辑已统一
    mod.widgets.ShadSonnerWithClose = mod.widgets.ShadSonner{}
    mod.widgets.ShadSonnerWithDescription = mod.widgets.ShadSonner{}
}

#[derive(Clone, Debug, Default)]
pub enum ShadSonnerAction {
    OpenChanged(bool),
    #[default]
    None,
}

#[derive(Default)]
struct SonnerGlobalState {
    host_uid: Option<WidgetUid>,
    host_overlay: Option<WidgetRef>,
    toasts: VecDeque<SonnerItem>,
    toast_timers: VecDeque<f64>, // 存储剩余毫秒数
    rendered_toasts: [Option<SonnerItem>; MAX_VISIBLE_TOASTS],
    rendered_open: Option<bool>,
    timer: Timer,
}

#[derive(Default, Clone)]
struct SonnerGlobal {
    state: Rc<RefCell<SonnerGlobalState>>,
}

#[derive(Script, Widget)]
pub struct ShadSonner {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[find]
    #[redraw]
    #[live]
    overlay: WidgetRef,
    #[live]
    open: bool,
    #[rust]
    is_synced_open: bool,
    #[rust]
    last_applied_open: Option<bool>,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ScriptHook for ShadSonner {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        let applied_open = self.open;
        vm.with_cx_mut(|cx| {
            self.register_global_host(cx);
            if let Some(previous) = self.last_applied_open {
                if previous != applied_open && !applied_open {
                    self.clear_global_toasts(cx, true);
                }
            }
            self.last_applied_open = Some(applied_open);
            self.sync_toast_visibility(cx);
        });
    }
}

impl ShadSonner {
    fn default_title(kind: SonnerKind) -> &'static str {
        match kind {
            SonnerKind::Info => "Info",
            SonnerKind::Success => "Success",
            SonnerKind::Warning => "Warning",
            SonnerKind::Error => "Error",
            SonnerKind::Close => "Notification",
        }
    }
    fn visible_toasts_snapshot(
        state: &SonnerGlobalState,
    ) -> [Option<SonnerItem>; MAX_VISIBLE_TOASTS] {
        let mut visible = [const { None }; MAX_VISIBLE_TOASTS];
        for (index, item) in state
            .toasts
            .iter()
            .rev()
            .take(MAX_VISIBLE_TOASTS)
            .enumerate()
        {
            visible[index] = Some(item.clone());
        }
        visible
    }

    fn register_global_host(&mut self, cx: &mut Cx) {
        let global = cx.global::<SonnerGlobal>();
        let mut state = global.state.borrow_mut();
        if state.host_uid.is_none() || state.host_uid == Some(self.widget_uid()) {
            state.host_uid = Some(self.widget_uid());
            state.host_overlay = Some(self.overlay.clone());
        }
    }

    fn sync_overlay_slot(
        cx: &mut Cx,
        overlay: &WidgetRef,
        index: usize,
        item: Option<SonnerItem>,
    ) -> bool {
        let slot = overlay.widget(cx, Self::toast_slot_path(index));
        if slot.is_empty() {
            return false;
        }

        let Some(item) = item else {
            slot.set_visible(cx, false);
            return true;
        };

        slot.set_visible(cx, true);

        // 标题处理
        let title = if item.title.is_empty() {
            Self::default_title(item.kind)
        } else {
            &item.title
        };
        slot.label(cx, ids!(title_label)).set_text(cx, title);
        match item.kind {
            SonnerKind::Success => {
                slot.widget(cx, ids!(success_icon)).set_visible(cx, true);
            }
            SonnerKind::Error => {
                slot.widget(cx, ids!(error_icon)).set_visible(cx, true);
            }
            SonnerKind::Warning => {
                slot.widget(cx, ids!(warning_icon)).set_visible(cx, true);
            }
            SonnerKind::Info => {
                slot.widget(cx, ids!(info_icon)).set_visible(cx, true);
            }
            SonnerKind::Close => {
                slot.widget(cx, ids!(info_icon)).set_visible(cx, true);
            } // Close类型默认显示Info图标
        }
        // 描述处理
        if let Some(desc) = &item.description {
            slot.label(cx, ids!(description_label)).set_text(cx, desc);
            slot.widget(cx, ids!(description_label))
                .set_visible(cx, true);
        } else {
            slot.widget(cx, ids!(description_label))
                .set_visible(cx, false);
        }

        true
    }

    fn sync_global_host_overlay(cx: &mut Cx) {
        let global = cx.global::<SonnerGlobal>().clone();
        let (host_overlay, visible_toasts, rendered_open) = {
            let state = global.state.borrow();
            (
                state.host_overlay.clone(),
                Self::visible_toasts_snapshot(&state),
                state.rendered_open,
            )
        };

        if let Some(overlay) = host_overlay {
            let next_open = visible_toasts[0].is_some();
            if let Some(mut popup) = overlay.borrow_mut::<PopupNotification>() {
                if next_open {
                    popup.open(cx);
                } else {
                    popup.close(cx);
                }
            }

            for index in 0..MAX_VISIBLE_TOASTS {
                Self::sync_overlay_slot(cx, &overlay, index, visible_toasts[index].clone());
            }

            let mut state = global.state.borrow_mut();
            state.rendered_toasts = visible_toasts;
            state.rendered_open = Some(next_open);
            overlay.redraw(cx);
        }
    }

    fn toast_slot_path(index: usize) -> &'static [LiveId] {
        match index {
            0 => &[live_id!(content), live_id!(toast_0)],
            1 => &[live_id!(content), live_id!(toast_1)],
            2 => &[live_id!(content), live_id!(toast_2)],
            _ => &[live_id!(content), live_id!(toast_3)],
        }
    }

    fn close_button_path(index: usize) -> &'static [LiveId] {
        match index {
            0 => &[
                live_id!(content),
                live_id!(toast_0),
                live_id!(header_row),
                live_id!(close_btn),
            ],
            1 => &[
                live_id!(content),
                live_id!(toast_1),
                live_id!(header_row),
                live_id!(close_btn),
            ],
            2 => &[
                live_id!(content),
                live_id!(toast_2),
                live_id!(header_row),
                live_id!(close_btn),
            ],
            _ => &[
                live_id!(content),
                live_id!(toast_3),
                live_id!(header_row),
                live_id!(close_btn),
            ],
        }
    }

    fn sync_overlay_open_state(&mut self, cx: &mut Cx) -> bool {
        let global = cx.global::<SonnerGlobal>().clone();
        let (host_uid, open) = {
            let state = global.state.borrow();
            (state.host_uid, !state.toasts.is_empty())
        };
        let is_host = host_uid == Some(self.widget_uid());
        self.open = open;
        if is_host && self.is_synced_open != open {
            if let Some(mut popup) = self.overlay.borrow_mut::<PopupNotification>() {
                if open {
                    popup.open(cx);
                } else {
                    popup.close(cx);
                }
            }
            self.is_synced_open = open;
        }
        is_host
    }

    fn sync_toast_visibility(&mut self, cx: &mut Cx) {
        if !cx
            .global::<SonnerGlobal>()
            .state
            .borrow()
            .host_uid
            .map_or(false, |uid| uid == self.widget_uid())
        {
            return;
        }
        let visible = self.visible_toasts(cx);
        for (index, item) in visible.into_iter().enumerate() {
            Self::sync_overlay_slot(cx, &self.overlay, index, item);

        let visible_toasts = self.visible_toasts(cx);
        for (index, kind) in visible_toasts
            .into_iter()
            .enumerate()
            .take(MAX_VISIBLE_TOASTS)
        {
            Self::sync_overlay_slot(cx, &self.overlay, index, kind);
        }
        self.sync_overlay_open_state(cx);
    }

    fn visible_toasts(&self, cx: &mut Cx) -> [Option<SonnerItem>; MAX_VISIBLE_TOASTS] {
        let state = cx.global::<SonnerGlobal>().state.borrow();
        Self::visible_toasts_snapshot(&state)
    }

    fn emit_open_state(&self, cx: &mut Cx, open: bool) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadSonnerAction::OpenChanged(open),
        );
    }

    // --- 核心推送方法 ---
    pub fn enqueue(&mut self, cx: &mut Cx, item: SonnerItem) {
        let (was_empty, is_open) = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();
            let was_empty = state.toasts.is_empty();

            if state.toasts.len() >= MAX_VISIBLE_TOASTS {
                state.toasts.pop_front();
                state.toast_timers.pop_front();
            }

            let timeout_ms = item.duration.unwrap_or(DEFAULT_TIMEOUT_SEC) * 1000.0;
            state.toasts.push_back(item);
            state.toast_timers.push_back(timeout_ms);

            if was_empty {
                state.timer = cx.start_timeout(0.1); 
            }
            (was_empty, true)
        };

        self.open = is_open;
        Self::sync_global_host_overlay(cx);
        if was_empty {
            self.emit_open_state(cx, true);
        }
    }

    pub fn clear_global_toasts(&mut self, cx: &mut Cx, emit_action: bool) {
        let was_open = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();
            let was_open = !state.toasts.is_empty();
            state.toasts.clear();
            state.toast_timers.clear();
            state.timer = Timer::default();
            was_open
        };
        self.open = false;
        Self::sync_global_host_overlay(cx);
        if emit_action && was_open {
            self.emit_open_state(cx, false);
        }
    }

    fn remove_visible_toast(&mut self, cx: &mut Cx, visible_index: usize) {
        let global = cx.global::<SonnerGlobal>().clone();
        let mut state = global.state.borrow_mut();
        if visible_index < state.toasts.len() {
            let queue_index = state.toasts.len() - 1 - visible_index;
            state.toasts.remove(queue_index);
            state.toast_timers.remove(queue_index);
            if state.toasts.is_empty() {
                state.timer = Timer::default();
            }
            drop(state);
            Self::sync_global_host_overlay(cx);
        }
    }
}

impl Widget for ShadSonner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let global = cx.global::<SonnerGlobal>().clone();

        // 定时器处理
        if let Event::Timer(te) = event {
            let mut state = global.state.borrow_mut();
            if state.timer.is_timer(te).is_some() {
                let mut changed = false;
                for time in state.toast_timers.iter_mut() {
                    *time -= 100.0;
                }

                while let Some(first_time) = state.toast_timers.front() {
                    if *first_time <= 0.0 {
                        state.toasts.pop_front();
                        state.toast_timers.pop_front();
                        changed = true;
                    } else {
                        break;
                    }
                }

                if !state.toasts.is_empty() {
                    state.timer = cx.start_timeout(0.1);
                } else {
                    state.timer = Timer::default();
                }

                drop(state);
                if changed {
                    Self::sync_global_host_overlay(cx);
                }
                return;
            }
        }

        self.register_global_host(cx);
        let is_host = self.sync_overlay_open_state(cx);
        if !is_host || !self.open {
            return;
        }

        self.overlay.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            for index in 0..MAX_VISIBLE_TOASTS {
                if button_clicked(&self.overlay, cx, Self::close_button_path(index), actions) {
                    self.remove_visible_toast(cx, index);
                    break;
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.register_global_host(cx);
        if !self.sync_overlay_open_state(cx) || !self.open {
            return DrawStep::done();
        }
        cx.begin_turtle(walk, self.layout);
        let step = self
            .overlay
            .draw_walk(cx, scope, Walk::new(Size::fill(), Size::fill()));
        cx.end_turtle();
        step
    }
}

// 为旧接口提供简单封装
impl ShadSonnerRef {
    pub fn enqueue(&self, cx: &mut Cx, item: SonnerItem) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.enqueue(cx, item);
        }
    }
}
