use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryCarouselPage,
    page: carousel_page,
    title: "Carousel",
    subtitle: "Sequential highlight surface with built-in prev/next controls and dot indicators for related content.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Anatomy" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "ShadCarousel owns slide switching, prev/next controls, and dot indicators internally. Treat it as one bounded storytelling surface."}
            ShadFieldDescription{text: "Use it for sequential highlights, onboarding steps, or related feature recaps. If people need to compare items side by side, static cards are usually the better choice."}
            ShadFieldDescription{text: "This first pass remains fixed to three named slides. Override `slide_0`, `slide_1`, and `slide_2` inside `content_wrap.carousel_flip` when you need custom content."}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Product Highlights" }

        View{
            width: Fill
            height: Fit
            flow: Down
            align: Align{x: 0.5, y: 0.0}
            spacing: 10.0

            carousel_demo := mod.widgets.ShadCarousel{
                width: Fill

                content_wrap +: {
                    carousel_flip +: {
                        slide_0 +: {
                            surface +: {
                                text_col +: {
                                    eyebrow +: {
                                        label +: {text: "Launch week"}
                                    }
                                    title +: {text: "Launch reliable realtime delivery from one surface"}
                                    description +: {text: "Pair product highlights with a live media panel so release notes, event status, and rollout messaging stay aligned."}
                                    meta +: {text: "Use a carousel when each panel builds on the previous one."}
                                }
                                media +: {
                                    image +: {
                                        src: crate_resource("self://resources/carousel/highlight-a.jpg")
                                    }
                                }
                            }
                        }

                        slide_1 +: {
                            surface +: {
                                text_col +: {
                                    eyebrow +: {
                                        label +: {text: "Automation"}
                                    }
                                    title +: {text: "Stage onboarding and approvals without breaking the story"}
                                    description +: {text: "Keep one focused message per panel: explain the workflow, show the supporting visual, then let the user move forward."}
                                    meta +: {text: "Good carousel slides are related; they are not a random set of dashboard cards."}
                                }
                                media +: {
                                    image +: {
                                        src: crate_resource("self://resources/carousel/highlight-b.jpg")
                                    }
                                }
                            }
                        }

                        slide_2 +: {
                            surface +: {
                                text_col +: {
                                    eyebrow +: {
                                        label +: {text: "Insights"}
                                    }
                                    title +: {text: "Close the loop with one outcome-focused summary"}
                                    description +: {text: "The final slide should reinforce the sequence with proof, recap, or next-step context instead of starting a new thread."}
                                    meta +: {text: "This pattern fits feature tours, launches, and related storytelling modules."}
                                }
                                media +: {
                                    image +: {
                                        src: crate_resource("self://resources/carousel/highlight-c.jpg")
                                    }
                                }
                            }
                        }
                    }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 4.0

                ShadFieldLabel{text: "Bounded 3-slide highlight module"}
                ShadFieldDescription{text: "Keep the carousel width deliberate so the slide content, buttons, and indicators read as one compact sequence instead of a full-page hero."}
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "Usage" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Use carousel when the panels are ordered and the user benefits from progressing through one related story. Product highlights, onboarding sequences, and feature recaps fit well."}
            ShadFieldDescription{text: "Prefer static cards or a grid when the user needs to scan multiple items at once, compare metrics, or jump non-linearly between unrelated content blocks."}
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep the carousel itself responsible for its prev/next buttons and dot wiring; page code should not try to reimplement those interactions."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use a ShadCarouselRef when surrounding UI needs to move it: next(cx), prev(cx), or go_to(cx, index)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Listen to changed(actions) when nearby copy, badges, analytics, or supporting UI should react to the active slide index."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use current() when restoring page state or when external UI needs to know which of the three named slides is visible."}
    },
}
