use crate::ui::snippets::ACCORDION_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAccordionPage = ShadScrollYView{
        ShadPageTitle{
            text: "Accordion"
        }

        ShadPageSubtitle{
            text: "Accordion component from makepad-components library"
        }

        ShadHr{}

        accordion_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            accordion_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                accordion_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    accordion_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    accordion_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                accordion_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    accordion_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    accordion_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            accordion_preview_panel := mod.widgets.ShadPanel{
                accordion_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        accordion_panel := ShadAccordion{
                    margin: Inset{top: 12, right: 12}
                    item_accessible := ShadAccordionItem{
                        title: "Is it accessible?"
                        is_open: true
                        body: View{
                            width: Fill
                            height: Fit
                            flow: Down
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                            ShadSectionHeader{ text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior." }
                        }
                    }

                    item_styled := ShadAccordionItem{
                        title: "Is it styled with complex elements?"
                        body: View{
                            width: Fill
                            height: Fit
                            flow: Down
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                            spacing: 8.0

                            ShadSectionHeader{ text: "We can put any view here, like a row with toggles." }

                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 16

                                ShadSwitch{text: "Switch"}
                                ShadCheckbox{label: "Or a CheckBox"}
                            }
                        }
                    }

                    item_third := ShadAccordionItem{
                        title: "This is third accordion"
                        body: View{
                            width: Fill
                            height: Fit
                            flow: Down
                            padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                            ShadSectionHeader{ text: "This is third accordion content. It can be any view, like a text view or a button." }
                        }
                    }
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(ACCORDION_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
