use makepad_components::makepad_widgets::*;
use crate::ui::snippets::HOVER_CARD_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryHoverCardPage = ShadScrollYView{
        ShadPageTitle{
            text: "Hover Card"
        }

        ShadPageSubtitle{
            text: "Card-style tooltip shown on hover."
        }

        ShadHr{}

        hover_card_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            hover_card_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                hover_card_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    hover_card_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    hover_card_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                hover_card_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    hover_card_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    hover_card_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            hover_card_preview_panel := mod.widgets.ShadPanel{
                hover_card_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 24.0
                        ShadSectionHeader{ text: "Default" }
                        hover_card_demo_area := View{
                            width: Fit
                            height: Fit
                            hover_card_trigger := mod.widgets.ShadButtonOutline{text: "Hover me"}
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(HOVER_CARD_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
