pub const ACCORDION_PREVIEW_CODE: &str = r#"ShadAccordion{
    width: Fill
    item_accessible := ShadAccordionItem{
        title: "Is it accessible?"
        is_open: true
        body: View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
            ShadFieldDescription{
                width: Fill
                text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior."
            }
        }
    }

    item_styled := ShadAccordionItem{
        title: "Is it styled with complex elements?"
        body: View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
            spacing: 10.0

            ShadFieldDescription{
                width: Fill
                text: "We can put any view here, like a row with toggles."
            }

            View{
                width: Fit
                height: Fit
                flow: Right{wrap: true}
                spacing: 20.0
                align: Align{y: 0.5}

                ShadSwitch{text: "Switch"}
                ShadCheckbox{label: "Or a CheckBox"}
            }
        }
    }

    item_third := ShadAccordionItem{
        title: "This is third accordion"
        body: View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
            ShadFieldDescription{
                width: Fill
                text: "This is third accordion content. It can be any view, like a text view or a button."
            }
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
pub const AVATAR_PREVIEW_CODE: &str = r#"// Photo Sizes
View{
    width: Fill
    height: Fit
    flow: Right
    spacing: 24.0
    align: Align{y: 0.0}

    View{
        width: Fit
        height: Fit
        flow: Down
        spacing: 8.0
        align: Align{x: 0.5, y: 0.0}

        ShadAvatarSm{
            fallback := ShadAvatarFallback{text: "ML"}
            image := ShadAvatarImage{
                src: crate_resource("self://resources/avatar/portrait-a.jpg")
            }
        }
        ShadFieldLabel{text: "Small"}
    }

    View{
        width: Fit
        height: Fit
        flow: Down
        spacing: 8.0
        align: Align{x: 0.5, y: 0.0}

        ShadAvatar{
            fallback := ShadAvatarFallback{text: "CN"}
            image := ShadAvatarImage{
                src: crate_resource("self://resources/avatar/portrait-a.jpg")
            }
        }
        ShadFieldLabel{text: "Default"}
    }

    View{
        width: Fit
        height: Fit
        flow: Down
        spacing: 8.0
        align: Align{x: 0.5, y: 0.0}

        ShadAvatarLg{
            fallback := ShadAvatarFallback{text: "AB"}
            image := ShadAvatarImage{
                src: crate_resource("self://resources/avatar/portrait-b.jpg")
            }
        }
        ShadFieldLabel{text: "Large"}
    }
}

// Fallbacks — no image
View{
    width: Fill
    height: Fit
    flow: Right
    spacing: 24.0
    align: Align{y: 0.0}

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "JD"}
    }

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "AB"}
    }

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "?"}
    }
}

// Presence badges
View{
    width: Fill
    height: Fit
    flow: Right
    spacing: 24.0
    align: Align{y: 0.0}

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "ML"}
        image := ShadAvatarImage{
            src: crate_resource("self://resources/avatar/portrait-a.jpg")
        }
        status := ShadAvatarStatusOnline{}
    }

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "AB"}
        image := ShadAvatarImage{
            src: crate_resource("self://resources/avatar/portrait-b.jpg")
        }
        status := ShadAvatarStatusAway{}
    }

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "CN"}
        image := ShadAvatarImage{
            src: crate_resource("self://resources/avatar/portrait-a.jpg")
        }
        status := ShadAvatarStatusBusy{}
    }
}"#;
pub const BADGE_PREVIEW_CODE: &str = r#"// In Context — badges beside field rows
ShadSurfaceMuted{
    width: Fill
    height: Fit
    flow: Down
    spacing: 14.0
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

            ShadFieldLabel{text: "Realtime API"}
            ShadFieldDescription{text: "Production webhook delivery is enabled for connected workspaces."}
        }

        ShadBadgeSuccess{
            label := ShadBadgeSuccessLabel{text: "Live"}
        }
    }

    ShadHr{}

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

            ShadFieldLabel{text: "Usage Analytics"}
            ShadFieldDescription{text: "Rolling out to selected teams before the wider release."}
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadBadgeWarning{
                label := ShadBadgeWarningLabel{text: "Beta"}
            }
            ShadBadgeSecondary{
                label := ShadBadgeSecondaryLabel{text: "Internal"}
            }
        }
    }

    ShadHr{}

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

            ShadFieldLabel{text: "Legacy Sync"}
            ShadFieldDescription{text: "Scheduled for removal after the new importer reaches parity."}
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadBadgeDestructive{
                label := ShadBadgeDestructiveLabel{text: "Deprecated"}
            }
            ShadBadgeOutline{
                label := ShadBadgeOutlineLabel{text: "Archived"}
            }
        }
    }
}

