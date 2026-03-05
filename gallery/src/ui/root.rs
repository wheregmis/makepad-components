use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAppUi = Root{
        main_window := Window{
            window.inner_size: vec2(1400 900)
            body +: {
                flow: Right

                sidebar := mod.widgets.GallerySidebar{}
                content_flip := mod.widgets.GalleryContentFlip{}
            }
        }
    }
}
