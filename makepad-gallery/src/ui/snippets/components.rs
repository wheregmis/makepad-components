pub const ACCORDION_PREVIEW_CODE: &str = r#"mod.widgets.ShadAccordion{
    item_accessible := mod.widgets.ShadAccordionItem{
        title: "Is it accessible?"
        is_open: true
        body: mod.widgets.ShadLabel{text: "Yes. It adheres to expected keyboard interactions and semantic structure."}
    }
    item_styled := mod.widgets.ShadAccordionItem{
        title: "Is it styled with complex elements?"
        body: View{
            flow: Down
            spacing: 8
            mod.widgets.ShadLabel{text: "Yes. It supports rich content inside the body."}
            mod.widgets.ShadToggle{text: "Enable feature" selected: true}
            mod.widgets.ShadCheckbox{label: "Accept terms" checked: true}
        }
    }
}

// Controller example (Rust):
// let faq = self.ui.shad_accordion_item(cx, ids!(item_accessible));
//
// if self.ui.button(cx, ids!(expand_faq_btn)).clicked(actions) {
//     faq.set_open(cx, true, animator::Animate::Yes);
// }
//
// if self.ui.button(cx, ids!(collapse_faq_btn)).clicked(actions) {
//     faq.set_open(cx, false, animator::Animate::Yes);
// }
//
// if let Some(is_open) = faq.open_changed(actions) {
//     log!("FAQ item open: {}", is_open);
// }
//
// if let Some(progress) = faq.animation_progress(actions) {
//     log!("Accordion animation progress: {}", progress);
// }"#;
pub const AVATAR_PREVIEW_CODE: &str = r#"// Keep fallback text in the avatar, then let the image cover it when present.
View{
    width: Fit
    height: Fit
    flow: Right
    align: Align{y: 0.5}
    spacing: 12.0

    mod.widgets.ShadAvatar{
        fallback := mod.widgets.ShadAvatarFallback{text: "ML"}
        image := mod.widgets.ShadAvatarImage{
            src: crate_resource("self://resources/avatar/portrait-a.jpg")
        }
        status := mod.widgets.ShadAvatarStatusOnline{}
    }

    mod.widgets.ShadAvatar{
        fallback := mod.widgets.ShadAvatarFallback{text: "JD"}
    }

    mod.widgets.ShadAvatarLg{
        fallback := mod.widgets.ShadAvatarFallback{text: "AB"}
        image := mod.widgets.ShadAvatarImage{
            src: crate_resource("self://resources/avatar/portrait-b.jpg")
        }
        status := mod.widgets.ShadAvatarStatusBusy{}
    }
}"#;
pub const BADGE_PREVIEW_CODE: &str = r#"// Put badges beside the content they annotate.
mod.widgets.ShadSurfaceMuted{
    width: 320
    height: Fit
    padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
    draw_bg +: {
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 12.0

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 2.0

            mod.widgets.ShadFieldLabel{text: "Realtime API"}
            mod.widgets.ShadFieldDescription{text: "Production webhook delivery is enabled."}
        }

        mod.widgets.ShadBadgeSuccess{
            label := mod.widgets.ShadBadgeSuccessLabel{text: "Live"}
        }
    }
}"#;
pub const BUTTON_GROUP_PREVIEW_CODE: &str = r#"ShadButtonGroup{
    archive_btn := ShadButtonGroupItem{text: "Archive"}
    ShadButtonGroupSeparator{}
    report_btn := ShadButtonGroupItem{text: "Report"}
}

// Controller example (Rust):
// if self.ui.button(cx, ids!(archive_btn)).clicked(actions) {
//     self.toolbar_action = ToolbarAction::Archive;
// }
//
// if self.ui.button(cx, ids!(report_btn)).clicked(actions) {
//     self.toolbar_action = ToolbarAction::Report;
// }
//
// Keep the chosen tool or mode in page state. The button group itself is
// presentational; each named child button emits the action you care about."#;
pub const BUTTON_PREVIEW_CODE: &str = r#"View{
    width: Fit
    height: Fit
    flow: Right
    spacing: 8.0
    save_btn := mod.widgets.ShadButton{text: "Save"}
    delete_btn := mod.widgets.ShadButtonDestructive{text: "Delete"}
    more_btn := mod.widgets.ShadButtonGhost{text: "More"}
}

