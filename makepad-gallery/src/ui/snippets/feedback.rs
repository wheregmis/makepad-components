pub const ALERT_PREVIEW_CODE: &str = r#"// Alerts are inline callouts, not overlay state.
// Configure the shell with tone + text props and let the component own the
// internal icon/text layout.
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0

    mod.widgets.ShadAlert{
        width: Fill
        title_text: "Heads up!"
        description_text: "You can add components and dependencies to your app using the cli."
    }

    mod.widgets.ShadAlert{
        width: Fill
        tone: ShadAlertTone.Destructive
        title_text: "Error"
        description_text: "Your session has expired. Please log in again."
    }
}"#;
pub const PROGRESS_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    ShadProgress{value: 0.33}\n    ShadProgress{value: 0.66}\n    ShadProgress{value: 1.0}\n    ShadProgressIndeterminate{}\n}";
pub const SKELETON_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 14.0

    ShadFieldDescription{
        width: Fill
        text: "Mirror the final content shape so loading transitions feel stable."
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 12.0
        align: Align{x: 0.0, y: 0.5}

        ShadSkeleton{
            width: 48
            height: 48
            draw_bg.border_radius: 24.0
        }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 8.0

            ShadSkeleton{ width: 220 height: 16 }
            ShadSkeleton{ width: 160 height: 14 }
        }
    }

    ShadSurface{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

        ShadSkeleton{ width: 180 height: 18 }
        ShadSkeleton{ width: Fill height: 14 }
        ShadSkeleton{ width: Fill height: 14 }
        ShadSkeleton{ width: 260 height: 14 }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            ShadSkeleton{ width: 96 height: 32 }
            ShadSkeleton{ width: 72 height: 32 animate: false }
        }
    }

    View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0

        ShadSkeleton{ width: Fill height: 14 }
        ShadSkeleton{ width: Fill height: 14 shimmer_speed: 0.8 }
        ShadSkeleton{ width: Fill height: 14 animation_fps: 12.0 }
        ShadSkeleton{ width: Fill height: 14 animate: false }
    }
}

// Usage notes:
// 1. Skeleton is a presentational loading placeholder, not a fetch controller.
// 2. Keep the geometry close to the resolved layout to avoid visible jumps.
// 3. animate, animation_fps, and shimmer_speed are declarative tuning knobs for motion."#;
pub const SPINNER_PREVIEW_CODE: &str = r#"View{
    width: Fit
    height: Fit
    flow: Right
    spacing: 10.0
    align: Align{x: 0.0, y: 0.5}

    ShadSpinner{size: ShadControlSize.Small}

    ShadLabel{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
        text: "Saving changes..."
    }
}

ShadSurface{
    variant: ShadSurfaceVariant.Muted
    width: Fill
    height: Fit
    flow: Right
    spacing: 12.0
    align: Align{y: 0.5}
    padding: Inset{left: 16, right: 16, top: 14, bottom: 14}
    draw_bg +: {
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    ShadSpinner{}

    View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 2.0

        ShadFieldLabel{text: "Syncing workspace activity"}
        ShadFieldDescription{text: "Use spinner for indeterminate waits. Prefer skeleton or progress when the UI can show structure or completion."}
    }
}"#;
pub const SONNER_PREVIEW_CODE: &str = r#"// Basic toast
View{
    flow: Right
    spacing: 8.0
ShadButton{text: "Event created"}
ShadButton{text: "Toast with description"}
}
ShadSonner{open: false}
ShadSonner{open: false}

// Toast with check icon + close button
ShadButton{text: "Show toast with close"}
ShadSonner{open: false}

// Controller example (Rust):
// let event_toast = self.ui.shad_sonner(cx, ids!(toast_event));
// let desc_toast = self.ui.shad_sonner(cx, ids!(toast_desc));
// let close_toast = self.ui.shad_sonner(cx, ids!(toast_close));
//
// if self.ui.shad_button(cx, ids!(toast_event_btn)).clicked(actions) {
//     event_toast.open(cx);
// }
// if self.ui.shad_button(cx, ids!(toast_desc_btn)).clicked(actions) {
//     desc_toast.open(cx);
// }
// if self.ui.shad_button(cx, ids!(toast_close_btn)).clicked(actions) {
//     close_toast.open(cx);
// }
//
// if let Some(is_open) = event_toast.open_changed(actions) {
//     log!("Toast open: {}", is_open);
// }"#;
