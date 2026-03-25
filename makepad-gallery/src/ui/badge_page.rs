use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryBadgePage,
    page: badge_page,
    title: "Badge",
    subtitle: "Short, non-interactive tags for state and metadata that sit beside the content they annotate. Configure emphasis with `tone:` on the badge itself.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "In Context" }

        ShadSurface{
            variant: ShadSurfaceVariant.Muted
            width: Fill
            height: Fit
            flow: Down
            spacing: 14.0
            padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
            draw_bg +: {
                border_size: 1.0
                border_color: (shad_theme.color_outline_border)
            }

            View{
                width: Fill
                height: Fit
                flow: Right
                align: Align{y: 0.5}
                spacing: 12.0

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 2.0

                    ShadFieldLabel{text: "Realtime API"}
                    ShadFieldDescription{text: "Production webhook delivery is enabled for connected workspaces."}
                }

                ShadBadge{
                    tone: ShadBadgeTone.Success
                    text: "Live"
                }
            }

            ShadHr{}

            View{
                width: Fill
                height: Fit
                flow: Right
                align: Align{y: 0.5}
                spacing: 12.0

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 2.0

                    ShadFieldLabel{text: "Usage Analytics"}
                    ShadFieldDescription{text: "Rolling out to selected teams before the wider release."}
                }

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 8.0

                    ShadBadge{
                        tone: ShadBadgeTone.Warning
                        text: "Beta"
                    }
                    ShadBadge{
                        tone: ShadBadgeTone.Secondary
                        text: "Internal"
                    }
                }
            }

            ShadHr{}

            View{
                width: Fill
                height: Fit
                flow: Right
                align: Align{y: 0.5}
                spacing: 12.0

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 2.0

                    ShadFieldLabel{text: "Legacy Sync"}
                    ShadFieldDescription{text: "Scheduled for removal after the new importer reaches parity."}
                }

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 8.0

                    ShadBadge{
                        tone: ShadBadgeTone.Destructive
                        text: "Deprecated"
                    }
                    ShadBadge{
                        tone: ShadBadgeTone.Outline
                        text: "Archived"
                    }
                }
            }
        }

        ShadSectionHeader{ text: "Variants" }

        ShadSurface{
            variant: ShadSurfaceVariant.Muted
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0
            padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
            draw_bg +: {
                border_size: 1.0
                border_color: (shad_theme.color_outline_border)
            }

            View{
                width: Fill
                height: Fit
                flow: Right
                spacing: 8.0

                ShadBadge{text: "Default"}
                ShadBadge{ tone: ShadBadgeTone.Secondary text: "Secondary" }
                ShadBadge{ tone: ShadBadgeTone.Outline text: "Outline" }
            }

            View{
                width: Fill
                height: Fit
                flow: Right
                spacing: 8.0

                ShadBadge{ tone: ShadBadgeTone.Success text: "Success" }
                ShadBadge{ tone: ShadBadgeTone.Warning text: "Warning" }
                ShadBadge{ tone: ShadBadgeTone.Destructive text: "Destructive" }
            }

            ShadFieldDescription{
                text: "Use default, secondary, or outline for neutral metadata. Switch to success, warning, or destructive only when the label itself communicates state."
            }
        }
    },
}
