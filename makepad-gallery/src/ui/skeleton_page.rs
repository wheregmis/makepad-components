use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySkeletonPage,
    page: skeleton_page,
    title: "Skeleton",
    subtitle: "Loading-state scaffolding that mirrors the final layout while page or app data is unresolved. Tune the shimmer with animate, animation_fps, and shimmer_speed.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        skeleton_demo_shell := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadFieldDescription{
                width: Fill
                text: "Skeletons should mimic the final layout geometry closely so loading states feel stable when the real content replaces them."
            }

            ShadSurface{
                width: Fill
                height: Fit
                flow: Down
                spacing: 14.0
                padding: Inset{left: 18, right: 18, top: 18, bottom: 18}
                draw_bg +: {
                    color: (shad_theme.color_secondary)
                    border_size: 1.0
                    border_color: (shad_theme.color_outline_border)
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 8.0

                    ShadSectionHeader{ text: "Profile row placeholder" }
                    ShadFieldDescription{
                        width: Fill
                        text: "Use circles, title bars, and secondary lines to match the eventual avatar row layout instead of showing generic blocks."
                    }

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
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 8.0

                            ShadSkeleton{
                                width: 220
                                height: 16
                            }
                            ShadSkeleton{
                                width: 160
                                height: 14
                            }
                        }
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 8.0

                    ShadSectionHeader{ text: "Card content placeholder" }
                    ShadFieldDescription{
                        width: Fill
                        text: "For cards or detail panes, keep the same block rhythm the final content will use so the page does not jump when data resolves."
                    }

                    ShadSurface{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 10.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        draw_bg +: {
                            color: (shad_theme.color_background)
                            border_size: 1.0
                            border_color: (shad_theme.color_outline_border)
                        }

                        ShadSkeleton{ width: 180 height: 18 }
                        ShadSkeleton{ width: Fill height: 14 }
                        ShadSkeleton{ width: Fill height: 14 }
                        ShadSkeleton{ width: 260 height: 14 }

                        View{
                            width: Fit
                            height: Fit
                            flow: Right
                            spacing: 8.0

                            ShadSkeleton{ width: 96 height: 32 }
                            ShadSkeleton{ width: 72 height: 32 animate: false }
                        }
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 8.0

                    ShadSectionHeader{ text: "Animation tuning" }
                    ShadFieldDescription{
                        width: Fill
                        text: "animate toggles shimmer entirely. animation_fps controls redraw cadence, and shimmer_speed changes how quickly the highlight sweeps across the placeholder."
                    }

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 10.0

                        ShadSkeleton{ width: Fill height: 14 }
                        ShadSkeleton{ width: Fill height: 14 shimmer_speed: 0.8 }
                        ShadSkeleton{ width: Fill height: 14 animation_fps: 12.0 }
                        ShadSkeleton{ width: Fill height: 14 animate: false }
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Match the skeleton geometry to the final layout so the loading state and resolved content occupy the same structure."}
        mod.widgets.GalleryActionFlowStep{text: "2. Show skeletons only while page or app data is unresolved, then swap in the real content from page code when the data arrives."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use animate, animation_fps, and shimmer_speed to soften or disable motion when the surface is dense or should feel calmer."}
        mod.widgets.GalleryActionFlowStep{text: "4. Treat ShadSkeleton as a presentational placeholder, not a stateful loader or fetch controller."}
    },
}
