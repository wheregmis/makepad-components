use crate::models::carousel::{next_index, normalize_index, prev_index};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadCarouselDotsBase = #(ShadCarouselDots::register_widget(vm))
    mod.widgets.ShadCarouselDots = set_type_default() do mod.widgets.ShadCarouselDotsBase{
        width: 40
        height: 8
        draw_bg +: {
            color_active: uniform(shad_theme.color_primary)
            color_inactive: uniform(shad_theme.color_muted_foreground)
            dot_0: instance(1.0)
            dot_1: instance(0.0)
            dot_2: instance(0.0)
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let r = 4.0
                let cx0 = 4.0
                let cx1 = 16.0
                let cx2 = 28.0
                let cy = 4.0
                let c0 = mix(self.color_inactive, self.color_active, self.dot_0)
                let c1 = mix(self.color_inactive, self.color_active, self.dot_1)
                let c2 = mix(self.color_inactive, self.color_active, self.dot_2)
                sdf.circle(cx0, cy, r)
                sdf.fill(c0)
                sdf.circle(cx1, cy, r)
                sdf.fill(c1)
                sdf.circle(cx2, cy, r)
                sdf.fill(c2)
                return sdf.result
            }
        }
        animator: Animator{
            active: {
                default: @on_0
                off: AnimatorState{
                    from: {all: Snap}
                    apply: { draw_bg: { dot_0: 0.0, dot_1: 0.0, dot_2: 0.0 } }
                }
                on_0: AnimatorState{
                    from: {all: Snap}
                    apply: { draw_bg: { dot_0: 1.0, dot_1: 0.0, dot_2: 0.0 } }
                }
                on_1: AnimatorState{
                    from: {all: Snap}
                    apply: { draw_bg: { dot_0: 0.0, dot_1: 1.0, dot_2: 0.0 } }
                }
                on_2: AnimatorState{
                    from: {all: Snap}
                    apply: { draw_bg: { dot_0: 0.0, dot_1: 0.0, dot_2: 1.0 } }
                }
            }
        }
    }

    mod.widgets.ShadCarouselBase = #(ShadCarousel::register_widget(vm))

    // Named nav button types for prev/next, sharing the same outline styling.
    mod.widgets.ShadCarouselPrevBtn = mod.widgets.IconButtonChevronLeft{
        width: 36
        height: 36
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_focus: (shad_theme.color_outline_border_hover)
        }
        draw_icon.color: (shad_theme.color_primary)
    }

    mod.widgets.ShadCarouselNextBtn = mod.widgets.IconButtonChevronRight{
        width: 36
        height: 36
        draw_bg +: {
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_outline_border_down)
            border_color_focus: (shad_theme.color_outline_border_hover)
        }
        draw_icon.color: (shad_theme.color_primary)
    }

    mod.widgets.ShadCarousel = set_type_default() do mod.widgets.ShadCarouselBase{
        width: Fill
        height: Fit
        flow: Down
        spacing: 12.0

        content_wrap := View{
            width: Fill
            height: Fit
            clip_x: true
            clip_y: true

            carousel_flip := PageFlip{
                width: Fill
                height: Fit
                active_page: @slide_0

                slide_0 := View{
                    width: Fill
                    height: Fit
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    RoundedView{
                        width: Fill
                        height: Fit
                        padding: Inset{left: 24, right: 24, top: 24, bottom: 24}
                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_radius: (shad_theme.radius)
                        }
                        Label{
                            text: "Slide 1"
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 14
                        }
                    }
                }

                slide_1 := View{
                    width: Fill
                    height: Fit
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    RoundedView{
                        width: Fill
                        height: Fit
                        padding: Inset{left: 24, right: 24, top: 24, bottom: 24}
                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_radius: (shad_theme.radius)
                        }
                        Label{
                            text: "Slide 2"
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 14
                        }
                    }
                }

                slide_2 := View{
                    width: Fill
                    height: Fit
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    RoundedView{
                        width: Fill
                        height: Fit
                        padding: Inset{left: 24, right: 24, top: 24, bottom: 24}
                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_radius: (shad_theme.radius)
                        }
                        Label{
                            text: "Slide 3"
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 14
                        }
                    }
                }
            }
        }

        nav := View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 12.0
            align: Align{x: 0.5, y: 0.5}

            prev_btn := mod.widgets.ShadCarouselPrevBtn{}

            dots := mod.widgets.ShadCarouselDots{}

            next_btn := mod.widgets.ShadCarouselNextBtn{}
        }
    }
}

const SLIDE_IDS: &[LiveId] = &[live_id!(slide_0), live_id!(slide_1), live_id!(slide_2)];

