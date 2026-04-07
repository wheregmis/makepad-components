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

#[derive(Script, Widget)]
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

fn clamp_progress_value(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

fn progress_value_changed(current: f64, next: f64) -> bool {
    clamp_progress_value(current) != clamp_progress_value(next)
}

fn shader_progress_value(value: f64) -> f32 {
    (clamp_progress_value(value) / 100.0) as f32
}

impl MyProgressBar {
    fn sync_progress_to_shader(&mut self) {
        self.draw_bg.pad1 = shader_progress_value(self.value);
    }
}

impl ScriptHook for MyProgressBar {
    fn on_after_apply(
        &mut self,
        _vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        self.sync_progress_to_shader();
    }
}

impl Widget for MyProgressBar {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
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
            if !progress_value_changed(inner.value, value) {
                return;
            }
            inner.value = clamp_progress_value(value);
            inner.sync_progress_to_shader();
            inner.redraw(cx);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn progress_update_noops_when_value_is_unchanged() {
        assert!(!super::progress_value_changed(42.0, 42.0));
        assert!(!super::progress_value_changed(-4.0, 0.0));
        assert!(!super::progress_value_changed(120.0, 100.0));
        assert!(super::progress_value_changed(42.0, 43.0));
    }
}
