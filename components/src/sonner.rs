use makepad_widgets::*;

use crate::overlay_base::{
    handle_overlay_dismissed, handle_overlay_script_call, sync_overlay_open,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadToast = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
        flow: Down
        spacing: 4.0

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadToastTitle = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 12
    }

    mod.widgets.ShadToastDescription = mod.widgets.ShadAlertDescription{
        width: Fit
        height: Fit
    }

    mod.widgets.ShadSonnerBase = #(ShadSonner::register_widget(vm))

    mod.widgets.ShadSonner = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            align: Align{x: 1.0 y: 0.0}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.0)
            }

            content +: {
                width: Fit
                height: Fit
                margin: Inset{top: 16, right: 16}

                toast_panel := RoundedView{
                    width: Fit
                    height: Fit
                    padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
                    flow: Down
                    spacing: 4.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    body := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        title_label := mod.widgets.ShadToastTitle{
                            text: "Event created"
                        }
                        description_label := mod.widgets.ShadToastDescription{
                            text: ""
                            visible: false
                        }
                    }
                }
            }
        }
    }

    mod.widgets.ShadSonnerWithDescription = set_type_default() do mod.widgets.ShadSonnerBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            align: Align{x: 1.0 y: 0.0}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.0)
            }

            content +: {
                width: Fit
                height: Fit
                margin: Inset{top: 16, right: 16}

                toast_panel := RoundedView{
                    width: Fit
                    height: Fit
                    padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
                    flow: Down
                    spacing: 4.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    body := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 4.0

                        title_label := mod.widgets.ShadToastTitle{
                            text: "Toast with description"
                        }
                        description_label := mod.widgets.ShadToastDescription{
                            text: "Your changes have been saved."
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadSonner {
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

    #[rust]
    is_synced_open: bool,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadSonner {
    fn sync_open_state(&mut self, cx: &mut Cx) {
        sync_overlay_open(self.open, &mut self.is_synced_open, &self.overlay, cx);
    }

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Widget for ShadSonner {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        handle_overlay_script_call(
            &mut self.open,
            &mut self.is_synced_open,
            &self.overlay,
            vm,
            method,
            args,
        )
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_open_state(cx);

        if self.open {
            self.overlay.handle_event(cx, event, scope);
            if let Event::Actions(actions) = event {
                handle_overlay_dismissed(
                    &mut self.open,
                    &mut self.is_synced_open,
                    &self.overlay,
                    cx,
                    actions,
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_open_state(cx);

        if !self.open {
            return DrawStep::done();
        }
        cx.begin_turtle(walk, self.layout);
        let step = self
            .overlay
            .draw_walk(cx, scope, Walk::new(Size::fill(), Size::fill()));
        cx.end_turtle();
        step
    }
}
