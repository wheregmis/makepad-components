use crate::ui::snippets::CHECKBOX_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCheckboxPage = ShadScrollYView{
        ShadPageTitle{
            text: "Checkbox"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired checkbox component from makepad-components library"
        }

        ShadHr{}

        checkbox_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

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

                    checkbox_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

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

                    checkbox_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    checkbox_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            checkbox_preview_panel := mod.widgets.ShadPanel{
                checkbox_preview_flip := PageFlip{
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
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 12.0

                    ShadCheckbox{label: "Accept terms and conditions"}
                    ShadCheckbox{label: "Pre-checked option" checked: true}
                    ShadCheckbox{label: "Subscribe to newsletter"}
                }

                ShadHr{}

                ShadSectionHeader{ text: "In a form row" }

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

                ShadHr{}
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
