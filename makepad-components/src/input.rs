use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadField = mod.widgets.View {
        width: Fill
        height: Fit
        flow: Down
        spacing: 6.0
    }

    mod.widgets.ShadFieldLabel = mod.widgets.Label {
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11.0
    }

    mod.widgets.ShadFieldDescription = mod.widgets.Label {
        width: Fill
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10.0
    }

    mod.widgets.ShadFieldMessage = mod.widgets.Label {
        width: Fill
        draw_text.color: (shad_theme.color_destructive)
        draw_text.text_style.font_size: 10.0
    }

    mod.widgets.ShadInputChrome = mod.widgets.View {
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        padding: Inset{
            left: (shad_theme.control_padding_x_md),
            right: (shad_theme.control_padding_x_md),
            top: 0,
            bottom: 0
        }

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0
            color: #0000
            border_color: (shad_theme.color_border)
        }
    }

    mod.widgets.ShadInput = mod.widgets.TextInput {
        width: Fill
        height: Fit
        padding: Inset{
            left: (shad_theme.control_padding_x_md - 4.0),
            right: (shad_theme.control_padding_x_md - 4.0),
            top: 12,
            bottom: 12
        }
        empty_text: "Enter text..."

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0

            color: #0000
            color_hover: #0000
            color_focus: #0000
            color_down: #0000
            color_empty: #0000
            color_disabled: (shad_theme.color_surface_muted)

            border_color: (shad_theme.color_border)
            border_color_hover: (shad_theme.color_border_hover)
            border_color_focus: (shad_theme.color_border_focus)
            border_color_down: (shad_theme.color_border_focus)
            border_color_empty: (shad_theme.color_border)
            border_color_disabled: (shad_theme.color_border)
        }

        draw_text +: {
            color: (shad_theme.color_text)
            color_hover: (shad_theme.color_text)
            color_focus: (shad_theme.color_text)
            color_down: (shad_theme.color_text)
            color_empty: (shad_theme.color_text_muted)
            color_disabled: (shad_theme.color_text_muted)
        }
        draw_text.text_style.font_size: (shad_theme.control_font_size_md)

        draw_cursor +: {
            color: (shad_theme.color_text)
        }

        draw_selection +: {
            color: (shad_theme.color_surface_muted)
        }
    }

    // A borderless version of ShadInput to be used inside custom wrappers
    mod.widgets.ShadInputBorderless = mod.widgets.ShadInput {
        draw_bg +: {
            border_size: 0.0
            border_color: #0000
            border_color_hover: #0000
            border_color_focus: #0000
            border_color_down: #0000
            border_color_empty: #0000
            border_color_disabled: #0000
        }
        padding: Inset{left: 0, right: 0, top: 12, bottom: 12}
    }

    // A composite View containing an Icon and a borderless Input field.
    // The View itself provides the Shadcn standard borders and focus rings.
    mod.widgets.ShadInputWithIcon = mod.widgets.ShadInputChrome {
        spacing: 8.0

        icon := mod.widgets.IconSearch {
            draw_icon.color: (shad_theme.color_text_muted)
        }

        input := mod.widgets.ShadInputBorderless {
            empty_text: "Search..."
        }
    }
}
