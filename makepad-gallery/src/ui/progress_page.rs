use crate::ui::snippets::PROGRESS_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryProgressPage = ShadScrollYView{
        ShadPageTitle{
            text: "Progress"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired progress bars. Determinate (value 0–1) and indeterminate (animated)."
        }

        ShadHr{}

        progress_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            progress_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                progress_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    progress_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    progress_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                progress_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    progress_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    progress_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            progress_preview_panel := mod.widgets.ShadPanel{
                progress_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Determinate" }

                        ShadProgress33{}
                        ShadProgress66{}
                        ShadProgressFull{}

                        ShadSectionHeader{ text: "Indeterminate (animated)" }

                        ShadProgressIndeterminate{}
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(PROGRESS_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
