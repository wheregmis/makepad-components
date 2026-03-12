use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.shad_theme = {
        color_primary: #fafafa
        color_primary_hover: #f4f4f5
        color_primary_down: #e4e4e7
        color_primary_foreground: #18181b

        color_secondary: #18181b
        color_secondary_hover: #27272a
        color_secondary_down: #3f3f46
        color_secondary_foreground: #fafafa

        color_muted: #18181b
        color_muted_foreground: #a1a1aa

        color_destructive: #dc2626
        color_destructive_hover: #b91c1c
        color_destructive_down: #991b1b
        color_destructive_foreground: #fafafa

        color_ghost_hover: #27272a
        color_ghost_down: #3f3f46

        color_outline_border: #3f3f46
        color_outline_border_hover: #52525b
        color_outline_border_down: #71717a

        color_background: #09090b
        color_popover: #09090b

        radius: 6.0

        color_kbd_bg: #x3c3c3c
        color_kbd_border: #555555
        color_kbd_foreground: #ffffff
    }
}
