use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadImage = Image{
        width: Fill
        height: Fill
        fit: ImageFit.Biggest
    }
}
