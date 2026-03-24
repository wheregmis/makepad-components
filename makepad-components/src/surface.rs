use makepad_widgets::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Script, ScriptHook)]
#[repr(u32)]
pub enum ShadSurfaceVariant {
    #[pick]
    #[default]
    Default,
    Muted,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ShadSurfaceVariant = set_type_default() do #(ShadSurfaceVariant::script_api(vm))
    mod.widgets.ShadSurfaceVariant = ShadSurfaceVariant

    mod.widgets.ShadSurfaceBase = #(ShadSurface::register_widget(vm))
    mod.widgets.ShadSurface = set_type_default() do mod.widgets.ShadSurfaceBase{
        width: Fill
        height: Fit
        variant: ShadSurfaceVariant.Default
        default_color: (shad_theme.color_background)
        muted_color: (shad_theme.color_muted)
        border_color: (shad_theme.color_outline_border)

        body := mod.widgets.RoundedView{
            width: Fill
            height: Fit

            draw_bg +: {
                border_radius: (shad_theme.radius)
                border_size: 1.0
                border_color: (shad_theme.color_outline_border)
            }
        }
    }

    mod.widgets.ShadSurfaceMuted = mod.widgets.ShadSurface{
        variant: ShadSurfaceVariant.Muted
    }

    mod.widgets.ShadMediaFrame = mod.widgets.ShadSurface{
        width: Fill
        height: Fill
        flow: Overlay
        clip_x: true
        clip_y: true
        variant: ShadSurfaceVariant.Muted
    }
}

#[derive(Script, Widget)]
pub struct ShadSurface {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live(ShadSurfaceVariant::Default)]
    variant: ShadSurfaceVariant,
    #[live]
    default_color: Vec4,
    #[live]
    muted_color: Vec4,
    #[live]
    border_color: Vec4,
}

impl ScriptHook for ShadSurface {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        let color = match self.variant {
            ShadSurfaceVariant::Default => self.default_color,
            ShadSurfaceVariant::Muted => self.muted_color,
        };
        vm.with_cx_mut(|cx| {
            let mut body = self.view.widget(cx, ids!(body));
            script_apply_eval!(cx, body, {
                draw_bg +: {
                    color: #(color)
                    border_color: #(self.border_color)
                }
            });
        });
    }
}

impl Widget for ShadSurface {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
