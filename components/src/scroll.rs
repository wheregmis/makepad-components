use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadScrollYView = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0
    }
}
