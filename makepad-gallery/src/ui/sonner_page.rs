use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::sonner::{ShadSonnerWidgetExt, SonnerItem, SonnerKind};

gallery_stateful_page_shell! {
    widget: GallerySonnerPage,
    page: sonner_page,
    title: "Sonner / Toast",
    subtitle: "Toast notifications in a non-blocking overlay. Sonner variants share one global queue, so `ShadSonner`, `ShadSonnerWithDescription`, and `ShadSonnerWithClose` all append into the same stack.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Basic" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0

            toast_event_btn := ShadButton{text: "Event created"}
            toast_desc_btn := ShadButton{text: "Toast with description"}
        }

        View{
            width: Fill
            height: 200
            toast_event := ShadSonner{
                width: Fill
                height: Fill
                open: false
            }
            toast_desc := ShadSonnerWithDescription{
                width: Fill
                height: Fill
                open: false
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "With Close Button" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0

            toast_close_btn := ShadButton{text: "Show toast with close"}
        }

        View{
            width: Fill
            height: 200
            toast_close := ShadSonnerWithClose{
                width: Fill
                height: Fill
                open: false
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Sonner variants share a single global queue; keep whichever variant refs match the triggers you want to expose."}
        mod.widgets.GalleryActionFlowStep{text: "2. Call open(cx) on any variant ref and it appends its own toast kind into that shared stack."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `open_changed(actions)` when the page or shell reacts to toast lifecycle."}
        mod.widgets.GalleryActionFlowStep{text: "4. Close buttons remain component-owned, so the page just triggers variant refs and observes the shared host state."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GallerySonnerPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySonnerPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let sonner = self.view.shad_sonner(cx, ids!(toast_close));

            if self.view.button(cx, ids!(toast_event_btn)).clicked(actions) {
                sonner.enqueue(
                    cx,
                    SonnerItem {
                        title: "连接成功".to_string(),
                        description: Some("服务器连接成功。".to_string()),
                        kind: SonnerKind::Success,
                        duration: Some(3.0),
                        show_close: true,
                    },
                );
                cx.redraw_all();
            }
            if self.view.button(cx, ids!(toast_desc_btn)).clicked(actions) {
                sonner.enqueue(
                    cx,
                    SonnerItem {
                        title: "提示".to_string(),
                        description: Some("网络连接不稳定，请稍后再试。".to_string()),
                        kind: SonnerKind::Info,
                        duration: Some(3.0),
                        show_close: true,
                    },
                );
                cx.redraw_all();
            }
            if self.view.button(cx, ids!(toast_close_btn)).clicked(actions) {
                sonner.enqueue(
                    cx,
                    SonnerItem {
                        title: "错误".to_string(),
                        description: Some("网络连接失败，请检查网络设置。".to_string()),
                        kind: SonnerKind::Error,
                        duration: Some(3.0),
                        show_close: true,
                    },
                );
                cx.redraw_all();
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
