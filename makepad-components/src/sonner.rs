use crate::internal::actions::emit_widget_action;
use crate::internal::overlay::button_clicked;
use crate::progress_test::MyProgressBarWidgetRefExt;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

const MAX_VISIBLE_TOASTS: usize = 4;
const DEFAULT_TIMEOUT_SEC: f64 = 5.0;

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

#[derive(Debug, Clone)]
struct SonnerToastEntry {
    item: SonnerItem,
    expires_at: Option<f64>,
    total_duration: f64,
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
            draw_icon.color: (shad_theme.color_success)
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
            draw_icon.color: (shad_theme.color_error)
            icon_walk: Walk{width: 24, height: 24}
        }
    }

    let WarningIcon = CheckIcon{
        icon +: {
            draw_icon.svg: crate_resource("self://resources/icons/warning.svg")
            draw_icon.color: (shad_theme.color_warning)
            icon_walk: Walk{width: 24, height: 24}
        }
    }
    let ToastSlotPanel = RoundedView {
        visible: false
        width: 280
        height: Fit
        flow: Down
        padding: 0.0
        spacing: 0.0

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        content_row := View {
            width: Fill
            height: Fit
            flow: Right
            align: Align {y: 0.5}
            padding: Inset {left: 14, right: 8, top: 10, bottom: 10}
            spacing: 8.0

            info_icon := InfoIcon { visible: false }
            success_icon := CheckIcon { visible: false }
            warning_icon := WarningIcon { visible: false }
            error_icon := ForbiddenIcon { visible: false }

            View {
                width: Fill
                height: Fit
                flow: Down
                spacing: 4.0
                title_label := mod.widgets.ShadToastTitle { text: "Notification" }
                description_label := mod.widgets.ShadToastDescription { text: "", visible: false }
            }

            close_btn := mod.widgets.IconButtonX{
                visible: false
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

        // 底部进度条
        progress_bar := mod.widgets.MyProgressBar {}
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
    toasts: VecDeque<SonnerToastEntry>,
    rendered_toasts: [Option<SonnerItem>; MAX_VISIBLE_TOASTS],
    rendered_open: Option<bool>,
    // timer: Timer,
    needs_next_frame: bool,
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
            if applied_open && self.last_applied_open != Some(true) {
                let should_enqueue = {
                    let global = cx.global::<SonnerGlobal>().clone();
                    let state = global.state.borrow();
                    state.toasts.is_empty()
                };
                if should_enqueue {
                    self.enqueue(cx, Self::default_open_item());
                }
            } else if !applied_open && self.last_applied_open == Some(true) {
                self.clear_global_toasts(cx, true);
            }
            self.last_applied_open = Some(applied_open);
            self.sync_toast_visibility(cx);
        });
    }
}

