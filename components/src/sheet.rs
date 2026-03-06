use makepad_widgets::*;

use crate::overlay_base::{
    handle_overlay_dismissed, handle_overlay_script_call, sync_overlay_open,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSheetTitle = mod.widgets.ShadAlertTitle{}
    mod.widgets.ShadSheetDescription = mod.widgets.ShadAlertDescription{}

    mod.widgets.ShadSheetBase = #(ShadSheet::register_widget(vm))

    mod.widgets.ShadSheet = set_type_default() do mod.widgets.ShadSheetBase{
        width: Fill
        height: Fit
        open: false
        side: "right"
        sheet_size: 420.0

        overlay: Modal{
            align: Align{x: 1.0, y: 0.0}
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 420
                height: Fill
                flow: Down

                sheet_frame := RoundedView{
                    width: Fill
                    height: Fill
                    flow: Down

                    draw_bg +: {
                        color: (shad_theme.color_background)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    header := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 6.0
                        padding: Inset{left: 20, right: 20, top: 20, bottom: 12}

                        title := ShadAlertTitle{
                            text: "Edit profile"
                        }

                        description := ShadAlertDescription{
                            text: "Make changes to your profile here."
                        }
                    }

                    body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 16}
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
                        align: Align{x: 1.0, y: 0.5}
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadSheet {
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
    #[live]
    side: ArcStringMut,
    #[live]
    sheet_size: f64,

    #[rust]
    is_synced_open: bool,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadSheet {
    fn sync_side_layout(&mut self, cx: &mut Cx) {
        let side = self.side.as_ref();
        let (align, content_width, content_height) = match side {
            "left" => (
                Align { x: 0.0, y: 0.0 },
                Size::Fixed(self.sheet_size),
                Size::fill(),
            ),
            "top" => (
                Align { x: 0.0, y: 0.0 },
                Size::fill(),
                Size::Fixed(self.sheet_size),
            ),
            "bottom" => (
                Align { x: 0.0, y: 1.0 },
                Size::fill(),
                Size::Fixed(self.sheet_size),
            ),
            _ => (
                Align { x: 1.0, y: 0.0 },
                Size::Fixed(self.sheet_size),
                Size::fill(),
            ),
        };

        let mut overlay = self.overlay.clone();
        script_apply_eval!(cx, overlay, {
            align: #(align)
        });

        let mut content = self.overlay.widget(cx, ids!(content));
        script_apply_eval!(cx, content, {
            width: #(content_width)
            height: #(content_height)
        });

        let mut sheet_frame = self.overlay.widget(cx, ids!(content.sheet_frame));
        script_apply_eval!(cx, sheet_frame, {
            width: #(Size::fill())
            height: #(Size::fill())
        });
    }

    fn sync_open_state(&mut self, cx: &mut Cx) {
        self.sync_side_layout(cx);
        sync_overlay_open(self.open, &mut self.is_synced_open, &self.overlay, cx);
    }

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Widget for ShadSheet {
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
