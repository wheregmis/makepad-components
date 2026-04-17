use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryOverviewPage,
    page: overview_page,
    title: "Overview",
    subtitle: "A collection of beautiful and accessible components built with Makepad.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 24.0

            ShadFieldDescription{
                width: Fill
                text: "This gallery showcases all the widgets available in makepad-components. Use the sidebar to navigate through the categories and discover what you can build."
            }

            View{
                width: Fill
                height: Fit
                flow: Right{wrap: true}
                spacing: 16.0

                ShadCard{
                    width: 250
                    height: 120
                    ShadCardHeader{
                        ShadCardTitle{text: "Forms"}
                        ShadCardDescription{text: "Inputs, switches, sliders, checkboxes."}
                    }
                }
                ShadCard{
                    width: 250
                    height: 120
                    ShadCardHeader{
                        ShadCardTitle{text: "Layout"}
                        ShadCardDescription{text: "Aspect Ratio, Scroll Area, Resizable, Separator."}
                    }
                }
                ShadCard{
                    width: 250
                    height: 120
                    ShadCardHeader{
                        ShadCardTitle{text: "Overlays"}
                        ShadCardDescription{text: "Dialog, Context Menu, Popover, Sheet."}
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Navigate using the left sidebar."}
        mod.widgets.GalleryActionFlowStep{text: "2. Check component examples and code snippets."}
        mod.widgets.GalleryActionFlowStep{text: "3. Copy code to your app and customize."}
    },
}
