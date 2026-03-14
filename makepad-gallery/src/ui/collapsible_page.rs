use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryCollapsiblePage,
    page: collapsible_page,
    title: "Collapsible",
    subtitle: "Single section toggle inspired by shadcn/ui collapsible. Use ShadCollapsibleRef::set_open(cx, ..), `open_changed(actions)`, and `animation_progress(actions)` when a page owns the expansion state.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
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
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Let the page or feature decide whether the collapsible should be open, especially when it mirrors routing or selection state."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use ShadCollapsibleRef::set_open(cx, bool, animator::Animate::Yes) from external buttons or restored state."}
        mod.widgets.GalleryActionFlowStep{text: "3. `open_changed(actions)` and `animation_progress(actions)` are the semantic outputs when sibling UI needs to react."}
        mod.widgets.GalleryActionFlowStep{text: "4. is_open(cx) gives you the current visual state if the page needs to reconcile after redraw."}
    },
}
