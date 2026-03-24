use makepad_widgets::*;

pub use makepad_widgets::{AreaChart, BarChart, DataPoint, LineChart};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadLineChart = mod.widgets.LineChart{
        width: Fill
        height: 220

        grid_color: (shad_theme.color_outline_border)
        grid_text_color: (shad_theme.color_muted_foreground)
        border_color: (shad_theme.color_outline_border)
        bg_color: (shad_theme.color_background)

        line_color: (shad_theme.color_chart)
        line_width: 2.0
        fill_color: (shad_theme.color_chart_fill)
        bar_color: (shad_theme.color_chart)
        dot_color: (shad_theme.color_chart_emphasis)
        dot_radius: 3.0
        plot_margin: Inset{left: 52.0, top: 12.0, right: 12.0, bottom: 26.0}

        draw_bg +: {
            color: (shad_theme.color_background)
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            text_style: theme.font_regular{font_size: 10.0}
        }
    }

    mod.widgets.ShadAreaChart = mod.widgets.AreaChart{
        width: Fill
        height: 220

        grid_color: (shad_theme.color_outline_border)
        grid_text_color: (shad_theme.color_muted_foreground)
        border_color: (shad_theme.color_outline_border)
        bg_color: (shad_theme.color_background)

        fill_color: (shad_theme.color_chart_fill)
        line_color: (shad_theme.color_chart_emphasis)
        line_width: 2.0
        bar_color: (shad_theme.color_chart)
        dot_color: (shad_theme.color_chart_emphasis)
        dot_radius: 3.0
        plot_margin: Inset{left: 52.0, top: 12.0, right: 12.0, bottom: 26.0}

        draw_bg +: {
            color: (shad_theme.color_background)
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            text_style: theme.font_regular{font_size: 10.0}
        }
    }

    mod.widgets.ShadBarChart = mod.widgets.BarChart{
        width: Fill
        height: 220

        grid_color: (shad_theme.color_outline_border)
        grid_text_color: (shad_theme.color_muted_foreground)
        border_color: (shad_theme.color_outline_border)
        bg_color: (shad_theme.color_background)

        line_color: (shad_theme.color_chart)
        line_width: 2.0
        fill_color: (shad_theme.color_chart_fill)
        bar_color: (shad_theme.color_chart)
        dot_color: (shad_theme.color_chart_emphasis)
        dot_radius: 3.0
        plot_margin: Inset{left: 52.0, top: 12.0, right: 12.0, bottom: 26.0}

        draw_bg +: {
            color: (shad_theme.color_background)
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            text_style: theme.font_regular{font_size: 10.0}
        }
    }
}

#[derive(Clone, Default)]
pub struct ShadLineChartRef(pub WidgetRef);

#[derive(Clone, Default)]
pub struct ShadAreaChartRef(pub WidgetRef);

#[derive(Clone, Default)]
pub struct ShadBarChartRef(pub WidgetRef);

pub trait ShadChartWidgetExt: Widget {
    fn shad_line_chart(&self, cx: &Cx, path: &[LiveId]) -> ShadLineChartRef {
        ShadLineChartRef(self.widget(cx, path))
    }

    fn shad_area_chart(&self, cx: &Cx, path: &[LiveId]) -> ShadAreaChartRef {
        ShadAreaChartRef(self.widget(cx, path))
    }

    fn shad_bar_chart(&self, cx: &Cx, path: &[LiveId]) -> ShadBarChartRef {
        ShadBarChartRef(self.widget(cx, path))
    }
}

impl<T: Widget + ?Sized> ShadChartWidgetExt for T {}

impl ShadLineChartRef {
    pub fn set_data(&self, points: Vec<DataPoint>) {
        if let Some(mut inner) = self.0.borrow_mut::<LineChart>() {
            inner.set_data(points);
        }
    }
}

impl ShadAreaChartRef {
    pub fn set_data(&self, points: Vec<DataPoint>) {
        if let Some(mut inner) = self.0.borrow_mut::<AreaChart>() {
            inner.set_data(points);
        }
    }
}

impl ShadBarChartRef {
    pub fn set_data(&self, points: Vec<DataPoint>) {
        if let Some(mut inner) = self.0.borrow_mut::<BarChart>() {
            inner.set_data(points);
        }
    }
}
