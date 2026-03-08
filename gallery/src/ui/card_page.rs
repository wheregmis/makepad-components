use crate::ui::snippets::CARD_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCardPage = ShadScrollYView{
        ShadPageTitle{
            text: "Card"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired card component from makepad-components library"
        }

        ShadHr{}

        card_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            card_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                card_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    card_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    card_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                card_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    card_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    card_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            card_preview_panel := mod.widgets.ShadPanel{
                card_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 16.0

                        ShadSectionHeader{ text: "Default" }

                        mod.widgets.ShadCard{
                            header := mod.widgets.ShadCardHeader{
                                title := mod.widgets.ShadCardTitle{text: "Card title"}
                                description := mod.widgets.ShadCardDescription{text: "Card description goes here."}
                            }
                            content := mod.widgets.ShadCardContent{
                                ShadLabel{
                                    text: "Card content area. Put any widgets here."
                                    draw_text.text_style.font_size: 14
                                }
                            }
                            footer := mod.widgets.ShadCardFooter{
                                mod.widgets.ShadButton{text: "Cancel"}
                                mod.widgets.ShadButton{text: "Save"}
                            }
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(CARD_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
