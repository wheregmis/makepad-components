use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAccordionPage,
    page: accordion_page,
    title: "Accordion",
    subtitle: "Accordion component from makepad-components library. Use ShadAccordionItemRef::set_open(cx, ..), `open_changed(actions)`, and `animation_progress(actions)` when external state drives expansion.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
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
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep expansion decisions in the page or feature state, not inside the app shell."}
        mod.widgets.GalleryActionFlowStep{text: "2. Get a ShadAccordionItemRef for the specific item you want to drive, then call set_open(cx, true/false, animator::Animate::Yes)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `open_changed(actions)` and `animation_progress(actions)` when surrounding UI needs to react to the transition."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use is_open(cx) when restoring layout or reconciling external state back into the page."}
    },
}
