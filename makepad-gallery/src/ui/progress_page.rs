use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryProgressPage,
    page: progress_page,
    title: "Progress",
    subtitle: "Shadcn-inspired progress bars. Determinate bars expose `value:` in the 0-1 range, while indeterminate bars stay as a separate animated component.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Determinate" }

        ShadProgress{ value: 0.33 }
        ShadProgress{ value: 0.66 }
        ShadProgress{ value: 1.0 }

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
