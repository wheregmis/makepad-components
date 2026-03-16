use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryBreadcrumbPage,
    page: breadcrumb_page,
    title: "Breadcrumb",
    subtitle: "Quiet inline navigation trails for page ancestry and current location.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Anatomy" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Use breadcrumbs for hierarchical navigation, not tags, tabs, or filter chips. They should sit close to the page title they describe."}
            ShadFieldDescription{text: "Intermediate ancestors are links. The final item is the current page and should read as text-only emphasis, not another interactive step."}
            ShadFieldDescription{text: "Collapsed ellipsis is visual-only in this pass. It helps communicate hidden ancestors, but it does not open an overflow menu yet."}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Default Trail" }

        ShadSurfaceMuted{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0
            padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
            draw_bg +: {
                border_size: 1.0
                border_color: (shad_theme.color_outline_border)
            }

            ShadBreadcrumb{
                ShadBreadcrumbLink{ text: "Workspace" }
                ShadBreadcrumbSeparator{}
                ShadBreadcrumbLink{ text: "Settings" }
                ShadBreadcrumbSeparator{}
                ShadBreadcrumbPage{ text: "Billing" }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 4.0

                ShadLabel{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 16
                    text: "Billing Settings"
                }
                ShadFieldDescription{
                    text: "Manage invoices, tax details, and workspace-level billing contacts."
                }
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "Collapsed / Ellipsis" }

        ShadSurfaceMuted{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0
            padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
            draw_bg +: {
                border_size: 1.0
                border_color: (shad_theme.color_outline_border)
            }

            ShadBreadcrumb{
                ShadBreadcrumbLink{ text: "Workspace" }
                ShadBreadcrumbSeparator{}
                ShadBreadcrumbEllipsis{}
                ShadBreadcrumbSeparator{}
                ShadBreadcrumbLink{ text: "Invoices" }
                ShadBreadcrumbSeparator{}
                ShadBreadcrumbPage{ text: "Archive" }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 4.0

                ShadLabel{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 16
                    text: "Archived Invoice #2048"
                }
                ShadFieldDescription{
                    text: "Use ellipsis when the hierarchy is deep and the hidden ancestors do not need an interactive overflow control yet."
                }
            }
        }
    },
}
