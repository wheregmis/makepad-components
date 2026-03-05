use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.MakepadIconBase = mod.widgets.Icon{
        icon_walk: Walk{width: 16, height: 16}
        draw_icon.color: #d4d4d8
    }

    mod.widgets.IconCheck = mod.widgets.MakepadIconBase{
        draw_icon.svg: crate_resource("self://resources/icons/check.svg")
    }

    mod.widgets.IconX = mod.widgets.MakepadIconBase{
        draw_icon.svg: crate_resource("self://resources/icons/x.svg")
    }

    mod.widgets.IconSearch = mod.widgets.MakepadIconBase{
        draw_icon.svg: crate_resource("self://resources/icons/search.svg")
    }
}
