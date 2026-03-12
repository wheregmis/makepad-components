use crate::ui::snippets::SLIDER_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySliderPage = ShadScrollYView{
        ShadPageTitle{
            text: "Slider"
        }

        ShadPageSubtitle{
            text: "Shadcn-style range slider. Extends makepad Slider with theme colors."
        }

        ShadHr{}

        slider_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            slider_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                slider_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    slider_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    slider_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                slider_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    slider_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    slider_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            slider_preview_panel := mod.widgets.ShadPanel{
                slider_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 16.0

                        ShadSlider{default: 0.5}
                        ShadSlider{default: 0.8}
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(SLIDER_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