// Variants palette
ShadSurfaceMuted{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0
    padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
    draw_bg +: {
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 8.0

        ShadBadge{
            label := ShadBadgeLabel{text: "Default"}
        }
        ShadBadgeSecondary{
            label := ShadBadgeSecondaryLabel{text: "Secondary"}
        }
        ShadBadgeOutline{
            label := ShadBadgeOutlineLabel{text: "Outline"}
        }
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 8.0

        ShadBadgeSuccess{
            label := ShadBadgeSuccessLabel{text: "Success"}
        }
        ShadBadgeWarning{
            label := ShadBadgeWarningLabel{text: "Warning"}
        }
        ShadBadgeDestructive{
            label := ShadBadgeDestructiveLabel{text: "Destructive"}
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
pub const BUTTON_PREVIEW_CODE: &str = r#"// Variants
View{
    width: Fill
    height: Fit
    flow: Right
    spacing: 8.0

    ShadButton{text: "Default"}
    ShadButtonDestructive{text: "Destructive"}
    ShadButtonOutline{text: "Outline"}
    ShadButtonSecondary{text: "Secondary"}
    ShadButtonGhost{text: "Ghost"}
    ShadButtonLink{text: "Link"}
}

// Sizes
View{
    width: Fill
    height: Fit
    flow: Right
    align: Align{y: 0.5}
    spacing: 8.0

    ShadButtonSm{text: "Small"}
    ShadButton{text: "Default"}
    ShadButtonLg{text: "Large"}
}

// Icon Buttons
View{
    width: Fill
    height: Fit
    flow: Right
    align: Align{y: 0.5}
    spacing: 8.0

    ShadButtonIcon{text: "✓"}

    IconButtonChevronLeft{
        width: 36
        height: 36
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
        draw_icon.color: (shad_theme.color_primary)
    }

    IconButtonChevronRight{
        width: 36
        height: 36
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
        draw_icon.color: (shad_theme.color_primary)
    }

    IconButtonX{
        width: 36
        height: 36
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
pub const CARD_PREVIEW_CODE: &str = r#"View{
    width: Fill
    height: Fit
    flow: Right{wrap: true}
    spacing: 16.0
    align: Align{y: 0.0}

    mod.widgets.ShadCard{
        width: 280

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

                    ShadFieldLabel{text: "Current plan"}
                    ShadFieldDescription{text: "Pro workspace with advanced sharing controls."}
                }

                ShadBadgeSecondary{
                    label := ShadBadgeSecondaryLabel{text: "Pro"}
                }
            }

            ShadHr{}

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 10.0

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}

                    ShadFieldDescription{text: "Seats in use"}
                    ShadLabel{
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 11
                        text: "18 of 25"
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}

                    ShadFieldDescription{text: "Pending invites"}
                    ShadLabel{
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 11
                        text: "3 awaiting acceptance"
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}

                    ShadFieldDescription{text: "Renewal"}
                    ShadLabel{
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 11
                        text: "September 30"
                    }
                }
            }
        }

        footer := mod.widgets.ShadCardFooter{
            mod.widgets.ShadButtonGhost{text: "Cancel"}
            mod.widgets.ShadButton{text: "Review changes"}
        }
    }

    mod.widgets.ShadCard{
        width: 280

        header := mod.widgets.ShadCardHeader{
            title := mod.widgets.ShadCardTitle{text: "API Usage"}
            description := mod.widgets.ShadCardDescription{text: "Last 7 days"}
        }

        content := mod.widgets.ShadCardContent{
            View{
                width: Fill
                height: Fit
                flow: Right
                align: Align{y: 0.5}
                spacing: 12.0

                ShadLabel{
                    width: Fill
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22
                    text: "3.4M"
                }

                ShadBadgeSuccess{
                    label := ShadBadgeSuccessLabel{text: "Live"}
                }
            }

            ShadFieldDescription{text: "Requests processed across webhooks, sync jobs, and realtime events."}

            ShadHr{}

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 8.0

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}

                    ShadFieldDescription{text: "Error rate"}
                    ShadLabel{
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 11
                        text: "0.12%"
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}

                    ShadFieldDescription{text: "Region"}
                    ShadLabel{
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 11
                        text: "North America"
                    }
                }
            }
        }
    }
}"#;
pub const CAROUSEL_PREVIEW_CODE: &str = r#"carousel_demo := mod.widgets.ShadCarousel{
    width: Fill

    content_wrap +: {
        carousel_flip +: {
            slide_0 +: {
                surface +: {
                    text_col +: {
                        eyebrow +: {
                            label +: {text: "Launch week"}
                        }
                        title +: {text: "Launch reliable realtime delivery from one surface"}
                        description +: {text: "Pair product highlights with a live media panel so release notes, event status, and rollout messaging stay aligned."}
                        meta +: {text: "Use a carousel when each panel builds on the previous one."}
                    }
                    media +: {
                        image +: {
                            src: crate_resource("self://resources/carousel/highlight-a.jpg")
                        }
                    }
                }
            }

            slide_1 +: {
                surface +: {
                    text_col +: {
                        eyebrow +: {
                            label +: {text: "Automation"}
                        }
                        title +: {text: "Stage onboarding and approvals without breaking the story"}
                        description +: {text: "Keep one focused message per panel: explain the workflow, show the supporting visual, then let the user move forward."}
                        meta +: {text: "Good carousel slides are related; they are not a random set of dashboard cards."}
                    }
                    media +: {
                        image +: {
                            src: crate_resource("self://resources/carousel/highlight-b.jpg")
                        }
                    }
                }
            }

            slide_2 +: {
                surface +: {
                    text_col +: {
                        eyebrow +: {
                            label +: {text: "Insights"}
                        }
                        title +: {text: "Close the loop with one outcome-focused summary"}
                        description +: {text: "The final slide should reinforce the sequence with proof, recap, or next-step context instead of starting a new thread."}
                        meta +: {text: "This pattern fits feature tours, launches, and related storytelling modules."}
                    }
                    media +: {
                        image +: {
                            src: crate_resource("self://resources/carousel/highlight-c.jpg")
                        }
                    }
                }
            }
        }
    }
}

