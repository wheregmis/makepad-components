use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAccordionPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Accordion"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Accordion component from makepad-components library"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

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
                    Label{
                        text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior."
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
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
                    spacing: 8.0

                    Label{
                        text: "We can put any view here, like a row with toggles."
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }

                    View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 16

                        GalleryToggle{text: "Switch"}
                        GalleryCheckBox{text: "Or a CheckBox"}
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
                    Label{
                        text: "This is third accordion content. It can be any view, like a text view or a button."
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }
                }
            }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        accordion_example_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadAccordion{\n    item_faq := mod.widgets.ShadAccordionItem{\n        title: \"What is Makepad?\"\n        body: Label{text: \"A Rust-native UI framework.\"}\n    }\n}"
        }
    }
}
