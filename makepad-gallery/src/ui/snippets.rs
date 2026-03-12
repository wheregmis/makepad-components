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
//     faq.set_is_open(cx, true, animator::Animate::Yes);
// }
//
// if self.ui.button(cx, ids!(collapse_faq_btn)).clicked(actions) {
//     faq.set_is_open(cx, false, animator::Animate::Yes);
// }
//
// if faq.opening(actions) {
//     log!("FAQ item is opening");
// }
//
// if faq.closing(actions) {
//     log!("FAQ item is closing");
// }"#;
pub const ALERT_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    mod.widgets.ShadAlert{\n        width: Fill\n        icon := mod.widgets.ShadAlertIcon{}\n        content := mod.widgets.ShadAlertContent{\n            title := mod.widgets.ShadAlertTitle{text: \"Heads up!\"}\n            description := mod.widgets.ShadAlertDescription{text: \"Action complete.\"}\n        }\n    }\n    mod.widgets.ShadAlertDestructive{\n        width: Fill\n        icon := mod.widgets.ShadAlertDestructiveIcon{}\n        content := mod.widgets.ShadAlertContent{\n            title := mod.widgets.ShadAlertDestructiveTitle{text: \"Error\"}\n            description := mod.widgets.ShadAlertDescription{text: \"Your session expired. Please sign in again.\"}\n        }\n    }\n}";
pub const ASPECT_RATIO_PREVIEW_CODE: &str = "ShadAspectRatio{\n    width: Fill\n    ratio: 1.7777777778\n    RoundedView{\n        width: Fill\n        height: Fill\n        draw_bg.color: (shad_theme.color_secondary)\n    }\n}";
pub const AVATAR_PREVIEW_CODE: &str = "View{\n    width: Fit\n    height: Fit\n    flow: Right\n    align: Align{y: 0.5}\n    spacing: 12.0\n    mod.widgets.ShadAvatarSm{\n        fallback := mod.widgets.ShadAvatarFallback{text: \"SM\"}\n    }\n    mod.widgets.ShadAvatar{\n        fallback := mod.widgets.ShadAvatarFallback{text: \"CN\"}\n    }\n    mod.widgets.ShadAvatarLg{\n        fallback := mod.widgets.ShadAvatarFallback{text: \"LG\"}\n    }\n}";
pub const BADGE_PREVIEW_CODE: &str = "View{\n    width: Fit\n    height: Fit\n    flow: Right\n    spacing: 8.0\n    mod.widgets.ShadBadge{\n        label := mod.widgets.ShadBadgeLabel{text: \"Default\"}\n    }\n    mod.widgets.ShadBadgeSecondary{\n        label := mod.widgets.ShadBadgeSecondaryLabel{text: \"Secondary\"}\n    }\n    mod.widgets.ShadBadgeDestructive{\n        label := mod.widgets.ShadBadgeDestructiveLabel{text: \"Destructive\"}\n    }\n    mod.widgets.ShadBadgeOutline{\n        label := mod.widgets.ShadBadgeOutlineLabel{text: \"Outline\"}\n    }\n}";
pub const BREADCRUMB_PREVIEW_CODE: &str = "mod.widgets.ShadBreadcrumb{\n    mod.widgets.ShadBreadcrumbLink{ text: \"Home\" }\n    mod.widgets.ShadBreadcrumbSeparator{}\n    mod.widgets.ShadBreadcrumbLink{ text: \"Components\" }\n    mod.widgets.ShadBreadcrumbSeparator{}\n    mod.widgets.ShadBreadcrumbPage{ text: \"Breadcrumb\" }\n}";
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
//     details.set_is_open(cx, !details.is_open(cx), animator::Animate::Yes);
// }
//
// if details.opening(actions) {
//     log!("Details panel opening");
// }
//
// if details.closing(actions) {
//     log!("Details panel closing");
// }"#;
pub const COMMAND_PALETTE_PREVIEW_CODE: &str = r#"mod.widgets.ShadButton{text: "Open Command Palette"}

// The gallery listens for Cmd/Ctrl + K globally.
//
// Page action flow:
// 1. The page-local trigger emits GalleryCommandPalettePageAction::OpenRequested.
// 2. The app shell listens to command_palette_page.open_requested(actions).
// 3. The shell opens the shared command palette overlay.
//
// This keeps page-local button clicks separate from shell-owned modal state."#;
pub const CONTEXT_MENU_PREVIEW_CODE: &str = r#"ShadContextMenu{
    labels: ["Open" "Duplicate" "Share" "Delete"]

    RoundedView{
        width: 360
        height: Fit
        flow: Down
        spacing: 6.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }

        ShadLabel{text: "Project brief.md"}
        ShadFieldDescription{text: "Right click this card to open the menu."}
    }
}

// Controller example (Rust):
// let menu = self.ui.shad_context_menu(cx, ids!(context_menu_basic));
//
// if let Some(index) = menu.selected(actions) {
//     match index {
//         0 => self.open_file(),
//         1 => self.duplicate_file(),
//         2 => self.share_file(),
//         3 => self.delete_file(),
//         _ => {}
//     }
// }"#;
pub const KBD_PREVIEW_CODE: &str = "View{\n    width: Fit\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    View{\n        flow: Right\n        spacing: 6.0\n        align: Align{y: 0.5}\n        ShadKbd{ label := ShadKbdLabel{text: \"Cmd\"} }\n        ShadKbd{ label := ShadKbdLabel{text: \"Shift\"} }\n        ShadKbd{ label := ShadKbdLabel{text: \"Option\"} }\n        ShadKbd{ label := ShadKbdLabel{text: \"Ctrl\"} }\n    }\n    View{\n        flow: Right\n        spacing: 6.0\n        align: Align{y: 0.5}\n        ShadKbd{ label := ShadKbdLabel{text: \"Ctrl\"} }\n        ShadKbdSeparator{}\n        ShadKbd{ label := ShadKbdLabel{text: \"B\"} }\n    }\n}";
pub const LABEL_PREVIEW_CODE: &str = "mod.widgets.ShadLabel{ text: \"Your email address\" }";
pub const PROGRESS_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    ShadProgress33{}\n    ShadProgress66{}\n    ShadProgressFull{}\n    ShadProgressIndeterminate{}\n}";
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
pub const SPINNER_PREVIEW_CODE: &str =
    "ShadSpinner{}\n\n// 24×24 circular loading indicator. Use for async/loading states.";
