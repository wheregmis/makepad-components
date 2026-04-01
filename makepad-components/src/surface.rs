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
        default_color: (shad_theme.color_popover)
        muted_color: (shad_theme.color_muted)
        border_color: (shad_theme.color_outline_border)
        radius: (shad_theme.radius)
        border_size: (shad_theme.border_size)

        body := mod.widgets.RoundedView{
            width: Fill
            height: Fit

            draw_bg +: {
                border_radius: (shad_theme.radius)
                border_size: (shad_theme.border_size)
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
    #[live]
    radius: f64,
    #[live]
    border_size: f64,
}

#[derive(Default)]
struct SurfaceDrawBgOverrides {
    color: Option<Vec4f>,
    border_color: Option<Vec4f>,
    border_radius: Option<f64>,
    border_size: Option<f64>,
}

fn script_number(value: ScriptValue) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_f32().map(|v| v as f64))
        .or_else(|| value.as_u40().map(|v| v as f64))
}

fn source_draw_bg_overrides(vm: &ScriptVm, value: ScriptValue) -> SurfaceDrawBgOverrides {
    let Some(mut current) = value.as_object() else {
        return SurfaceDrawBgOverrides::default();
    };

    loop {
        let draw_bg = vm
            .bx
            .heap
            .map_ref(current)
            .get(&id!(draw_bg).into())
            .map(|entry| entry.value);

        if let Some(draw_bg) = draw_bg {
            let Some(draw_bg_obj) = draw_bg.as_object() else {
                return SurfaceDrawBgOverrides::default();
            };
            let draw_bg_map = vm.bx.heap.map_ref(draw_bg_obj);
            return SurfaceDrawBgOverrides {
                color: draw_bg_map
                    .get(&id!(color).into())
                    .and_then(|entry| entry.value.as_color().map(Vec4f::from_u32)),
                border_color: draw_bg_map
                    .get(&id!(border_color).into())
                    .and_then(|entry| entry.value.as_color().map(Vec4f::from_u32)),
                border_radius: draw_bg_map
                    .get(&id!(border_radius).into())
                    .and_then(|entry| script_number(entry.value)),
                border_size: draw_bg_map
                    .get(&id!(border_size).into())
                    .and_then(|entry| script_number(entry.value)),
            };
        }

        let Some(proto) = vm.bx.heap.proto(current).as_object() else {
            return SurfaceDrawBgOverrides::default();
        };
        current = proto;
    }
}

impl ScriptHook for ShadSurface {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        value: ScriptValue,
    ) {
        let default_color = match self.variant {
            ShadSurfaceVariant::Default => self.default_color,
            ShadSurfaceVariant::Muted => self.muted_color,
        };
        let draw_bg_overrides = source_draw_bg_overrides(vm, value);
        let color = draw_bg_overrides.color.unwrap_or(default_color);
        let border_color = draw_bg_overrides.border_color.unwrap_or(self.border_color);
        let border_radius = draw_bg_overrides.border_radius.unwrap_or(self.radius);
        let border_size = draw_bg_overrides.border_size.unwrap_or(self.border_size);
        vm.with_cx_mut(|cx| {
            let mut body = self.view.widget(cx, ids!(body));
            script_apply_eval!(cx, body, {
                draw_bg +: {
                    color: #(color)
                    border_color: #(border_color)
                    border_radius: #(border_radius)
                    border_size: #(border_size)
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
