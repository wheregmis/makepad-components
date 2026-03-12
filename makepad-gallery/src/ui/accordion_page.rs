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

        accordion_preview_section := mod.widgets.GalleryPreviewSection{
            width: Fill
            height: Fit

            code_snippet +: {
                code: #(ACCORDION_PREVIEW_CODE)
            }

            preview_flip +: {
                root_view +: {
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 12.0

                    accordion_demo_shell := View{
                        width: 840
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadFieldDescription{
                            width: Fill
                            text: "Contained preview with realistic spacing, nested content, and divider rhythm."
                        }

                        accordion_panel := ShadAccordion{
                            width: Fill
                            item_accessible := ShadAccordionItem{
                                title: "Is it accessible?"
                                is_open: true
                                body: View{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                                    ShadFieldDescription{
                                        width: Fill
                                        text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior."
                                    }
                                }
                            }

                            item_styled := ShadAccordionItem{
                                title: "Is it styled with complex elements?"
                                body: View{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                                    spacing: 10.0

                                    ShadFieldDescription{
                                        width: Fill
                                        text: "We can put any view here, like a row with toggles."
                                    }

                                    View{
                                        width: Fit
                                        height: Fit
                                        flow: Right
                                        spacing: 20.0
                                        align: Align{y: 0.5}

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
                                    ShadFieldDescription{
                                        width: Fill
                                        text: "This is third accordion content. It can be any view, like a text view or a button."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
