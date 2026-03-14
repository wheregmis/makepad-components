use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryBadgePage,
    page: badge_page,
    title: "Badge",
    subtitle: "Badge variants showcasing label, secondary, destructive, and outline styles.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Variants" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 12.0

            ShadBadge{
                label := ShadBadgeLabel{text: "Default"}
            }
            ShadBadgeSecondary{
                label := ShadBadgeSecondaryLabel{text: "Secondary"}
            }
            ShadBadgeDestructive{
                label := ShadBadgeDestructiveLabel{text: "Destructive"}
            }
            ShadBadgeOutline{
                label := ShadBadgeOutlineLabel{text: "Outline"}
            }
        }
    },
}
