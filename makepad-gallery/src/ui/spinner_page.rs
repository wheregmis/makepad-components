use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySpinnerPage,
    page: spinner_page,
    title: "Spinner",
    subtitle: "Circular loading indicator. Use for async operations and loading states.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{x: 0.5, y: 0.5}

            ShadSpinner{}
        }
    },
}
