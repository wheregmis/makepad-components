use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySliderPage,
    page: slider_page,
    title: "Slider",
    subtitle: "Sliders expose continuous and committed values separately, so pages can preview during drag and commit on release.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        volume_slider := ShadSlider{default: 0.5}
        ShadSlider{default: 0.8}
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Name the slider you want to observe, then get it with view.slider(cx, ids!(volume_slider))."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use slided(actions) for live preview while the thumb is moving."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use end_slide(actions) when you want to commit the final value to saved state."}
        mod.widgets.GalleryActionFlowStep{text: "4. Restore or override the control from outside with set_value(cx, f64), and inspect the current value() when needed."}
    },
}
