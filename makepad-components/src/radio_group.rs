use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadRadioGroup = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0
    }

    mod.widgets.ShadRadioGroupInline = View{
        width: Fit
        height: Fit
        flow: Right
        spacing: 16.0
        align: Align{y: 0.5}
    }

    mod.widgets.ShadRadioItem = RadioButtonFlat{
        width: Fit
        height: Fit
        grab_key_focus: true
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
        label_walk +: {
            margin: theme.mspace_h_1{left: 18.0}
        }

        draw_bg +: {
            size: 16.0
            color: #0000
            color_hover: #0000
            color_down: #0000
            color_focus: #0000
            color_disabled: #0000
            color_active: #0000

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_primary)
            border_color_focus: (shad_theme.color_primary)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)

            mark_color: #0000
            mark_color_active: (shad_theme.color_primary)
            mark_color_disabled: (shad_theme.color_muted_foreground)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let sz_px = self.size
                let radius_px = sz_px * 0.5
                let center_px = vec2(radius_px, self.rect_size.y * 0.5)

                // Draw background circle
                sdf.circle(center_px.x, center_px.y, radius_px - self.border_size)

                let color_fill = self.color
                    .mix(self.color_focus, self.focus)
                    .mix(self.color_active, self.active)
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                    .mix(self.color_disabled, self.disabled)

                let color_stroke = self.border_color
                    .mix(self.border_color_focus, self.focus)
                    .mix(self.border_color_active, self.active)
                    .mix(self.border_color_hover, self.hover)
                    .mix(self.border_color_down, self.down)
                    .mix(self.border_color_disabled, self.disabled)

                sdf.fill_keep(color_fill)
                sdf.stroke(color_stroke, self.border_size)

                // Draw mark (inner dot)
                sdf.circle(center_px.x, center_px.y + self.mark_offset, radius_px * 0.5 - self.border_size * 0.75)

                let mark_color = self.mark_color
                    .mix(self.mark_color_active, self.active)
                    .mix(self.mark_color_disabled, self.disabled)

                sdf.fill(mark_color)

                // Focus ring (drawn last so it's visible outside the bounds)
                if self.focus > 0.0 {
                    sdf.circle(center_px.x, center_px.y, radius_px + 1.0)
                    sdf.stroke(mix(vec4(0.0, 0.0, 0.0, 0.0), self.border_color_focus, self.focus), 2.0)
                }

                return sdf.result
            }
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11.0
        }
    }
}
