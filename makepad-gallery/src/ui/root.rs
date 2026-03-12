use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryContentFlip = mod.widgets.GalleryPageFlipBase{
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
        card_page := mod.widgets.GalleryCardPage{}
        carousel_page := mod.widgets.GalleryCarouselPage{}
        checkbox_page := mod.widgets.GalleryCheckboxPage{}
        collapsible_page := mod.widgets.GalleryCollapsiblePage{}
        dialog_page := mod.widgets.GalleryDialogPage{}
        input_page := mod.widgets.GalleryInputPage{}
        radio_group_page := mod.widgets.GalleryRadioGroupPage{}
        resizable_page := mod.widgets.GalleryResizablePage{}
        scroll_area_page := mod.widgets.GalleryScrollAreaPage{}
        select_page := mod.widgets.GallerySelectPage{}
        separator_page := mod.widgets.GallerySeparatorPage{}
        sheet_page := mod.widgets.GallerySheetPage{}
        skeleton_page := mod.widgets.GallerySkeletonPage{}
        switch_page := mod.widgets.GallerySwitchPage{}
        tabs_page := mod.widgets.GalleryTabsPage{}
        kbd_page := mod.widgets.GalleryKbdPage{}
        label_page := mod.widgets.GalleryLabelPage{}
        progress_page := mod.widgets.GalleryProgressPage{}
        sidebar_page := mod.widgets.GallerySidebarPage{}
        slider_page := mod.widgets.GallerySliderPage{}
        sonner_page := mod.widgets.GallerySonnerPage{}
        spinner_page := mod.widgets.GallerySpinnerPage{}
    }

    mod.widgets.GalleryAppUi = Root{
        main_window := Window{
            window.inner_size: vec2(1400 900)
            window.title: "Makepad Components Gallery"
            pass +: { clear_color: (shad_theme.color_background) }
            body +: {
                width: Fill
                height: Fill
                flow: Overlay
                draw_bg.color: (shad_theme.color_background)

                View{
                    width: Fill
                    height: Fill
                    flow: Right
                    sidebar := mod.widgets.GallerySidebar{}
                    content_flip := mod.widgets.GalleryContentFlip{}
                }
            }
        }
    }
}
