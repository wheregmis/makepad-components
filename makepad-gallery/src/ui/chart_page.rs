use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::chart::{AreaChart, BarChart, DataPoint, LineChart};
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryChartPage,
    page: chart_page,
    title: "Chart",
    subtitle: "Opinionated wrappers over Makepad's built-in charts for line, area, and bar views. Feed typed Rust `DataPoint` series into the widgets and keep dataset switching in page state.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            chart_growth_btn := ShadButton{
                text: "Growth data"
            }

            chart_ops_btn := ShadButtonOutline{
                text: "Ops data"
            }

            chart_revenue_btn := ShadButtonOutline{
                text: "Revenue data"
            }
        }

        chart_status := ShadFieldDescription{
            text: "Showing growth dataset."
        }

        ShadSectionHeader{ text: "Line" }
        line_chart_demo := ShadLineChart{}

        ShadSectionHeader{ text: "Area" }
        area_chart_demo := ShadAreaChart{}

        ShadSectionHeader{ text: "Bar" }
        bar_chart_demo := ShadBarChart{}
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Keep chart datasets in Rust types and push them into the chart widget with `set_data(...)`."}
        mod.widgets.GalleryActionFlowStep{text: "2. Treat the selected dataset as page state so buttons, tabs, or filters can swap the visualized series."}
        mod.widgets.GalleryActionFlowStep{text: "3. Reuse the same points across line, area, and bar wrappers when you want comparable views with different emphasis."}
        mod.widgets.GalleryActionFlowStep{text: "4. The themed wrappers only style the chart primitives; dataset transforms and fetching stay in app code."}
    },
}

#[derive(Script, Widget)]
pub struct GalleryChartPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    dataset_index: usize,
}

impl ScriptHook for GalleryChartPage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.dataset_index = 0;
            self.apply_dataset(cx);
        });
    }
}

impl GalleryChartPage {
    fn apply_dataset(&mut self, cx: &mut Cx) {
        let (title, line_points, area_points, bar_points) = chart_dataset(self.dataset_index);

        if let Some(mut chart) = self
            .view
            .widget_flood(cx, ids!(line_chart_demo))
            .borrow_mut::<LineChart>()
        {
            chart.set_data(line_points);
        }
        if let Some(mut chart) = self
            .view
            .widget_flood(cx, ids!(area_chart_demo))
            .borrow_mut::<AreaChart>()
        {
            chart.set_data(area_points);
        }
        if let Some(mut chart) = self
            .view
            .widget_flood(cx, ids!(bar_chart_demo))
            .borrow_mut::<BarChart>()
        {
            chart.set_data(bar_points);
        }

        self.view
            .label(cx, ids!(chart_status))
            .set_text(cx, &format!("Showing {title} dataset."));
        self.view.redraw(cx);
    }
}

impl Widget for GalleryChartPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let next = if self
                .view
                .button(cx, ids!(chart_growth_btn))
                .clicked(actions)
            {
                Some(0)
            } else if self.view.button(cx, ids!(chart_ops_btn)).clicked(actions) {
                Some(1)
            } else if self
                .view
                .button(cx, ids!(chart_revenue_btn))
                .clicked(actions)
            {
                Some(2)
            } else {
                None
            };

            if let Some(next) = next {
                self.dataset_index = next;
                self.apply_dataset(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

fn chart_dataset(index: usize) -> (&'static str, Vec<DataPoint>, Vec<DataPoint>, Vec<DataPoint>) {
    match index {
        1 => (
            "ops",
            vec![
                point(0.0, 41.0),
                point(1.0, 44.0),
                point(2.0, 39.0),
                point(3.0, 48.0),
                point(4.0, 52.0),
                point(5.0, 50.0),
            ],
            vec![
                point(0.0, 24.0),
                point(1.0, 28.0),
                point(2.0, 26.0),
                point(3.0, 30.0),
                point(4.0, 36.0),
                point(5.0, 34.0),
            ],
            vec![
                point(0.0, 12.0),
                point(1.0, 18.0),
                point(2.0, 14.0),
                point(3.0, 22.0),
                point(4.0, 25.0),
                point(5.0, 19.0),
            ],
        ),
        2 => (
            "revenue",
            vec![
                point(0.0, 80.0),
                point(1.0, 92.0),
                point(2.0, 88.0),
                point(3.0, 101.0),
                point(4.0, 117.0),
                point(5.0, 126.0),
            ],
            vec![
                point(0.0, 40.0),
                point(1.0, 48.0),
                point(2.0, 50.0),
                point(3.0, 63.0),
                point(4.0, 72.0),
                point(5.0, 76.0),
            ],
            vec![
                point(0.0, 16.0),
                point(1.0, 20.0),
                point(2.0, 24.0),
                point(3.0, 30.0),
                point(4.0, 34.0),
                point(5.0, 38.0),
            ],
        ),
        _ => (
            "growth",
            vec![
                point(0.0, 18.0),
                point(1.0, 26.0),
                point(2.0, 31.0),
                point(3.0, 42.0),
                point(4.0, 54.0),
                point(5.0, 61.0),
            ],
            vec![
                point(0.0, 10.0),
                point(1.0, 14.0),
                point(2.0, 21.0),
                point(3.0, 29.0),
                point(4.0, 37.0),
                point(5.0, 45.0),
            ],
            vec![
                point(0.0, 8.0),
                point(1.0, 11.0),
                point(2.0, 13.0),
                point(3.0, 16.0),
                point(4.0, 21.0),
                point(5.0, 24.0),
            ],
        ),
    }
}

fn point(x: f64, y: f64) -> DataPoint {
    DataPoint { x, y }
}
