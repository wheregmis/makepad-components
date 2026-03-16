use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GallerySpinnerPage,
    page: spinner_page,
    title: "Spinner",
    subtitle: "Inline loading indicator for indeterminate waits, compact async actions, and small busy surfaces.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 8.0

                ShadSectionHeader{text: "Usage"}
                ShadFieldDescription{text: "Use spinner when the UI is waiting but cannot report percentage progress. Keep it close to the label or surface it supports."}
                ShadFieldDescription{text: "Use skeleton when the final content shape is known, and use progress when completion can be measured."}
            }
        }

        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 14.0

                ShadSectionHeader{text: "Inline loading states"}

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 12.0
                    align: Align{x: 0.0, y: 0.5}

                    ShadSurface{
                        width: Fit
                        height: Fit
                        flow: Right
                        spacing: 10.0
                        align: Align{y: 0.5}
                        padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_radius: (shad_theme.radius)
                            border_size: 1.0
                            border_color: (shad_theme.color_outline_border)
                        }

                        ShadSpinnerSm{}
                        ShadLabel{
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 11
                            text: "Saving changes..."
                        }
                    }
                }

                ShadSurfaceMuted{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 12.0
                    align: Align{y: 0.5}
                    padding: Inset{left: 16, right: 16, top: 14, bottom: 14}
                    draw_bg +: {
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    ShadSpinner{}

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 2.0

                        ShadFieldLabel{text: "Syncing workspace activity"}
                        ShadFieldDescription{text: "Fetching the latest comments and deploy events for this project."}
                    }
                }

                ShadSurfaceMuted{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 10.0
                    align: Align{x: 0.5}
                    padding: Inset{left: 20, right: 20, top: 20, bottom: 20}
                    draw_bg +: {
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    ShadSpinnerLg{}
                    ShadLabel{
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 13
                        text: "Loading activity"
                    }
                    ShadFieldDescription{
                        align: Align{x: 0.5}
                        text: "Preparing the latest deploys, incidents, and subscriber events."
                    }
                }
            }
        }

        ShadPanel{
            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                ShadSectionHeader{text: "Sizes"}

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 24.0
                    align: Align{x: 0.0, y: 0.5}

                    View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        align: Align{x: 0.5}

                        ShadSpinnerSm{}
                        ShadFieldDescription{text: "Sm"}
                    }

                    View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        align: Align{x: 0.5}

                        ShadSpinner{}
                        ShadFieldDescription{text: "Default"}
                    }

                    View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        align: Align{x: 0.5}

                        ShadSpinnerLg{}
                        ShadFieldDescription{text: "Lg"}
                    }
                }
            }
        }
    },
}
