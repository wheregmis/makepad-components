use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.shad_themes = {
        dark: {
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
            color_disabled: #27272a
            color_disabled_foreground: #a1a1aa
            color_disabled_border: #3f3f46

            color_destructive: #dc2626
            color_destructive_hover: #b91c1c
            color_destructive_down: #991b1b
            color_destructive_foreground: #fafafa

            color_success: #22c55e
            color_warning: #fbbf24
            color_error: #dc2626
            color_success_subtle: #052e16
            color_warning_subtle: #451a03

            color_ghost_hover: #27272a
            color_ghost_down: #3f3f46

            color_outline_border: #3f3f46
            color_outline_border_hover: #52525b
            color_outline_border_down: #71717a

            color_background: #09090b
            color_popover: #09090b
            color_overlay: vec4(0.0, 0.0, 0.0, 0.55)

            color_surface_default: #09090b
            color_surface_muted: #18181b
            color_surface_popover: #09090b

            color_border: #3f3f46
            color_border_hover: #52525b
            color_border_down: #71717a
            color_border_focus: #fafafa

            color_text: #fafafa
            color_text_muted: #a1a1aa
            color_text_inverse: #18181b

            color_chart: #x3b82f6
            color_chart_emphasis: #x2563eb
            color_chart_fill: #x3b82f633

            radius: 6.0
            control_height_sm: 28.0
            control_height_md: 36.0
            control_height_lg: 44.0
            control_padding_x_sm: 12.0
            control_padding_x_md: 16.0
            control_padding_x_lg: 32.0
            control_font_size_sm: 10.0
            control_font_size_md: 11.0
            control_font_size_lg: 13.0

            color_kbd_bg: #x3c3c3c
            color_kbd_border: #555555
            color_kbd_foreground: #ffffff
        }

        light: {
            color_primary: #09090b
            color_primary_hover: #18181b
            color_primary_down: #27272a
            color_primary_foreground: #fafafa

            color_secondary: #f4f4f5
            color_secondary_hover: #e4e4e7
            color_secondary_down: #d4d4d8
            color_secondary_foreground: #09090b

            color_muted: #f4f4f5
            color_muted_foreground: #71717a
            color_disabled: #e4e4e7
            color_disabled_foreground: #71717a
            color_disabled_border: #d4d4d8

            color_destructive: #dc2626
            color_destructive_hover: #b91c1c
            color_destructive_down: #991b1b
            color_destructive_foreground: #fafafa

            color_success: #16a34a
            color_warning: #b45309
            color_error: #dc2626
            color_success_subtle: #dcfce7
            color_warning_subtle: #fef3c7

            color_ghost_hover: #f4f4f5
            color_ghost_down: #e4e4e7

            color_outline_border: #e4e4e7
            color_outline_border_hover: #d4d4d8
            color_outline_border_down: #a1a1aa

            color_background: #ffffff
            color_popover: #ffffff
            color_overlay: vec4(0.0, 0.0, 0.0, 0.55)

            color_surface_default: #ffffff
            color_surface_muted: #f4f4f5
            color_surface_popover: #ffffff

            color_border: #e4e4e7
            color_border_hover: #d4d4d8
            color_border_down: #a1a1aa
            color_border_focus: #09090b

            color_text: #09090b
            color_text_muted: #71717a
            color_text_inverse: #fafafa

            color_chart: #x3b82f6
            color_chart_emphasis: #x2563eb
            color_chart_fill: #x3b82f633

            radius: 6.0
            control_height_sm: 28.0
            control_height_md: 36.0
            control_height_lg: 44.0
            control_padding_x_sm: 12.0
            control_padding_x_md: 16.0
            control_padding_x_lg: 32.0
            control_font_size_sm: 10.0
            control_font_size_md: 11.0
            control_font_size_lg: 13.0

            color_kbd_bg: #fafafa
            color_kbd_border: #e4e4e7
            color_kbd_foreground: #18181b


        }
    }

    mod.widgets.shad_theme = mod.widgets.shad_themes.dark
}
