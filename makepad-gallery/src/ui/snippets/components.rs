pub const ACCORDION_PREVIEW_CODE: &str = r#"mod.widgets.ShadAccordion{
    item_accessible := mod.widgets.ShadAccordionItem{
        title: "Is it accessible?"
        is_open: true
        body: Label{text: "Yes. It adheres to expected keyboard interactions and semantic structure."}
    }
    item_styled := mod.widgets.ShadAccordionItem{
        title: "Is it styled with complex elements?"
        body: View{
            flow: Down
            spacing: 8
            Label{text: "Yes. It supports rich content inside the body."}
            Toggle{text: "Enable feature" selected: true}
            CheckBox{text: "Accept terms" selected: true}
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
pub const AVATAR_PREVIEW_CODE: &str = "View{\n    width: Fit\n    height: Fit\n    flow: Right\n    align: Align{y: 0.5}\n    spacing: 12.0\n    mod.widgets.ShadAvatarSm{\n        fallback := mod.widgets.ShadAvatarFallback{text: \"SM\"}\n    }\n    mod.widgets.ShadAvatar{\n        fallback := mod.widgets.ShadAvatarFallback{text: \"CN\"}\n    }\n    mod.widgets.ShadAvatarLg{\n        fallback := mod.widgets.ShadAvatarFallback{text: \"LG\"}\n    }\n}";
pub const BADGE_PREVIEW_CODE: &str = "View{\n    width: Fit\n    height: Fit\n    flow: Right\n    spacing: 8.0\n    mod.widgets.ShadBadge{\n        label := mod.widgets.ShadBadgeLabel{text: \"Default\"}\n    }\n    mod.widgets.ShadBadgeSecondary{\n        label := mod.widgets.ShadBadgeSecondaryLabel{text: \"Secondary\"}\n    }\n    mod.widgets.ShadBadgeDestructive{\n        label := mod.widgets.ShadBadgeDestructiveLabel{text: \"Destructive\"}\n    }\n    mod.widgets.ShadBadgeOutline{\n        label := mod.widgets.ShadBadgeOutlineLabel{text: \"Outline\"}\n    }\n}";
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
pub const CARD_PREVIEW_CODE: &str = "mod.widgets.ShadCard{\n    header := mod.widgets.ShadCardHeader{\n        title := mod.widgets.ShadCardTitle{text: \"Card title\"}\n        description := mod.widgets.ShadCardDescription{text: \"Card description goes here.\"}\n    }\n    content := mod.widgets.ShadCardContent{\n        Label{text: \"Card content area. Put any widgets here.\" draw_text.color: (shad_theme.color_muted_foreground) draw_text.text_style.font_size: 14}\n    }\n    footer := mod.widgets.ShadCardFooter{\n        mod.widgets.ShadButton{text: \"Cancel\"}\n        mod.widgets.ShadButton{text: \"Save\"}\n    }\n}";
pub const CAROUSEL_PREVIEW_CODE: &str = r#"mod.widgets.ShadCarousel{}

// Default: 3 slides with prev/next icon buttons (chevron-left/right)
// and dot indicators. Override slide_0, slide_1, slide_2 in
// content_wrap.carousel_flip to customize.
//
// Controller example (Rust):
// let carousel = self.ui.shad_carousel(cx, ids!(carousel_demo));
//
// if self.ui.button(cx, ids!(next_slide_btn)).clicked(actions) {
//     carousel.next(cx);
// }
//
// if self.ui.button(cx, ids!(jump_to_first_btn)).clicked(actions) {
//     carousel.go_to(cx, 0);
// }
//
// if let Some(index) = carousel.changed(actions) {
//     log!("Active slide changed to {}", index);
// }"#;
pub const COLLAPSIBLE_PREVIEW_CODE: &str = r#"mod.widgets.ShadCollapsible{
    title: "Order #4189"
    is_open: true
    body: View{
        flow: Down
        spacing: 6
        Label{text: "Status: Shipped"}
        Label{text: "Shipping Address: 123 Main St, New York, NY"}
        Label{text: "Items: 2 × T-shirt, 1 × Hoodie"}
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
