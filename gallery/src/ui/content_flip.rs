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
        avatar_page := mod.widgets.GalleryAvatarPage{}
        badge_page := mod.widgets.GalleryBadgePage{}
        button_page := mod.widgets.GalleryButtonPage{}
        checkbox_page := mod.widgets.GalleryCheckboxPage{}
    }
}
