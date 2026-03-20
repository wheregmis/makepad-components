use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    mod.widgets.MyProgressBar = #(MyProgressBar::register_widget(vm)){
        width: Fill,
        height: 8,
        draw_bg +: {
            // progress: instance(0.0) // 这里的 progress 会被 animator 控制

            // pixel: fn() {
            //     let sdf = Sdf2d.viewport(self.pos * self.rect_size);
            //     let sz = self.rect_size;
            //     let r = sz.y * 0.5;

            //     // 1. 定义基本的胶囊体形状
            //     sdf.circle(r, r, r);
            //     sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
            //     sdf.circle(sz.x - r, r, r);

            //     let capsule_dist = sdf.result;

            //     // 绘制背景
            //     sdf.fill(#e2e8f0);

            //     // 2. 定义进度矩形
            //     sdf.rect(0.0, 0.0, sz.x * self.progress, sz.y);

            //     // 手动取交集实现裁剪效果
            //     sdf.result.w = max(sdf.result.w, capsule_dist.w);

            //     // 填充进度颜色
            //     sdf.fill(#3b82f6);

            //     return sdf.result;
            // }
            progress: instance(0.0)

                        border_size: uniform(theme.beveling)
                        border_radius: uniform(theme.corner_radius)
                        color: uniform(theme.color_inset)
                        fill_color: uniform(#3b82f6)

                        pixel: fn() {
                            let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                            let sz = self.rect_size
                            let r = self.border_radius

                            // 1. 背景层
                            sdf.box(0., 0., sz.x, sz.y, r)
                            sdf.fill(self.color)

                            // 2. 进度填充
                            let fill_width = self.progress * sz.x
                            if fill_width > 0.1 {
                                let margin = self.border_size
                                let safe_w = max(0.0, fill_width - margin * 2.0)
                                let safe_h = max(0.0, sz.y - margin * 2.0)

                                if safe_w > 0.1 {
                                    sdf.box(
                                        margin,
                                        margin,
                                        safe_w,
                                        safe_h,
                                        max(1.0, r - margin)
                                    )
                                    sdf.fill(self.fill_color)
                                }
                            }

                            // 3. 边框
                            if self.border_size > 0.0 {
                                sdf.box(0., 0., sz.x, sz.y, r)
                                sdf.stroke(theme.color_bevel, self.border_size)
                            }

                            return sdf.result
                        }
                    }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct MyProgressBar {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live(0.0)]
    value: f64,
}

impl Widget for MyProgressBar {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let progress = (self.value / 100.0).clamp(0.0, 1.0);
        self.draw_bg.begin(cx, walk, self.layout);
        script_apply_eval!(cx, self, {
            draw_bg +: {
                progress: #(progress)
            }
        });
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl MyProgressBarRef {
    pub fn progress(&self) -> f64 {
        self.borrow().map(|inner| inner.value).unwrap_or(0.0)
    }
    pub fn set_progress(&self, cx: &mut Cx, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.value = value.clamp(0.0, 100.0);
            inner.redraw(cx);
        }
    }
}