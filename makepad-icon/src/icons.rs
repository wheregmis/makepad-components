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

    mod.widgets.IconInfo = mod.widgets.MakepadIconBase{
        draw_icon.svg: crate_resource("self://resources/icons/info.svg")
    }

    mod.widgets.IconChevronLeft = mod.widgets.MakepadIconBase{
        draw_icon.svg: crate_resource("self://resources/icons/chevron-left.svg")
    }

    mod.widgets.IconChevronRight = mod.widgets.MakepadIconBase{
        draw_icon.svg: crate_resource("self://resources/icons/chevron-right.svg")
    }

    mod.widgets.IconChevronDown = mod.widgets.MakepadIconBase{
        draw_icon.svg: crate_resource("self://resources/icons/chevron-down.svg")
    }

    // Icon-button types: ButtonFlatIcon with SVG pre-bound.
    // Callers apply draw_bg / draw_icon.color styling on top.
    mod.widgets.IconButtonX = mod.widgets.ButtonFlatIcon{
        icon_walk: Walk{width: 14, height: 14}
        draw_icon.svg: crate_resource("self://resources/icons/x.svg")
    }

    mod.widgets.IconButtonChevronLeft = mod.widgets.ButtonFlatIcon{
        icon_walk: Walk{width: 14, height: 14}
        draw_icon.svg: crate_resource("self://resources/icons/chevron-left.svg")
    }

    mod.widgets.IconButtonChevronRight = mod.widgets.ButtonFlatIcon{
        icon_walk: Walk{width: 14, height: 14}
        draw_icon.svg: crate_resource("self://resources/icons/chevron-right.svg")
    }
}
