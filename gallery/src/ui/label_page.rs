use crate::ui::snippets::LABEL_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryLabelPage = ShadScrollYView{
        ShadPageTitle{
            text: "Label"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired accessible label associated with controls."
        }

        ShadHr{}

        label_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            label_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                label_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    label_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    label_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                label_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    label_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    label_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            label_preview_panel := mod.widgets.ShadPanel{
                label_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Default Label" }

                        ShadLabel{ text: "Your email address" }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(LABEL_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
