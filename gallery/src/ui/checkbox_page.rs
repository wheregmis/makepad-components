use makepad_components::makepad_widgets::*;
use crate::ui::snippets::CHECKBOX_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCheckboxPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Checkbox"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Shadcn-inspired checkbox component from makepad-components library"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        checkbox_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            checkbox_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                checkbox_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    checkbox_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

                    checkbox_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                checkbox_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    checkbox_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    checkbox_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            checkbox_preview_panel := mod.widgets.GalleryPreviewPanel{
                checkbox_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                Label{
                    text: "Default"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 12.0

                    ShadCheckbox{label: "Accept terms and conditions"}
                    ShadCheckbox{label: "Pre-checked option" checked: true}
                    ShadCheckbox{label: "Subscribe to newsletter"}
                }

                GalleryHr{}

                Label{
                    text: "In a form row"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 24.0
                    align: Align{y: 0.5}

                    ShadCheckbox{label: "Option A"}
                    ShadCheckbox{label: "Option B" checked: true}
                    ShadCheckbox{label: "Option C"}
                }

                GalleryHr{}
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(CHECKBOX_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