impl ShadSonner {
    fn default_open_item() -> SonnerItem {
        SonnerItem {
            title: String::new(),
            description: None,
            kind: SonnerKind::Info,
            duration: None,
            show_close: false,
        }
    }

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
        for (index, entry) in state
            .toasts
            .iter()
            .rev()
            .take(MAX_VISIBLE_TOASTS)
            .enumerate()
        {
            visible[index] = Some(entry.item.clone());
        }
        visible
    }

    fn prune_expired_toasts(state: &mut SonnerGlobalState, now: f64) -> bool {
        let mut changed = false;
        let mut index = 0;
        while index < state.toasts.len() {
            if let Some(exp) = state.toasts[index].expires_at {
                if exp <= now {
                    state.toasts.remove(index);
                    changed = true;
                    continue;
                }
            }
            index += 1;
        }
        changed
    }

    fn register_global_host(&mut self, cx: &mut Cx) {
        let global = cx.global::<SonnerGlobal>();
        let mut state = global.state.borrow_mut();
        if state.host_uid.is_none() || state.host_uid == Some(self.widget_uid()) {
            state.host_uid = Some(self.widget_uid());
            state.host_overlay = Some(self.overlay.clone());
        }
    }

    fn is_global_host(&self, cx: &mut Cx) -> bool {
        let global = cx.global::<SonnerGlobal>().clone();
        let state = global.state.borrow();
        state.host_uid == Some(self.widget_uid())
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
        slot.widget(cx, ids!(info_icon)).set_visible(cx, false);
        slot.widget(cx, ids!(success_icon)).set_visible(cx, false);
        slot.widget(cx, ids!(warning_icon)).set_visible(cx, false);
        slot.widget(cx, ids!(error_icon)).set_visible(cx, false);
        slot.widget(cx, ids!(close_btn)).set_visible(cx, false);

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
        slot.widget(cx, ids!(close_btn))
            .set_visible(cx, item.show_close);
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
        let (host_overlay, visible_toasts) = {
            let state = global.state.borrow();
            (
                state.host_overlay.clone(),
                Self::visible_toasts_snapshot(&state),
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

            for (index, toast) in visible_toasts.iter().enumerate().take(MAX_VISIBLE_TOASTS) {
                Self::sync_overlay_slot(cx, &overlay, index, toast.clone());
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
                live_id!(content_row),
                live_id!(close_btn),
            ],
            1 => &[
                live_id!(content),
                live_id!(toast_1),
                live_id!(content_row),
                live_id!(close_btn),
            ],
            2 => &[
                live_id!(content),
                live_id!(toast_2),
                live_id!(content_row),
                live_id!(close_btn),
            ],
            _ => &[
                live_id!(content),
                live_id!(toast_3),
                live_id!(content_row),
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
        if !self.is_global_host(cx) {
            return;
        }

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
        let (was_empty, needs_schedule) = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();

            let was_empty = state.toasts.is_empty();

            if state.toasts.len() >= MAX_VISIBLE_TOASTS {
                state.toasts.pop_front();
            }

            let timeout_sec = item.duration.unwrap_or(DEFAULT_TIMEOUT_SEC).max(0.0);
            state.toasts.push_back(SonnerToastEntry {
                item,
                expires_at: None, // 交由 NextFrame 处理初始化
                total_duration: timeout_sec,
            });
            (was_empty, true)
        };

        self.open = true;
        Self::sync_global_host_overlay(cx);
        if was_empty {
            self.emit_open_state(cx, true);
        }
        if needs_schedule {
            cx.new_next_frame();
        }
    }

    pub fn clear_global_toasts(&mut self, cx: &mut Cx, emit_action: bool) {
        let was_open = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();
            let was_open = !state.toasts.is_empty();
            state.toasts.clear();
            state.needs_next_frame = false;
            was_open
        };
        self.open = false;
        Self::sync_global_host_overlay(cx);
        if emit_action && was_open {
            self.emit_open_state(cx, false);
        }
    }

    fn remove_visible_toast(&mut self, cx: &mut Cx, visible_index: usize) {
        let (was_open, is_open_now, needs_schedule) = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();
            if visible_index >= state.toasts.len() {
                return;
            }

            let was_open = !state.toasts.is_empty();
            let queue_index = state.toasts.len() - 1 - visible_index;
            state.toasts.remove(queue_index);
            let is_open_now = !state.toasts.is_empty();
            let needs_schedule = is_open_now;
            (was_open, is_open_now, needs_schedule)
        };
        self.open = is_open_now;
        Self::sync_global_host_overlay(cx);
        if was_open && !is_open_now {
            self.emit_open_state(cx, false);
        }
        if needs_schedule {
            cx.new_next_frame();
        }
    }
}

impl Widget for ShadSonner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.register_global_host(cx);

        if let Event::NextFrame(ne) = event {
            if !self.is_global_host(cx) {
                return;
            }
            let now = ne.time;
            let mut needs_redraw = false;
            let progresses: Vec<f64> = {
                let global = cx.global::<SonnerGlobal>().clone();
                let mut state = global.state.borrow_mut();

                // 初始化新 enqueue 的 toast 的过期时间
                for entry in state.toasts.iter_mut() {
                    if entry.expires_at.is_none() {
                        entry.expires_at = Some(now + entry.total_duration);
                    }
                }

                state
                    .toasts
                    .iter()
                    .rev()
                    .take(MAX_VISIBLE_TOASTS)
                    .map(|entry| {
                        if entry.total_duration <= 0.0 {
                            return 0.0;
                        }
                        let exp = entry.expires_at.unwrap();
                        let remaining = if exp > now { exp - now } else { 0.0 };
                        (remaining / entry.total_duration).clamp(0.0, 1.0)
                    })
                    .collect()
            };
            for (i, &prog) in progresses.iter().enumerate() {
                let slot = self.overlay.widget(cx, Self::toast_slot_path(i));
                if !slot.is_empty() {
                    let progress_bar = slot.my_progress_bar(cx, ids!(progress_bar));
                    progress_bar.set_progress(cx, prog * 100.0);
                    needs_redraw = true;
                }
            }
            if needs_redraw {
                self.overlay.redraw(cx); // 触发重绘以更新进度条
            }
            let (changed, still_has_toasts, needs_schedule) = {
                let global = cx.global::<SonnerGlobal>().clone();
                let mut state = global.state.borrow_mut();
                let changed = Self::prune_expired_toasts(&mut state, now);
                let still_has_toasts = !state.toasts.is_empty();
                let needs_schedule = still_has_toasts;
                state.needs_next_frame = needs_schedule;
                (changed, still_has_toasts, needs_schedule)
            };
            if changed || still_has_toasts != self.open {
                self.open = still_has_toasts;
                Self::sync_global_host_overlay(cx);
                if !still_has_toasts {
                    self.emit_open_state(cx, false);
                }
            }
            if needs_schedule {
                cx.new_next_frame();
            }
            return;
        }

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
