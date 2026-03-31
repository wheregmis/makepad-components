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
                aria_label: "Close"
            }

            mod.widgets.IconButtonChevronLeft = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon.svg: crate_resource("self://resources/icons/chevron-left.svg")
                aria_label: "Previous"
            }

            mod.widgets.IconButtonChevronRight = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon.svg: crate_resource("self://resources/icons/chevron-right.svg")
                aria_label: "Next"
            }

            mod.widgets.IconButtonMenu = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon.svg: crate_resource("self://resources/icons/menu.svg")
                aria_label: "Menu"
            }

            mod.widgets.IconButtonMoreHorizontal = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon.svg: crate_resource("self://resources/icons/more-horizontal.svg")
                aria_label: "More options"
            }
        }
    };
}

// Generated at build time by makepad-icon/build.rs.
include!(concat!(env!("OUT_DIR"), "/lucide_icon_bindings.rs"));
