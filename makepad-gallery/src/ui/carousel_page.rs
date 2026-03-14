use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryCarouselPage,
    page: carousel_page,
    title: "Carousel",
    subtitle: "Shadcn-inspired carousel with prev/next navigation and slide indicators. Use ShadCarouselRef::next/prev/go_to and changed(actions) to sync app state.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        carousel_demo := mod.widgets.ShadCarousel{}
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Treat the carousel as a component that owns its prev/next buttons and dot wiring internally."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use a ShadCarouselRef when outside UI wants to move it: next(cx), prev(cx), or go_to(cx, index)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Listen to changed(actions) when labels, badges, analytics, or related content need the active slide index."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use current() when a page redraws or restores state and needs to know which slide is active now."}
    },
}
