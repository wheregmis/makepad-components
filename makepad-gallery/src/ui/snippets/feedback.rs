pub const ALERT_PREVIEW_CODE: &str = r#"// Default
ShadAlert{
    width: Fill
    ShadAlertIcon{}
    ShadAlertContent{
        ShadAlertTitle{text: "Heads up!"}
        ShadAlertDescription{
            text: "You can add components and dependencies to your app using the cli."
        }
    }
}

// Destructive
ShadAlertDestructive{
    width: Fill
    ShadAlertDestructiveIcon{}
    ShadAlertContent{
        ShadAlertDestructiveTitle{text: "Error"}
        ShadAlertDestructiveDescription{
            text: "Your session has expired. Please log in again."
        }
    }
}"#;
pub const PROGRESS_PREVIEW_CODE: &str = r#"// Determinate
ShadProgress33{}
ShadProgress66{}
ShadProgressFull{}

// Indeterminate (animated)
ShadProgressIndeterminate{}

// Animation stress
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 8.0

    ShadProgressIndeterminate{}
    ShadProgressIndeterminate{}
    ShadProgressIndeterminate{}
    ShadProgressIndeterminate{}
}"#;
pub const SKELETON_PREVIEW_CODE: &str = r#"// Profile row placeholder
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

// Card content placeholder
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

// Animation tuning knobs
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 10.0

    ShadSkeleton{ width: Fill height: 14 }
    ShadSkeleton{ width: Fill height: 14 shimmer_speed: 0.8 }
    ShadSkeleton{ width: Fill height: 14 animation_fps: 12.0 }
    ShadSkeleton{ width: Fill height: 14 animate: false }
}

// Usage notes:
// 1. Skeleton is a presentational loading placeholder, not a fetch controller.
// 2. Keep the geometry close to the resolved layout to avoid visible jumps.
// 3. animate, animation_fps, and shimmer_speed are declarative tuning knobs for motion."#;
pub const SPINNER_PREVIEW_CODE: &str = r#"// Inline saving state
ShadSurface{
    width: Fit
    height: Fit
    flow: Right
    spacing: 10.0
    align: Align{y: 0.5}
    padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
    draw_bg +: {
        color: (shad_theme.color_secondary)
        border_radius: (shad_theme.radius)
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    ShadSpinnerSm{}
    ShadLabel{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
        text: "Saving changes..."
    }
}

// Banner loading row
ShadSurfaceMuted{
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
        ShadFieldDescription{text: "Fetching the latest comments and deploy events for this project."}
    }
}

// Centered full-page loading
ShadSurfaceMuted{
    width: Fill
    height: Fit
    flow: Down
    spacing: 10.0
    align: Align{x: 0.5}
    padding: Inset{left: 20, right: 20, top: 20, bottom: 20}
    draw_bg +: {
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    ShadSpinnerLg{}
    ShadLabel{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 13
        text: "Loading activity"
    }
    ShadFieldDescription{
        align: Align{x: 0.5}
        text: "Preparing the latest deploys, incidents, and subscriber events."
    }
}

// Sizes reference
View{
    width: Fill
    height: Fit
    flow: Right
    spacing: 24.0
    align: Align{x: 0.0, y: 0.5}

    View{
        width: Fit
        height: Fit
        flow: Down
        spacing: 8.0
        align: Align{x: 0.5}

        ShadSpinnerSm{}
        ShadFieldDescription{text: "Sm"}
    }

    View{
        width: Fit
        height: Fit
        flow: Down
        spacing: 8.0
        align: Align{x: 0.5}

        ShadSpinner{}
        ShadFieldDescription{text: "Default"}
    }

    View{
        width: Fit
        height: Fit
        flow: Down
        spacing: 8.0
        align: Align{x: 0.5}

        ShadSpinnerLg{}
        ShadFieldDescription{text: "Lg"}
    }
}"#;
pub const SONNER_PREVIEW_CODE: &str = r#"// DSL — place ShadSonner in an overlay area
View{
    width: Fill
    height: Fit
    flow: Right
    spacing: 8.0

    toast_event_btn := ShadButton{text: "Event created"}
    toast_desc_btn := ShadButton{text: "Toast with description"}
    toast_close_btn := ShadButton{text: "Show toast with close"}
}

View{
    width: Fill
    height: 200
    toast_sonner := ShadSonner{
        width: Fill
        height: Fill
        open: false
    }
}

// Controller example (Rust):
// use makepad_components::sonner::{ShadSonnerWidgetExt, SonnerItem, SonnerKind};
//
// let sonner = self.view.shad_sonner(cx, ids!(toast_sonner));
//
// if self.view.button(cx, ids!(toast_event_btn)).clicked(actions) {
//     sonner.enqueue(cx, SonnerItem {
//         title: "Connection successful".to_string(),
//         description: Some("Your workspace is now connected and ready to sync.".to_string()),
//         kind: SonnerKind::Success,
//         duration: Some(3.0),
//         show_close: true,
//     });
// }
//
// if self.view.button(cx, ids!(toast_desc_btn)).clicked(actions) {
//     sonner.enqueue(cx, SonnerItem {
//         title: "Heads up".to_string(),
//         description: Some("Network quality dropped. We will keep retrying in the background.".to_string()),
//         kind: SonnerKind::Info,
//         duration: Some(3.0),
//         show_close: true,
//     });
// }
//
// if self.view.button(cx, ids!(toast_close_btn)).clicked(actions) {
//     sonner.enqueue(cx, SonnerItem {
//         title: "Sync failed".to_string(),
//         description: Some("We could not reach the server. Check your connection and try again.".to_string()),
//         kind: SonnerKind::Error,
//         duration: Some(3.0),
//         show_close: false,
//     });
// }
//
// if let Some(is_open) = sonner.open_changed(actions) {
//     log!("Toast open: {}", is_open);
// }"#;
