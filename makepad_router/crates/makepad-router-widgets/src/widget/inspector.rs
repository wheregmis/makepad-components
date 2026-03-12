use makepad_draw::draw_list_2d::DrawListExt;
use makepad_widgets::{dvec2, Cx2d, Rect};

// Debug inspector overlay for RouterWidget.

use super::RouterWidget;

impl RouterWidget {
    fn debug_inspector_lines(&self) -> Vec<String> {
        fn fmt_tail_stack(history: &crate::navigation::NavigationHistory, max: usize) -> String {
            let stack = history.all_routes();
            let idx = history.current_index();
            if stack.is_empty() {
                return "stack: (empty)".to_string();
            }
            let start = stack.len().saturating_sub(max);
            let mut parts = Vec::<String>::new();
            for (i, r) in stack.iter().enumerate().skip(start) {
                if i == idx {
                    parts.push(format!("[{}]", r.id));
                } else {
                    parts.push(r.id.to_string());
                }
            }
            format!("stack: {}", parts.join(" > "))
        }

        let mut out = Vec::<String>::new();
        let route_id = self
            .router
            .current_route_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "(none)".to_string());
        out.push(format!("route: {}", route_id));

        let idx = self.router.history.current_index();
        out.push(format!(
            "history: {}/{}  back:{} forward:{}",
            idx,
            self.router.depth().saturating_sub(1),
            if self.router.can_go_back() { 1 } else { 0 },
            if self.router.can_go_forward() { 1 } else { 0 }
        ));

        out.push(fmt_tail_stack(&self.router.history, 4));
        out.push(format!("url: {}", self.current_url()));

        if let Some(t) = &self.transition_rt.state {
            out.push(format!(
                "transition: {:?} {:.0}%",
                t.preset,
                (t.progress * 100.0).clamp(0.0, 100.0)
            ));
        }
        if self.pending_navigation.is_some() {
            out.push("pending: guard/before-leave".to_string());
        }

        if let Some(route) = self.router.current_route() {
            if !route.params.is_empty() {
                let mut parts = Vec::<String>::new();
                for (k, v) in route.params.iter().take(3) {
                    parts.push(format!("{}={}", k, v));
                }
                out.push(format!("params: {}", parts.join(" ")));
            }
            if !route.query.data.is_empty() {
                let mut parts = Vec::<String>::new();
                for (k, v) in route.query.data.iter().take(3) {
                    parts.push(format!("{}={}", k, v));
                }
                out.push(format!("query: {}", parts.join(" ")));
            }
        }

        out
    }

    pub(super) fn draw_debug_inspector(&mut self, cx: &mut Cx2d, rect: Rect) {
        if !self.debug_inspector {
            return;
        }
        self.draw_lists.inspector.begin_always(cx);

        // Ensure overlay draws above the routed content (ortho z-range is [-100..100]).
        self.inspector_bg.draw_depth = 10.0;
        self.inspector_text.draw_depth = 11.0;

        let lines = self.debug_inspector_lines();
        let font_size = self.inspector_text.text_style.font_size.max(6.0);
        let line_h = font_size as f64 + 3.0;
        let margin = 10.0;
        let pad = 8.0;
        let max_width = 280.0f64;
        let width = max_width.min((rect.size.x - margin * 2.0).max(0.0));
        let height =
            (lines.len() as f64 * line_h + pad * 2.0).min((rect.size.y - margin * 2.0).max(0.0));

        if width <= 0.0 || height <= 0.0 {
            self.draw_lists.inspector.end(cx);
            return;
        }

        let pos = rect.pos + dvec2(rect.size.x - width - margin, rect.size.y - height - margin);
        let bg = Rect {
            pos,
            size: dvec2(width, height),
        };

        self.inspector_bg.draw_abs(cx, bg);

        let mut y = pos.y + pad;
        for line in lines {
            self.inspector_text
                .draw_abs(cx, dvec2(pos.x + pad, y), &line);
            y += line_h;
            if y > pos.y + height - 4.0 {
                break;
            }
        }

        self.draw_lists.inspector.end(cx);
    }
}
