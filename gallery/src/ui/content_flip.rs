use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryContentFlip = PageFlip{
        width: Fill
        height: Fill
        active_page: @accordion_page

        accordion_page := mod.widgets.GalleryAccordionPage{}
        button_page := mod.widgets.GalleryButtonPage{}
        alert_page := mod.widgets.GalleryAlertPage{}
        checkbox_page := mod.widgets.GalleryCheckboxPage{}
    }
}
