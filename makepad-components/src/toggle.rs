use crate::button::ShadControlSize;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadToggleBase = set_type_default() do #(ShadToggle::register_widget(vm)){
        width: Fit
        height: 36
        size: ShadControlSize.Default
        size_is_managed: true
        size_small_height: 28
        size_default_height: 36
        size_large_height: 44
        size_small_padding_x: 10
        size_default_padding_x: 12
        size_large_padding_x: 16
        size_small_font_size: 10
        size_default_font_size: 11
        size_large_font_size: 12
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        align: Align{x: 0.5, y: 0.5}
        icon_walk: Walk{width: 0.0, height: 0.0}
        label_walk: Walk{
            width: Fit
            height: Fit
            margin: Inset{left: 0, right: 0, top: 0, bottom: 0}
        }
        label_align: Align{x: 0.5, y: 0.5}

        draw_bg +: {
            hover: instance(0.0)
            focus: instance(0.0)
            down: instance(0.0)
            active: instance(0.0)
            disabled: instance(0.0)

            border_radius: (shad_theme.radius)
            border_size: (shad_theme.border_size)

            color: (shad_theme.color_clear)
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_clear)
            color_active: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted)

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_focus: (shad_theme.color_primary)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)

            mark_color: (shad_theme.color_clear)
            mark_color_hover: (shad_theme.color_clear)
            mark_color_down: (shad_theme.color_clear)
            mark_color_active: (shad_theme.color_clear)
            mark_color_active_hover: (shad_theme.color_clear)
            mark_color_focus: (shad_theme.color_clear)
            mark_color_disabled: (shad_theme.color_clear)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let radius = self.border_radius
                let inset = self.border_size * 0.5

                sdf.box(
                    inset,
                    inset,
                    self.rect_size.x - inset * 2.0,
                    self.rect_size.y - inset * 2.0,
                    radius
                )

                let base_fill = self.color
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                    .mix(self.color_disabled, self.disabled)

                let active_fill = self.color_active
                    .mix(self.color_disabled, self.disabled)

                let color_fill = base_fill.mix(active_fill, self.active)

                let base_stroke = self.border_color
                    .mix(self.border_color_hover, self.hover)
                    .mix(self.border_color_down, self.down)
                    .mix(self.border_color_disabled, self.disabled)

                let active_stroke = self.border_color_active
                    .mix(self.border_color_disabled, self.disabled)

                let color_stroke = base_stroke.mix(active_stroke, self.active)

                sdf.fill_keep(color_fill)
                sdf.stroke(color_stroke, self.border_size)

                if self.focus > 0.0 {
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, radius + 1.0)
                    sdf.stroke(
                        self.border_color_focus.mix(vec4(0.0, 0.0, 0.0, 0.0), self.active * 0.5),
                        2.0
                    )
                }
                return sdf.result
            }
        }

        draw_text +: {
            hover: instance(0.0)
            focus: instance(0.0)
            down: instance(0.0)
            active: instance(0.0)
            disabled: instance(0.0)

            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary_foreground)
            color_disabled: (shad_theme.color_muted_foreground)

            get_color: fn() {
                let base = self.color
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                    .mix(self.color_focus, self.focus * 0.35)
                    .mix(self.color_disabled, self.disabled)

                return base.mix(self.color_active, self.active)
            }
            text_style.font_size: 11
        }
    }

    mod.widgets.ShadToggle = mod.widgets.ShadToggleBase{}

    mod.widgets.ShadToggleGroup = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 4.0
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: (shad_theme.border_size)
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadToggleGroupItem = mod.widgets.ShadToggle{
        draw_bg +: {
            border_size: 0.0
            border_color: (shad_theme.color_clear)
            border_color_hover: (shad_theme.color_clear)
            border_color_down: (shad_theme.color_clear)
            border_color_focus: (shad_theme.color_clear)
            border_color_active: (shad_theme.color_clear)
            border_color_disabled: (shad_theme.color_clear)
        }
    }

}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ManagedToggleSize {
    height: f64,
    padding_x: f64,
    font_size: f64,
}

#[derive(Script, Widget)]
pub struct ShadToggle {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    check_box: CheckBox,
    #[live(ShadControlSize::Default)]
    size: ShadControlSize,
    #[live(false)]
    size_is_managed: bool,
    #[live(28.0)]
    size_small_height: f64,
    #[live(36.0)]
    size_default_height: f64,
    #[live(44.0)]
    size_large_height: f64,
    #[live(10.0)]
    size_small_padding_x: f64,
    #[live(12.0)]
    size_default_padding_x: f64,
    #[live(16.0)]
    size_large_padding_x: f64,
    #[live(10.0)]
    size_small_font_size: f64,
    #[live(11.0)]
    size_default_font_size: f64,
    #[live(12.0)]
    size_large_font_size: f64,
    #[rust]
    applied_size: Option<ManagedToggleSize>,
}

impl ScriptHook for ShadToggle {
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

impl ShadToggle {
    fn managed_size(&self) -> Option<ManagedToggleSize> {
        if !self.size_is_managed {
            return None;
        }

        Some(match self.size {
            ShadControlSize::Small => ManagedToggleSize {
                height: self.size_small_height,
                padding_x: self.size_small_padding_x,
                font_size: self.size_small_font_size,
            },
            ShadControlSize::Default => ManagedToggleSize {
                height: self.size_default_height,
                padding_x: self.size_default_padding_x,
                font_size: self.size_default_font_size,
            },
            ShadControlSize::Large => ManagedToggleSize {
                height: self.size_large_height,
                padding_x: self.size_large_padding_x,
                font_size: self.size_large_font_size,
            },
        })
    }

    fn sync_managed_size(&mut self, cx: &mut Cx) {
        let Some(size) = self.managed_size() else {
            return;
        };

        if self.applied_size == Some(size) {
            return;
        }

        let padding = Inset {
            left: size.padding_x,
            right: size.padding_x,
            top: 0.0,
            bottom: 0.0,
        };

        script_apply_eval!(cx, self.check_box, {
            height: #(size.height)
            padding: #(padding)
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
}

impl Widget for ShadToggle {
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

impl ShadToggleRef {
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
}
