use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    root: ShadScrollArea,
    widget: GalleryResizablePage,
    page: resizable_page,
    title: "Resizable",
    subtitle: "Resizable panes emit splitter alignment changes, so pages can persist and restore layout without reaching into child internals.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Horizontal panes" }
        ShadPanel{
            height: 240

            horizontal_resizable := ShadResizable{
                width: Fill
                height: Fill
                axis: SplitterAxis.Horizontal
                align: SplitterAlign.FromA(180.0)
                a: ShadSurface{
                    variant: ShadSurfaceVariant.Muted
                    width: Fill
                    height: Fill
                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                    flow: Down
                    spacing: 8.0
                    draw_bg.border_size: 0.0
                    ShadSectionHeader{text: "Navigation"}
                    ShadFieldDescription{text: "Keep filters, folders, or nav trees here."}
                }

                b: ShadSurface{
                    width: Fill
                    height: Fill
                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                    flow: Down
                    spacing: 8.0
                    draw_bg.color: #0000
                    draw_bg.border_size: 0.0
                    ShadSectionHeader{text: "Content"}
                    ShadFieldDescription{text: "Main editing or reading surface."}
                }
            }
        }

        ShadSectionHeader{ text: "Vertical panes" }
        ShadPanel{
            height: 260

            vertical_resizable := ShadResizable{
                width: Fill
                height: Fill
                axis: SplitterAxis.Vertical
                align: SplitterAlign.FromA(120.0)
                a: ShadSurface{
                    variant: ShadSurfaceVariant.Muted
                    width: Fill
                    height: Fill
                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                    flow: Down
                    spacing: 8.0
                    draw_bg.border_size: 0.0
                    ShadSectionHeader{text: "Metrics"}
                    ShadFieldDescription{text: "Compact summary cards or charts."}
                }

                b: ShadSurface{
                    width: Fill
                    height: Fill
                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                    flow: Down
                    spacing: 8.0
                    draw_bg.color: #0000
                    draw_bg.border_size: 0.0
                    ShadSectionHeader{text: "Details"}
                    ShadFieldDescription{text: "Expanded logs, notes, or tables."}
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
