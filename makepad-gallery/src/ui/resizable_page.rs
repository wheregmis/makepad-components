use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    root: ShadScrollArea,
    widget: GalleryResizablePage,
    page: resizable_page,
    title: "Resizable",
    subtitle: "Resizable panes emit splitter alignment changes, and the splitter now uses a larger grab area so touch and trackpad adjustments are less finicky.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Horizontal panes" }
        ShadCard{
            spacing: 12.0
            padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

            ShadFieldDescription{
                width: Fill
                text: "Treat split layouts like surfaced work zones with a subtle divider, not just two raw rectangles with a draggable line."
            }

            View{
                height: 240
                width: Fill

                horizontal_resizable := ShadResizable{
                    width: Fill
                    height: Fill
                    axis: SplitterAxis.Horizontal
                    align: SplitterAlign.FromA(180.0)
                    a: ShadCard{
                        width: Fill
                        height: Fill
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        flow: Down
                        spacing: 8.0
                        draw_bg.border_size: 0.0
                        ShadSectionHeader{text: "Navigation"}
                        ShadFieldDescription{text: "Keep filters, folders, or nav trees here."}
                    }

                    b: ShadCard{
                        width: Fill
                        height: Fill
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        flow: Down
                        spacing: 8.0
                        draw_bg.border_size: 0.0
                        ShadSectionHeader{text: "Content"}
                        ShadFieldDescription{text: "Main editing or reading surface."}
                    }
                }
            }
        }

        ShadSectionHeader{ text: "Vertical panes" }
        ShadCard{
            spacing: 12.0
            padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

            ShadFieldDescription{
                width: Fill
                text: "Vertical splits should feel like stacked modules with a clear handle, not a layout debugging aid."
            }

            View{
                height: 260
                width: Fill

                vertical_resizable := ShadResizable{
                    width: Fill
                    height: Fill
                    axis: SplitterAxis.Vertical
                    align: SplitterAlign.FromA(120.0)
                    a: ShadCard{
                        width: Fill
                        height: Fill
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        flow: Down
                        spacing: 8.0
                        draw_bg.border_size: 0.0
                        ShadSectionHeader{text: "Metrics"}
                        ShadFieldDescription{text: "Compact summary cards or charts."}
                    }

                    b: ShadCard{
                        width: Fill
                        height: Fill
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        flow: Down
                        spacing: 8.0
                        draw_bg.border_size: 0.0
                        ShadSectionHeader{text: "Details"}
                        ShadFieldDescription{text: "Expanded logs, notes, or tables."}
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Name the splitter you want to persist, like horizontal_resizable or vertical_resizable."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read layout changes with view.splitter(cx, ids!(horizontal_resizable)).changed(actions)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Persist the returned SplitterAlign in page state, local storage, or app settings."}
        mod.widgets.GalleryActionFlowStep{text: "4. Restore the saved layout with set_align(cx, align) when the page is rebuilt."}
    },
}
