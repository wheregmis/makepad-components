use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryBreadcrumbPage,
    page: breadcrumb_page,
    title: "Breadcrumb",
    subtitle: "Breadcrumb flows for navigation hierarchies.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }
        ShadBreadcrumb{
            ShadBreadcrumbLink{ text: "Home" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbLink{ text: "Components" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbPage{ text: "Breadcrumb" }
        }

        ShadHr{}
        ShadSectionHeader{ text: "Collapsed / Ellipsis" }
        ShadBreadcrumb{
            ShadBreadcrumbLink{ text: "Home" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbEllipsis{}
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbLink{ text: "Components" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbPage{ text: "Breadcrumb" }
        }
    },
}
