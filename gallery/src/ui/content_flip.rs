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
        alert_dialog_page := mod.widgets.GalleryAlertDialogPage{}
        aspect_ratio_page := mod.widgets.GalleryAspectRatioPage{}
        avatar_page := mod.widgets.GalleryAvatarPage{}
        badge_page := mod.widgets.GalleryBadgePage{}
        breadcrumb_page := mod.widgets.GalleryBreadcrumbPage{}
        button_page := mod.widgets.GalleryButtonPage{}
        button_group_page := mod.widgets.GalleryButtonGroupPage{}
        card_page := mod.widgets.GalleryCardPage{}
        carousel_page := mod.widgets.GalleryCarouselPage{}
        checkbox_page := mod.widgets.GalleryCheckboxPage{}
        collapsible_page := mod.widgets.GalleryCollapsiblePage{}
        skeleton_page := mod.widgets.GallerySkeletonPage{}
        switch_page := mod.widgets.GallerySwitchPage{}
        input_page := mod.widgets.GalleryInputPage{}
        kbd_page := mod.widgets.GalleryKbdPage{}
        label_page := mod.widgets.GalleryLabelPage{}
        progress_page := mod.widgets.GalleryProgressPage{}
        sidebar_page := mod.widgets.GallerySidebarPage{}
        slider_page := mod.widgets.GallerySliderPage{}
        sonner_page := mod.widgets.GallerySonnerPage{}
        spinner_page := mod.widgets.GallerySpinnerPage{}
    }
}
