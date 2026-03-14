use crate::internal::actions::first_widget_action;
use crate::internal::overlay::{
    button_clicked, draw_modal_overlay, modal_dismissed, set_modal_widget_open,
    sync_modal_open_state,
};
use crate::internal::script_args::bool_arg;
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

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

    // Toast with a leading check icon and a close (X) button.
    // The close button dismisses the toast when clicked.
    mod.widgets.ShadSonnerWithClose = set_type_default() do mod.widgets.ShadSonnerBase{
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
                    width: 260
                    height: Fit
                    padding: Inset{left: 14, right: 8, top: 10, bottom: 10}
                    flow: Down
                    spacing: 4.0

                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    header_row := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        spacing: 8.0

                        check_icon := mod.widgets.IconCheck{
                            icon_walk: Walk{width: 14, height: 14}
                            draw_icon.color: (shad_theme.color_primary)
                        }

                        title_label := mod.widgets.ShadToastTitle{
                            width: Fill
                            text: "Event created"
                        }

                        close_btn := mod.widgets.IconButtonX{
                            width: 24
                            height: 24
                            draw_bg +: {
                                color: #0000
                                color_hover: (shad_theme.color_ghost_hover)
                                color_down: (shad_theme.color_ghost_down)
                                border_size: 0.0
                                border_radius: (shad_theme.radius)
                            }
                            draw_icon.color: (shad_theme.color_muted_foreground)
                        }
                    }

                    description_label := mod.widgets.ShadToastDescription{
                        visible: false
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadSonnerAction {
    OpenChanged(bool),
    #[default]
    None,
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
    #[action_data]
    #[rust]
    action_data: WidgetActionData,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
}

impl ShadSonner {
    fn sync_open_state(&mut self, cx: &mut Cx) {
        sync_modal_open_state(cx, &mut self.overlay, &mut self.is_synced_open, self.open);
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        let uid = self.widget_uid();
        set_modal_widget_open(
            cx,
            &mut self.overlay,
            &mut self.open,
            &mut self.is_synced_open,
            &self.action_data,
            uid,
            open,
            ShadSonnerAction::OpenChanged,
        );
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.set_open(cx, true);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.set_open(cx, false);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(ShadSonnerAction::OpenChanged(open)) =
            first_widget_action::<ShadSonnerAction>(actions, self.widget_uid())
        {
            return Some(open);
        }
        None
    }
}

impl Widget for ShadSonner {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_open) {
            if let Some(open) = bool_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_open(cx, open));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(is_open) {
            return ScriptAsyncResult::Return(ScriptValue::from_bool(self.open));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_open_state(cx);

        if self.open {
            self.overlay.handle_event(cx, event, scope);
            if let Event::Actions(actions) = event {
                if modal_dismissed(&self.overlay, cx, actions) {
                    self.close(cx);
                }
                if button_clicked(
                    &self.overlay,
                    cx,
                    &[
                        live_id!(content),
                        live_id!(toast_panel),
                        live_id!(header_row),
                        live_id!(close_btn),
                    ],
                    actions,
                ) {
                    self.close(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_open_state(cx);
        draw_modal_overlay(cx, scope, walk, self.layout, self.open, &mut self.overlay)
    }
}

impl ShadSonnerRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }
}
