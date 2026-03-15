use makepad_widgets::*;

// Expands to a script_mod with one icon widget per Lucide SVG binding.
// Input format: IconTypeName => "self://resources/icons/name.svg"
macro_rules! define_lucide_icons {
    ($($icon_name:ident => $icon_path:literal,)*) => {
        script_mod! {
            use mod.prelude.widgets.*
            use mod.widgets.*

            mod.widgets.MakepadIconBase = mod.widgets.Icon{
                icon_walk: Walk{width: 16, height: 16}
                draw_icon.color: #d4d4d8
            }

            $(mod.widgets.$icon_name = mod.widgets.MakepadIconBase{
                draw_icon.svg: crate_resource($icon_path)
            })*

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

            mod.widgets.IconButtonMenu = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon.svg: crate_resource("self://resources/icons/menu.svg")
            }

            mod.widgets.IconButtonMoreHorizontal = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon.svg: crate_resource("self://resources/icons/more-horizontal.svg")
            }
        }
    };
}

// Generated at build time by makepad-icon/build.rs.
include!(concat!(env!("OUT_DIR"), "/lucide_icon_bindings.rs"));
