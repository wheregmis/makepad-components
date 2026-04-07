use crate::internal::script_args::number_arg;
use crate::models::pagination::{
    clamped_current_page, clamped_max_visible_pages, compute_window, normalized_page_count,
    PaginationWindow,
};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::fmt::Write;

const MAX_PAGE_BUTTONS: usize = 7;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadPaginationNavButton = mod.widgets.ShadButtonOutline{
        height: 36
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
    }

    mod.widgets.ShadPaginationPageButtonActiveInner = mod.widgets.ShadButtonGhost{
        width: Fill
        height: Fill
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}

        draw_bg +: {
            color: (shad_theme.color_secondary)
            color_hover: (shad_theme.color_secondary_hover)
            color_down: (shad_theme.color_secondary_down)
            color_focus: (shad_theme.color_secondary_hover)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_border)
            border_color_hover: (shad_theme.color_border_hover)
            border_color_down: (shad_theme.color_border_down)
            border_color_focus: (shad_theme.color_border_hover)
        }

        draw_text +: {
            color: (shad_theme.color_secondary_foreground)
            color_hover: (shad_theme.color_secondary_foreground)
            color_down: (shad_theme.color_secondary_foreground)
            color_focus: (shad_theme.color_secondary_foreground)
        }
    }

    mod.widgets.ShadPaginationPageButtonInactiveInner = mod.widgets.ShadButtonGhost{
        width: Fill
        height: Fill
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }

    mod.widgets.ShadPaginationPageButtonBase = #(ShadPaginationPageButton::register_widget(vm))

    mod.widgets.ShadPaginationPageButton = set_type_default() do mod.widgets.ShadPaginationPageButtonBase{
        width: 36
        height: 36
        active: false
        flow: Overlay

        inactive_btn := mod.widgets.ShadPaginationPageButtonInactiveInner{text: "1"}
        active_btn := mod.widgets.ShadPaginationPageButtonActiveInner{text: "1" visible: false}
    }

    mod.widgets.ShadPaginationEllipsis = Label{
        width: Fit
        height: Fit
        margin: Inset{left: 2, right: 2, top: 0, bottom: 0}
        text: "..."
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadPaginationBase = #(ShadPagination::register_widget(vm))

    mod.widgets.ShadPagination = set_type_default() do mod.widgets.ShadPaginationBase{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 6.0

        current_page: 1
        page_count: 10
        max_visible_pages: 7

        prev_btn := mod.widgets.ShadPaginationNavButton{text: "Previous"}
        page_0 := mod.widgets.ShadPaginationPageButton{text: "1"}
        ellipsis_left := mod.widgets.ShadPaginationEllipsis{}
        page_1 := mod.widgets.ShadPaginationPageButton{text: "2"}
        page_2 := mod.widgets.ShadPaginationPageButton{text: "3"}
        page_3 := mod.widgets.ShadPaginationPageButton{text: "4"}
        page_4 := mod.widgets.ShadPaginationPageButton{text: "5"}
        page_5 := mod.widgets.ShadPaginationPageButton{text: "6"}
        ellipsis_right := mod.widgets.ShadPaginationEllipsis{}
        page_6 := mod.widgets.ShadPaginationPageButton{text: "7"}
        next_btn := mod.widgets.ShadPaginationNavButton{text: "Next"}
    }
}

#[derive(Script, Widget)]
pub struct ShadPaginationPageButton {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    text: ArcStringMut,
    #[live(false)]
    active: bool,
    #[rust]
    synced_active: bool,
    #[rust]
    synced_text: String,
}

impl ScriptHook for ShadPaginationPageButton {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.sync_label(cx);
            self.sync_active(cx);
        });
    }
}

impl ShadPaginationPageButton {
    fn reset_inner_button_states(&self, cx: &mut Cx) {
        for path in [ids!(active_btn), ids!(inactive_btn)] {
            let button_ref = self.view.button(cx, path);
            let borrowed = button_ref.borrow_mut();
            if let Some(mut button) = borrowed {
                button.animator_cut(cx, ids!(hover.off));
                button.animator_cut(cx, ids!(focus.off));
            }
        }
    }

    fn sync_label(&mut self, cx: &mut Cx) {
        if self.synced_text == self.text.as_ref() {
            return;
        }
        self.view
            .button(cx, ids!(active_btn))
            .set_text(cx, self.text.as_ref());
        self.view
            .button(cx, ids!(inactive_btn))
            .set_text(cx, self.text.as_ref());
        self.synced_text.clear();
        self.synced_text.push_str(self.text.as_ref());
        self.reset_inner_button_states(cx);
    }

