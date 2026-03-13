use crate::ui::snippets::COLLAPSIBLE_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCollapsiblePageBase = #(GalleryCollapsiblePage::register_widget(vm))

    mod.widgets.GalleryCollapsiblePage = set_type_default() do mod.widgets.GalleryCollapsiblePageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Collapsible"
            }

            ShadPageSubtitle{
                text: "Single section toggle inspired by shadcn/ui collapsible. Use ShadCollapsibleRef::set_is_open(cx, ..) and opening/closing(actions) when a page owns the expansion state."
            }

            ShadHr{}

            collapsible_preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_flip +: {
                        root_view +: {
                            preview_content +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                ShadCollapsible{
                                    margin: Inset{top: 12, right: 12}
                                    title: "Order #4189"
                                    is_open: true
                                    body: View{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8.0

                                        RoundedView{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                                            draw_bg +: {
                                                color: #0000
                                                border_size: 1.0
                                                border_radius: 6.0
                                                border_color: (shad_theme.color_outline_border)
                                            }

                                            ShadSectionHeader{
                                                width: Fill
                                                text: "Status"
                                            }
                                            ShadLabel{
                                                text: "Shipped"
                                                draw_text.text_style.font_size: 10
                                            }
                                        }

                                        RoundedView{
                                            width: Fill
                                            height: Fit
                                            flow: Down
                                            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                                            spacing: 4.0
                                            draw_bg +: {
                                                color: #0000
                                                border_size: 1.0
                                                border_radius: 6.0
                                                border_color: (shad_theme.color_outline_border)
                                            }

                                            ShadLabel{
                                                text: "Shipping address"
                                                draw_text.text_style.font_size: 10
                                            }
                                            ShadSectionHeader{ text: "100 Market St, San Francisco" }
                                        }

                                        RoundedView{
                                            width: Fill
                                            height: Fit
                                            flow: Down
                                            padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                                            spacing: 4.0
                                            draw_bg +: {
                                                color: #0000
                                                border_size: 1.0
                                                border_radius: 6.0
                                                border_color: (shad_theme.color_outline_border)
                                            }

                                            ShadLabel{
                                                text: "Items"
                                                draw_text.text_style.font_size: 10
                                            }
                                            ShadSectionHeader{ text: "2x Studio Headphones" }
                                        }
                                    }
                                }
                            }

                            action_flow +: {
                                visible: true
                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. Let the page or feature decide whether the collapsible should be open, especially when it mirrors routing or selection state."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. Use ShadCollapsibleRef::set_is_open(cx, bool, animator::Animate::Yes) from external buttons or restored state."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. opening(actions) and closing(actions) are the semantic outputs when sibling UI needs to react."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. is_open(cx) gives you the current visual state if the page needs to reconcile after redraw."}
                                    }
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(COLLAPSIBLE_PREVIEW_CODE)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryCollapsiblePage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryCollapsiblePage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
