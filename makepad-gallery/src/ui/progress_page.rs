use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryProgressPage,
    page: progress_page,
    title: "Progress",
    subtitle: "Shadcn-inspired progress bars. Determinate (value 0–1) and indeterminate (animated).",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Determinate" }

        ShadProgress33{}
        ShadProgress66{}
        ShadProgressFull{}

        ShadSectionHeader{ text: "Indeterminate (animated)" }

        ShadProgressIndeterminate{}

        ShadSectionHeader{ text: "Animation stress" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 8.0

            ShadProgressIndeterminate{}
            ShadProgressIndeterminate{}
            ShadProgressIndeterminate{}
            ShadProgressIndeterminate{}
        }
    },
}
