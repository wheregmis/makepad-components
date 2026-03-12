use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadProgressBase = mod.widgets.RoundedView{
        width: Fill
        height: 8

        draw_bg +: {
            color: (shad_theme.color_secondary)
            color_fill: (shad_theme.color_primary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000
            progress: instance(0.5)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let p = clamp(self.progress, 0.0, 1.0)
                let fill_w = self.rect_size.x * p
                let r = max(1.0, self.border_radius)

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, r)
                sdf.fill_keep(self.color)

                if p > 0.001 {
                    sdf.box(0.0, 0.0, fill_w, self.rect_size.y, r)
                    sdf.intersect()
                    sdf.fill_keep(self.color_fill)
                }

                if self.border_size > 0.0 {
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, r)
                    sdf.stroke(self.border_color, self.border_size)
                }

                return sdf.result
            }
        }
    }

    mod.widgets.ShadProgress = mod.widgets.ShadProgressBase{}
    mod.widgets.ShadProgress33 = mod.widgets.ShadProgressBase{ draw_bg +: { progress: instance(0.33) } }
    mod.widgets.ShadProgress66 = mod.widgets.ShadProgressBase{ draw_bg +: { progress: instance(0.66) } }
    mod.widgets.ShadProgressFull = mod.widgets.ShadProgressBase{ draw_bg +: { progress: instance(1.0) } }

    mod.widgets.ShadProgressIndeterminate = mod.widgets.RoundedView{
        width: Fill
        height: 8

        draw_bg +: {
            color: (shad_theme.color_secondary)
            color_fill: (shad_theme.color_primary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000
            bar_width: uniform(0.4)
            sweep_duration: uniform(1.5)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let r = max(1.0, self.border_radius)
                let bw = self.rect_size.x * self.bar_width
                let phase = fract(self.draw_pass.time / self.sweep_duration)
                let start_x = (self.rect_size.x - bw) * phase

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, r)
                sdf.fill_keep(self.color)

                sdf.box(start_x, 0.0, bw, self.rect_size.y, r)
                sdf.intersect()
                sdf.fill_keep(self.color_fill)

                if self.border_size > 0.0 {
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, r)
                    sdf.stroke(self.border_color, self.border_size)
                }

                return sdf.result
            }
        }

        animator: Animator{
            time: {
                default: @on
                on: AnimatorState{
                    redraw: true
                    from: {all: Loop {duration: 100.0, end: 1000000000.0}}
                    apply: {}
                }
            }
        }
    }
}
