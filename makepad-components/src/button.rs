use makepad_widgets::makepad_script::ScriptFnRef;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ShadControlSize = set_type_default() do #(ShadControlSize::script_api(vm))
    mod.widgets.ShadControlSize = ShadControlSize
    let ShadButtonVariant = set_type_default() do #(ShadButtonVariant::script_api(vm))
    mod.widgets.ShadButtonVariant = ShadButtonVariant

    mod.widgets.ShadNavButtonBase = set_type_default() do #(ShadNavButton::register_widget(vm)){
        text: "Button"
        width: Fit
        height: Fit
        spacing: theme.space_2
        align: Center
        padding: theme.mspace_1{left: theme.space_2, right: theme.space_2}
        margin: theme.mspace_v_1
        label_walk: Walk{width: Fit, height: Fit}
        grab_key_focus: true

        draw_text +: {
            hover: 0.0
            down: instance(0.0)
            focus: instance(0.0)
            active: instance(0.0)
            disabled: instance(0.0)

            color: theme.color_label_inner
            color_hover: theme.color_label_inner_hover
            color_down: uniform(theme.color_label_inner_down)
            color_focus: uniform(theme.color_label_inner_focus)
            color_active: uniform(theme.color_label_inner_focus)
            color_disabled: uniform(theme.color_label_inner_disabled)

            text_style: theme.font_regular{
                font_size: theme.font_size_p
            }
            get_color: fn() {
                return self.color
                    .mix(self.color_active, self.active)
                    .mix(self.color_focus, self.focus)
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                    .mix(self.color_disabled, self.disabled)
            }
        }

        draw_bg +: {
            hover: instance(0.0)
            focus: instance(0.0)
            down: instance(0.0)
            active: instance(0.0)
            disabled: instance(0.0)

            border_size: uniform(theme.beveling)
            border_radius: uniform(theme.corner_radius)

            color_dither: uniform(1.0)
            gradient_border_horizontal: uniform(0.0)
            gradient_fill_horizontal: uniform(0.0)

            color: uniform(theme.color_outset)
            color_hover: uniform(theme.color_outset_hover)
            color_down: uniform(theme.color_outset_down)
            color_focus: uniform(theme.color_outset_focus)
            color_active: uniform(theme.color_outset_focus)
            color_disabled: uniform(theme.color_outset_disabled)

            color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            color_2_hover: uniform(theme.color_outset_2_hover)
            color_2_down: uniform(theme.color_outset_2_down)
            color_2_focus: uniform(theme.color_outset_2_focus)
            color_2_active: uniform(theme.color_outset_2_focus)
            color_2_disabled: uniform(theme.color_outset_2_disabled)

            border_color: uniform(theme.color_bevel)
            border_color_hover: uniform(theme.color_bevel_hover)
            border_color_down: uniform(theme.color_bevel_down)
            border_color_focus: uniform(theme.color_bevel_focus)
            border_color_active: uniform(theme.color_bevel_focus)
            border_color_disabled: uniform(theme.color_bevel_disabled)

            border_color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            border_color_2_hover: uniform(theme.color_bevel_outset_2_hover)
            border_color_2_down: uniform(theme.color_bevel_outset_2_down)
            border_color_2_focus: uniform(theme.color_bevel_outset_2_focus)
            border_color_2_active: uniform(theme.color_bevel_outset_2_focus)
            border_color_2_disabled: uniform(theme.color_bevel_outset_2_disabled)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let border_sz_uv = vec2(
                    self.border_size / self.rect_size.x
                    self.border_size / self.rect_size.y
                )

                let sz_inner_px = vec2(
                    self.rect_size.x - self.border_size * 2.
                    self.rect_size.y - self.border_size * 2.
                )

                let scale_factor_fill = vec2(
                    self.rect_size.x / sz_inner_px.x
                    self.rect_size.y / sz_inner_px.y
                )

                sdf.box(
                    self.border_size
                    self.border_size
                    self.rect_size.x - self.border_size * 2.
                    self.rect_size.y - self.border_size * 2.
                    self.border_radius
                )

                let mut color_fill = self.color
                let mut color_fill_hover = self.color_hover
                let mut color_fill_down = self.color_down
                let mut color_fill_focus = self.color_focus
                let mut color_fill_active = self.color_active
                let mut color_fill_disabled = self.color_disabled

                if self.color_2.x > -0.5 {
                    let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                    let gradient_fill = vec2(
                        self.pos.x * scale_factor_fill.x - border_sz_uv.x * 2. + dither
                        self.pos.y * scale_factor_fill.y - border_sz_uv.y * 2. + dither
                    )
                    let dir = if self.gradient_fill_horizontal > 0.5 gradient_fill.x else gradient_fill.y
                    color_fill = mix(self.color, self.color_2, dir)
                    color_fill_hover = mix(self.color_hover, self.color_2_hover, dir)
                    color_fill_down = mix(self.color_down, self.color_2_down, dir)
                    color_fill_focus = mix(self.color_focus, self.color_2_focus, dir)
                    color_fill_active = mix(self.color_active, self.color_2_active, dir)
                    color_fill_disabled = mix(self.color_disabled, self.color_2_disabled, dir)
                }

                let mut color_stroke = self.border_color
                let mut color_stroke_hover = self.border_color_hover
                let mut color_stroke_down = self.border_color_down
                let mut color_stroke_focus = self.border_color_focus
                let mut color_stroke_active = self.border_color_active
                let mut color_stroke_disabled = self.border_color_disabled

                if self.border_color_2.x > -0.5 {
                    let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                    let gradient_border = vec2(
                        self.pos.x + dither
                        self.pos.y + dither
                    )
                    let dir = if self.gradient_border_horizontal > 0.5 gradient_border.x else gradient_border.y
                    color_stroke = mix(self.border_color, self.border_color_2, dir)
                    color_stroke_hover = mix(self.border_color_hover, self.border_color_2_hover, dir)
                    color_stroke_down = mix(self.border_color_down, self.border_color_2_down, dir)
                    color_stroke_focus = mix(self.border_color_focus, self.border_color_2_focus, dir)
                    color_stroke_active = mix(self.border_color_active, self.border_color_2_active, dir)
                    color_stroke_disabled = mix(self.border_color_disabled, self.border_color_2_disabled, dir)
                }

                let fill = color_fill
                    .mix(color_fill_active, self.active)
                    .mix(color_fill_focus, self.focus)
                    .mix(color_fill_hover, self.hover)
                    .mix(color_fill_down, self.down)
                    .mix(color_fill_disabled, self.disabled)

                let stroke = color_stroke
                    .mix(color_stroke_active, self.active)
                    .mix(color_stroke_focus, self.focus)
                    .mix(color_stroke_hover, self.hover)
                    .mix(color_stroke_down, self.down)
                    .mix(color_stroke_disabled, self.disabled)

                sdf.fill_keep(fill)
                sdf.stroke(stroke, self.border_size)
                return sdf.result
            }
        }

        animator: Animator{
            disabled: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.0}}
                    apply: {
                        draw_bg: {disabled: 0.0}
                        draw_text: {disabled: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {disabled: 1.0}
                        draw_text: {disabled: 1.0}
                    }
                }
            }
            time: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.0}}
                    apply: {
                    }
                }
                on: AnimatorState{
                    from: {all: Loop {duration: 1.0, end: 1000000000.0}}
                    apply: {
                        draw_bg: {anim_time: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]}
                    }
                }
            }
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {down: 0.0, hover: 0.0}
                        draw_text: {down: 0.0, hover: 0.0}
                    }
                }

                on: AnimatorState{
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {down: 0.0, hover: snap(1.0)}
                        draw_text: {down: 0.0, hover: snap(1.0)}
                    }
                }

                down: AnimatorState{
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {down: snap(1.0), hover: 1.0}
                        draw_text: {down: snap(1.0), hover: 1.0}
                    }
                }
            }
            focus: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {focus: 0.0}
                        draw_text: {focus: 0.0}
                    }
                }
                on: AnimatorState{
                    cursor: MouseCursor.Arrow
                    from: {all: Forward {duration: 0.0}}
                    apply: {
                        draw_bg: {focus: 1.0}
                        draw_text: {focus: 1.0}
                    }
                }
            }
            active: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.12}}
                    apply: {
                        draw_bg: {active: 0.0}
                        draw_text: {active: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.12}}
                    apply: {
                        draw_bg: {active: 1.0}
                        draw_text: {active: 1.0}
                    }
                }
            }
        }
    }

    mod.widgets.ShadButtonBase = set_type_default() do mod.widgets.ShadNavButtonBase{
        width: Fit
        size_is_managed: true
        variant_is_managed: true
        size: ShadControlSize.Default
        variant: ShadButtonVariant.Primary
        size_small_height: 28
        size_default_height: 36
        size_large_height: 44
        size_small_padding_x: 12
        size_default_padding_x: 16
        size_large_padding_x: 32
        size_small_font_size: 10
        size_default_font_size: 11
        size_large_font_size: 13
        variant_link_small_padding_x: 4
        variant_link_default_padding_x: 4
        variant_link_large_padding_x: 6
        managed_radius: (shad_theme.radius)
        managed_link_radius: 0.0
        managed_disabled_fill: (shad_theme.color_disabled)
        managed_disabled_text: (shad_theme.color_disabled_foreground)
        managed_disabled_border: (shad_theme.color_disabled_border)
        primary_fill: (shad_theme.color_primary_foreground)
        primary_fill_hover: (shad_theme.color_secondary)
        primary_fill_down: (shad_theme.color_secondary_hover)
        primary_fill_focus: (shad_theme.color_secondary)
        primary_text: (shad_theme.color_primary)
        primary_text_hover: (shad_theme.color_primary)
        primary_text_down: (shad_theme.color_primary)
        primary_text_focus: (shad_theme.color_primary)
        secondary_fill: (shad_theme.color_secondary)
        secondary_fill_hover: (shad_theme.color_secondary_hover)
        secondary_fill_down: (shad_theme.color_secondary_down)
        secondary_fill_focus: (shad_theme.color_secondary_hover)
        secondary_text: (shad_theme.color_secondary_foreground)
        secondary_text_hover: (shad_theme.color_secondary_foreground)
        secondary_text_down: (shad_theme.color_secondary_foreground)
        secondary_text_focus: (shad_theme.color_secondary_foreground)
        outline_fill: #0000
        outline_fill_hover: (shad_theme.color_ghost_hover)
        outline_fill_down: (shad_theme.color_ghost_down)
        outline_fill_focus: (shad_theme.color_ghost_hover)
        outline_border: (shad_theme.color_outline_border)
        outline_border_hover: (shad_theme.color_outline_border_hover)
        outline_border_down: (shad_theme.color_outline_border_down)
        outline_border_focus: (shad_theme.color_primary)
        outline_text: (shad_theme.color_primary)
        outline_text_hover: (shad_theme.color_primary)
        outline_text_down: (shad_theme.color_primary)
        outline_text_focus: (shad_theme.color_primary)
        ghost_fill: #0000
        ghost_fill_hover: (shad_theme.color_ghost_hover)
        ghost_fill_down: (shad_theme.color_ghost_down)
        ghost_fill_focus: (shad_theme.color_ghost_hover)
        ghost_text: (shad_theme.color_primary)
        ghost_text_hover: (shad_theme.color_primary)
        ghost_text_down: (shad_theme.color_primary)
        ghost_text_focus: (shad_theme.color_primary)
        destructive_fill: (shad_theme.color_destructive)
        destructive_fill_hover: (shad_theme.color_destructive_hover)
        destructive_fill_down: (shad_theme.color_destructive_down)
        destructive_fill_focus: (shad_theme.color_destructive_hover)
        destructive_text: (shad_theme.color_destructive_foreground)
        destructive_text_hover: (shad_theme.color_destructive_foreground)
        destructive_text_down: (shad_theme.color_destructive_foreground)
        destructive_text_focus: (shad_theme.color_destructive_foreground)
        link_fill: #0000
        link_fill_hover: #0000
        link_fill_down: #0000
        link_fill_focus: #0000
        link_fill_disabled: #0000
        link_text: (shad_theme.color_muted_foreground)
        link_text_hover: (shad_theme.color_primary)
        link_text_down: (shad_theme.color_primary_down)
        link_text_focus: (shad_theme.color_primary)
    }

    mod.widgets.ShadButton = mod.widgets.ShadButtonBase{}

    mod.widgets.ShadPreviewTab = mod.widgets.ShadTabsTrigger{
        height: 36
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: #0000
            color_down: #0000
            border_size: 0.0
            border_radius: 0.0
        }
    }

    mod.widgets.ShadButtonIcon = mod.widgets.ShadButtonBase{
        width: 36
        spacing: 0.0
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }

    mod.widgets.ShadButtonIconSm = mod.widgets.ShadButtonIcon{
        width: 28
        size: ShadControlSize.Small
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }

    mod.widgets.ShadButtonIconLg = mod.widgets.ShadButtonIcon{
        width: 44
        size: ShadControlSize.Large
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }

}

