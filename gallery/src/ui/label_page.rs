use makepad_components::makepad_widgets::*;
use crate::ui::snippets::LABEL_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryLabelPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Label"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired accessible label associated with controls."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        label_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

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

                    label_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

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

                    label_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    label_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            label_preview_panel := mod.widgets.GalleryPreviewPanel{
                label_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        Label{
                            text: "Default Label"
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                        }

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
