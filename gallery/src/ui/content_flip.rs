use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryContentFlip = PageFlip{
        width: Fill
        height: Fill
        active_page: @accordion_page

        accordion_page := mod.widgets.GalleryAccordionPage{}
        alert_page := mod.widgets.GalleryAlertPage{}
        aspect_ratio_page := mod.widgets.GalleryAspectRatioPage{}
        avatar_page := mod.widgets.GalleryAvatarPage{}
        badge_page := mod.widgets.GalleryBadgePage{}
        breadcrumb_page := mod.widgets.GalleryBreadcrumbPage{}
        button_page := mod.widgets.GalleryButtonPage{}
        button_group_page := mod.widgets.GalleryButtonGroupPage{}
        checkbox_page := mod.widgets.GalleryCheckboxPage{}
        collapsible_page := mod.widgets.GalleryCollapsiblePage{}
        input_page := mod.widgets.GalleryInputPage{}
        label_page := mod.widgets.GalleryLabelPage{}
        sidebar_page := mod.widgets.GallerySidebarPage{}
    }
}
