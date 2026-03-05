use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAccordionPage = View{
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

        Hr{}

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 6.0

            ButtonFlatter{text: "XSmall"}
            ButtonFlatter{text: "Small"}
            size_medium := Button{text: "Medium"}
            ButtonFlatter{text: "Large"}

            View{width: Fill, height: Fit}

            option_icon := CheckBox{text: "Icon"}
            option_disabled := CheckBox{text: "Disabled"}
            option_bordered := CheckBox{text: "Bordered"}
        }

        Label{
            text: "Normal"
            draw_text.color: (shad_theme.color_muted_foreground)
        }

        accordion_wrap := RoundedView{
            width: Fill
            height: Fit
            draw_bg.color: (shad_theme.color_secondary)
            draw_bg.radius: (shad_theme.radius)
            padding: Inset{top: 8, right: 8, bottom: 8, left: 8}

            accordion_panel := Accordion{
                item_accessible := AccordionItem{
                    header: View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        padding: Inset{top: 10, bottom: 10, left: 12, right: 12}
                        spacing: 8.0

                        title := Label{text: "Is it accessible?"}
                        View{width: Fill, height: Fit}
                        fold_button := FoldButton{}
                    }
                    body: View{
                        width: Fill
                        height: Fit
                        flow: Down
                        padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
                        Label{
                            text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior."
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                        }
                    }
                }

                item_styled := AccordionItem{
                    header: View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        padding: Inset{top: 10, bottom: 10, left: 12, right: 12}
                        spacing: 8.0

                        title := Label{text: "Is it styled with complex elements?"}
                        View{width: Fill, height: Fit}
                        fold_button := FoldButton{}
                    }
                    body: View{
                        width: Fill
                        height: Fit
                        flow: Down
                        padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
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

                            Toggle{text: "Switch"}
                            CheckBox{text: "Or a CheckBox"}
                        }
                    }
                }

                item_third := AccordionItem{
                    header: View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        padding: Inset{top: 10, bottom: 10, left: 12, right: 12}
                        spacing: 8.0

                        title := Label{text: "This is third accordion"}
                        View{width: Fill, height: Fit}
                        fold_button := FoldButton{}
                    }
                    body: View{
                        width: Fill
                        height: Fit
                        flow: Down
                        padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
                        Label{
                            text: "This is third accordion content. It can be any view, like a text view or a button."
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                        }
                    }
                }
            }
        }

        View{width: Fill, height: Fill}
    }
}