// Controller example (Rust):
// let carousel = self.ui.shad_carousel(cx, ids!(carousel_demo));
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
pub const COLLAPSIBLE_PREVIEW_CODE: &str = r#"ShadCollapsible{
    margin: Inset{top: 12, right: 12}
    title: "Order #4189"
    is_open: true
    body: View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0

        ShadSurface{
            width: Fill
            height: Fit
            flow: Right
            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
            draw_bg +: {
                color: #0000
                border_size: 1.0
                border_radius: 6.0
                border_color: (shad_theme.color_outline_border)
            }

            ShadSectionHeader{
                width: Fill
                text: "Status"
            }
            ShadLabel{
                text: "Shipped"
                draw_text.text_style.font_size: 10
            }
        }

        ShadSurface{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
            spacing: 4.0
            draw_bg +: {
                color: #0000
                border_size: 1.0
                border_radius: 6.0
                border_color: (shad_theme.color_outline_border)
            }

            ShadLabel{
                text: "Shipping address"
                draw_text.text_style.font_size: 10
            }
            ShadSectionHeader{ text: "100 Market St, San Francisco" }
        }

        ShadSurface{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
            spacing: 4.0
            draw_bg +: {
                color: #0000
                border_size: 1.0
                border_radius: 6.0
                border_color: (shad_theme.color_outline_border)
            }

            ShadLabel{
                text: "Items"
                draw_text.text_style.font_size: 10
            }
            ShadSectionHeader{ text: "2x Studio Headphones" }
        }
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
