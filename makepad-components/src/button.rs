use makepad_widgets::makepad_script::ScriptFnRef;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadNavButtonBase = set_type_default() do #(ShadNavButton::register_widget(vm)){
        text: "Button"
        width: Fit
        height: Fit
        spacing: theme.space_2
        align: Center
        padding: theme.mspace_1{left: theme.space_2, right: theme.space_2}
        margin: theme.mspace_v_1
        label_walk: Walk{width: Fit, height: Fit}

        draw_text +: {
            hover: 0.0
            down: instance(0.0)
            focus: instance(0.0)
            disabled: instance(0.0)

            color: theme.color_label_inner
            color_hover: theme.color_label_inner_hover
            color_down: uniform(theme.color_label_inner_down)
            color_focus: uniform(theme.color_label_inner_focus)
            color_disabled: uniform(theme.color_label_inner_disabled)

            text_style: theme.font_regular{
                font_size: theme.font_size_p
            }
            get_color: fn() {
                return self.color
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
            color_disabled: uniform(theme.color_outset_disabled)

            color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            color_2_hover: uniform(theme.color_outset_2_hover)
            color_2_down: uniform(theme.color_outset_2_down)
            color_2_focus: uniform(theme.color_outset_2_focus)
            color_2_disabled: uniform(theme.color_outset_2_disabled)

            border_color: uniform(theme.color_bevel)
            border_color_hover: uniform(theme.color_bevel_hover)
            border_color_down: uniform(theme.color_bevel_down)
            border_color_focus: uniform(theme.color_bevel_focus)
            border_color_disabled: uniform(theme.color_bevel_disabled)

            border_color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            border_color_2_hover: uniform(theme.color_bevel_outset_2_hover)
            border_color_2_down: uniform(theme.color_bevel_outset_2_down)
            border_color_2_focus: uniform(theme.color_bevel_outset_2_focus)
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
                    color_fill_disabled = mix(self.color_disabled, self.color_2_disabled, dir)
                }

                let mut color_stroke = self.border_color
                let mut color_stroke_hover = self.border_color_hover
                let mut color_stroke_down = self.border_color_down
                let mut color_stroke_focus = self.border_color_focus
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
                    color_stroke_disabled = mix(self.border_color_disabled, self.border_color_2_disabled, dir)
                }

                let fill = color_fill
                    .mix(color_fill_focus, self.focus)
                    .mix(color_fill_hover, self.hover)
                    .mix(color_fill_down, self.down)
                    .mix(color_fill_disabled, self.disabled)

                let stroke = color_stroke
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
        }
    }

    mod.widgets.ShadButton = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_primary_foreground)
            color_hover: (shad_theme.color_secondary)
            color_down: (shad_theme.color_secondary_hover)
            color_focus: (shad_theme.color_secondary)
            color_disabled: (shad_theme.color_disabled)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.color_disabled: (shad_theme.color_disabled_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonDestructive = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_destructive)
            color_hover: (shad_theme.color_destructive_hover)
            color_down: (shad_theme.color_destructive_down)
            color_focus: (shad_theme.color_destructive_hover)
            color_disabled: (shad_theme.color_disabled)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_destructive_foreground)
        draw_text.color_hover: (shad_theme.color_destructive_foreground)
        draw_text.color_down: (shad_theme.color_destructive_foreground)
        draw_text.color_focus: (shad_theme.color_destructive_foreground)
        draw_text.color_disabled: (shad_theme.color_disabled_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonOutline = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
            color_disabled: (shad_theme.color_disabled)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_focus: (shad_theme.color_outline_border_hover)
            border_color_disabled: (shad_theme.color_disabled_border)
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.color_disabled: (shad_theme.color_disabled_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonSecondary = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_secondary)
            color_hover: (shad_theme.color_secondary_hover)
            color_down: (shad_theme.color_secondary_down)
            color_focus: (shad_theme.color_secondary_hover)
            color_disabled: (shad_theme.color_disabled)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_secondary_foreground)
        draw_text.color_hover: (shad_theme.color_secondary_foreground)
        draw_text.color_down: (shad_theme.color_secondary_foreground)
        draw_text.color_focus: (shad_theme.color_secondary_foreground)
        draw_text.color_disabled: (shad_theme.color_disabled_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonGhost = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
            color_disabled: (shad_theme.color_disabled)
            border_size: 0.0
            border_radius: (shad_theme.radius)
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.color_disabled: (shad_theme.color_disabled_foreground)
        draw_text.text_style.font_size: 11
    }

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

    mod.widgets.ShadButtonLink = mod.widgets.ButtonFlat{
        height: 36
        padding: Inset{left: 4, right: 4, top: 0, bottom: 0}
        draw_bg +: {
            color: #0000
            color_hover: #0000
            color_down: #0000
            color_focus: #0000
            color_disabled: #0000
            border_size: 0.0
            border_radius: 0.0
            border_color: #0000
        }
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary_down)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.color_disabled: (shad_theme.color_disabled_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadButtonSm = mod.widgets.ShadButton{
        height: 28
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadButtonLg = mod.widgets.ShadButton{
        height: 44
        padding: Inset{left: 32, right: 32, top: 0, bottom: 0}
        draw_text.text_style.font_size: 13
    }

    mod.widgets.ShadButtonIcon = mod.widgets.ShadButton{
        width: 36
        height: 36
        spacing: 0.0
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }
}

#[derive(Script, ScriptHook, Widget, Animator)]
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
    #[live(true)]
    #[visible]
    visible: bool,
    #[live]
    reset_hover_on_click: bool,
    #[live]
    text: ArcStringMut,
    #[live]
    on_click: ScriptFnRef,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
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
            return ScriptAsyncResult::Return(str_val.into());
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
    fn emit_click(&self, cx: &mut Cx, uid: WidgetUid, modifiers: KeyModifiers) {
        cx.widget_action_with_data(&self.action_data, uid, ButtonAction::Clicked(modifiers));
        cx.widget_to_script_call(uid, NIL, self.source.clone(), self.on_click.clone(), &[]);
    }
}
