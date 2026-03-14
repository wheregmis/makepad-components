pub const CALENDAR_PREVIEW_CODE: &str = r#"calendar_demo := ShadCalendar{
    value: "2026-03-13"
}

// Controller example (Rust):
// let calendar = self.ui.shad_calendar(cx, ids!(calendar_demo));
//
// if let Some(date) = calendar.changed(actions) {
//     self.selected_due_date = Some(date);
// }
//
// if self.ui.button(cx, ids!(next_month_btn)).clicked(actions) {
//     calendar.next_month(cx);
// }
//
// calendar.set_value(cx, Some(ShadDate{year: 2026, month: 4, day: 1}));
// calendar.clear(cx);"#;
pub const CHECKBOX_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0
    accept_terms := ShadCheckbox{label: "Accept terms and conditions"}
    product_updates := ShadCheckbox{label: "Receive product updates"}
    notifications := ShadCheckbox{label: "Enable notifications" checked: true}
}

// Controller example (Rust):
// let accept_terms = self.ui.shad_checkbox(cx, ids!(accept_terms));
//
// if let Some(checked) = accept_terms.changed(actions) {
//     self.can_submit = checked;
// }
//
// if self.reset_requested {
//     accept_terms.set_checked(cx, false, animator::Animate::No);
// }"#;
pub const DATE_PICKER_PREVIEW_CODE: &str = r#"deadline_picker := ShadDatePicker{
    value: "2026-03-13"
}

// Controller example (Rust):
// let picker = self.ui.shad_date_picker(cx, ids!(deadline_picker));
//
// if self.ui.button(cx, ids!(open_picker_btn)).clicked(actions) {
//     picker.set_open(cx, true);
// }
//
// if let Some(date) = picker.changed(actions) {
//     self.deadline = Some(date);
// }
//
// if let Some(is_open) = picker.open_changed(actions) {
//     self.is_picker_visible = is_open;
// }
//
// picker.set_value(cx, Some(ShadDate{year: 2026, month: 4, day: 1}));
// picker.clear(cx);"#;
pub const KBD_PREVIEW_CODE: &str = "View{\n    width: Fit\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    View{\n        flow: Right\n        spacing: 6.0\n        align: Align{y: 0.5}\n        ShadKbd{ label := ShadKbdLabel{text: \"Cmd\"} }\n        ShadKbd{ label := ShadKbdLabel{text: \"Shift\"} }\n        ShadKbd{ label := ShadKbdLabel{text: \"Option\"} }\n        ShadKbd{ label := ShadKbdLabel{text: \"Ctrl\"} }\n    }\n    View{\n        flow: Right\n        spacing: 6.0\n        align: Align{y: 0.5}\n        ShadKbd{ label := ShadKbdLabel{text: \"Ctrl\"} }\n        ShadKbdSeparator{}\n        ShadKbd{ label := ShadKbdLabel{text: \"B\"} }\n    }\n}";
pub const LABEL_PREVIEW_CODE: &str = "mod.widgets.ShadLabel{ text: \"Your email address\" }";
pub const SLIDER_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 16.0
    volume_slider := ShadSlider{default: 0.5}
}

// Controller example (Rust):
// let volume = self.view.slider(cx, ids!(volume_slider));
//
// if let Some(value) = volume.slided(actions) {
//     self.preview_volume = value;
// }
//
// if let Some(value) = volume.end_slide(actions) {
//     self.saved_volume = value;
// }
//
// volume.set_value(cx, 0.8);
// let current = volume.value();"#;
pub const INPUT_PREVIEW_CODE: &str = r#"ShadField{
    ShadFieldLabel{text: "Email"}
    email_input := ShadInput{empty_text: "you@example.com"}
    ShadFieldDescription{text: "We'll never share your email."}
}

// Controller example (Rust):
// let email_input = self.view.text_input(cx, ids!(email_input));
//
// if let Some(text) = email_input.changed(actions) {
//     self.email_draft = text;
// }
//
// if let Some((submitted, _modifiers)) = email_input.returned(actions) {
//     self.submit_email(submitted);
// }
//
// if self.clear_requested {
//     email_input.set_text(cx, "");
// }"#;
pub const INPUT_OTP_PREVIEW_CODE: &str = r#"View{
    width: Fit
    height: Fit
    flow: Down
    spacing: 8.0
    ShadLabel{text: "Enter the 6-digit code"}
    ShadInputOtp{}
    ShadFieldDescription{text: "Paste is supported. Only digits are accepted."}
}

