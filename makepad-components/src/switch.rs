use crate::button::ShadControlSize;
use makepad_widgets::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct ManagedSwitchSize {
    height: f64,
    track_size: f64,
    track_width: f64,
    label_margin_left: f64,
    font_size: f64,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSwitchBase = set_type_default() do #(ShadSwitch::register_widget(vm)){
        width: Fit
        height: 36
        size: ShadControlSize.Default
        size_is_managed: true
        size_small_height: 28.0
        size_default_height: 36.0
        size_large_height: 44.0
        size_small_track_size: 12.0
        size_default_track_size: 15.0
        size_large_track_size: 18.0
        size_small_label_margin_left: 22.0
        size_default_label_margin_left: 27.0
        size_large_label_margin_left: 32.0
        size_small_font_size: 10.0
        size_default_font_size: 11.0
        size_large_font_size: 12.0

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11
        }

        draw_bg +: {
            hover: instance(0.0)
            focus: instance(0.0)
            down: instance(0.0)
            active: instance(0.0)
            disabled: instance(0.0)

            size: 15.0
            track_width: instance(24.0)
            border_size: (shad_theme.border_size)
            border_radius: (shad_theme.radius)

            color: (shad_theme.color_secondary)
            color_hover: (shad_theme.color_secondary_hover)
            color_down: (shad_theme.color_secondary_down)
            color_active: (shad_theme.color_primary)
            color_focus: (shad_theme.color_secondary_hover)
            color_disabled: (shad_theme.color_disabled)

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_active: (shad_theme.color_outline_border)
            border_color_focus: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_disabled_border)

            mark_color: (shad_theme.color_primary)
            mark_color_hover: (shad_theme.color_primary)
            mark_color_down: (shad_theme.color_primary)
            mark_color_active: (shad_theme.color_primary_foreground)
            mark_color_active_hover: (shad_theme.color_primary_foreground)
            mark_color_focus: (shad_theme.color_primary)
            mark_color_disabled: (shad_theme.color_muted_foreground)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let sz_px = vec2(self.track_width, self.size)
                let center_px = vec2(sz_px.x * 0.5, self.rect_size.y * 0.5)
                let offset_px = vec2(0., center_px.y - sz_px.y * 0.5)

                sdf.box(
                    offset_px.x + self.border_size,
                    offset_px.y + self.border_size,
                    sz_px.x - self.border_size * 2.,
                    sz_px.y - self.border_size * 2.,
                    self.border_radius * self.size * 0.1
                )

                let color_fill = self.color
                    .mix(self.color_focus, self.focus)
                    .mix(self.color_active, self.active)
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                    .mix(self.color_disabled, self.disabled)

                let color_stroke = self.border_color
                    .mix(self.border_color_focus, self.focus)
                    .mix(self.border_color_active, self.active)
                    .mix(self.border_color_hover, self.hover)
                    .mix(self.border_color_down, self.down)
                    .mix(self.border_color_disabled, self.disabled)

                sdf.fill_keep(color_fill)
                sdf.stroke(color_stroke, self.border_size)

                let mark_padding = 1.5
                let mark_size = sz_px.y * 0.5 - self.border_size - mark_padding
                let mark_target_y = sz_px.y - sz_px.x + self.border_size + mark_padding
                let mark_pos_y = sz_px.y * 0.5 + self.border_size - mark_target_y * self.active

                sdf.circle(mark_pos_y, center_px.y, mark_size)
                sdf.circle(mark_pos_y, center_px.y, mark_size * 0.45)
                sdf.subtract()

                sdf.circle(mark_pos_y, center_px.y, mark_size)
                sdf.blend(self.active)

                let mark_color = self.mark_color
                    .mix(self.mark_color_hover, self.hover)
                    .mix(self.mark_color_active, self.active)
                    .mix(self.mark_color_disabled, self.disabled)

                sdf.fill(mark_color)
                return sdf.result
            }
        }

        animator +: {
            active: {
                default: @off
                off: AnimatorState{
                    ease: OutQuad
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {active: 0.0}
                        draw_text: {active: 0.0}
                    }
                }
                on: AnimatorState{
                    ease: OutQuad
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {active: 1.0}
                        draw_text: {active: 1.0}
                    }
                }
            }
        }
    }

    mod.widgets.ShadSwitch = mod.widgets.ShadSwitchBase{}
}