#[derive(Script, ScriptHook, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShadControlSize {
    Small,
    #[pick]
    #[default]
    Default,
    Large,
}

#[derive(Script, ScriptHook, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShadButtonVariant {
    #[pick]
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

#[derive(Clone, Copy, Debug)]
struct ManagedButtonSize {
    height: f64,
    padding_x: f64,
    font_size: f64,
}

#[derive(Clone, Copy, Debug)]
struct ManagedButtonVariantStyle {
    fill: Vec4,
    fill_hover: Vec4,
    fill_down: Vec4,
    fill_focus: Vec4,
    fill_disabled: Vec4,
    border_size: f64,
    border_radius: f64,
    border_color: Vec4,
    border_color_hover: Vec4,
    border_color_down: Vec4,
    border_color_focus: Vec4,
    border_color_disabled: Vec4,
    text: Vec4,
    text_hover: Vec4,
    text_down: Vec4,
    text_focus: Vec4,
    text_disabled: Vec4,
}

#[derive(Script, Widget, Animator)]
pub struct ShadNavButton {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[rust]
    area: Area,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[live]
    label_walk: Walk,

    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live(true)]
    grab_key_focus: bool,
    #[live(true)]
    enabled: bool,
    #[live(false)]
    active: bool,
    #[live(true)]
    #[visible]
    visible: bool,
    #[live]
    reset_hover_on_click: bool,
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
    #[live(12.0)]
    size_small_padding_x: f64,
    #[live(16.0)]
    size_default_padding_x: f64,
    #[live(20.0)]
    size_large_padding_x: f64,
    #[live(10.0)]
    size_small_font_size: f64,
    #[live(11.0)]
    size_default_font_size: f64,
    #[live(13.0)]
    size_large_font_size: f64,
    #[live(ShadButtonVariant::Primary)]
    variant: ShadButtonVariant,
    #[live(false)]
    variant_is_managed: bool,
    #[live(4.0)]
    variant_link_small_padding_x: f64,
    #[live(4.0)]
    variant_link_default_padding_x: f64,
    #[live(6.0)]
    variant_link_large_padding_x: f64,
    #[live(6.0)]
    managed_radius: f64,
    #[live(0.0)]
    managed_link_radius: f64,
    #[live]
    managed_disabled_fill: Vec4,
    #[live]
    managed_disabled_text: Vec4,
    #[live]
    managed_disabled_border: Vec4,
    #[live]
    primary_fill: Vec4,
    #[live]
    primary_fill_hover: Vec4,
    #[live]
    primary_fill_down: Vec4,
    #[live]
    primary_fill_focus: Vec4,
    #[live]
    primary_text: Vec4,
    #[live]
    primary_text_hover: Vec4,
    #[live]
    primary_text_down: Vec4,
    #[live]
    primary_text_focus: Vec4,
    #[live]
    secondary_fill: Vec4,
    #[live]
    secondary_fill_hover: Vec4,
    #[live]
    secondary_fill_down: Vec4,
    #[live]
    secondary_fill_focus: Vec4,
    #[live]
    secondary_text: Vec4,
    #[live]
    secondary_text_hover: Vec4,
    #[live]
    secondary_text_down: Vec4,
    #[live]
    secondary_text_focus: Vec4,
    #[live]
    outline_fill: Vec4,
    #[live]
    outline_fill_hover: Vec4,
    #[live]
    outline_fill_down: Vec4,
    #[live]
    outline_fill_focus: Vec4,
    #[live]
    outline_border: Vec4,
    #[live]
    outline_border_hover: Vec4,
    #[live]
    outline_border_down: Vec4,
    #[live]
    outline_border_focus: Vec4,
    #[live]
    outline_text: Vec4,
    #[live]
    outline_text_hover: Vec4,
    #[live]
    outline_text_down: Vec4,
    #[live]
    outline_text_focus: Vec4,
    #[live]
    ghost_fill: Vec4,
    #[live]
    ghost_fill_hover: Vec4,
    #[live]
    ghost_fill_down: Vec4,
    #[live]
    ghost_fill_focus: Vec4,
    #[live]
    ghost_text: Vec4,
    #[live]
    ghost_text_hover: Vec4,
    #[live]
    ghost_text_down: Vec4,
    #[live]
    ghost_text_focus: Vec4,
    #[live]
    destructive_fill: Vec4,
    #[live]
    destructive_fill_hover: Vec4,
    #[live]
    destructive_fill_down: Vec4,
    #[live]
    destructive_fill_focus: Vec4,
    #[live]
    destructive_text: Vec4,
    #[live]
    destructive_text_hover: Vec4,
    #[live]
    destructive_text_down: Vec4,
    #[live]
    destructive_text_focus: Vec4,
    #[live]
    link_fill: Vec4,
    #[live]
    link_fill_hover: Vec4,
    #[live]
    link_fill_down: Vec4,
    #[live]
    link_fill_focus: Vec4,
    #[live]
    link_fill_disabled: Vec4,
    #[live]
    link_text: Vec4,
    #[live]
    link_text_hover: Vec4,
    #[live]
    link_text_down: Vec4,
    #[live]
    link_text_focus: Vec4,
    #[live]
    text: ArcStringMut,
    #[live]
    on_click: ScriptFnRef,
    #[rust]
    applied_variant: Option<ShadButtonVariant>,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadNavButton {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            self.sync_active_state_if_needed(cx, Animate::No);
        });
    }

    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        self.applied_variant = None;
        vm.with_cx_mut(|cx| {
            self.sync_managed_size(cx);
            self.sync_managed_variant(cx);
        });
    }
}