// Controller example (Rust):
// let otp = self.ui.shad_input_otp(cx, ids!(otp_demo));
//
// if let Some(value) = otp.changed(actions) {
//     self.status = format!("Current value: {}", value);
// }
//
// if let Some(code) = otp.completed(actions) {
//     self.status = format!("Completed: {}", code);
// }
//
// // Restore from saved state:
// otp.set_value(cx, "123456");
//
// // On redraw/startup you can also inspect the current value:
// let current = otp.value();"#;
pub const RADIO_GROUP_PREVIEW_CODE: &str = r#"ShadRadioGroup{
    starter_plan := ShadRadioItem{text: "Starter"}
    pro_plan := ShadRadioItem{text: "Pro"}
    enterprise_plan := ShadRadioItem{text: "Enterprise"}
}

// Controller example (Rust):
// if let Some(index) = self.view
//     .radio_button_set(ids!(starter_plan, pro_plan, enterprise_plan))
//     .selected(cx, actions)
// {
//     self.selected_plan = match index {
//         0 => Plan::Starter,
//         1 => Plan::Pro,
//         _ => Plan::Enterprise,
//     };
// }
//
// When you restore state, call set_active(cx, ...) on the individual radio
// items that should match the current domain value."#;
pub const SELECT_PREVIEW_CODE: &str = r#"status_select := ShadSelect{
    labels: ["Pending" "In Progress" "Done"]
}

// Controller example (Rust):
// let status = self.view.drop_down(cx, ids!(status_select));
//
// if let Some(index) = status.changed(actions) {
//     self.status_index = index;
// }
//
// if let Some(label) = status.changed_label(actions) {
//     self.status_label = label;
// }
//
// status.set_selected_item(cx, 2);
// let current_label = status.selected_label();"#;
pub const TEXTAREA_PREVIEW_CODE: &str = r#"ShadField{
    ShadFieldLabel{text: "Bio"}
    bio_input := ShadTextarea{
        empty_text: "Tell us a little bit about yourself"
    }
    ShadFieldDescription{text: "Keep it short. You can always edit this later."}
}

// Controller example (Rust):
// let bio = self.view.text_input(cx, ids!(bio_input));
//
// if let Some(text) = bio.changed(actions) {
//     self.bio_draft = text;
// }
//
// if self.restore_previous_draft {
//     bio.set_text(cx, &self.cached_bio);
// }"#;
pub const SWITCH_PREVIEW_CODE: &str = r#"email_alerts_switch := ShadSwitch{text: "Email alerts"}

// Controller example (Rust):
// let email_alerts = self.view.check_box(cx, ids!(email_alerts_switch));
//
// if let Some(enabled) = email_alerts.changed(actions) {
//     self.email_alerts_enabled = enabled;
// }
//
// if self.reset_preferences {
//     email_alerts.set_active(cx, false);
// }
//
// let is_enabled = email_alerts.active(cx);"#;
pub const TOGGLE_PREVIEW_CODE: &str = r#"View{
    width: Fit
    height: Fit
    flow: Down
    spacing: 12.0
    View{
        width: Fit
        height: Fit
        flow: Right
        spacing: 8.0
        ShadToggle{text: "Bold"}
        ShadToggle{text: "Italic" active: true}
        ShadToggle{text: "Underline"}
    }
    ShadToggleGroup{
        ShadToggleGroupItem{text: "Left"}
        ShadToggleGroupItem{text: "Center" active: true}
        ShadToggleGroupItem{text: "Right"}
    }
}

// Toggle flow in practice:
// 1. Treat the selected/pressed value as page state.
// 2. On trigger click, update that page state.
// 3. Re-render the matching ShadToggle / ShadToggleGroupItem with active: true.
//
// This is the same pattern used by the Tabs demo: page-owned state,
// widget tree reflects that state."#;
