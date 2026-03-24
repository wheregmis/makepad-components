use makepad_widgets::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Script, ScriptHook)]
#[repr(u32)]
pub enum ShadAlertTone {
    #[pick]
    #[default]
    Default,
    Destructive,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ShadAlertTone = set_type_default() do #(ShadAlertTone::script_api(vm))
    mod.widgets.ShadAlertTone = ShadAlertTone

    mod.widgets.ShadAlertHeader = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.0}
        spacing: 12.0
    }

    mod.widgets.ShadAlertIcon = mod.widgets.IconInfo{
        width: 16
        height: 16
        icon_walk: Walk{width: 16, height: 16}
        draw_icon.color: (shad_theme.color_primary)
    }

    mod.widgets.ShadAlertContent = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        align: Align{x: 0.0, y: 0.0}
        spacing: 6.0
    }

    mod.widgets.ShadAlertTitle = mod.widgets.Label{
        width: Fill
        padding: 0.
        align: Align{x: 0.0, y: 0.0}
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
        draw_text.text_style.line_spacing: 1.0
    }

    mod.widgets.ShadAlertDescription = mod.widgets.Label{
        width: Fill
        padding: 0.
        align: Align{x: 0.0, y: 0.0}
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10
        draw_text.text_style.line_spacing: 1.2
    }

    mod.widgets.ShadAlertBase = #(ShadAlert::register_widget(vm))
    mod.widgets.ShadAlert = set_type_default() do mod.widgets.ShadAlertBase{
        width: Fill
        height: Fit
        tone: ShadAlertTone.Default
        title_text: ""
        description_text: ""
        default_border_color: (shad_theme.color_outline_border)
        default_icon_color: (shad_theme.color_primary)
        default_title_color: (shad_theme.color_primary)
        default_description_color: (shad_theme.color_muted_foreground)
        destructive_border_color: (shad_theme.color_destructive)
        destructive_icon_color: (shad_theme.color_destructive)
        destructive_title_color: (shad_theme.color_destructive)
        destructive_description_color: (shad_theme.color_destructive)

        body := mod.widgets.RoundedView{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.0}
            spacing: 12.0
            padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

            draw_bg +: {
                color: #0000
                border_size: 1.0
                border_radius: (shad_theme.radius)
                border_color: (shad_theme.color_outline_border)
            }

            icon := mod.widgets.ShadAlertIcon{}

            content := mod.widgets.ShadAlertContent{
                title := mod.widgets.ShadAlertTitle{
                    text: "Alert title"
                }
                description := mod.widgets.ShadAlertDescription{
                    text: "Alert description"
                }
            }
        }
    }

}

#[derive(Script, Widget)]
pub struct ShadAlert {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live(ShadAlertTone::Default)]
    tone: ShadAlertTone,
    #[live]
    title_text: ArcStringMut,
    #[live]
    description_text: ArcStringMut,
    #[live]
    default_border_color: Vec4,
    #[live]
    default_icon_color: Vec4,
    #[live]
    default_title_color: Vec4,
    #[live]
    default_description_color: Vec4,
    #[live]
    destructive_border_color: Vec4,
    #[live]
    destructive_icon_color: Vec4,
    #[live]
    destructive_title_color: Vec4,
    #[live]
    destructive_description_color: Vec4,
}

impl ScriptHook for ShadAlert {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        let title_text = self.title_text.as_ref().to_string();
        let description_text = self.description_text.as_ref().to_string();
        let (border_color, icon_color, title_color, description_color) = match self.tone {
            ShadAlertTone::Default => (
                self.default_border_color,
                self.default_icon_color,
                self.default_title_color,
                self.default_description_color,
            ),
            ShadAlertTone::Destructive => (
                self.destructive_border_color,
                self.destructive_icon_color,
                self.destructive_title_color,
                self.destructive_description_color,
            ),
        };
        vm.with_cx_mut(|cx| {
            let mut body = self.view.widget(cx, ids!(body));
            script_apply_eval!(cx, body, {
                draw_bg +: {
                    border_color: #(border_color)
                }
            });

            let mut icon = self.view.widget(cx, ids!(body.icon));
            script_apply_eval!(cx, icon, {
                draw_icon.color: #(icon_color)
            });

            let mut title = self.view.widget(cx, ids!(body.content.title));
            if !title_text.is_empty() {
                script_apply_eval!(cx, title, {
                    text: #(title_text.clone())
                    draw_text.color: #(title_color)
                });
            } else {
                script_apply_eval!(cx, title, {
                    draw_text.color: #(title_color)
                });
            }

            let mut description = self.view.widget(cx, ids!(body.content.description));
            if !description_text.is_empty() {
                script_apply_eval!(cx, description, {
                    text: #(description_text.clone())
                    draw_text.color: #(description_color)
                });
            } else {
                script_apply_eval!(cx, description, {
                    draw_text.color: #(description_color)
                });
            }
        });
    }
}

impl Widget for ShadAlert {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
