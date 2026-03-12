use crate::ui::snippets::DROPDOWN_MENU_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryDropdownMenuPage = SolidView{
        width: Fill
        height: Fill
        draw_bg.color: (shad_theme.color_background)

        ScrollYView{
            width: Fill
            height: Fill
            flow: Down
            padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
            spacing: 12.0

            ShadPageTitle{
                text: "Dropdown Menu"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired dropdown for selecting from a list of options."
            }

            ShadHr{}

            dropdown_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                dropdown_tabs_row := View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    dropdown_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        dropdown_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        dropdown_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    dropdown_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        dropdown_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        dropdown_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                dropdown_preview_panel := mod.widgets.ShadPanel{
                    dropdown_preview_flip := PageFlip{
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
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 12.0

                                ShadDropdownMenu{
                                    labels: ["Option A" "Option B" "Option C" "Option D"]
                                }

                                ShadDropdownMenu{
                                    labels: ["Small" "Medium" "Large" "Extra Large"]
                                }
                            }
                        }

                        code_page := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            GalleryCodeSnippetSimple{
                                code: #(DROPDOWN_MENU_PREVIEW_CODE)
                            }
                        }
                    }
                }
            }
        }
    }
}