#[derive(Script, Widget)]
pub struct ShadSwitch {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    check_box: CheckBox,
    #[live(ShadControlSize::Default)]
    size: ShadControlSize,
    #[live(true)]
    size_is_managed: bool,
    #[live(28.0)]
    size_small_height: f64,
    #[live(36.0)]
    size_default_height: f64,
    #[live(44.0)]
    size_large_height: f64,
    #[live(12.0)]
    size_small_track_size: f64,
    #[live(15.0)]
    size_default_track_size: f64,
    #[live(18.0)]
    size_large_track_size: f64,
    #[live(22.0)]
    size_small_label_margin_left: f64,
    #[live(27.0)]
    size_default_label_margin_left: f64,
    #[live(32.0)]
    size_large_label_margin_left: f64,
    #[live(10.0)]
    size_small_font_size: f64,
    #[live(11.0)]
    size_default_font_size: f64,
    #[live(12.0)]
    size_large_font_size: f64,
    #[rust]
    applied_size: Option<ManagedSwitchSize>,
}

impl ScriptHook for ShadSwitch {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.sync_managed_size(cx);
        });
    }
}

impl ShadSwitch {
    fn managed_size(&self) -> Option<ManagedSwitchSize> {
        if !self.size_is_managed {
            return None;
        }

        Some(match self.size {
            ShadControlSize::Small => ManagedSwitchSize {
                height: self.size_small_height,
                track_size: self.size_small_track_size,
                track_width: self.size_small_track_size * 1.6,
                label_margin_left: self.size_small_label_margin_left,
                font_size: self.size_small_font_size,
            },
            ShadControlSize::Default => ManagedSwitchSize {
                height: self.size_default_height,
                track_size: self.size_default_track_size,
                track_width: self.size_default_track_size * 1.6,
                label_margin_left: self.size_default_label_margin_left,
                font_size: self.size_default_font_size,
            },
            ShadControlSize::Large => ManagedSwitchSize {
                height: self.size_large_height,
                track_size: self.size_large_track_size,
                track_width: self.size_large_track_size * 1.6,
                label_margin_left: self.size_large_label_margin_left,
                font_size: self.size_large_font_size,
            },
        })
    }

    fn sync_managed_size(&mut self, cx: &mut Cx) {
        let Some(size) = self.managed_size() else {
            self.applied_size = None;
            return;
        };

        if self.applied_size == Some(size) {
            return;
        }

        let margin = Inset {
            left: size.label_margin_left,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        };

        script_apply_eval!(cx, self.check_box, {
            height: #(size.height)
            icon_walk.width: #(size.track_width)
            label_walk.margin: #(margin)
            draw_bg.size: #(size.track_size)
            draw_bg.track_width: #(size.track_width)
            draw_text.text_style.font_size: #(size.font_size)
        });

        self.applied_size = Some(size);
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        self.check_box.changed(actions)
    }

    pub fn active(&self, cx: &Cx) -> bool {
        self.check_box.active(cx)
    }

    pub fn set_active(&mut self, cx: &mut Cx, value: bool) {
        self.check_box.set_active(cx, value);
    }

    pub fn set_size(&mut self, cx: &mut Cx, size: ShadControlSize) {
        if self.size == size {
            return;
        }
        self.size = size;
        self.sync_managed_size(cx);
    }

    pub fn size(&self) -> ShadControlSize {
        self.size
    }
}

impl Widget for ShadSwitch {
    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.check_box.set_disabled(cx, disabled);
    }

    fn disabled(&self, cx: &Cx) -> bool {
        self.check_box.disabled(cx)
    }

    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        self.check_box.script_call(vm, method, args)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.check_box.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_managed_size(&mut *cx);
        self.check_box.draw_walk(cx, scope, walk)
    }
}

impl ShadSwitchRef {
    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }

    pub fn active(&self, cx: &Cx) -> bool {
        self.borrow().is_some_and(|inner| inner.active(cx))
    }

    pub fn set_active(&self, cx: &mut Cx, value: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_active(cx, value);
        }
    }

    pub fn set_size(&self, cx: &mut Cx, size: ShadControlSize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_size(cx, size);
        }
    }

    pub fn size(&self) -> ShadControlSize {
        self.borrow()
            .map_or(ShadControlSize::Default, |inner| inner.size())
    }
}
