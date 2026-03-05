use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSkeleton = mod.widgets.RoundedView{
        width: 100
        height: 20

        // Background with hover-driven shimmer
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: #0000

            anim_time: instance(0.0)
            shimmer_speed: uniform(2.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let base = self.color
                let highlight = mix(base, vec4(1.0, 1.0, 1.0, base.w), 0.75)

                let phase = (self.pos.x * 6.28318) - (self.anim_time * self.shimmer_speed * 6.28318)
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

        // Shimmer runs while hovered
        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.0}}
                    apply: {draw_bg: {anim_time: 0.0}}
                }
                on: AnimatorState{
                    redraw: true
                    from: {all: Loop {duration: 1.4, end: 1000000000.0}}
                    apply: {
                        draw_bg: {anim_time: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]}
                    }
                }
            }
        }
    }
}

