pub const CALENDAR_PREVIEW_CODE: &str = r#"calendar_demo := ShadCalendar{
    value: "2026-03-13"
}"#;
pub const CHECKBOX_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0
    accept_terms := ShadCheckbox{label: "Accept terms and conditions"}
    product_updates := ShadCheckbox{label: "Receive product updates"}
    notifications := ShadCheckbox{label: "Enable notifications" checked: true}
}"#;
pub const DATE_PICKER_PREVIEW_CODE: &str = r#"deadline_picker := ShadDatePicker{
    value: "2026-03-13"
}"#;
pub const KBD_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 14.0

    ShadFieldDescription{
        width: Fill
        text: "Use Kbd chips as shortcut copy in menus, dialogs, or onboarding hints."
    }

    View{
        width: Fit
        height: Fit
        flow: Right
        spacing: 6.0
        align: Align{y: 0.5}

        ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
        ShadKbd{ label := ShadKbdLabel{text: "Shift"} }
        ShadKbd{ label := ShadKbdLabel{text: "Option"} }
        ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 16.0
        align: Align{y: 0.5}

        ShadLabel{
            width: 180
            text: "Open command palette"
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 6.0
            align: Align{y: 0.5}

            ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
            ShadKbdSeparator{}
            ShadKbd{ label := ShadKbdLabel{text: "K"} }
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 6.0
            align: Align{y: 0.5}

            ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
            ShadKbdSeparator{}
            ShadKbd{ label := ShadKbdLabel{text: "K"} }
        }
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 16.0
        align: Align{y: 0.5}

        ShadLabel{
            width: 180
            text: "Duplicate current row"
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 6.0
            align: Align{y: 0.5}

            ShadKbd{ label := ShadKbdLabel{text: "Shift"} }
            ShadKbdSeparator{}
            ShadKbd{ label := ShadKbdLabel{text: "Alt"} }
            ShadKbdSeparator{}
            ShadKbd{ label := ShadKbdLabel{text: "Down"} }
        }
    }
}

// Usage notes:
// 1. Treat Kbd as presentational copy, not as the keyboard handler.
// 2. Render one ShadKbd per key and place ShadKbdSeparator between keys in a combo.
// 3. Keep the actual shortcut behavior in the surrounding page or app shell."#;
pub const LABEL_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 14.0

    ShadFieldDescription{
        width: Fill
        text: "Use ShadLabel for nearby UI copy, summaries, and status text."
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 12.0

        ShadSurface{
            width: 248
            height: Fit
            flow: Down
            spacing: 6.0
            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}

            ShadLabel{text: "Current plan"}
            ShadFieldDescription{text: "Pro workspace with annual billing."}
        }

        ShadSurface{
            width: 248
            height: Fit
            flow: Down
            spacing: 6.0
            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}

            ShadLabel{text: "Environment"}
            ShadFieldDescription{text: "Production API connected."}
        }
    }

    View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0

        sync_status := ShadLabel{
            text: "Last synced 2 minutes ago"
        }

        results_count := ShadLabel{
            text: "Showing 24 of 120 results"
        }
    }
}

// Runtime update example (Rust):
// self.view.label(cx, ids!(sync_status)).set_text(cx, "Last synced just now");
// self.view.label(cx, ids!(results_count)).set_text(cx, "Showing 36 of 120 results");
//
// Use ShadFieldLabel inside ShadField when the text belongs to an input stack,
// instead of reusing ShadLabel as a form caption."#;
pub const SLIDER_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 16.0
    volume_slider := ShadSlider{default: 0.5}
}"#;
pub const INPUT_PREVIEW_CODE: &str = r#"ShadField{
    ShadFieldLabel{text: "Email"}
    email_input := ShadInput{empty_text: "you@example.com"}
    ShadFieldDescription{text: "We'll never share your email."}
}"#;
pub const INPUT_OTP_PREVIEW_CODE: &str = r#"View{
    width: Fit
    height: Fit
    flow: Down
    spacing: 8.0
    ShadLabel{text: "Enter the 6-digit code"}
    ShadInputOtp{}
    ShadFieldDescription{text: "Paste is supported. Only digits are accepted."}
}"#;
pub const RADIO_GROUP_PREVIEW_CODE: &str = r#"ShadRadioGroup{
    starter_plan := ShadRadioItem{text: "Starter"}
    pro_plan := ShadRadioItem{text: "Pro"}
    enterprise_plan := ShadRadioItem{text: "Enterprise"}
}"#;
pub const SELECT_PREVIEW_CODE: &str = r#"status_select := ShadSelect{
    labels: ["Pending" "In Progress" "Done"]
}"#;
pub const TEXTAREA_PREVIEW_CODE: &str = r#"ShadField{
    ShadFieldLabel{text: "Bio"}
    bio_input := ShadTextarea{
        empty_text: "Tell us a little bit about yourself"
    }
    ShadFieldDescription{text: "Keep it short. You can always edit this later."}
}"#;
pub const SWITCH_PREVIEW_CODE: &str = r#"email_alerts_switch := ShadSwitch{text: "Email alerts"}"#;
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