pub const SONNER_PREVIEW_CODE: &str = r#"// Basic toast
View{
    flow: Right
    spacing: 8.0
    ShadButton{text: "Event created"}
    ShadButton{text: "Toast with description"}
}
ShadSonner{open: false}
ShadSonnerWithDescription{open: false}

// Toast with check icon + close button
ShadButton{text: "Show toast with close"}
ShadSonnerWithClose{open: false}

// Controller example (Rust):
// let toast = self.ui.shad_sonner(cx, ids!(toast_event));
//
// if self.ui.button(cx, ids!(toast_event_btn)).clicked(actions) {
//     toast.show(cx);
// }
//
// if toast.opened(actions) {
//     log!("Toast opened");
// }
//
// if toast.closed(actions) {
//     log!("Toast dismissed");
// }"#;

pub const DIALOG_PREVIEW_CODE: &str = r#"// Generic: ShadDialog with overlay +: { content +: { body +: { ... } } }
// Alert: ShadDialogAlert{ open: false } — closes on Cancel/Confirm/backdrop
// Destructive: ShadDialogAlertDestructive{ open: false }

mod.widgets.ShadButton{text: "Open dialog"}
mod.widgets.ShadDialog{ open: false }
mod.widgets.ShadDialogAlert{ open: false }
mod.widgets.ShadDialogAlertDestructive{ open: false }

// Controller example (Rust):
// let dialog = self.ui.shad_dialog(cx, ids!(default_dialog));
//
// if self.ui.button(cx, ids!(open_dialog_btn)).clicked(actions) {
//     dialog.open(cx);
// }
//
// if self.ui.button(cx, ids!(close_btn)).clicked(actions) {
//     dialog.close(cx);
// }
//
// if dialog.opened(actions) {
//     log!("Dialog opened");
// }
//
// if dialog.closed(actions) {
//     log!("Dialog closed");
// }"#;
#[allow(dead_code)]
pub const EMPTY_PREVIEW_CODE: &str = "ShadEmpty{\n    ShadEmptyIconContainer{ ShadEmptyIcon{text: \"📁\"} }\n    ShadEmptyContent{\n        ShadEmptyTitle{text: \"No files yet\"}\n        ShadEmptyDescription{text: \"Upload or create a file to get started.\"}\n        ShadEmptyAction{ ShadButton{text: \"Upload file\"} }\n    }\n}";
#[allow(dead_code)]
pub const FIELD_PREVIEW_CODE: &str = "ShadField{\n    ShadFieldLabel{text: \"Email\"}\n    TextInput{empty_text: \"you@example.com\"}\n    ShadFieldDescription{text: \"We'll never share your email.\"}\n}";
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
// // On redraw/startup you can also inspect the current value:
// let current = otp.value();"#;
pub const PAGINATION_PREVIEW_CODE: &str = r#"projects_pagination := ShadPagination{
    current_page: 5
    page_count: 12
}

