use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    root: ShadScrollArea,
    widget: GalleryScrollAreaPage,
    page: scroll_area_page,
    title: "Scroll Area",
    subtitle: "Canonical scroll wrappers for vertical, horizontal, and two-axis overflow.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Vertical" }
        ShadPanel{
            scroll_area_demo := ShadScrollArea{
                width: Fill
                height: 220

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 10.0

                    ShadLabel{text: "Recent activity"}
                    ShadSeparator{}
                    ShadLabel{text: "Project Alpha updated 2 minutes ago"}
                    ShadLabel{text: "Billing statement exported"}
                    ShadLabel{text: "New teammate invited to workspace"}
                    ShadLabel{text: "API key rotated successfully"}
                    ShadLabel{text: "Audit log downloaded"}
                    ShadLabel{text: "Staging deploy completed"}
                    ShadLabel{text: "Design review scheduled for Friday"}
                    ShadLabel{text: "Feature flag enabled for beta cohort"}
                    ShadLabel{text: "Customer note added to ticket #438"}
                }
            }
        }

        ShadSectionHeader{ text: "Horizontal" }
        ShadPanel{
            ShadScrollAreaX{
                width: Fill
                height: Fit

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 12.0

                    ShadBadge{ label := ShadBadgeLabel{text: "Analytics"} }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Retention"} }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Growth"} }
                    ShadBadge{ label := ShadBadgeLabel{text: "Revenue"} }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Operations"} }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Launch Week"} }
                }
            }
        }
    },
}
