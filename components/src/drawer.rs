use makepad_widgets::*;

use crate::overlay_base::{
    handle_overlay_close_button, handle_overlay_dismissed, handle_overlay_script_call,
    sync_overlay_open,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadDrawerBase = #(ShadDrawer::register_widget(vm))

    mod.widgets.ShadDrawer = set_type_default() do mod.widgets.ShadDrawerBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            align: Align{x: 0.5, y: 0.0}

            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 640
                height: Fit
                margin: Inset{top: 0, right: 0, bottom: 0, left: 0}

                drawer_panel := RoundedView{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 0.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    drawer_body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        padding: Inset{left: 20, right: 20, top: 20, bottom: 16}

                        drawer_handle := View{
                            width: Fill
                            height: Fit
                            align: Center
                            handle := RoundedView{
                                width: 40
                                height: 4
                                draw_bg +: {
                                    color: (shad_theme.color_outline_border)
                                    border_radius: 2.0
                                }
                            }
                        }

                        title_label := ShadAlertTitle{
                            text: "Edit profile"
                        }
                        description_label := ShadAlertDescription{
                            text: "Make changes to your profile here. Click submit when you're done."
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}

                        cancel := ButtonFlat{
                            text: "Cancel"
                            height: 36
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
                            draw_bg +: {
                                color: #0000
                                color_hover: (shad_theme.color_ghost_hover)
                                color_down: (shad_theme.color_ghost_down)
                                border_size: 1.0
                                border_radius: (shad_theme.radius)
                                border_color: (shad_theme.color_outline_border)
                            }
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 11
                        }

                        confirm := ButtonFlat{
                            text: "Submit"
                            height: 36
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
                            draw_bg +: {
                                color: (shad_theme.color_primary_foreground)
                                color_hover: (shad_theme.color_secondary)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                            }
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 11
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadDrawer {
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

impl ShadDrawer {
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

impl Widget for ShadDrawer {
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
            // Close when Cancel/Confirm is clicked or modal is dismissed (backdrop/Escape)
            if let Event::Actions(actions) = event {
                handle_overlay_dismissed(
                    &mut self.open,
                    &mut self.is_synced_open,
                    &self.overlay,
                    cx,
                    actions,
                );
                handle_overlay_close_button(
                    &mut self.open,
                    &mut self.is_synced_open,
                    &self.overlay,
                    cx,
                    actions,
                    &[
                        live_id!(content),
                        live_id!(drawer_panel),
                        live_id!(footer),
                        live_id!(cancel),
                    ],
                );
                handle_overlay_close_button(
                    &mut self.open,
                    &mut self.is_synced_open,
                    &self.overlay,
                    cx,
                    actions,
                    &[
                        live_id!(content),
                        live_id!(drawer_panel),
                        live_id!(footer),
                        live_id!(confirm),
                    ],
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