// Controller example (Rust):
// let pagination = self.view.shad_pagination(cx, ids!(projects_pagination));
//
// if let Some(page) = pagination.changed(actions) {
//     self.current_page = page;
//     self.reload_rows_for(page);
// }
//
// if self.view.button(cx, ids!(next_page_btn)).clicked(actions) {
//     pagination.next(cx);
// }
//
// pagination.set_page(cx, 1);
// let active_page = pagination.page();
// let total_pages = pagination.page_count();"#;
pub const POPOVER_PREVIEW_CODE: &str = r#"profile_popover := ShadPopover{
    side: "bottom"
    align: "start"

    trigger := ShadButtonOutline{
        text: "Open profile editor"
    }

    content: ShadPopoverContent{
        title := ShadSectionHeader{
            text: "Edit profile"
        }

        description := ShadFieldDescription{
            text: "Quick edits belong in a popover when the page context should stay visible."
        }

        footer := View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0

            popover_close_btn := ShadButtonGhost{
                text: "Cancel"
            }

            popover_apply_btn := ShadButton{
                text: "Save"
            }
        }
    }
}

// Controller example (Rust):
// let popover = self.view.shad_popover(cx, ids!(profile_popover));
// let content = popover.content_widget();
//
// if content.button(cx, ids!(popover_apply_btn)).clicked(actions) {
//     self.save_profile_changes();
//     popover.close(cx);
// }
//
// if popover.opened(actions) {
//     log!("Popover opened");
// }
//
// if popover.closed(actions) {
//     log!("Popover closed");
// }"#;
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
pub const RESIZABLE_PREVIEW_CODE: &str = r#"horizontal_split := ShadResizable{
    axis: SplitterAxis.Horizontal
    align: SplitterAlign.FromA(180.0)
    a: View{width: Fill height: Fill}
    b: View{width: Fill height: Fill}
}

// Controller example (Rust):
// let split = self.view.splitter(cx, ids!(horizontal_split));
//
// if let Some((axis, align)) = split.changed(actions) {
//     self.saved_split = Some((axis, align));
// }
//
// if let Some(align) = self.saved_split.map(|(_, align)| align) {
//     split.set_align(cx, align);
// }"#;
pub const SIDEBAR_PREVIEW_CODE: &str = r#"mod.widgets.ShadSidebar{
    nav_playground := ShadSidebarItem{text: "Playground"}
    nav_history := ShadSidebarItem{text: "History"}
    nav_settings := ShadSidebarItem{text: "Settings"}
}

// Controller example (Rust):
// if self.ui.button(cx, ids!(nav_playground)).clicked(actions) {
//     self.router.go_to_route(cx, live_id!(playground));
// }
//
// Sidebar items are button actions with navigation styling. Keep the selected
// route or active section in page/app state, then render the matching item as
// active from that state."#;
pub const SCROLL_AREA_PREVIEW_CODE: &str = "ShadScrollArea{\n    width: Fill\n    height: 220\n    View{\n        width: Fill\n        height: Fit\n        flow: Down\n        spacing: 8.0\n        ShadLabel{text: \"Row 1\"}\n        ShadLabel{text: \"Row 2\"}\n        ShadLabel{text: \"Row 3\"}\n    }\n}";
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
pub const SEPARATOR_PREVIEW_CODE: &str = "View{\n    width: Fill\n    height: Fit\n    flow: Down\n    spacing: 12.0\n    ShadLabel{text: \"Account\"}\n    ShadSeparator{}\n    ShadLabel{text: \"Billing\"}\n}";
pub const SHEET_PREVIEW_CODE: &str = r#"ShadButton{text: "Open sheet"}
ShadSheet{open: false}

// Controller example (Rust):
// let sheet = self.ui.shad_sheet(cx, ids!(right_sheet));
//
// if self.ui.button(cx, ids!(open_right_sheet_btn)).clicked(actions) {
//     sheet.open(cx);
// }
//
// if self.ui.button(cx, ids!(close_right_sheet_btn)).clicked(actions) {
//     sheet.close(cx);
// }
//
// if sheet.opened(actions) {
//     log!("Sheet opened");
// }
//
// if sheet.closed(actions) {
//     log!("Sheet closed");
// }
//
// Override overlay.content.sheet_frame align/size for left/top/bottom variants."#;
pub const TABS_PREVIEW_CODE: &str = r#"ShadTabs{
    ShadTabsList{
        ShadTabsTrigger{text: "Overview"}
        ShadTabsTrigger{text: "Usage"}
        ShadTabsTrigger{text: "Settings"}
    }
    ShadTabsContent{
        ShadLabel{text: "Switch content in app code with PageFlip or another state holder."}
    }
}

// Page-controller example (Rust):
// if self.ui.button(cx, ids!(tabs_usage_trigger)).clicked(actions) {
//     self.set_selected_tab(cx, live_id!(usage));
// }
//
// fn set_selected_tab(&mut self, cx: &mut Cx, page: LiveId) {
//     self.view.router_widget(cx, ids!(tabs_content_flip)).go_to_route(cx, page);
//     // Also update the active indicator visibility here.
// }"#;
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
