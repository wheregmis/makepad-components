use crate::ui::snippets::SHEET_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySheetPage = SolidView{
        width: Fill
        height: Fill
        draw_bg.color: (shad_theme.color_background)
        flow: Overlay

        ShadScrollArea{
            ShadPageTitle{
                text: "Sheet"
            }

            ShadPageSubtitle{
                text: "Modal sheet overlays for contextual editing and supporting flows."
            }

            ShadSeparator{}

            ShadPanel{
                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 12.0

                    ShadSectionHeader{ text: "Sides" }
                    View{
                        width: Fit
                        height: Fit
                        flow: Right
                        spacing: 12.0

                        open_right_sheet_btn := ShadButton{text: "Right"}
                        open_left_sheet_btn := ShadButtonOutline{text: "Left"}
                        open_top_sheet_btn := ShadButtonOutline{text: "Top"}
                        open_bottom_sheet_btn := ShadButtonOutline{text: "Bottom"}
                    }
                }
            }

            GalleryCodeSnippetSimple{
                code: #(SHEET_PREVIEW_CODE)
            }
        }

        right_sheet := ShadSheet{
            side: "right"
            sheet_size: 360.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Edit workspace"}
                            description +: {text: "Quick edits that should not take you away from the current screen."}
                        }
                        body +: {
                            ShadField{
                                ShadFieldLabel{text: "Workspace name"}
                                ShadInput{empty_text: "Northwind"}
                            }
                            close_right_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }

        left_sheet := ShadSheet{
            side: "left"
            sheet_size: 360.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Project navigation"}
                            description +: {text: "Use left sheets for secondary navigation and drill-in menus."}
                        }
                        body +: {
                            ShadSidebarItem{text: "Roadmap"}
                            ShadSidebarItem{text: "Backlog"}
                            ShadSidebarItem{text: "Releases"}
                            close_left_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }

        top_sheet := ShadSheet{
            side: "top"
            sheet_size: 220.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Filter panel"}
                            description +: {text: "Top sheets work well for lightweight filters or global controls."}
                        }
                        body +: {
                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 12.0

                                ShadSelect{labels: ["All teams" "Design" "Engineering" "Ops"]}
                                ShadSelect{labels: ["Any status" "Open" "Blocked" "Done"]}
                            }
                            close_top_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }

        bottom_sheet := ShadSheet{
            side: "bottom"
            sheet_size: 220.0
            overlay +: {
                content +: {
                    sheet_frame +: {
                        header +: {
                            title +: {text: "Activity feed"}
                            description +: {text: "Bottom sheets suit notifications and short-lived supporting context."}
                        }
                        body +: {
                            ShadFieldDescription{text: "Lucas mentioned your team in release notes."}
                            ShadFieldDescription{text: "A new build finished successfully 3 minutes ago."}
                            close_bottom_sheet_btn := ShadButtonOutline{text: "Close"}
                        }
                    }
                }
            }
        }
    }
}