#[derive(Clone, Debug, Default)]
pub enum ShadCarouselAction {
    Changed(usize),
    #[default]
    None,
}

#[derive(Clone, Debug, Default)]
pub enum ShadCarouselDotsAction {
    #[default]
    None,
    Clicked(usize),
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct ShadCarouselDots {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[rust]
    area: Area,
    #[rust]
    active_index: usize,
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ShadCarouselDots {
    const DOT_W: f32 = 12.0;
    const DOTS_W: f32 = 40.0;

    pub fn set_active(&mut self, cx: &mut Cx, index: usize) {
        if self.active_index == index {
            return;
        }

        self.active_index = index;
        match index {
            0 => self.animator_play(cx, ids!(active.on_0)),
            1 => self.animator_play(cx, ids!(active.on_1)),
            _ => self.animator_play(cx, ids!(active.on_2)),
        }
    }

    fn hit_to_index(&self, cx: &mut Cx, pos: makepad_draw::Vec2d) -> Option<usize> {
        let rect = self.area.rect(cx);
        if !rect.contains(pos) {
            return None;
        }
        let x = pos.x - rect.pos.x;
        let dot_w = Self::DOT_W as f64;
        let dots_w = Self::DOTS_W as f64;
        if x < dot_w {
            Some(0)
        } else if x < dot_w * 2.0 {
            Some(1)
        } else if x < dots_w {
            Some(2)
        } else {
            None
        }
    }
}

impl Widget for ShadCarouselDots {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.area.redraw(cx);
        }
        if let Hit::FingerDown(fe) = event.hits(cx, self.area) {
            if fe.is_primary_hit() {
                if let Some(i) = self.hit_to_index(cx, fe.abs) {
                    cx.widget_action_with_data(
                        &self.action_data,
                        self.widget_uid(),
                        ShadCarouselDotsAction::Clicked(i),
                    );
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let rect = cx.turtle().rect();
        self.draw_bg.draw_abs(cx, rect);
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadCarousel {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    current: usize,
    #[rust]
    synced_current: Option<usize>,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ShadCarousel {
    fn sync_visual_state(&mut self, cx: &mut Cx) {
        if self.synced_current == Some(self.current) {
            return;
        }

        if let Some(mut page_flip) = self
            .view
            .widget_flood(cx, ids!(carousel_flip))
            .borrow_mut::<PageFlip>()
        {
            page_flip.set_active_page(cx, SLIDE_IDS[self.current]);
        }
        if let Some(mut dots) = self
            .view
            .widget_flood(cx, ids!(dots))
            .borrow_mut::<ShadCarouselDots>()
        {
            dots.set_active(cx, self.current);
        }

        self.synced_current = Some(self.current);
    }

    fn set_current(&mut self, cx: &mut Cx, index: usize, emit_action: bool) {
        let Some(index) = normalize_index(index, SLIDE_IDS.len()) else {
            return;
        };

        if self.current == index {
            return;
        }

        self.current = index;
        self.synced_current = None;
        self.sync_visual_state(cx);
        self.view.redraw(cx);

        if emit_action {
            cx.widget_action_with_data(
                &self.action_data,
                self.widget_uid(),
                ShadCarouselAction::Changed(index),
            );
        }
    }

    pub fn go_to(&mut self, cx: &mut Cx, index: usize) {
        self.set_current(cx, index, true);
    }

    pub fn next(&mut self, cx: &mut Cx) {
        let next = next_index(self.current, SLIDE_IDS.len());
        self.go_to(cx, next);
    }

    pub fn prev(&mut self, cx: &mut Cx) {
        let next = prev_index(self.current, SLIDE_IDS.len());
        self.go_to(cx, next);
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadCarouselAction::Changed(index) = item.cast() {
                return Some(index);
            }
        }
        None
    }

    fn handle_component_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.view.button(cx, ids!(prev_btn)).clicked(actions) {
            self.prev(cx);
        }
        if self.view.button(cx, ids!(next_btn)).clicked(actions) {
            self.next(cx);
        }
        let dots_uid = self.view.widget_flood(cx, ids!(dots)).widget_uid();
        if let Some(item) = actions.find_widget_action(dots_uid) {
            if let ShadCarouselDotsAction::Clicked(index) = item.cast() {
                self.go_to(cx, index);
            }
        }
    }
}

impl Widget for ShadCarousel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if let Event::Actions(actions) = event {
            self.handle_component_actions(cx, actions);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_visual_state(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShadCarouselRef {
    pub fn next(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.next(cx);
        }
    }

    pub fn prev(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.prev(cx);
        }
    }

    pub fn go_to(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.go_to(cx, index);
        }
    }

    pub fn current(&self) -> Option<usize> {
        self.borrow().map(|inner| inner.current())
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }
}
