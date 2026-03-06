use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadDropdownMenu = mod.widgets.DropDownBase{
        width: Fit
        height: 36
        align: Align{x: 0.0, y: 0.5}

        padding: Inset{left: 12, right: 28, top: 0, bottom: 0}

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)

            text_style.font_size: 11

            get_color: fn() {
                return mix(
                    mix(
                        mix(
                            self.color
                            mix(
                                self.color_focus
                                self.color_hover
                                self.hover
                            )
                            self.focus
                        )
                        mix(
                            self.color_hover
                            self.color_down
                            self.down
                        )
                        self.hover
                    )
                    self.color_disabled
                    self.disabled
                )
            }
        }

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0

            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_focus: #0000
            color_down: (shad_theme.color_ghost_down)
            color_active: (shad_theme.color_ghost_hover)
            color_disabled: #0000

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_outline_border)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_active: (shad_theme.color_outline_border_hover)
            border_color_disabled: (shad_theme.color_outline_border)

            get_color: fn() {
                return mix(
                    mix(
                        mix(
                            self.color
                            mix(
                                self.color_focus
                                self.color_hover
                                self.hover
                            )
                            self.focus
                        )
                        mix(
                            self.color_hover
                            self.color_down
                            self.down
                        )
                        self.hover
                    )
                    self.color_disabled
                    self.disabled
                )
            }
        }

        draw_icon +: {
            color: (shad_theme.color_muted_foreground)
            get_color: fn() {
                return self.color
            }
        }

        popup_menu: {
            draw_bg +: {
                color: (shad_theme.color_background)
                border_radius: (shad_theme.radius)
                border_size: 1.0
                border_color: (shad_theme.color_outline_border)
            }
            item: {
                height: 32
                padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                draw_bg +: {
                    color: (shad_theme.color_background)
                    color_hover: (shad_theme.color_secondary)
                    color_active: (shad_theme.color_secondary_hover)
                    border_radius: 4.0
                }
                draw_text +: {
                    color: (shad_theme.color_primary)
                    text_style.font_size: 11
                }
            }
        }
    }
}
