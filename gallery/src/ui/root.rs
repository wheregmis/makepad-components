use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAppUi = Root{
        main_window := Window{
            window.inner_size: vec2(1400 900)
            pass +: { clear_color: (shad_theme.color_background) }
            body +: {
                flow: Right
                draw_bg.color: (shad_theme.color_background)

                sidebar := mod.widgets.GallerySidebar{}
                content_flip := mod.widgets.GalleryContentFlip{}
            }
        }
    }
}
