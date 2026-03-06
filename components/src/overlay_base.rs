use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadOverlayBaseWidget = #(ShadOverlayBase::register_widget(vm))

    mod.widgets.ShadOverlayBase = set_type_default() do mod.widgets.ShadOverlayBaseWidget{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: Fit
                height: Fit
            }
        }
    }
}

/// Sync the modal's open/closed state with the `open` flag.
/// Only calls `modal.open` / `modal.close` when the state actually changes.
pub(crate) fn sync_overlay_open(
    open: bool,
    is_synced_open: &mut bool,
    overlay: &WidgetRef,
    cx: &mut Cx,
) {
    if *is_synced_open == open {
        return;
    }
    if let Some(mut modal) = overlay.borrow_mut::<Modal>() {
        if open {
            modal.open(cx);
        } else {
            modal.close(cx);
        }
    }
    *is_synced_open = open;
}

/// Handle the `set_open` and `is_open` script methods common to all overlay widgets.
/// Returns `MethodNotFound` for any other method so callers can extend with their own.
pub(crate) fn handle_overlay_script_call(
    open: &mut bool,
    is_synced_open: &mut bool,
    overlay: &WidgetRef,
    vm: &mut ScriptVm,
    method: LiveId,
    args: ScriptValue,
) -> ScriptAsyncResult {
    if method == live_id!(set_open) {
        if let Some(args_obj) = args.as_object() {
            let trap = vm.bx.threads.cur().trap.pass();
            let value = vm.bx.heap.vec_value(args_obj, 0, trap);
            if let Some(new_open) = value.as_bool() {
                vm.with_cx_mut(|cx| {
                    *open = new_open;
                    sync_overlay_open(*open, is_synced_open, overlay, cx);
                });
            }
        }
        return ScriptAsyncResult::Return(NIL);
    }
    if method == live_id!(is_open) {
        return ScriptAsyncResult::Return(ScriptValue::from_bool(*open));
    }
    ScriptAsyncResult::MethodNotFound
}

/// Check `ModalAction::Dismissed` (backdrop click / Escape key) and close the overlay.
/// Returns `true` if the overlay was dismissed.
pub(crate) fn handle_overlay_dismissed(
    open: &mut bool,
    is_synced_open: &mut bool,
    overlay: &WidgetRef,
    cx: &mut Cx,
    actions: &Actions,
) -> bool {
    let content = overlay.widget(cx, ids!(content));
    if actions
        .find_widget_action(content.widget_uid())
        .is_some_and(|a| matches!(a.cast(), ModalAction::Dismissed))
    {
        *open = false;
        sync_overlay_open(*open, is_synced_open, overlay, cx);
        return true;
    }
    false
}

/// Check whether the button at `path` inside the overlay was clicked and close the overlay.
/// Returns `true` if the button was clicked.
pub(crate) fn handle_overlay_close_button(
    open: &mut bool,
    is_synced_open: &mut bool,
    overlay: &WidgetRef,
    cx: &mut Cx,
    actions: &Actions,
    path: &[LiveId],
) -> bool {
    let btn = overlay.widget(cx, path);
    if !btn.is_empty()
        && actions
            .find_widget_action(btn.widget_uid())
            .is_some_and(|a| matches!(a.cast(), ButtonAction::Clicked(_)))
    {
        *open = false;
        sync_overlay_open(*open, is_synced_open, overlay, cx);
        return true;
    }
    false
}

/// Standalone base widget for overlay-style components (Drawer, Sheet, Dialog, Sonner).
///
/// Contains the common `overlay`/`open`/`is_synced_open` fields and implements the
/// standard `Widget` lifecycle so derived components only need to add their own fields
/// and customize layout or action handling.
#[derive(Script, ScriptHook, Widget)]
pub struct ShadOverlayBase {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,

    #[find]
    #[redraw]
    #[live]
    pub overlay: WidgetRef,

    #[live]
    pub open: bool,

    #[rust]
    pub is_synced_open: bool,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadOverlayBase {
    /// Open or close the underlying modal, tracking state to avoid redundant calls.
    pub fn sync_open_state(&mut self, cx: &mut Cx) {
        sync_overlay_open(self.open, &mut self.is_synced_open, &self.overlay, cx);
    }

    /// Set the open flag. The modal will be synced on the next widget lifecycle call
    /// (`handle_event` or `draw_walk`). Use `ShadOverlayBaseRef::set_open` when you need
    /// immediate sync and have a `&mut Cx` available.
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Widget for ShadOverlayBase {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        handle_overlay_script_call(
            &mut self.open,
            &mut self.is_synced_open,
            &self.overlay,
            vm,
            method,
            args,
        )
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_open_state(cx);
        if self.open {
            self.overlay.handle_event(cx, event, scope);
            if let Event::Actions(actions) = event {
                handle_overlay_dismissed(
                    &mut self.open,
                    &mut self.is_synced_open,
                    &self.overlay,
                    cx,
                    actions,
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_open_state(cx);
        if !self.open {
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

impl ShadOverlayBaseRef {
    /// Open or close the overlay and immediately sync the modal state.
    /// Prefer this method when you have a `&mut Cx` available (e.g. from an event handler).
    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(open);
            inner.sync_open_state(cx);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }
}
