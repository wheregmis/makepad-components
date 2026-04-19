use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryCatalogPage,
    page: catalog_page,
    title: "Component Catalog",
    subtitle: "A birds-eye view of all components in the library. Click any category in the sidebar for detailed documentation and code examples.",
    divider: { ShadHr{} },
    preview_spacing: 48.0,
    preview: {
        ShadPageTitle{ text: "Core Components" margin: Inset{top: 0} }

        View {
            width: Fill, height: Fit, flow: Down, spacing: 32.0
            
            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Buttons & Interaction" }
                View {
                    width: Fill, height: Fit, flow: Right, spacing: 12.0
                    ShadButton { text: "Default" }
                    ShadButtonSecondary { text: "Secondary" }
                    ShadButtonOutline { text: "Outline" }
                    ShadButtonGhost { text: "Ghost" }
                    ShadButtonDestructive { text: "Destructive" }
                }
            }

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Badges & Labels" }
                View {
                    width: Fill, height: Fit, flow: Right, spacing: 12.0
                    ShadBadge { ShadBadgeLabel { text: "Badge" } }
                    ShadBadgeSecondary { ShadBadgeSecondaryLabel { text: "Secondary" } }
                    ShadBadgeOutline { ShadBadgeOutlineLabel { text: "Outline" } }
                    ShadBadgeDestructive { ShadBadgeDestructiveLabel { text: "Destructive" } }
                }
            }

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Avatars" }
                View {
                    width: Fill, height: Fit, flow: Right, spacing: 12.0
                    ShadAvatar {
                        fallback := { text: "JD" }
                    }
                    ShadAvatar {
                        status := ShadAvatarStatusOnline {}
                        fallback := { text: "JD" }
                    }
                }
            }
        }

        ShadHr{}

        ShadPageTitle{ text: "Forms & Input" }

        View {
            width: Fill, height: Fit, flow: Down, spacing: 32.0

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Inputs" }
                ShadInput {
                    text: "Search..."
                }
                ShadTextarea {
                    text: "Write your message here."
                }
            }

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Selection" }
                View {
                    width: Fill, height: Fit, flow: Right, spacing: 24.0
                    ShadCheckbox { label: "Check me" }
                    ShadSwitch { text: "Toggle me" }
                }
                ShadSlider {
                    default: 0.5
                }
            }
        }

        ShadHr{}

        ShadPageTitle{ text: "Feedback" }

        View {
            width: Fill, height: Fit, flow: Down, spacing: 32.0

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Alerts" }
                ShadAlert {
                    ShadAlertIcon {}
                    ShadAlertContent {
                        ShadAlertTitle { text: "Heads up!" }
                        ShadAlertDescription { text: "You can add components to your app with a single copy-paste." }
                    }
                }
            }

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Progress & Spinners" }
                View {
                    width: Fill, height: Fit, flow: Right, align: Align{y: 0.5}, spacing: 24.0
                    ShadProgress { width: 200, draw_bg +: { progress: instance(0.7) } }
                    ShadSpinner {}
                }
            }
        }

        ShadHr{}

        ShadPageTitle{ text: "Navigation & Layout" }

        View {
            width: Fill, height: Fit, flow: Down, spacing: 32.0

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Breadcrumbs" }
                ShadBreadcrumb {
                    ShadBreadcrumbLink { text: "Home" }
                    ShadBreadcrumbSeparator {}
                    ShadBreadcrumbLink { text: "Components" }
                    ShadBreadcrumbSeparator {}
                    ShadBreadcrumbPage { text: "Catalog" }
                }
            }

            View {
                width: Fill, height: Fit, flow: Down, spacing: 16.0
                ShadSectionHeader { text: "Tabs" }
                ShadTabs {
                    width: 400
                    ShadTabsList {
                        ShadTabsTrigger { text: "Account" }
                        ShadTabsTrigger { text: "Password" }
                    }
                }
            }
        }
    }
}