    fn sync_active(&mut self, cx: &mut Cx) {
        if self.synced_active == self.active {
            return;
        }
        self.view
            .button(cx, ids!(active_btn))
            .set_visible(cx, self.active);
        self.view
            .button(cx, ids!(inactive_btn))
            .set_visible(cx, !self.active);
        self.synced_active = self.active;
    }

    pub fn set_active(&mut self, cx: &mut Cx, active: bool) {
        if self.active == active {
            return;
        }
        self.active = active;
        self.sync_active(cx);
        if !active {
            self.reset_inner_button_states(cx);
        }
    }

    pub fn set_label(&mut self, cx: &mut Cx, text: &str) {
        if self.text.as_ref() == text {
            return;
        }
        self.text.as_mut_empty().push_str(text);
        self.sync_label(cx);
    }

    pub fn clicked(&self, cx: &Cx, actions: &Actions) -> bool {
        self.view.button(cx, ids!(active_btn)).clicked(actions)
            || self.view.button(cx, ids!(inactive_btn)).clicked(actions)
    }
}

impl Widget for ShadPaginationPageButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadPaginationAction {
    Changed(usize),
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadPagination {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    current_page: u32,
    #[live]
    page_count: u32,
    #[live]
    max_visible_pages: u32,

    #[rust]
    slot_pages: [usize; MAX_PAGE_BUTTONS],
    #[rust]
    slot_visible: [bool; MAX_PAGE_BUTTONS],
    #[rust]
    slot_active: [bool; MAX_PAGE_BUTTONS],
    #[rust]
    prev_disabled: bool,
    #[rust]
    next_disabled: bool,
    #[rust]
    show_left_ellipsis: bool,
    #[rust]
    show_right_ellipsis: bool,
    #[rust]
    slot_text_cache: [String; MAX_PAGE_BUTTONS],
    #[rust]
    view_synced: bool,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadPagination {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        self.view_synced = false;
        vm.with_cx_mut(|cx| {
            self.normalize_state();
            self.sync_view(cx);
        });
    }
}

impl ShadPagination {
    fn normalize_state(&mut self) {
        self.current_page =
            clamped_current_page(self.current_page, normalized_page_count(self.page_count)) as u32;
    }

    fn page_button_ref(&self, cx: &Cx, index: usize) -> WidgetRef {
        match index {
            0 => self.view.widget(cx, ids!(page_0)),
            1 => self.view.widget(cx, ids!(page_1)),
            2 => self.view.widget(cx, ids!(page_2)),
            3 => self.view.widget(cx, ids!(page_3)),
            4 => self.view.widget(cx, ids!(page_4)),
            5 => self.view.widget(cx, ids!(page_5)),
            _ => self.view.widget(cx, ids!(page_6)),
        }
    }

    fn ellipsis_ref(&self, cx: &Cx, left: bool) -> WidgetRef {
        if left {
            self.view.widget(cx, ids!(ellipsis_left))
        } else {
            self.view.widget(cx, ids!(ellipsis_right))
        }
    }

    fn compute_pages(&self) -> PaginationWindow {
        compute_window(
            self.page(),
            normalized_page_count(self.page_count),
            clamped_max_visible_pages(self.max_visible_pages, self.page_count, MAX_PAGE_BUTTONS),
        )
    }

    fn sync_page_button(
        &mut self,
        cx: &mut Cx,
        button_ref: &WidgetRef,
        index: usize,
        page: usize,
        is_active: bool,
    ) {
        if let Some(mut button) = button_ref.borrow_mut::<ShadPaginationPageButton>() {
            if self.slot_pages[index] != page {
                let text = &mut self.slot_text_cache[index];
                text.clear();
                let _ = write!(text, "{page}");
                button.set_label(cx, text);
            }
            if self.slot_active[index] != is_active {
                button.set_active(cx, is_active);
            }
        }
    }

