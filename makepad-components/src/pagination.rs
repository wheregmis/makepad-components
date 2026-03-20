use crate::internal::script_args::number_arg;
use crate::models::pagination::{
    clamped_current_page, clamped_max_visible_pages, compute_window, normalized_page_count,
    PaginationWindow,
};
use makepad_widgets::makepad_script::NoTrap;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::fmt::Write;

const MAX_PAGE_BUTTONS: usize = 7;

#[derive(Clone, Copy, Debug)]
struct PaginationThemeStyle {
    active_bg: Vec4f,
    active_bg_hover: Vec4f,
    active_bg_down: Vec4f,
    active_text: Vec4f,
    inactive_bg_hover: Vec4f,
    inactive_bg_down: Vec4f,
    inactive_text: Vec4f,
    border: Vec4f,
    border_hover: Vec4f,
    border_down: Vec4f,
    radius: f64,
}

impl Default for PaginationThemeStyle {
    fn default() -> Self {
        Self {
            active_bg: Vec4f::from_u32(0x18181bff),
            active_bg_hover: Vec4f::from_u32(0x27272aff),
            active_bg_down: Vec4f::from_u32(0x3f3f46ff),
            active_text: Vec4f::from_u32(0xfafafaff),
            inactive_bg_hover: Vec4f::from_u32(0x27272aff),
            inactive_bg_down: Vec4f::from_u32(0x3f3f46ff),
            inactive_text: Vec4f::from_u32(0xfafafaff),
            border: Vec4f::from_u32(0x3f3f46ff),
            border_hover: Vec4f::from_u32(0x52525bff),
            border_down: Vec4f::from_u32(0x71717aff),
            radius: 6.0,
        }
    }
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadPaginationNavButton = mod.widgets.ShadButtonOutline{
        height: 36
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
    }

    mod.widgets.ShadPaginationPageButton = mod.widgets.ShadButtonGhost{
        width: 36
        height: 36
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
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
    #[rust]
    theme_style: PaginationThemeStyle,

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
        self.theme_style = Self::resolve_theme_style(vm);
        self.view_synced = false;
        vm.with_cx_mut(|cx| {
            self.normalize_state();
            self.sync_view(cx);
        });
    }
}

impl ShadPagination {
    fn sync_page_button_text(
        &mut self,
        cx: &mut Cx,
        button: &mut ButtonRef,
        index: usize,
        page: usize,
    ) {
        let text = &mut self.slot_text_cache[index];
        text.clear();
        let _ = write!(text, "{page}");
        button.set_text(cx, text);
    }

    fn resolve_theme_style(vm: &mut ScriptVm) -> PaginationThemeStyle {
        fn theme_value(vm: &mut ScriptVm, key: LiveId) -> ScriptValue {
            let mod_obj = vm.module(id!(mod));
            let widgets = vm.bx.heap.value(mod_obj, id!(widgets).into(), NoTrap);
            let Some(widgets_obj) = widgets.as_object() else {
                return NIL;
            };
            let theme = vm
                .bx
                .heap
                .value(widgets_obj, id!(shad_theme).into(), NoTrap);
            let Some(theme_obj) = theme.as_object() else {
                return NIL;
            };
            vm.bx.heap.value(theme_obj, key.into(), NoTrap)
        }

        fn theme_color(
            vm: &mut ScriptVm,
            primary: LiveId,
            secondary: LiveId,
            fallback: u32,
        ) -> Vec4f {
            theme_value(vm, primary)
                .as_color()
                .map(Vec4f::from_u32)
                .or_else(|| theme_value(vm, secondary).as_color().map(Vec4f::from_u32))
                .unwrap_or_else(|| Vec4f::from_u32(fallback))
        }

        let defaults = PaginationThemeStyle::default();
        PaginationThemeStyle {
            active_bg: theme_color(
                vm,
                id!(color_pagination_active),
                id!(color_secondary),
                defaults.active_bg.to_u32(),
            ),
            active_bg_hover: theme_color(
                vm,
                id!(color_pagination_active_hover),
                id!(color_secondary_hover),
                defaults.active_bg_hover.to_u32(),
            ),
            active_bg_down: theme_color(
                vm,
                id!(color_pagination_active_down),
                id!(color_secondary_down),
                defaults.active_bg_down.to_u32(),
            ),
            active_text: theme_color(
                vm,
                id!(color_pagination_active_foreground),
                id!(color_secondary_foreground),
                defaults.active_text.to_u32(),
            ),
            inactive_bg_hover: theme_color(
                vm,
                id!(color_pagination_inactive_hover),
                id!(color_ghost_hover),
                defaults.inactive_bg_hover.to_u32(),
            ),
            inactive_bg_down: theme_color(
                vm,
                id!(color_pagination_inactive_down),
                id!(color_ghost_down),
                defaults.inactive_bg_down.to_u32(),
            ),
            inactive_text: theme_color(
                vm,
                id!(color_pagination_inactive_foreground),
                id!(color_primary),
                defaults.inactive_text.to_u32(),
            ),
            border: theme_color(
                vm,
                id!(color_pagination_border),
                id!(color_outline_border),
                defaults.border.to_u32(),
            ),
            border_hover: theme_color(
                vm,
                id!(color_pagination_border_hover),
                id!(color_outline_border_hover),
                defaults.border_hover.to_u32(),
            ),
            border_down: theme_color(
                vm,
                id!(color_pagination_border_down),
                id!(color_outline_border_down),
                defaults.border_down.to_u32(),
            ),
            radius: theme_value(vm, id!(radius))
                .as_number()
                .unwrap_or(defaults.radius),
        }
    }

