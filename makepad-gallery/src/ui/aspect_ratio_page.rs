use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAspectRatioPage,
    page: aspect_ratio_page,
    title: "Aspect Ratio",
    subtitle: "Ratio-constrained frames for photos, video, and media surfaces. Let the shell own the shape, then let the child content fill it.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Anatomy" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "ShadAspectRatio owns the frame dimensions. Give it a width or height and a ratio, then let the child fill that frame."}
            ShadFieldDescription{text: "For media, wrap the child in a clipped rounded container and let ShadImage use an explicit fit mode. ImageFit.Biggest gives you a cover-style crop."}
            ShadFieldDescription{text: "Bound larger demos with a deliberate width. A fill-width hero technically works, but it overwhelms the page and weakens the component story."}
        }

        ShadHr{}

        ShadSectionHeader{ text: "16:9 Hero" }

        View{
            width: Fill
            height: Fit
            flow: Down
            align: Align{x: 0.5, y: 0.0}
            spacing: 10.0

            ShadAspectRatio{
                width: Fill
                ratio: 1.7777777778

                ShadMediaFrame{
                    width: Fill
                    height: Fill
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    ShadImage{
                        width: Fill
                        height: Fill
                        fit: ImageFit.Biggest
                        src: crate_resource("self://resources/aspect-ratio/royal-esplanade.jpg")
                    }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 4.0

                ShadFieldLabel{text: "16:9 cover crop"}
                ShadFieldDescription{text: "A bounded hero keeps the ratio demonstration readable while the photo fills the frame edge to edge."}
            }
        }

        ShadSectionHeader{ text: "Common Ratios" }

        View{
            width: Fill
            height: Fit
            flow: Right{wrap: true}
            spacing: 16.0

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0

                ShadAspectRatio{
                    width: 160
                    ratio: 1.0

                    ShadMediaFrame{
                        width: Fill
                        height: Fill
                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_size: 1.0
                            border_color: (shad_theme.color_outline_border)
                        }

                        ShadImage{
                            width: Fill
                            height: Fill
                            fit: ImageFit.Biggest
                            src: crate_resource("self://resources/aspect-ratio/portrait.jpg")
                        }
                    }
                }

                ShadFieldLabel{text: "1:1"}
                ShadFieldDescription{
                    width: 160
                    text: "Square avatar or gallery crop."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0

                ShadAspectRatio{
                    width: 200
                    ratio: 1.3333333333

                    ShadMediaFrame{
                        width: Fill
                        height: Fill
                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_size: 1.0
                            border_color: (shad_theme.color_outline_border)
                        }

                        ShadImage{
                            width: Fill
                            height: Fill
                            fit: ImageFit.Biggest
                            src: crate_resource("self://resources/aspect-ratio/royal-esplanade.jpg")
                        }
                    }
                }

                ShadFieldLabel{text: "4:3"}
                ShadFieldDescription{
                    width: 200
                    text: "Editorial image or embedded document frame."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0

                ShadAspectRatio{
                    width: 132
                    ratio: 0.5625

                    ShadMediaFrame{
                        width: Fill
                        height: Fill
                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_size: 1.0
                            border_color: (shad_theme.color_outline_border)
                        }

                        ShadImage{
                            width: Fill
                            height: Fill
                            fit: ImageFit.Biggest
                            src: crate_resource("self://resources/aspect-ratio/royal-esplanade.jpg")
                        }
                    }
                }

                ShadFieldLabel{text: "9:16"}
                ShadFieldDescription{
                    width: 132
                    text: "Portrait-first crop for reels or mobile stories."
                }
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "Usage" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Use ShadAspectRatio as the layout shell, not as an image widget. The child decides how media is fit, clipped, and decorated."}
            ShadFieldDescription{text: "Use ImageFit.Biggest when you want a cover crop that fills the frame. Use ImageFit.Smallest when you want the entire image visible inside the ratio."}
            ShadFieldDescription{text: "If you need rounded corners, overlays, or borders, put them on a clipped child container inside ShadAspectRatio so the ratio math stays separate from the media treatment."}
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Choose the ratio shell first. The parent page decides whether the frame should be 16:9, 4:3, 1:1, or something custom."}
        mod.widgets.GalleryActionFlowStep{text: "2. Give ShadAspectRatio one controlling dimension. Width-driven layouts are the most common for media cards and gallery rows."}
        mod.widgets.GalleryActionFlowStep{text: "3. Put a clipped child container inside it, then let ShadImage fill that container with an explicit fit mode such as ImageFit.Biggest."}
        mod.widgets.GalleryActionFlowStep{text: "4. Add borders, badges, or overlays on the child container instead of baking those concerns into the aspect-ratio primitive itself."}
    },
}
