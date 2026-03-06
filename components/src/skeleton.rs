use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSkeleton = mod.widgets.RoundedView{
        width: 100
        height: 20

        // Background with continuous shimmer (uses draw_pass.time)
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000

            shimmer_speed: uniform(2.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let base = self.color
                let highlight = mix(base, vec4(1.0, 1.0, 1.0, base.w), 0.75)

                let phase = (self.pos.x * 6.28318) - (self.draw_pass.time * self.shimmer_speed * 6.28318)
                let wave = cos(phase) * 0.5 + 0.5
                let fill_color = mix(base, highlight, wave)

                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    max(1.0, self.border_radius),
                )

                sdf.fill_keep(fill_color)

                if self.border_size > 0.0 {
                    sdf.stroke(self.border_color, self.border_size)
                }

                return sdf.result
            }
        }

        // Keep redrawing so draw_pass.time drives the shimmer
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
