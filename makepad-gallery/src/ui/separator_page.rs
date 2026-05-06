use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySeparatorPage,
    page: separator_page,
    title: "Separator",
    subtitle: "Horizontal and vertical dividers for visually grouping related content sections without adding heavy structural chrome.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Horizontal" }
        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                ShadLabel{text: "Account"}
                ShadFieldDescription{text: "Profile settings and personal information."}
                ShadSeparator{}
                ShadLabel{text: "Billing"}
                ShadFieldDescription{text: "Invoices, payment methods, and tax details."}
                ShadSeparator{}
                ShadLabel{text: "Security"}
                ShadFieldDescription{text: "Sessions, MFA, and access tokens."}
            }
        }

        ShadSectionHeader{ text: "Vertical" }
        ShadPanel{
            View{
                width: Fit
                height: 20
                flow: Right
                align: Align{x: 0.0, y: 0.5}
                spacing: 16.0

                ShadLabel{text: "Blog"}
                ShadSeparatorVertical{}
                ShadLabel{text: "Docs"}
                ShadSeparatorVertical{}
                ShadLabel{text: "Source"}
            }
        }

        ShadSectionHeader{ text: "Between cards" }
        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 0.0

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 12.0
                    padding: Inset{top: 10, bottom: 10, left: 0, right: 0}

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 2.0

                        ShadFieldLabel{text: "Workspace plan"}
                        ShadFieldDescription{text: "Pro · 25 seats included"}
                    }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Active"} }
                }

                ShadSeparator{}

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 12.0
                    padding: Inset{top: 10, bottom: 10, left: 0, right: 0}

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 2.0

                        ShadFieldLabel{text: "Billing cycle"}
                        ShadFieldDescription{text: "Annual · renews September 30"}
                    }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Annual"} }
                }

                ShadSeparator{}

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 12.0
                    padding: Inset{top: 10, bottom: 0, left: 0, right: 0}

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 2.0

                        ShadFieldLabel{text: "Next invoice"}
                        ShadFieldDescription{text: "$299 due on October 1"}
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Use ShadSeparator between stacked content sections to create visual hierarchy without adding heavy structural chrome."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use ShadSeparatorVertical inside a horizontal flow to divide inline items such as breadcrumb links, navigation pills, or toolbar segments."}
        mod.widgets.GalleryActionFlowStep{text: "3. Keep spacing in the parent container, not inside the separator, so the divider stays reusable across different layout contexts."}
        mod.widgets.GalleryActionFlowStep{text: "4. Both variants are purely decorative layout primitives and carry no interactive state or semantic role."}
    },
}
