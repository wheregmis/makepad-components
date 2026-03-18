use crate::internal::actions::{emit_widget_action, first_widget_action};
use crate::internal::overlay::button_clicked;
use crate::internal::script_args::bool_arg;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadToastTitle = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 12
    }

    mod.widgets.ShadToastDescription = mod.widgets.ShadAlertDescription{
        width: Fit
        height: Fit
    }

    let ToastSlotPanel = RoundedView{
        visible: false
        width: 260
        height: Fit
        padding: Inset{left: 14, right: 8, top: 10, bottom: 10}
        flow: Down
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

            check_icon := mod.widgets.IconCheck{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon.color: (shad_theme.color_primary)
            }

            title_label := mod.widgets.ShadToastTitle{
                width: Fill
                text: "Event created"
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

        description_label := mod.widgets.ShadToastDescription{
            text: ""
            visible: false
        }
    }

    mod.widgets.ShadToast = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
        flow: Down
        spacing: 4.0

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadSonnerBase = #(ShadSonner::register_widget(vm))

    mod.widgets.ShadSonner = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false
        toast_kind: "basic"

        overlay: PopupNotification{
            content +: {
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                margin: Inset{top: 16, right: 16}

                toast_0 := ToastSlotPanel{}
                toast_1 := ToastSlotPanel{}
                toast_2 := ToastSlotPanel{}
                toast_3 := ToastSlotPanel{}
            }
        }
    }

    mod.widgets.ShadSonnerWithDescription = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false
        toast_kind: "description"

        overlay: PopupNotification{
            content +: {
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                margin: Inset{top: 16, right: 16}

                toast_0 := ToastSlotPanel{}
                toast_1 := ToastSlotPanel{}
                toast_2 := ToastSlotPanel{}
                toast_3 := ToastSlotPanel{}
            }
        }
    }

    // Toast with a leading check icon and a close (X) button.
    // The close button dismisses the toast when clicked.
    mod.widgets.ShadSonnerWithClose = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false
        toast_kind: "close"

        overlay: PopupNotification{
            content +: {
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                margin: Inset{top: 16, right: 16}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SonnerToastKind {
    Basic,
    Description,
    Close,
}

#[derive(Default)]
struct SonnerGlobalState {
    host_uid: Option<WidgetUid>,
    host_overlay: Option<WidgetRef>,
    toasts: VecDeque<SonnerToastKind>,
    rendered_toasts: [Option<SonnerToastKind>; MAX_VISIBLE_TOASTS],
    rendered_open: Option<bool>,
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
    #[live]
    toast_kind: ArcStringMut,
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

const MAX_VISIBLE_TOASTS: usize = 4;

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
            match self.last_applied_open {
                None => {
                    if applied_open && !self.global_is_open(cx) {
                        self.push_global_toast(cx, self.default_toast_kind(), false);
                    }
                }
                Some(previous) if previous != applied_open => {
                    if applied_open {
                        if !self.global_is_open(cx) {
                            self.push_global_toast(cx, self.default_toast_kind(), true);
                        }
                    } else {
                        self.clear_global_toasts(cx, true);
                    }
                }
                _ => {}
            }
            self.last_applied_open = Some(applied_open);
            self.sync_toast_visibility(cx);
        });
    }
}

impl ShadSonner {
    fn visible_toasts_snapshot(
        state: &SonnerGlobalState,
    ) -> [Option<SonnerToastKind>; MAX_VISIBLE_TOASTS] {
        let mut visible = [None; MAX_VISIBLE_TOASTS];
        for (index, kind) in state
            .toasts
            .iter()
            .rev()
            .take(MAX_VISIBLE_TOASTS)
            .copied()
            .enumerate()
        {
            visible[index] = Some(kind);
        }
        visible
    }

    fn default_toast_kind(&self) -> SonnerToastKind {
        match self.toast_kind.as_ref() {
            "description" => SonnerToastKind::Description,
            "close" => SonnerToastKind::Close,
            _ => SonnerToastKind::Basic,
        }
    }

