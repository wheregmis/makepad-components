use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSpinner = mod.widgets.RoundedView{
        width: 24
        height: 24

        draw_bg +: {
            color: uniform(shad_theme.color_primary)
            rotation_speed: uniform(1.0)
            stroke_width: uniform(2.5)
            arc_gap: uniform(0.25)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let radius = min(self.rect_size.x * 0.5, self.rect_size.y * 0.5) - self.stroke_width * 0.5
                let center = self.rect_size * 0.5
                let rotation = self.draw_pass.time * self.rotation_speed * 2.0 * PI
                let gap_radians = self.arc_gap * 2.0 * PI
                let start_angle = rotation
                sdf.arc_round_caps(
                    center.x
                    center.y
                    radius
                    start_angle
                    start_angle + 2.0 * PI - gap_radians
                    self.stroke_width
                )
                return sdf.fill(self.color)
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
