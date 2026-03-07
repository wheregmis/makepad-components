use crate::ui::snippets::INPUT_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryInputPage = ShadScrollYView{
        ShadPageTitle{
            text: "Input"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired text input field component."
        }

        ShadHr{}

        input_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            input_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                input_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    input_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    input_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                input_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    input_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    input_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            input_preview_panel := mod.widgets.ShadPanel{
                input_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                ShadSectionHeader{ text: "Default" }

                View{
                    width: 320
                    height: Fit

                    ShadInput{
                        empty_text: "Email"
                    }
                }

                ShadHr{}

                ShadSectionHeader{ text: "Disabled" }

                View{
                    width: 320
                    height: Fit

                    ShadInput{
                        is_read_only: true
                        empty_text: "Read Only Value"
                    }
                }

                ShadHr{}

                ShadSectionHeader{ text: "With Label" }

                View{
                    width: 320
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    ShadLabel{ text: "Email" }
                    ShadInput{ empty_text: "Email" }
                }

                ShadSectionHeader{ text: "With Leading Icon" }

                View{
                    width: 320
                    height: Fit

                    ShadInputWithIcon{}
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippetSimple{
                            code: #(INPUT_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