// Controller example (Rust):
// if self.ui.button(cx, ids!(save_btn)).clicked(actions) {
//     self.save_document();
// }
//
// if self.ui.button(cx, ids!(delete_btn)).clicked(actions) {
//     self.confirm_delete = true;
// }
//
// Buttons stay intentionally small: give the button an id, then listen for
// clicked(actions) in the page or feature controller."#;
pub const CARD_PREVIEW_CODE: &str = r#"mod.widgets.ShadCard{
    width: 380

    header := mod.widgets.ShadCardHeader{
        title := mod.widgets.ShadCardTitle{text: "Team Access"}
        description := mod.widgets.ShadCardDescription{text: "Review seats, pending invites, and billing impact before applying changes."}
    }

    content := mod.widgets.ShadCardContent{
        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 12.0

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 2.0

                mod.widgets.ShadFieldLabel{text: "Current plan"}
                mod.widgets.ShadFieldDescription{text: "Pro workspace with advanced sharing controls."}
            }

            mod.widgets.ShadBadgeSecondary{
                label := mod.widgets.ShadBadgeSecondaryLabel{text: "Pro"}
            }
        }

        mod.widgets.ShadHr{}

        mod.widgets.ShadFieldDescription{text: "Seats in use: 18 of 25"}
        mod.widgets.ShadFieldDescription{text: "Pending invites: 3 awaiting acceptance"}
    }

    footer := mod.widgets.ShadCardFooter{
        mod.widgets.ShadButtonGhost{text: "Cancel"}
        mod.widgets.ShadButton{text: "Review changes"}
    }
}"#;
pub const CAROUSEL_PREVIEW_CODE: &str = r#"featured_carousel := mod.widgets.ShadCarousel{
    width: 720
}

// The current carousel ships with 3 named slides and matching dots.
// Override slide_0, slide_1, and slide_2 inside content_wrap.carousel_flip
// when you need custom highlight content.
//
// Controller example (Rust):
// let carousel = self.ui.shad_carousel(cx, ids!(featured_carousel));
//
// if self.ui.button(cx, ids!(open_next_highlight_btn)).clicked(actions) {
//     carousel.next(cx);
// }
//
// if self.ui.button(cx, ids!(jump_to_first_highlight_btn)).clicked(actions) {
//     carousel.go_to(cx, 0);
// }
//
// if let Some(index) = carousel.changed(actions) {
//     log!("Active highlight changed to {}", index);
// }"#;
pub const COLLAPSIBLE_PREVIEW_CODE: &str = r#"mod.widgets.ShadCollapsible{
    title: "Order #4189"
    is_open: true
    body: View{
        flow: Down
        spacing: 6
        mod.widgets.ShadLabel{text: "Status: Shipped"}
        mod.widgets.ShadLabel{text: "Shipping Address: 123 Main St, New York, NY"}
        mod.widgets.ShadLabel{text: "Items: 2 × T-shirt, 1 × Hoodie"}
    }
}

// Controller example (Rust):
// let details = self.ui.shad_collapsible(cx, ids!(order_details));
//
// if self.ui.button(cx, ids!(toggle_details_btn)).clicked(actions) {
//     details.set_open(cx, !details.is_open(cx), animator::Animate::Yes);
// }
//
// if let Some(is_open) = details.open_changed(actions) {
//     log!("Details panel open: {}", is_open);
// }
//
// if let Some(progress) = details.animation_progress(actions) {
//     log!("Collapsible animation progress: {}", progress);
// }"#;

pub const ICON_GALLERY_PREVIEW_CODE: &str = r#"// The Icons page is generated from makepad-icon/resources/icons/*.svg.
//
// 1) Sync assets:
// python3 makepad-icon/scripts/download_lucide_icons.py --clean
//
// 2) Build:
// cargo check -p makepad-icon
//
// 3) Open /icons in makepad-gallery to preview all generated icon components."#;
