use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Script, ScriptHook)]
#[repr(u32)]
pub enum ShadBadgeTone {
    #[pick]
    #[default]
    Default,
    Secondary,
    Destructive,
    Success,
    Warning,
    Outline,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ShadBadgeTone = set_type_default() do #(ShadBadgeTone::script_api(vm))
    mod.widgets.ShadBadgeTone = ShadBadgeTone

    mod.widgets.ShadBadgeLabel = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.text_style.font_size: 9
        text: "Badge"
    }

    mod.widgets.ShadBadgeBase = #(ShadBadge::register_widget(vm))
    mod.widgets.ShadBadge = set_type_default() do mod.widgets.ShadBadgeBase{
        width: Fit
        height: Fit
        tone: ShadBadgeTone.Default
        text: ""
        default_color: (shad_theme.color_primary)
        default_label_color: (shad_theme.color_primary_foreground)
        secondary_color: (shad_theme.color_secondary)
        secondary_label_color: (shad_theme.color_secondary_foreground)
        destructive_color: (shad_theme.color_destructive)
        destructive_label_color: (shad_theme.color_destructive_foreground)
        success_color: (shad_theme.color_success_subtle)
        success_label_color: (shad_theme.color_success)
        warning_color: (shad_theme.color_warning_subtle)
        warning_label_color: (shad_theme.color_warning)
        outline_color: #0000
        outline_label_color: (shad_theme.color_muted_foreground)
        outline_border_color: (shad_theme.color_outline_border_hover)

        body := mod.widgets.RoundedView{
            width: Fit
            height: Fit
            flow: Right
            align: Align{x: 0.5, y: 0.5}
            padding: Inset{left: 8, right: 8, top: 3, bottom: 3}

            draw_bg +: {
                color: (shad_theme.color_primary)
                border_radius: 5.0
                border_size: 0.0
                border_color: #0000
            }

            label := mod.widgets.ShadBadgeLabel{
                text: "Badge"
            }
        }
    }

}

#[derive(Script, Widget)]
pub struct ShadBadge {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live(ShadBadgeTone::Default)]
    tone: ShadBadgeTone,
    #[live]
    text: ArcStringMut,
    #[live]
    default_color: Vec4,
    #[live]
    default_label_color: Vec4,
    #[live]
    secondary_color: Vec4,
    #[live]
    secondary_label_color: Vec4,
    #[live]
    destructive_color: Vec4,
    #[live]
    destructive_label_color: Vec4,
    #[live]
    success_color: Vec4,
    #[live]
    success_label_color: Vec4,
    #[live]
    warning_color: Vec4,
    #[live]
    warning_label_color: Vec4,
    #[live]
    outline_color: Vec4,
    #[live]
    outline_label_color: Vec4,
    #[live]
    outline_border_color: Vec4,
    #[action_data]
    #[rust]
    _action_data: WidgetActionData,
}

impl ScriptHook for ShadBadge {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        let text = self.text.as_ref().to_string();
        let (color, label_color, border_size, border_color) = match self.tone {
            ShadBadgeTone::Default => (
                self.default_color,
                self.default_label_color,
                0.0,
                vec4(0.0, 0.0, 0.0, 0.0),
            ),
            ShadBadgeTone::Secondary => (
                self.secondary_color,
                self.secondary_label_color,
                0.0,
                vec4(0.0, 0.0, 0.0, 0.0),
            ),
            ShadBadgeTone::Destructive => (
                self.destructive_color,
                self.destructive_label_color,
                0.0,
                vec4(0.0, 0.0, 0.0, 0.0),
            ),
            ShadBadgeTone::Success => (
                self.success_color,
                self.success_label_color,
                1.0,
                vec4(0.0, 0.0, 0.0, 0.0),
            ),
            ShadBadgeTone::Warning => (
                self.warning_color,
                self.warning_label_color,
                1.0,
                vec4(0.0, 0.0, 0.0, 0.0),
            ),
            ShadBadgeTone::Outline => (
                self.outline_color,
                self.outline_label_color,
                1.0,
                self.outline_border_color,
            ),
        };
        vm.with_cx_mut(|cx| {
            let mut body = self.view.widget(cx, ids!(body));
            script_apply_eval!(cx, body, {
                draw_bg +: {
                    color: #(color)
                    border_size: #(border_size)
                    border_color: #(border_color)
                }
            });

            let mut label = self.view.widget(cx, ids!(body.label));
            script_apply_eval!(cx, label, {
                text: #(text.clone())
                draw_text +: {
                    color: #(label_color)
                }
            });
        });
    }
}

impl Widget for ShadBadge {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
