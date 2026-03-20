use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryLabelPage,
    page: label_page,
    title: "Label",
    subtitle: "Presentational text primitive for nearby UI copy, summaries, and inline status text. Use ShadFieldLabel inside ShadField for form captions.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        label_demo_shell := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadFieldDescription{
                width: Fill
                text: "ShadLabel works best as primary nearby copy inside cards, settings rows, and inline UI summaries where the text supports surrounding content without acting like a form caption."
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

                    ShadSectionHeader{ text: "Card and detail copy" }
                    ShadFieldDescription{
                        width: Fill
                        text: "Use ShadLabel for the primary line of UI copy inside summary cards and metadata blocks."
                    }

                    View{
                        width: Fill
                        height: Fit
                        flow: Right{wrap: true}
                        spacing: 12.0
                        align: Align{y: 0.0}

                        ShadSurface{
                            width: 220
                            height: Fit
                            flow: Down
                            spacing: 6.0
                            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                            draw_bg +: {
                                color: (shad_theme.color_background)
                                border_size: 1.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            ShadLabel{text: "Current plan"}
                            ShadFieldDescription{text: "Pro workspace with annual billing."}
                        }

                        ShadSurface{
                            width: 220
                            height: Fit
                            flow: Down
                            spacing: 6.0
                            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                            draw_bg +: {
                                color: (shad_theme.color_background)
                                border_size: 1.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            ShadLabel{text: "Environment"}
                            ShadFieldDescription{text: "Production API connected."}
                        }

                        ShadSurface{
                            width: 220
                            height: Fit
                            flow: Down
                            spacing: 6.0
                            padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                            draw_bg +: {
                                color: (shad_theme.color_background)
                                border_size: 1.0
                                border_color: (shad_theme.color_outline_border)
                            }

                            ShadLabel{text: "Owner"}
                            ShadFieldDescription{text: "Design Systems Team"}
                        }
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 10.0

                    ShadSectionHeader{ text: "Inline UI copy" }
                    ShadFieldDescription{
                        width: Fill
                        text: "Named labels are useful for nearby status text or result summaries that the page may update from state."
                    }

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 10.0

                        View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 16.0
                            align: Align{y: 0.5}

                            sync_status := ShadLabel{
                                width: 160
                                text: "Last synced 2 minutes ago"
                            }
                            ShadFieldDescription{
                                width: Fill
                                text: "Good for status copy that sits beside actions, filters, or badges."
                            }
                        }

                        View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 16.0
                            align: Align{y: 0.5}

                            results_count := ShadLabel{
                                width: 160
                                text: "Showing 24 of 120 results"
                            }
                            ShadFieldDescription{
                                width: Fill
                                text: "Keep the visible string in page state and push updates back into the label when counts change."
                            }
                        }
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 8.0

                    ShadSectionHeader{ text: "Form guidance" }
                    ShadFieldDescription{
                        width: Fill
                        text: "When text belongs to an input stack, use ShadFieldLabel inside ShadField above ShadInput instead of reusing ShadLabel."
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Use ShadLabel for nearby primary copy in cards, settings rows, and inline summaries where the text is descriptive, not form-associated."}
        mod.widgets.GalleryActionFlowStep{text: "2. Give labels ids when page state needs to update them, then call view.label(cx, ids!(sync_status)).set_text(cx, ...)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Pair ShadLabel with ShadFieldDescription when you need a primary line plus supporting secondary copy in the same region."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use ShadFieldLabel inside ShadField for input captions and field stacks; keep that form pattern separate from ShadLabel."}
    },
}
