use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAspectRatioPage,
    page: aspect_ratio_page,
    title: "Aspect Ratio",
    subtitle: "Displays content within a desired ratio.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "16:9 Preview" }

        RoundedView{
            width: Fill
            height: Fit
            padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
            draw_bg +: {
                color: #0000
                border_size: 1.0
                border_radius: (shad_theme.radius)
                border_color: (shad_theme.color_outline_border)
            }

            ShadAspectRatio{
                width: Fill
                ratio: 1.7777777778

                RoundedView{
                    width: Fill
                    height: Fill
                    flow: Overlay
                    align: Align{x: 0.5, y: 0.5}
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                    }

                    ShadSectionHeader{
                        text: "16:9"
                        draw_text.text_style.font_size: 11
                    }
                }
            }
        }

        ShadSectionHeader{ text: "Common Ratios" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 12.0

            ShadAspectRatio{
                width: 180
                ratio: 1.0

                RoundedView{
                    width: Fill
                    height: Fill
                    flow: Overlay
                    align: Align{x: 0.5, y: 0.5}
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                    }
                    ShadSectionHeader{ text: "1:1" }
                }
            }

            ShadAspectRatio{
                width: 180
                ratio: 1.3333333333

                RoundedView{
                    width: Fill
                    height: Fill
                    flow: Overlay
                    align: Align{x: 0.5, y: 0.5}
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                    }
                    ShadSectionHeader{ text: "4:3" }
                }
            }

            ShadAspectRatio{
                width: 180
                ratio: 0.5625

                RoundedView{
                    width: Fill
                    height: Fill
                    flow: Overlay
                    align: Align{x: 0.5, y: 0.5}
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                    }
                    ShadSectionHeader{ text: "9:16" }
                }
            }
        }
    },
}
