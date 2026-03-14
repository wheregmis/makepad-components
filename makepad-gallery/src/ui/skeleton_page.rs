use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySkeletonPage,
    page: skeleton_page,
    title: "Skeleton",
    subtitle: "Use to show a placeholder while content is loading.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Preview" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 12.0
            align: Align{x: 0.0, y: 0.5}

            ShadSkeleton{
                width: 48
                height: 48
                draw_bg.border_radius: 24.0
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0

                ShadSkeleton{
                    width: 200
                    height: 16
                }
                ShadSkeleton{
                    width: 150
                    height: 16
                }
            }
        }

        ShadSectionHeader{ text: "Stress stack" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0

            View{
                width: Fill
                height: Fit
                flow: Right
                spacing: 12.0

                ShadSkeleton{ width: 40 height: 40 draw_bg.border_radius: 20.0 }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 6.0
                    ShadSkeleton{ width: Fill height: 14 }
                    ShadSkeleton{ width: 220 height: 14 }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Right
                spacing: 12.0

                ShadSkeleton{ width: 40 height: 40 draw_bg.border_radius: 20.0 }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 6.0
                    ShadSkeleton{ width: Fill height: 14 }
                    ShadSkeleton{ width: 180 height: 14 }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Right
                spacing: 12.0

                ShadSkeleton{ width: 40 height: 40 draw_bg.border_radius: 20.0 }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 6.0
                    ShadSkeleton{ width: Fill height: 14 }
                    ShadSkeleton{ width: 240 height: 14 }
                }
            }
        }
    },
}
