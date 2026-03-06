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
                flow: Right
                draw_bg.color: (shad_theme.color_background)

                sidebar := mod.widgets.GallerySidebar{}
                content := View{
                    width: Fill
                    height: Fill
                    flow: Down

                    mobile_header := View{
                        visible: false
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{x: 0.0, y: 0.5}
                        padding: Inset{top: 12, right: 12, bottom: 8, left: 12}

                        mobile_sidebar_button := mod.widgets.ShadButtonGhost{
                            text: "☰"
                            width: 36
                            padding: Inset{}
                        }
                    }

                    content_flip := mod.widgets.GalleryContentFlip{}
                }
            }
        }
    }
}
