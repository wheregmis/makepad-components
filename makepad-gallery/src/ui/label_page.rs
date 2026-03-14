use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryLabelPage,
    page: label_page,
    title: "Label",
    subtitle: "Shadcn-inspired accessible label associated with controls.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default Label" }

        ShadLabel{ text: "Your email address" }
    },
}
