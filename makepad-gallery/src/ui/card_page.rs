use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryCardPage,
    page: card_page,
    title: "Card",
    subtitle: "Contained surfaces for grouped content, metadata, and related actions.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Anatomy" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Use ShadCard as a contained surface for one related slice of information. It should group content that belongs together, not become a generic page shell."}
            ShadFieldDescription{text: "Header, content, and footer are compositional. Omit any section you do not need, but keep the title and description close to the content they explain."}
            ShadFieldDescription{text: "Let the card body carry the story: metadata rows, helper copy, badges, and buttons should reinforce one task or summary instead of becoming a grab bag of unrelated controls."}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Examples" }

        View{
            width: Fill
            height: Fit
            flow: Right{wrap: true}
            spacing: 16.0
            align: Align{y: 0.0}

            mod.widgets.ShadCard{
                width: 280

                header := mod.widgets.ShadCardHeader{
                    title := mod.widgets.ShadCardTitle{text: "Team Access"}
                    description := mod.widgets.ShadCardDescription{text: "Review seats, pending invites, and billing impact before applying changes."}
                }

                content := mod.widgets.ShadCardContent{
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

                            ShadFieldLabel{text: "Current plan"}
                            ShadFieldDescription{text: "Pro workspace with advanced sharing controls."}
                        }

                        ShadBadgeSecondary{
                            label := ShadBadgeSecondaryLabel{text: "Pro"}
                        }
                    }

                    ShadHr{}

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 10.0

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{y: 0.5}

                            ShadFieldDescription{text: "Seats in use"}
                            ShadLabel{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 11
                                text: "18 of 25"
                            }
                        }

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{y: 0.5}

                            ShadFieldDescription{text: "Pending invites"}
                            ShadLabel{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 11
                                text: "3 awaiting acceptance"
                            }
                        }

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{y: 0.5}

                            ShadFieldDescription{text: "Renewal"}
                            ShadLabel{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 11
                                text: "September 30"
                            }
                        }
                    }
                }

                footer := mod.widgets.ShadCardFooter{
                    mod.widgets.ShadButtonGhost{text: "Cancel"}
                    mod.widgets.ShadButton{text: "Review changes"}
                }
            }

            mod.widgets.ShadCard{
                width: 280

                header := mod.widgets.ShadCardHeader{
                    title := mod.widgets.ShadCardTitle{text: "API Usage"}
                    description := mod.widgets.ShadCardDescription{text: "Last 7 days"}
                }

                content := mod.widgets.ShadCardContent{
                    View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        spacing: 12.0

                        ShadLabel{
                            width: Fill
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 22
                            text: "3.4M"
                        }

                        ShadBadgeSuccess{
                            label := ShadBadgeSuccessLabel{text: "Live"}
                        }
                    }

                    ShadFieldDescription{text: "Requests processed across webhooks, sync jobs, and realtime events."}

                    ShadHr{}

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 8.0

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{y: 0.5}

                            ShadFieldDescription{text: "Error rate"}
                            ShadLabel{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 11
                                text: "0.12%"
                            }
                        }

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            align: Align{y: 0.5}

                            ShadFieldDescription{text: "Region"}
                            ShadLabel{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 11
                                text: "North America"
                            }
                        }
                    }
                }
            }
        }
    },
}
