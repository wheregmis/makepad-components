use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    root: ShadScrollArea,
    widget: GalleryScrollAreaPage,
    page: scroll_area_page,
    title: "Scroll Area",
    subtitle: "Canonical scroll wrappers for vertical, horizontal, and two-axis overflow, including centered-content patterns now supported by the current Makepad dev branch.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Vertical" }
        ShadCard{
            spacing: 12.0
            padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

            ShadFieldDescription{
                width: Fill
                text: "Use a bounded scroll surface when surrounding content should stay fixed and readable."
            }

            scroll_area_demo := ShadScrollArea{
                width: Fill
                height: 220

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 10.0

                    ShadLabel{text: "Recent activity"}
                    ShadSeparator{}
                    ShadLabel{text: "Project Alpha updated 2 minutes ago"}
                    ShadLabel{text: "Billing statement exported"}
                    ShadLabel{text: "New teammate invited to workspace"}
                    ShadLabel{text: "API key rotated successfully"}
                    ShadLabel{text: "Audit log downloaded"}
                    ShadLabel{text: "Staging deploy completed"}
                    ShadLabel{text: "Design review scheduled for Friday"}
                    ShadLabel{text: "Feature flag enabled for beta cohort"}
                    ShadLabel{text: "Customer note added to ticket #438"}
                }
            }
        }

        ShadSectionHeader{ text: "Horizontal" }
        ShadCard{
            spacing: 12.0
            padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

            ShadFieldDescription{
                width: Fill
                text: "Horizontal rails should still feel surfaced and navigable, not like loose chips floating on the page."
            }

            ShadScrollAreaX{
                width: Fill
                height: Fit

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 12.0

                    ShadBadge{ label := ShadBadgeLabel{text: "Analytics"} }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Retention"} }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Growth"} }
                    ShadBadge{ label := ShadBadgeLabel{text: "Revenue"} }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Operations"} }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Launch Week"} }
                    ShadBadge{ label := ShadBadgeLabel{text: "Experiment Cohort"} }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Infrastructure"} }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Feature Flags"} }
                    ShadBadge{ label := ShadBadgeLabel{text: "Release Train"} }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Customer Success"} }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "Incident Review"} }
                    ShadBadge{ label := ShadBadgeLabel{text: "Commit History"} }
                    ShadBadgeSecondary{ label := ShadBadgeSecondaryLabel{text: "Datepicker"} }
                    ShadBadgeOutline{ label := ShadBadgeOutlineLabel{text: "User Selector"} }
                }
            }
        }

        ShadSectionHeader{ text: "Centered horizontal overflow" }
        ShadCard{
            spacing: 12.0
            padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

            ShadFieldDescription{
                width: Fill
                text: "The current Makepad dev branch supports horizontal scrolling even when the scroll container keeps its content centered while it still fits. Put the align on the scrollable view itself, not on a separate wrapper. This demo intentionally includes multiple oversized rows so you can clearly tell horizontal scrolling is active."
            }

            ShadScrollAreaX{
                width: 250
                height: Fit
                flow: Down
                align: Align{x: 0.5, y: 0.0}

                RoundedView{
                    width: 520
                    height: 44
                    new_batch: true
                    align: Align{x: 0.5, y: 0.5}
                    padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    ShadLabel{text: "Wide centered row A — User selector · Datepicker · Commit History · scroll right to read the full line."}
                }

                RoundedView{
                    width: 680
                    height: 44
                    new_batch: true
                    align: Align{x: 0.5, y: 0.5}
                    padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                    draw_bg +: {
                        color: (shad_theme.color_background)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border_hover)
                    }

                    ShadLabel{text: "Wide centered row B — Commits on Apr 7, 2026 · Support scrolling while centered, both vertically and horizontally (#1017)."}
                }

                RoundedView{
                    width: 160
                    height: 36
                    new_batch: true
                    align: Align{x: 0.5, y: 0.5}
                    padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                    draw_bg +: {
                        color: (shad_theme.color_muted)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    ShadFieldDescription{text: "Narrow row still centers when it fits."}
                }

                RoundedView{
                    width: 740
                    height: 44
                    new_batch: true
                    align: Align{x: 0.5, y: 0.5}
                    padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                    draw_bg +: {
                        color: (shad_theme.color_secondary)
                        border_radius: (shad_theme.radius)
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    ShadLabel{text: "Wide centered row C — This row is deliberately long so the horizontal scrollbar and drag-scroll behavior are unmistakable in the gallery preview."}
                }
            }
        }
    },
}
