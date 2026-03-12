use crate::ui::snippets::SPINNER_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySpinnerPage = ShadScrollYView{
        ShadPageTitle{
            text: "Spinner"
        }

        ShadPageSubtitle{
            text: "Circular loading indicator. Use for async operations and loading states."
        }

        ShadHr{}

        spinner_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            spinner_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                spinner_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    spinner_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    spinner_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                spinner_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    spinner_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    spinner_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            spinner_preview_panel := mod.widgets.ShadPanel{
                spinner_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 24.0
                            align: Align{x: 0.5, y: 0.5}

                            ShadSpinner{}
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(SPINNER_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