    fn register_global_host(&mut self, cx: &mut Cx) {
        let global = cx.global::<SonnerGlobal>().clone();
        let mut state = global.state.borrow_mut();
        if state.host_uid.is_none() || state.host_uid == Some(self.widget_uid()) {
            if state.host_uid != Some(self.widget_uid()) || state.host_overlay.is_none() {
                state.rendered_toasts = [None; MAX_VISIBLE_TOASTS];
                state.rendered_open = None;
            }
            state.host_uid = Some(self.widget_uid());
            state.host_overlay = Some(self.overlay.clone());
        }
    }

    fn is_global_host(&self, cx: &mut Cx) -> bool {
        let global = cx.global::<SonnerGlobal>().clone();
        let is_host = global.state.borrow().host_uid == Some(self.widget_uid());
        is_host
    }

    fn global_is_open(&self, cx: &mut Cx) -> bool {
        let global = cx.global::<SonnerGlobal>().clone();
        let is_open = !global.state.borrow().toasts.is_empty();
        is_open
    }

    fn visible_toasts(&self, cx: &mut Cx) -> [Option<SonnerToastKind>; MAX_VISIBLE_TOASTS] {
        let global = cx.global::<SonnerGlobal>().clone();
        let state = global.state.borrow();
        Self::visible_toasts_snapshot(&state)
    }

    fn sync_overlay_slot(
        cx: &mut Cx,
        overlay: &WidgetRef,
        index: usize,
        kind: Option<SonnerToastKind>,
    ) -> bool {
        let slot = overlay.widget(cx, Self::toast_slot_path(index));
        if slot.is_empty() {
            return false;
        }

        let Some(kind) = kind else {
            slot.set_visible(cx, false);
            return true;
        };

        slot.set_visible(cx, true);
        slot.label(cx, ids!(title_label))
            .set_text(cx, Self::title_for_kind(kind));
        slot.label(cx, ids!(description_label))
            .set_text(cx, Self::description_for_kind(kind));
        slot.widget(cx, ids!(description_label))
            .set_visible(cx, Self::kind_shows_description(kind));
        slot.widget(cx, ids!(check_icon))
            .set_visible(cx, Self::kind_shows_check(kind));
        slot.widget(cx, ids!(close_btn))
            .set_visible(cx, Self::kind_shows_close(kind));
        true
    }

    fn sync_global_host_overlay(cx: &mut Cx) {
        let global = cx.global::<SonnerGlobal>().clone();
        let (host_overlay, visible_toasts, rendered_toasts, rendered_open) = {
            let state = global.state.borrow();
            (
                state.host_overlay.clone(),
                Self::visible_toasts_snapshot(&state),
                state.rendered_toasts,
                state.rendered_open,
            )
        };

        if let Some(overlay) = host_overlay {
            let next_open = visible_toasts[0].is_some();
            let mut changed = false;
            if let Some(mut popup) = overlay.borrow_mut::<PopupNotification>() {
                if rendered_open != Some(next_open) {
                    changed = true;
                }
                if !next_open {
                    popup.close(cx);
                } else {
                    popup.open(cx);
                }
            }

            for index in 0..MAX_VISIBLE_TOASTS {
                if rendered_toasts[index] != visible_toasts[index] {
                    changed |= Self::sync_overlay_slot(cx, &overlay, index, visible_toasts[index]);
                }
            }

            if changed {
                let mut state = global.state.borrow_mut();
                state.rendered_toasts = visible_toasts;
                state.rendered_open = Some(next_open);
                overlay.redraw(cx);
            }
        }
    }

