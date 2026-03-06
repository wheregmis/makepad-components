use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

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
                hover_card_tooltip := CalloutTooltip{width: Fill height: Fill}
            }
        }
    }
}
