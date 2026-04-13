use crate::internal::touch::is_primary_tap;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadToggleWidgetBase = set_type_default() do #(ShadToggleWidget::register_widget(vm)){
        text: ""
        width: Fit
        height: Fit
        grab_key_focus: true
        label_walk: Walk{width: Fit, height: Fit}
        label_align: Align{x: 0.5, y: 0.5}

        draw_bg +: {
            hover: instance(0.0)
            down: instance(0.0)
            disabled: instance(0.0)
            focus: instance(0.0)
            active: instance(0.0)
        }

        draw_text +: {
            hover: instance(0.0)
            down: instance(0.0)
            disabled: instance(0.0)
            focus: instance(0.0)
            active: instance(0.0)
            text_style: theme.font_regular{font_size: theme.font_size_p}
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
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {active: 0.0}
                        draw_text: {active: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {active: 1.0}
                        draw_text: {active: 1.0}
                    }
                }
            }
        }
    }

    mod.widgets.ShadToggle = mod.widgets.ShadToggleWidgetBase{
        width: Fit
        height: 36
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        align: Align{x: 0.5, y: 0.5}
        label_walk: Walk{
            width: Fit
            height: Fit
            margin: Inset{left: 0, right: 0, top: 0, bottom: 0}
        }
        label_align: Align{x: 0.5, y: 0.5}

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0

            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: #0000
            color_active: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted)

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_focus: (shad_theme.color_outline_border_hover)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)

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
                    sdf.stroke(self.border_color_focus, 2.0)
                }
                return sdf.result
            }
        }

        draw_text +: {
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

    mod.widgets.ShadToggleSm = mod.widgets.ShadToggle{
        height: 28
        padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadToggleLg = mod.widgets.ShadToggle{
        height: 44
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_text.text_style.font_size: 12
    }

    mod.widgets.ShadToggleGroup = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 4.0
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadToggleGroupItem = mod.widgets.ShadToggle{
        draw_bg +: {
            border_size: 0.0
            border_color: #0000
            border_color_hover: #0000
            border_color_down: #0000
            border_color_focus: #0000
            border_color_active: #0000
            border_color_disabled: #0000
        }
    }

    mod.widgets.ShadToggleGroupItemSm = mod.widgets.ShadToggleGroupItem{
        height: 28
        padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadToggleGroupItemLg = mod.widgets.ShadToggleGroupItem{
        height: 44
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_text.text_style.font_size: 12
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadToggleAction {
    Changed(bool),
    #[default]
    None,
}

#[derive(Script, Widget, Animator)]
pub struct ShadToggleWidget {
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
    #[live]
    label_align: Align,

    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live(true)]
    grab_key_focus: bool,
    #[live(true)]
    enabled: bool,

    #[live]
    active: Option<bool>,

    #[live]
    text: ArcStringMut,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadToggleWidget {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        if let Some(active) = self.active.take() {
            vm.with_cx_mut(|cx| {
                self.animator_toggle(cx, active, Animate::No, ids!(active.on), ids!(active.off));
            });
        }
    }
}

impl Widget for ShadToggleWidget {
    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.enabled = !disabled;
        self.animator_toggle(
            cx,
            disabled,
            Animate::Yes,
            ids!(disabled.on),
            ids!(disabled.off),
        );
    }

    fn disabled(&self, _cx: &Cx) -> bool {
        !self.enabled
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
                self.toggle_active(cx, uid);
            }
            Hit::FingerDown(fe) if self.enabled && fe.is_primary_hit() => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area);
                }
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
            Hit::FingerUp(fe) if self.enabled && fe.is_primary_hit() => {
                if is_primary_tap(&fe) {
                    self.toggle_active(cx, uid);
                    if fe.has_hovers() {
                        self.animator_play(cx, ids!(hover.on));
                    } else {
                        self.animator_play(cx, ids!(hover.off));
                    }
                } else {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text
            .draw_walk(cx, self.label_walk, self.label_align, self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl ShadToggleWidget {
    fn toggle_active(&mut self, cx: &mut Cx, uid: WidgetUid) {
        let is_active = self.animator_in_state(cx, ids!(active.on));
        let new_active = !is_active;
        self.animator_toggle(cx, new_active, Animate::Yes, ids!(active.on), ids!(active.off));
        cx.widget_action_with_data(&self.action_data, uid, ShadToggleAction::Changed(new_active));
    }
}