    fn title_for_kind(kind: SonnerToastKind) -> &'static str {
        match kind {
            SonnerToastKind::Description => "Toast with description",
            _ => "Event created",
        }
    }

    fn description_for_kind(kind: SonnerToastKind) -> &'static str {
        match kind {
            SonnerToastKind::Description => "Your changes have been saved.",
            _ => "",
        }
    }

    fn kind_shows_description(kind: SonnerToastKind) -> bool {
        matches!(kind, SonnerToastKind::Description)
    }

    fn kind_shows_check(kind: SonnerToastKind) -> bool {
        matches!(kind, SonnerToastKind::Close)
    }

    fn kind_shows_close(kind: SonnerToastKind) -> bool {
        matches!(kind, SonnerToastKind::Close)
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
        if !is_host {
            self.is_synced_open = open;
            return false;
        }
        if self.is_synced_open == open {
            return true;
        }

        if let Some(mut popup) = self.overlay.borrow_mut::<PopupNotification>() {
            if open {
                popup.open(cx);
            } else {
                popup.close(cx);
            }
        }

        self.is_synced_open = open;
        true
    }

    fn sync_toast_visibility(&mut self, cx: &mut Cx) {
        if !self.is_global_host(cx) {
            return;
        }

        let visible_toasts = self.visible_toasts(cx);
        for index in 0..MAX_VISIBLE_TOASTS {
            Self::sync_overlay_slot(cx, &self.overlay, index, visible_toasts[index]);
        }
        self.sync_overlay_open_state(cx);
    }

    fn emit_open_state(&self, cx: &mut Cx, open: bool) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadSonnerAction::OpenChanged(open),
        );
    }

    fn push_global_toast(&mut self, cx: &mut Cx, kind: SonnerToastKind, emit_action: bool) {
        let (was_open, is_open) = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();
            let was_open = !state.toasts.is_empty();
            if state.toasts.len() == MAX_VISIBLE_TOASTS {
                state.toasts.pop_front();
            }
            state.toasts.push_back(kind);
            (was_open, !state.toasts.is_empty())
        };

        self.open = is_open;
        Self::sync_global_host_overlay(cx);
        if emit_action && was_open != is_open {
            self.emit_open_state(cx, is_open);
        }
    }

    fn clear_global_toasts(&mut self, cx: &mut Cx, emit_action: bool) {
        let (was_open, is_open) = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();
            let was_open = !state.toasts.is_empty();
            state.toasts.clear();
            (was_open, false)
        };

        self.open = is_open;
        Self::sync_global_host_overlay(cx);
        if emit_action && was_open != is_open {
            self.emit_open_state(cx, is_open);
        }
    }

    fn remove_visible_toast(&mut self, cx: &mut Cx, visible_index: usize) {
        let removed = {
            let global = cx.global::<SonnerGlobal>().clone();
            let mut state = global.state.borrow_mut();
            if visible_index >= state.toasts.len() {
                return;
            }
            let queue_index = state.toasts.len() - 1 - visible_index;
            let had_toasts = !state.toasts.is_empty();
            state.toasts.remove(queue_index);
            (had_toasts, !state.toasts.is_empty())
        };

        self.open = removed.1;
        Self::sync_global_host_overlay(cx);
        if removed.0 != removed.1 {
            self.emit_open_state(cx, removed.1);
        }
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if open {
            if !self.global_is_open(cx) {
                self.push_global_toast(cx, self.default_toast_kind(), true);
            }
        } else {
            self.clear_global_toasts(cx, true);
        }
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.push_global_toast(cx, self.default_toast_kind(), true);
    }

    pub fn open_basic(&mut self, cx: &mut Cx) {
        self.push_global_toast(cx, SonnerToastKind::Basic, true);
    }

    pub fn open_description(&mut self, cx: &mut Cx) {
        self.push_global_toast(cx, SonnerToastKind::Description, true);
    }

    pub fn open_close(&mut self, cx: &mut Cx) {
        self.push_global_toast(cx, SonnerToastKind::Close, true);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.set_open(cx, false);
    }

    pub fn is_open(&self, cx: &mut Cx) -> bool {
        self.global_is_open(cx)
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(ShadSonnerAction::OpenChanged(open)) =
            first_widget_action::<ShadSonnerAction>(actions, self.widget_uid())
        {
            return Some(open);
        }
        None
    }
}

impl Widget for ShadSonner {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_open) {
            if let Some(open) = bool_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_open(cx, open));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(is_open) {
            let is_open = vm.with_cx_mut(|cx| self.is_open(cx));
            return ScriptAsyncResult::Return(ScriptValue::from_bool(is_open));
        }
        if method == live_id!(open_basic) {
            vm.with_cx_mut(|cx| self.open_basic(cx));
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(open_description) {
            vm.with_cx_mut(|cx| self.open_description(cx));
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(open_close) {
            vm.with_cx_mut(|cx| self.open_close(cx));
            return ScriptAsyncResult::Return(NIL);
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
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
        let is_host = self.sync_overlay_open_state(cx);
        if !is_host || !self.open {
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

impl ShadSonnerRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }

    pub fn open_basic(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_basic(cx);
        }
    }

    pub fn open_description(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_description(cx);
        }
    }

    pub fn open_close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_close(cx);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.open)
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }
}
