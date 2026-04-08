use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadToggle = mod.widgets.CheckBoxFlat{
        width: Fit
        height: 36
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        align: Align{x: 0.5, y: 0.5}
        icon_walk: Walk{width: 0.0, height: 0.0}
        label_walk: Walk{
            width: Fit
            height: Fit
            margin: Inset{left: 0, right: 0, top: 0, bottom: 0}
        }
        label_align: Align{x: 0.5, y: 0.5}

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0

            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: #0000
            color_active: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted)

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_focus: (shad_theme.color_outline_border_hover)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)

            mark_color: #0000
            mark_color_hover: #0000
            mark_color_down: #0000
            mark_color_active: #0000
            mark_color_active_hover: #0000
            mark_color_focus: #0000
            mark_color_disabled: #0000

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let radius = self.border_radius
                let inset = self.border_size * 0.5

                sdf.box(
                    inset,
                    inset,
                    self.rect_size.x - inset * 2.0,
                    self.rect_size.y - inset * 2.0,
                    radius
                )

                let base_fill = self.color
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                    .mix(self.color_disabled, self.disabled)

                let active_fill = self.color_active
                    .mix(self.color_disabled, self.disabled)

                let color_fill = base_fill.mix(active_fill, self.active)

                let base_stroke = self.border_color
                    .mix(self.border_color_hover, self.hover)
                    .mix(self.border_color_down, self.down)
                    .mix(self.border_color_disabled, self.disabled)

                let active_stroke = self.border_color_active
                    .mix(self.border_color_disabled, self.disabled)

                let color_stroke = base_stroke.mix(active_stroke, self.active)

                sdf.fill_keep(color_fill)
                sdf.stroke(color_stroke, self.border_size)

                if self.focus > 0.0 && self.active > 0.0 {
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, radius + 1.0)
                    sdf.stroke(self.border_color_focus, 2.0)
                }
                return sdf.result
            }
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary_foreground)
            color_disabled: (shad_theme.color_muted_foreground)

            get_color: fn() {
                let base = self.color
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                    .mix(self.color_focus, self.focus * 0.35)
                    .mix(self.color_disabled, self.disabled)

                return base.mix(self.color_active, self.active)
            }
            text_style.font_size: 11
        }
    }

    mod.widgets.ShadToggleSm = mod.widgets.ShadToggle{
        height: 28
        padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadToggleLg = mod.widgets.ShadToggle{
        height: 44
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_text.text_style.font_size: 12
    }

    mod.widgets.ShadToggleGroup = mod.widgets.RoundedView{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 4.0
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}

        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadToggleGroupItem = mod.widgets.ShadToggle{
        draw_bg +: {
            border_size: 0.0
            border_color: #0000
            border_color_hover: #0000
            border_color_down: #0000
            border_color_focus: #0000
            border_color_active: #0000
            border_color_disabled: #0000
        }
    }

    mod.widgets.ShadToggleGroupItemSm = mod.widgets.ShadToggleGroupItem{
        height: 28
        padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.ShadToggleGroupItemLg = mod.widgets.ShadToggleGroupItem{
        height: 44
        padding: Inset{left: 16, right: 16, top: 0, bottom: 0}
        draw_text.text_style.font_size: 12
    }
}
