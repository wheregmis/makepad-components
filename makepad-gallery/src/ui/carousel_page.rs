use crate::ui::snippets::CAROUSEL_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCarouselPage = ShadScrollYView{
        ShadPageTitle{
            text: "Carousel"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired carousel with prev/next navigation and slide indicators."
        }

        ShadHr{}

        carousel_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            carousel_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                carousel_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    carousel_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    carousel_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                carousel_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    carousel_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    carousel_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            carousel_preview_panel := mod.widgets.ShadPanel{
                carousel_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Default" }

                        carousel_demo := mod.widgets.ShadCarousel{}
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(CAROUSEL_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