    fn normalize_state(&mut self) {
        self.current_page =
            clamped_current_page(self.current_page, normalized_page_count(self.page_count)) as u32;
    }

    fn page_button_ref(&self, cx: &Cx, index: usize) -> ButtonRef {
        match index {
            0 => self.view.button(cx, ids!(page_0)),
            1 => self.view.button(cx, ids!(page_1)),
            2 => self.view.button(cx, ids!(page_2)),
            3 => self.view.button(cx, ids!(page_3)),
            4 => self.view.button(cx, ids!(page_4)),
            5 => self.view.button(cx, ids!(page_5)),
            _ => self.view.button(cx, ids!(page_6)),
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

    fn apply_page_button_style(&self, cx: &mut Cx, button: &mut ButtonRef, is_active: bool) {
        let style = self.theme_style;
        if is_active {
            script_apply_eval!(cx, button, {
                draw_bg +: {
                    color: #(style.active_bg)
                    color_hover: #(style.active_bg_hover)
                    color_down: #(style.active_bg_down)
                    color_focus: #(style.active_bg_hover)
                    border_size: 1.0
                    border_radius: #(style.radius)
                    border_color: #(style.border)
                    border_color_hover: #(style.border_hover)
                    border_color_down: #(style.border_down)
                    border_color_focus: #(style.border_hover)
                }
                draw_text +: {
                    color: #(style.active_text)
                    color_hover: #(style.active_text)
                    color_down: #(style.active_text)
                    color_focus: #(style.active_text)
                }
            });
        } else {
            script_apply_eval!(cx, button, {
                draw_bg +: {
                    color: #0000
                    color_hover: #(style.inactive_bg_hover)
                    color_down: #(style.inactive_bg_down)
                    color_focus: #(style.inactive_bg_hover)
                    border_size: 0.0
                    border_radius: #(style.radius)
                    border_color: #0000
                    border_color_hover: #0000
                    border_color_down: #0000
                    border_color_focus: #0000
                }
                draw_text +: {
                    color: #(style.inactive_text)
                    color_hover: #(style.inactive_text)
                    color_down: #(style.inactive_text)
                    color_focus: #(style.inactive_text)
                }
            });
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
            let mut button = self.page_button_ref(cx, index);
            let page = window.pages.get(index).copied().unwrap_or(0);
            let is_visible = page > 0;

            if force || self.slot_visible[index] != is_visible {
                button.set_visible(cx, is_visible);
                self.slot_visible[index] = is_visible;
            }

            if is_visible {
                if force || self.slot_pages[index] != page {
                    self.sync_page_button_text(cx, &mut button, index, page);
                }
                self.slot_pages[index] = page;
                let is_active = page == current_page;
                if force || self.slot_active[index] != is_active {
                    self.apply_page_button_style(cx, &mut button, is_active);
                    self.slot_active[index] = is_active;
                }
            } else {
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
                if self.page_button_ref(cx, index).clicked(actions) {
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