    fn sync_view(&mut self, cx: &mut Cx) {
        let force = !self.view_synced;
        let current_page = self.page();
        let page_count = normalized_page_count(self.page_count);
        let window = self.compute_pages();

        let prev_disabled = current_page <= 1;
        if force || self.prev_disabled != prev_disabled {
            self.view
                .button(cx, ids!(prev_btn))
                .set_disabled(cx, prev_disabled);
            self.prev_disabled = prev_disabled;
        }
        let next_disabled = current_page >= page_count;
        if force || self.next_disabled != next_disabled {
            self.view
                .button(cx, ids!(next_btn))
                .set_disabled(cx, next_disabled);
            self.next_disabled = next_disabled;
        }

        if force || self.show_left_ellipsis != window.show_left_ellipsis {
            self.ellipsis_ref(cx, true)
                .set_visible(cx, window.show_left_ellipsis);
            self.show_left_ellipsis = window.show_left_ellipsis;
        }
        if force || self.show_right_ellipsis != window.show_right_ellipsis {
            self.ellipsis_ref(cx, false)
                .set_visible(cx, window.show_right_ellipsis);
            self.show_right_ellipsis = window.show_right_ellipsis;
        }

        for index in 0..MAX_PAGE_BUTTONS {
            let button_ref = self.page_button_ref(cx, index);
            let page = window.pages.get(index).copied().unwrap_or(0);
            let is_visible = page > 0;

            if force || self.slot_visible[index] != is_visible {
                button_ref.set_visible(cx, is_visible);
                self.slot_visible[index] = is_visible;
            }

            if is_visible {
                let is_active = page == current_page;
                if force || self.slot_pages[index] != page || self.slot_active[index] != is_active {
                    self.sync_page_button(cx, &button_ref, index, page, is_active);
                }
                self.slot_pages[index] = page;
                self.slot_active[index] = is_active;
            } else {
                if let Some(mut button) = button_ref.borrow_mut::<ShadPaginationPageButton>() {
                    button.set_active(cx, false);
                }
                self.slot_pages[index] = 0;
                self.slot_active[index] = false;
            }
        }
        self.view_synced = true;
    }

    fn set_page_internal(&mut self, cx: &mut Cx, page: usize, emit_action: bool) {
        let clamped = page.clamp(1, normalized_page_count(self.page_count));
        if self.current_page as usize == clamped {
            return;
        }

        self.current_page = clamped as u32;
        self.sync_view(cx);

        if emit_action {
            cx.widget_action_with_data(
                &self.action_data,
                self.widget_uid(),
                ShadPaginationAction::Changed(clamped),
            );
        }
    }

    pub fn set_page(&mut self, cx: &mut Cx, page: usize) {
        self.set_page_internal(cx, page, true);
    }

    pub fn set_page_count(&mut self, cx: &mut Cx, page_count: usize) {
        let prev_page_count = self.page_count;
        let prev_page = self.current_page;
        self.page_count = page_count.max(1) as u32;
        self.normalize_state();
        if self.page_count == prev_page_count && self.current_page == prev_page {
            return;
        }
        self.sync_view(cx);
    }

    pub fn next(&mut self, cx: &mut Cx) {
        let next_page = self.page().saturating_add(1);
        self.set_page_internal(cx, next_page, true);
    }

    pub fn prev(&mut self, cx: &mut Cx) {
        let prev_page = self.page().saturating_sub(1).max(1);
        self.set_page_internal(cx, prev_page, true);
    }

    pub fn page(&self) -> usize {
        self.current_page.max(1) as usize
    }

    pub fn page_count(&self) -> usize {
        normalized_page_count(self.page_count)
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        for action in actions.filter_widget_actions_cast::<ShadPaginationAction>(self.widget_uid())
        {
            if let ShadPaginationAction::Changed(page) = action {
                return Some(page);
            }
        }
        None
    }
}

impl Widget for ShadPagination {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_page) {
            if let Some(page) = number_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_page(cx, page as usize));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(page) {
            return ScriptAsyncResult::Return(ScriptValue::from_u32(self.page() as u32));
        }
        if method == live_id!(set_page_count) {
            if let Some(page_count) = number_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_page_count(cx, page_count as usize));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(page_count) {
            return ScriptAsyncResult::Return(ScriptValue::from_u32(self.page_count() as u32));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self.view.button(cx, ids!(prev_btn)).clicked(actions) {
                self.prev(cx);
                return;
            }
            if self.view.button(cx, ids!(next_btn)).clicked(actions) {
                self.next(cx);
                return;
            }

            for index in 0..MAX_PAGE_BUTTONS {
                let button_ref = self.page_button_ref(cx, index);
                if button_ref
                    .borrow::<ShadPaginationPageButton>()
                    .is_some_and(|button| button.clicked(cx, actions))
                {
                    let page = self.slot_pages[index];
                    if page > 0 {
                        self.set_page(cx, page);
                    }
                    return;
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShadPaginationRef {
    pub fn set_page(&self, cx: &mut Cx, page: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_page(cx, page);
        }
    }

    pub fn set_page_count(&self, cx: &mut Cx, page_count: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_page_count(cx, page_count);
        }
    }

    pub fn next(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.next(cx);
        }
    }

    pub fn prev(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.prev(cx);
        }
    }

    pub fn page(&self) -> usize {
        self.borrow().map_or(1, |inner| inner.page())
    }

    pub fn page_count(&self) -> usize {
        self.borrow().map_or(1, |inner| inner.page_count())
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }
}
