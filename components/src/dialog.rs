use makepad_widgets::*;

use crate::overlay_base::{
    handle_overlay_close_button, handle_overlay_dismissed, handle_overlay_script_call,
    sync_overlay_open,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadDialogBase = #(ShadDialog::register_widget(vm))

    mod.widgets.ShadDialog = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 360
                height: Fit

                body := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 16}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }
                }
            }
        }
    }

    mod.widgets.ShadDialogAlert = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 360
                height: Fit

                dialog_panel := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 16}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    dialog_body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        margin: Inset{left: 20, right: 20}
                        title_label := ShadAlertTitle{
                            text: "Are you absolutely sure?"
                        }
                        description_label := ShadAlertDescription{
                            text: "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        margin: Inset{top: 8}

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
                            text: "Continue"
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

    mod.widgets.ShadDialogAlertDestructive = set_type_default() do mod.widgets.ShadDialogBase{
        width: Fill
        height: Fit
        open: false

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.55)
            }

            content +: {
                width: 360
                height: Fit

                dialog_panel := RoundedView{
                    width: Fill
                    height: Fit
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 16}
                    flow: Down
                    spacing: 12.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    dialog_body := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                        margin: Inset{left: 20, right: 20}
                        title_label := ShadAlertTitle{
                            text: "Are you absolutely sure?"
                        }
                        description_label := ShadAlertDescription{
                            text: "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        margin: Inset{top: 8}

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
                            text: "Delete"
                            height: 36
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
                            draw_bg +: {
                                color: (shad_theme.color_destructive)
                                color_hover: (shad_theme.color_destructive_hover)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                            }
                            draw_text.color: (shad_theme.color_destructive_foreground)
                            draw_text.text_style.font_size: 11
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadDialog {
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

impl ShadDialog {
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

impl Widget for ShadDialog {
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
            // Close when modal is dismissed (backdrop/Escape) or when cancel/confirm clicked
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
                        live_id!(dialog_panel),
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
                        live_id!(dialog_panel),
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
