use crate::internal::actions::first_widget_action;
use crate::internal::overlay::{
    draw_modal_overlay, modal_dismissed, set_modal_widget_open, sync_modal_open_state,
};
use crate::internal::script_args::bool_arg;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSheetTitle = mod.widgets.ShadAlertTitle{}
    mod.widgets.ShadSheetDescription = mod.widgets.ShadAlertDescription{}

    mod.widgets.ShadSheetBase = #(ShadSheet::register_widget(vm))

    mod.widgets.ShadSheet = set_type_default() do mod.widgets.ShadSheetBase{
        width: Fill
        height: Fit
        open: false
        side: "right"
        sheet_size: 360.0

        overlay: Modal{
            align: Align{x: 1.0, y: 0.0}
            bg_view +: {
                draw_bg.color: (shad_theme.color_overlay)
            }

            content +: {
                width: 360
                height: Fill
                flow: Down

                sheet_frame := RoundedView{
                    width: Fill
                    height: Fill
                    flow: Down

                    draw_bg +: {
                        color: (shad_theme.color_background)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    header := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 6.0
                        padding: Inset{left: 20, right: 20, top: 20, bottom: 12}

                        title := ShadAlertTitle{
                            text: "Edit profile"
                        }

                        description := ShadAlertDescription{
                            text: "Make changes to your profile here."
                        }
                    }

                    body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 16}
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
                        align: Align{x: 1.0, y: 0.5}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadSheetAction {
    OpenChanged(bool),
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadSheet {
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
    side: ArcStringMut,
    #[live]
    sheet_size: f64,

    #[rust]
    is_synced_open: bool,

    #[rust]
    last_side: String,
    #[rust]
    is_side_initialized: bool,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadSheet {
    fn sync_side_layout(&mut self, cx: &mut Cx) {
        let current_side = self.side.as_ref();

        // Optimization: only reapply script evaluation if the side has changed after initialization
        if self.is_side_initialized && current_side == self.last_side.as_str() {
            return;
        }

        // Optimization: avoid repeated allocation in layout sync loop by reusing string capacity
        // Previously: allocated a new String using `current_side.to_string()`
        // Now: reuse `self.last_side` buffer, avoiding a heap allocation
        self.last_side.clear();
        self.last_side.push_str(current_side);
        self.is_side_initialized = true;

        let (align, content_width, content_height) = match current_side {
            "left" => (
                Align { x: 0.0, y: 0.0 },
                Size::Fixed(self.sheet_size),
                Size::fill(),
            ),
            "top" => (
                Align { x: 0.0, y: 0.0 },
                Size::fill(),
                Size::Fixed(self.sheet_size),
            ),
            "bottom" => (
                Align { x: 0.0, y: 1.0 },
                Size::fill(),
                Size::Fixed(self.sheet_size),
            ),
            _ => (
                Align { x: 1.0, y: 0.0 },
                Size::Fixed(self.sheet_size),
                Size::fill(),
            ),
        };

        // Optimization: avoid cloning the `WidgetRef`
        // Previously: created a new cloned reference using `self.overlay.clone()`
        // Now: apply directly to `self.overlay`, eliminating clone overhead
        script_apply_eval!(cx, self.overlay, {
            align: #(align)
        });

        let mut content = self.overlay.widget(cx, ids!(content));
        script_apply_eval!(cx, content, {
            width: #(content_width)
            height: #(content_height)
        });

        let mut sheet_frame = self.overlay.widget(cx, ids!(content.sheet_frame));
        script_apply_eval!(cx, sheet_frame, {
            width: #(Size::fill())
            height: #(Size::fill())
        });
    }

    fn sync_open_state(&mut self, cx: &mut Cx) {
        self.sync_side_layout(cx);
        sync_modal_open_state(cx, &mut self.overlay, &mut self.is_synced_open, self.open);
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        self.sync_side_layout(cx);
        let uid = self.widget_uid();
        set_modal_widget_open(
            cx,
            &mut self.overlay,
            &mut self.open,
            &mut self.is_synced_open,
            &self.action_data,
            uid,
            open,
            ShadSheetAction::OpenChanged,
        );
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.set_open(cx, true);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.set_open(cx, false);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(ShadSheetAction::OpenChanged(open)) =
            first_widget_action::<ShadSheetAction>(actions, self.widget_uid())
        {
            return Some(open);
        }
        None
    }
}

impl Widget for ShadSheet {
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
            return ScriptAsyncResult::Return(ScriptValue::from_bool(self.open));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_open_state(cx);

        if self.open {
            self.overlay.handle_event(cx, event, scope);
            if let Event::Actions(actions) = event {
                if modal_dismissed(&self.overlay, cx, actions) {
                    self.close(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_open_state(cx);
        draw_modal_overlay(cx, scope, walk, self.layout, self.open, &mut self.overlay)
    }
}

impl ShadSheetRef {
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

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }
}