impl Widget for ShadNavButton {
    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.animator_toggle(
            cx,
            disabled,
            Animate::Yes,
            ids!(disabled.on),
            ids!(disabled.off),
        );
    }

    fn disabled(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(disabled.on))
    }

    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(text) {
            let str_val = vm.bx.heap.new_string_from_str(self.text.as_ref());
            return ScriptAsyncResult::Return(str_val);
        }
        if method == live_id!(set_text) {
            if let Some(args_obj) = args.as_object() {
                let trap = vm.bx.threads.cur().trap.pass();
                let value = vm.bx.heap.vec_value(args_obj, 0, trap);
                if !value.is_err() {
                    let new_text = vm.bx.heap.temp_string_with(|heap, out| {
                        heap.cast_to_string(value, out);
                        out.to_string()
                    });
                    vm.with_cx_mut(|cx| {
                        self.set_text(cx, &new_text);
                    });
                }
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(on_click) {
            let uid = self.widget_uid();
            vm.with_cx_mut(|cx| {
                self.emit_click(cx, uid, KeyModifiers::default());
            });
            return ScriptAsyncResult::Return(TRUE);
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.area.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::KeyFocus(_) => {
                self.animator_play(cx, ids!(focus.on));
            }
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, ids!(focus.off));
                self.area.redraw(cx);
            }
            Hit::KeyDown(ke)
                if self.enabled && matches!(ke.key_code, KeyCode::ReturnKey | KeyCode::Space) =>
            {
                self.animator_play(cx, ids!(hover.down));
                self.emit_click(cx, uid, ke.modifiers);
                if self.reset_hover_on_click {
                    self.animator_cut(cx, ids!(hover.off));
                } else {
                    self.animator_play(cx, ids!(focus.on));
                }
            }
            Hit::FingerDown(fe) if self.enabled && fe.is_primary_hit() => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area);
                }
                cx.widget_action_with_data(
                    &self.action_data,
                    uid,
                    ButtonAction::Pressed(fe.modifiers),
                );
                self.animator_play(cx, ids!(hover.down));
            }
            Hit::FingerHoverIn(_) => {
                if self.enabled {
                    cx.set_cursor(MouseCursor::Hand);
                    self.animator_play(cx, ids!(hover.on));
                } else {
                    cx.set_cursor(MouseCursor::NotAllowed);
                }
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerLongPress(_lp) if self.enabled => {
                cx.widget_action_with_data(&self.action_data, uid, ButtonAction::LongPressed);
            }
            Hit::FingerUp(fe) if self.enabled && fe.is_primary_hit() => {
                if fe.is_over {
                    self.emit_click(cx, uid, fe.modifiers);
                    if self.reset_hover_on_click {
                        self.animator_cut(cx, ids!(hover.off));
                    } else if fe.has_hovers() {
                        self.animator_play(cx, ids!(hover.on));
                    } else {
                        self.animator_play(cx, ids!(hover.off));
                    }
                } else {
                    cx.widget_action_with_data(
                        &self.action_data,
                        uid,
                        ButtonAction::Released(fe.modifiers),
                    );
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.sync_managed_variant(&mut *cx);
        self.sync_active_state_if_needed(&mut *cx, Animate::No);
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text
            .draw_walk(cx, self.label_walk, Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();

        if self.grab_key_focus {
            cx.add_nav_stop(self.area, NavRole::TextInput, Inset::default());
        }

        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }

    fn set_text(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.area.redraw(cx);
    }
}

impl ShadNavButton {
    fn managed_size(&self) -> Option<ManagedButtonSize> {
        if !self.size_is_managed {
            return None;
        }

        Some(match self.size {
            ShadControlSize::Small => ManagedButtonSize {
                height: self.size_small_height,
                padding_x: self.size_small_padding_x,
                font_size: self.size_small_font_size,
            },
            ShadControlSize::Default => ManagedButtonSize {
                height: self.size_default_height,
                padding_x: self.size_default_padding_x,
                font_size: self.size_default_font_size,
            },
            ShadControlSize::Large => ManagedButtonSize {
                height: self.size_large_height,
                padding_x: self.size_large_padding_x,
                font_size: self.size_large_font_size,
            },
        })
    }

    fn managed_variant_style(&self) -> Option<ManagedButtonVariantStyle> {
        if !self.variant_is_managed {
            return None;
        }

        let transparent = vec4(0.0, 0.0, 0.0, 0.0);
        Some(match self.variant {
            ShadButtonVariant::Primary => ManagedButtonVariantStyle {
                fill: self.primary_fill,
                fill_hover: self.primary_fill_hover,
                fill_down: self.primary_fill_down,
                fill_focus: self.primary_fill_focus,
                fill_disabled: self.managed_disabled_fill,
                border_size: 0.0,
                border_radius: self.managed_radius,
                border_color: transparent,
                border_color_hover: transparent,
                border_color_down: transparent,
                border_color_focus: transparent,
                border_color_disabled: transparent,
                text: self.primary_text,
                text_hover: self.primary_text_hover,
                text_down: self.primary_text_down,
                text_focus: self.primary_text_focus,
                text_disabled: self.managed_disabled_text,
            },
            ShadButtonVariant::Secondary => ManagedButtonVariantStyle {
                fill: self.secondary_fill,
                fill_hover: self.secondary_fill_hover,
                fill_down: self.secondary_fill_down,
                fill_focus: self.secondary_fill_focus,
                fill_disabled: self.managed_disabled_fill,
                border_size: 0.0,
                border_radius: self.managed_radius,
                border_color: transparent,
                border_color_hover: transparent,
                border_color_down: transparent,
                border_color_focus: transparent,
                border_color_disabled: transparent,
                text: self.secondary_text,
                text_hover: self.secondary_text_hover,
                text_down: self.secondary_text_down,
                text_focus: self.secondary_text_focus,
                text_disabled: self.managed_disabled_text,
            },
            ShadButtonVariant::Outline => ManagedButtonVariantStyle {
                fill: self.outline_fill,
                fill_hover: self.outline_fill_hover,
                fill_down: self.outline_fill_down,
                fill_focus: self.outline_fill_focus,
                fill_disabled: self.managed_disabled_fill,
                border_size: 1.0,
                border_radius: self.managed_radius,
                border_color: self.outline_border,
                border_color_hover: self.outline_border_hover,
                border_color_down: self.outline_border_down,
                border_color_focus: self.outline_border_focus,
                border_color_disabled: self.managed_disabled_border,
                text: self.outline_text,
                text_hover: self.outline_text_hover,
                text_down: self.outline_text_down,
                text_focus: self.outline_text_focus,
                text_disabled: self.managed_disabled_text,
            },
            ShadButtonVariant::Ghost => ManagedButtonVariantStyle {
                fill: self.ghost_fill,
                fill_hover: self.ghost_fill_hover,
                fill_down: self.ghost_fill_down,
                fill_focus: self.ghost_fill_focus,
                fill_disabled: self.managed_disabled_fill,
                border_size: 0.0,
                border_radius: self.managed_radius,
                border_color: transparent,
                border_color_hover: transparent,
                border_color_down: transparent,
                border_color_focus: transparent,
                border_color_disabled: transparent,
                text: self.ghost_text,
                text_hover: self.ghost_text_hover,
                text_down: self.ghost_text_down,
                text_focus: self.ghost_text_focus,
                text_disabled: self.managed_disabled_text,
            },
            ShadButtonVariant::Destructive => ManagedButtonVariantStyle {
                fill: self.destructive_fill,
                fill_hover: self.destructive_fill_hover,
                fill_down: self.destructive_fill_down,
                fill_focus: self.destructive_fill_focus,
                fill_disabled: self.managed_disabled_fill,
                border_size: 0.0,
                border_radius: self.managed_radius,
                border_color: transparent,
                border_color_hover: transparent,
                border_color_down: transparent,
                border_color_focus: transparent,
                border_color_disabled: transparent,
                text: self.destructive_text,
                text_hover: self.destructive_text_hover,
                text_down: self.destructive_text_down,
                text_focus: self.destructive_text_focus,
                text_disabled: self.managed_disabled_text,
            },
            ShadButtonVariant::Link => ManagedButtonVariantStyle {
                fill: self.link_fill,
                fill_hover: self.link_fill_hover,
                fill_down: self.link_fill_down,
                fill_focus: self.link_fill_focus,
                fill_disabled: self.link_fill_disabled,
                border_size: 0.0,
                border_radius: self.managed_link_radius,
                border_color: transparent,
                border_color_hover: transparent,
                border_color_down: transparent,
                border_color_focus: transparent,
                border_color_disabled: transparent,
                text: self.link_text,
                text_hover: self.link_text_hover,
                text_down: self.link_text_down,
                text_focus: self.link_text_focus,
                text_disabled: self.managed_disabled_text,
            },
        })
    }

    fn sync_managed_variant(&mut self, cx: &mut Cx) {
        let Some(style) = self.managed_variant_style() else {
            return;
        };
        if self.applied_variant == Some(self.variant) {
            return;
        }
        script_apply_eval!(cx, self, {
            draw_bg +: {
                color: #(style.fill)
                color_hover: #(style.fill_hover)
                color_down: #(style.fill_down)
                color_focus: #(style.fill_focus)
                color_active: #(style.fill_focus)
                color_disabled: #(style.fill_disabled)
                border_size: #(style.border_size)
                border_radius: #(style.border_radius)
                border_color: #(style.border_color)
                border_color_hover: #(style.border_color_hover)
                border_color_down: #(style.border_color_down)
                border_color_focus: #(style.border_color_focus)
                border_color_active: #(style.border_color_focus)
                border_color_disabled: #(style.border_color_disabled)
            }
            draw_text +: {
                color: #(style.text)
                color_hover: #(style.text_hover)
                color_down: #(style.text_down)
                color_focus: #(style.text_focus)
                color_active: #(style.text_focus)
                color_disabled: #(style.text_disabled)
            }
        });
        self.applied_variant = Some(self.variant);
    }

    fn managed_padding_x(&self, default_padding_x: f64) -> f64 {
        if self.variant_is_managed && matches!(self.variant, ShadButtonVariant::Link) {
            return match self.size {
                ShadControlSize::Small => self.variant_link_small_padding_x,
                ShadControlSize::Default => self.variant_link_default_padding_x,
                ShadControlSize::Large => self.variant_link_large_padding_x,
            };
        }
        default_padding_x
    }

    fn sync_managed_size(&mut self, cx: &mut Cx) {
        let Some(size) = self.managed_size() else {
            return;
        };
        let padding_x = self.managed_padding_x(size.padding_x);

        let mut changed = false;

        if !matches!(self.walk.height, Size::Fixed(value) if (value - size.height).abs() <= f64::EPSILON) {
            self.walk.height = Size::Fixed(size.height);
            changed = true;
        }

        let current_padding = self.layout.padding;
        if (current_padding.left - padding_x).abs() > f64::EPSILON
            || (current_padding.right - padding_x).abs() > f64::EPSILON
            || current_padding.top.abs() > f64::EPSILON
            || current_padding.bottom.abs() > f64::EPSILON
        {
            self.layout.padding = Inset {
                left: padding_x,
                right: padding_x,
                top: 0.0,
                bottom: 0.0,
            };
            changed = true;
        }

        if (self.draw_text.text_style.font_size as f64 - size.font_size).abs() > f64::EPSILON {
            self.draw_text.text_style.font_size = size.font_size as f32;
            changed = true;
        }

        if changed {
            self.area.redraw(cx);
        }
    }

    fn sync_active_state_if_needed(&mut self, cx: &mut Cx, animate: Animate) {
        if self.animator_in_state(cx, ids!(active.on)) == self.active {
            return;
        }
        self.sync_active_state(cx, animate);
    }

    fn sync_active_state(&mut self, cx: &mut Cx, animate: Animate) {
        self.animator_toggle(cx, self.active, animate, ids!(active.on), ids!(active.off));
        self.area.redraw(cx);
    }

    fn emit_click(&self, cx: &mut Cx, uid: WidgetUid, modifiers: KeyModifiers) {
        cx.widget_action_with_data(&self.action_data, uid, ButtonAction::Clicked(modifiers));
        cx.widget_to_script_call(uid, NIL, self.source.clone(), self.on_click.clone(), &[]);
    }

    pub fn set_active(&mut self, cx: &mut Cx, active: bool, animate: Animate) {
        if self.active == active {
            return;
        }
        self.active = active;
        self.sync_active_state(cx, animate);
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_hovered(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(hover.on)) || self.animator_in_state(cx, ids!(hover.down))
    }

    pub fn is_down(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(hover.down))
    }

    pub fn is_focused(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(focus.on))
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

#[derive(Clone, Default)]
pub struct ShadButtonRef(pub WidgetRef);

impl std::ops::Deref for ShadButtonRef {
    type Target = WidgetRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ShadButtonRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait ShadButtonWidgetExt {
    fn shad_button(&self, cx: &Cx, path: &[LiveId]) -> ShadButtonRef;
}

impl ShadButtonWidgetExt for View {
    fn shad_button(&self, cx: &Cx, path: &[LiveId]) -> ShadButtonRef {
        ShadButtonRef(self.widget(cx, path))
    }
}

impl ShadButtonWidgetExt for WidgetRef {
    fn shad_button(&self, cx: &Cx, path: &[LiveId]) -> ShadButtonRef {
        ShadButtonRef(self.widget(cx, path))
    }
}

impl ShadButtonWidgetExt for ViewRef {
    fn shad_button(&self, cx: &Cx, path: &[LiveId]) -> ShadButtonRef {
        ShadButtonRef(self.widget(cx, path))
    }
}

impl ShadButtonRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.clicked_modifiers(actions).is_some()
    }

    pub fn pressed(&self, actions: &Actions) -> bool {
        self.pressed_modifiers(actions).is_some()
    }

    pub fn released(&self, actions: &Actions) -> bool {
        self.released_modifiers(actions).is_some()
    }

    pub fn long_pressed(&self, actions: &Actions) -> bool {
        actions
            .find_widget_action(self.0.widget_uid())
            .is_some_and(|action| matches!(action.cast(), ButtonAction::LongPressed))
    }

    pub fn clicked_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        actions
            .find_widget_action(self.0.widget_uid())
            .and_then(|action| match action.cast() {
                ButtonAction::Clicked(modifiers) => Some(modifiers),
                _ => None,
            })
    }

    pub fn pressed_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        actions
            .find_widget_action(self.0.widget_uid())
            .and_then(|action| match action.cast() {
                ButtonAction::Pressed(modifiers) => Some(modifiers),
                _ => None,
            })
    }

    pub fn released_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        actions
            .find_widget_action(self.0.widget_uid())
            .and_then(|action| match action.cast() {
                ButtonAction::Released(modifiers) => Some(modifiers),
                _ => None,
            })
    }

    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.0.borrow_mut::<ShadNavButton>() {
            inner.set_text(cx, text);
        }
    }

    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.0.borrow_mut::<ShadNavButton>() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }

    pub fn set_enabled(&self, cx: &mut Cx, enabled: bool) {
        if let Some(mut inner) = self.0.borrow_mut::<ShadNavButton>() {
            inner.enabled = enabled;
            inner.redraw(cx);
        }
    }

    pub fn reset_hover(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.0.borrow_mut::<ShadNavButton>() {
            inner.animator_cut(cx, ids!(hover.off));
        }
    }

    pub fn set_active(&self, cx: &mut Cx, active: bool, animate: Animate) {
        if let Some(mut inner) = self.0.borrow_mut::<ShadNavButton>() {
            inner.set_active(cx, active, animate);
        }
    }

    pub fn is_active(&self) -> bool {
        self.0
            .borrow::<ShadNavButton>()
            .is_some_and(|inner| inner.is_active())
    }

    pub fn is_hovered(&self, cx: &Cx) -> bool {
        self.0
            .borrow::<ShadNavButton>()
            .is_some_and(|inner| inner.is_hovered(cx))
    }

    pub fn is_down(&self, cx: &Cx) -> bool {
        self.0
            .borrow::<ShadNavButton>()
            .is_some_and(|inner| inner.is_down(cx))
    }

    pub fn is_focused(&self, cx: &Cx) -> bool {
        self.0
            .borrow::<ShadNavButton>()
            .is_some_and(|inner| inner.is_focused(cx))
    }

    pub fn is_enabled(&self) -> bool {
        self.0
            .borrow::<ShadNavButton>()
            .is_some_and(|inner| inner.is_enabled())
    }

    pub fn is_visible(&self) -> bool {
        self.0
            .borrow::<ShadNavButton>()
            .is_some_and(|inner| inner.is_visible())
    }
}
