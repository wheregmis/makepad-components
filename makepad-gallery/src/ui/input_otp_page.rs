use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::input_otp::ShadInputOtpWidgetExt;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryInputOtpPage,
    page: input_otp_page,
    title: "Input OTP",
    subtitle: "Segmented one-time passcode entry with numeric filtering and paste support. Read partial updates with changed(actions) and final codes with completed(actions).",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Verification code" }

        View{
            width: Fit
            height: Fit
            flow: Down
            spacing: 8.0

            ShadLabel{text: "Enter the 6-digit code"}
            otp_demo := ShadInputOtp{}
            otp_status := ShadFieldDescription{
                text: "Waiting for input."
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Use changed(actions) for partial entry so the page can update validation or helper text as the user types."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use completed(actions) when the full code is available and the feature should verify or submit it."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use set_value(cx, ...) to restore saved codes or clear the control from outside, and value() to inspect the current state."}
        mod.widgets.GalleryActionFlowStep{text: "4. Numeric filtering and paste handling stay inside the component, so the page only reacts to semantic values."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryInputOtpPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryInputOtpPage {
    fn sync_status(&mut self, cx: &mut Cx, actions: &Actions) {
        let otp_demo = self.view.shad_input_otp(cx, ids!(otp_demo));
        let status = if let Some(value) = otp_demo.completed(actions) {
            format!("Completed: {}", value)
        } else if let Some(value) = otp_demo.changed(actions) {
            format!("Current value: {}", value)
        } else if let Some(current_value) = otp_demo.value() {
            if current_value.len() >= 6 {
                format!("Completed: {}", current_value)
            } else if !current_value.is_empty() {
                format!("Current value: {}", current_value)
            } else {
                "Waiting for input.".to_string()
            }
        } else {
            "Waiting for input.".to_string()
        };
        self.view.label(cx, ids!(otp_status)).set_text(cx, &status);
    }
}

impl Widget for GalleryInputOtpPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            self.sync_status(cx, actions);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
