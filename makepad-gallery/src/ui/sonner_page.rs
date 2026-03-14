use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::sonner::ShadSonnerWidgetExt;

gallery_stateful_page_shell! {
    widget: GallerySonnerPage,
    page: sonner_page,
    title: "Sonner / Toast",
    subtitle: "Toast notifications with Modal overlay. Use ShadSonnerRef::open/close and `open_changed(actions)` when a page or app shell triggers toasts.",
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
        mod.widgets.GalleryActionFlowStep{text: "1. Keep one ShadSonnerRef per toast variant the page can trigger."}
        mod.widgets.GalleryActionFlowStep{text: "2. Call open(cx) from buttons, async completions, or other semantic page events."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use `open_changed(actions)` when the page or shell reacts to toast lifecycle."}
        mod.widgets.GalleryActionFlowStep{text: "4. Close buttons and modal dismissal remain component-owned, so the page just triggers and observes."}
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
            if self.view.button(cx, ids!(toast_event_btn)).clicked(actions) {
                self.view.shad_sonner(cx, ids!(toast_event)).open(cx);
            }
            if self.view.button(cx, ids!(toast_desc_btn)).clicked(actions) {
                self.view.shad_sonner(cx, ids!(toast_desc)).open(cx);
            }
            if self.view.button(cx, ids!(toast_close_btn)).clicked(actions) {
                self.view.shad_sonner(cx, ids!(toast_close)).open(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
