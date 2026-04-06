use crate::internal::actions::first_widget_action;
use crate::internal::overlay::{
    draw_modal_overlay, modal_dismissed, sync_modal_open_state,
};
use crate::internal::script_args::bool_arg;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

#[derive(Clone, Debug)]
struct SheetAnimation {
    from_progress: f64,
    to_progress: f64,
    start_time: Option<f64>,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSheetTitle = mod.widgets.ShadAlertTitle{}
    mod.widgets.ShadSheetDescription = mod.widgets.ShadAlertDescription{}

    mod.widgets.ShadSheetBase = #(ShadSheet::register_widget(vm))

    mod.widgets.ShadSheet = set_type_default() do mod.widgets.ShadSheetBase{
        width: Fill
        height: Fill
        open: false
        side: "right"
        sheet_size: 360.0
        color_overlay: (shad_theme.color_overlay)

        overlay: Modal{
            align: Align{x: 1.0, y: 0.0}
            bg_view +: {
                draw_bg.color: (shad_theme.color_overlay)
            }

            content +: {
                width: 360
                height: Fill
                flow: Down
                clip_x: true
                clip_y: true

                sheet_frame := mod.widgets.ShadSurfacePanel{
                    width: Fill
                    height: Fill

                    header := mod.widgets.ShadSurfaceHeader{
                        title := ShadAlertTitle{
                            text: "Edit profile"
                        }

                        description := ShadAlertDescription{
                            text: "Make changes to your profile here."
                        }
                    }

                    body := mod.widgets.ShadSurfaceContent{}

                    footer := mod.widgets.ShadSurfaceFooter{}
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
    #[live]
    color_overlay: Vec4,

    #[rust]
    is_synced_open: bool,
    #[rust]
    open_progress: f64,
    #[rust]
    animation: Option<SheetAnimation>,
    #[rust]
    animation_next_frame: NextFrame,
    #[rust]
    last_pass_size: Vec2d,

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
    const ANIMATION_DURATION: f64 = 0.16;

    fn panel_animation_progress(time: f64, start_time: &mut Option<f64>) -> f64 {
        let start_time = start_time.get_or_insert(time);
        let elapsed = (time - *start_time).max(0.0);
        let progress = (elapsed / Self::ANIMATION_DURATION).min(1.0);
        1.0 - (1.0 - progress).powi(3)
    }

    fn sync_side_layout(&mut self, cx: &mut Cx) {
        let current_side = self.side.as_ref();

        // Optimization: avoid repeated allocation in layout sync loop by reusing string capacity
        // Previously: allocated a new String using `current_side.to_string()`
        // Now: reuse `self.last_side` buffer, avoiding a heap allocation
        self.last_side.clear();
        self.last_side.push_str(current_side);
        self.is_side_initialized = true;

        // Optimization: avoid cloning the `WidgetRef`
        // Previously: created a new cloned reference using `self.overlay.clone()`
        // Now: apply directly to `self.overlay`, eliminating clone overhead
        script_apply_eval!(cx, self.overlay, {
            align: #(Align { x: 0.0, y: 0.0 })
        });

        let mut bg_view = self.overlay.widget(cx, ids!(bg_view));
        let overlay_color = vec4(
            self.color_overlay.x,
            self.color_overlay.y,
            self.color_overlay.z,
            self.color_overlay.w * self.open_progress as f32,
        );
        script_apply_eval!(cx, bg_view, {
            draw_bg +: {
                color: #(overlay_color)
            }
        });

        let content = self.overlay.widget(cx, ids!(content));
        if let Some(mut content_view) = content.borrow_mut::<View>() {
            let progress = self.open_progress.clamp(0.0, 1.0);
            let pass_width = self.last_pass_size.x.max(0.0);
            let pass_height = self.last_pass_size.y.max(0.0);

            let (content_width, content_height, abs_pos) = match current_side {
                "left" => (
                    self.sheet_size,
                    pass_height,
                    dvec2(-self.sheet_size * (1.0 - progress), 0.0),
                ),
                "top" => (
                    pass_width,
                    self.sheet_size,
                    dvec2(0.0, -self.sheet_size * (1.0 - progress)),
                ),
                "bottom" => (
                    pass_width,
                    self.sheet_size,
                    dvec2(0.0, pass_height - self.sheet_size * progress),
                ),
                _ => (
                    self.sheet_size,
                    pass_height,
                    dvec2(pass_width - self.sheet_size * progress, 0.0),
                ),
            };

            content_view.walk.width = Size::Fixed(content_width.max(0.0));
            content_view.walk.height = Size::Fixed(content_height.max(0.0));
            content_view.walk.abs_pos = Some(abs_pos);
        }

        let mut sheet_frame = self.overlay.widget(cx, ids!(content.sheet_frame));
        script_apply_eval!(cx, sheet_frame, {
            width: #(Size::fill())
            height: #(Size::fill())
        });
    }

    fn start_animation(&mut self, cx: &mut Cx, open: bool) {
        self.animation = Some(SheetAnimation {
            from_progress: self.open_progress,
            to_progress: if open { 1.0 } else { 0.0 },
            start_time: None,
        });
        self.animation_next_frame = cx.new_next_frame();
    }

    fn step_animation(&mut self, cx: &mut Cx, time: f64) {
        let Some((progress, target_progress)) = self.animation.as_mut().map(|animation| {
            let progress = Self::panel_animation_progress(time, &mut animation.start_time);
            self.open_progress = animation.from_progress
                + (animation.to_progress - animation.from_progress) * progress;
            (progress, animation.to_progress)
        }) else {
            return;
        };
        self.sync_side_layout(cx);
        self.overlay.redraw(cx);

        if progress >= 1.0 {
            self.open_progress = target_progress;
            self.animation = None;
            self.sync_open_state(cx);
            self.sync_side_layout(cx);
        } else {
            self.animation_next_frame = cx.new_next_frame();
        }
    }

    fn sync_open_state(&mut self, cx: &mut Cx) {
        if self.animation.is_none() {
            self.open_progress = if self.open { 1.0 } else { 0.0 };
        }
        self.sync_side_layout(cx);
        let render_open = self.open || self.open_progress > 0.0;
        sync_modal_open_state(cx, &mut self.overlay, &mut self.is_synced_open, render_open);
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if self.open == open && self.animation.is_none() {
            return;
        }
        let uid = self.widget_uid();
        self.open = open;
        self.start_animation(cx, open);
        self.sync_open_state(cx);
        cx.widget_action_with_data(
            &self.action_data,
            uid,
            ShadSheetAction::OpenChanged(open),
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

        if let Event::NextFrame(ne) = event {
            if self.animation_next_frame.is_event(event).is_some() {
                self.animation_next_frame = NextFrame::default();
                self.step_animation(cx, ne.time);
                return;
            }
        }

        if self.open || self.open_progress > 0.0 {
            self.overlay.handle_event(cx, event, scope);
            if let Event::Actions(actions) = event {
                if self.open && modal_dismissed(&self.overlay, cx, actions) {
                    self.close(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.last_pass_size = cx.current_pass_size();
        self.sync_open_state(cx);
        draw_modal_overlay(
            cx,
            scope,
            walk,
            self.layout,
            self.open || self.open_progress > 0.0,
            &mut self.overlay,
        )
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
