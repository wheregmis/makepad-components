use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    root: ShadScrollArea,
    widget: GallerySelectPage,
    page: select_page,
    title: "Select",
    subtitle: "Select uses the dropdown ref API: read changed(actions) or changed_label(actions), then store the chosen index or label in page state.",
    divider: { ShadSeparator{} },
    preview_spacing: 12.0,
    preview: {
        ShadCard{
            spacing: 14.0
            padding: Inset{left: 18, right: 18, top: 18, bottom: 18}

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadFieldDescription{
                    width: Fill
                    text: "Use explicit field stacks on mobile so each trigger gets the full row width instead of competing inside a tight horizontal layout."
                }

                ShadField{
                    width: Fill
                    ShadFieldLabel{text: "Status"}
                    status_select := ShadSelect{
                        width: Fill
                        labels: ["Pending" "In Progress" "Done"]
                    }
                    ShadFieldDescription{
                        width: Fill
                        text: "Good for short state lists where the surrounding page owns the selected index or label."
                    }
                }

                ShadField{
                    width: Fill
                    ShadFieldLabel{text: "City"}
                    city_select := ShadSelect{
                        width: Fill
                        labels: ["Toronto" "Montreal" "Vancouver" "Calgary"]
                    }
                    ShadFieldDescription{
                        width: Fill
                        text: "Treat the select like any other form control: let the container decide width, then keep helper copy below it."
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Right{wrap: true}
                    spacing: 8.0

                    select_done_btn := ShadButton{
                        text: "Set Done"
                    }

                    select_montreal_btn := ShadButtonOutline{
                        text: "Choose Montreal"
                    }

                    select_reset_btn := ShadButtonGhost{
                        text: "Reset"
                    }
                }

                select_status := ShadFieldDescription{
                    width: Fill
                    text: "Status: Pending. City: Toronto."
                }

                ShadFieldDescription{
                    width: Fill
                    text: "Known limitation: popup-style selects can still be unreliable inside the current gallery PageFlip shell. The splash app remains the best place to verify interaction."
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give the select an id, then get the dropdown ref with view.drop_down(cx, ids!(status_select))."}
        mod.widgets.GalleryActionFlowStep{text: "2. Use changed(actions) when you want the selected index, or changed_label(actions) when the label is enough."}
        mod.widgets.GalleryActionFlowStep{text: "3. Persist the chosen item in page state, then restore it with set_selected_item(cx, ...) or set_selected_by_label(..., cx)."}
        mod.widgets.GalleryActionFlowStep{text: "4. The popup interaction stays inside the component; the page only reacts to the semantic selection result."}
    },
}

#[derive(Script, Widget)]
pub struct GallerySelectPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GallerySelectPage {
    fn sync_ui(&self, cx: &mut Cx) {
        let status = self.view.drop_down(cx, ids!(status_select));
        let city = self.view.drop_down(cx, ids!(city_select));

        if status.selected_label().is_empty() {
            status.set_selected_item(cx, 0);
        }
        if city.selected_label().is_empty() {
            city.set_selected_item(cx, 0);
        }

        let status_label = status.selected_label();
        let city_label = city.selected_label();
        let is_default = status_label == "Pending" && city_label == "Toronto";

        self.view
            .button(cx, ids!(select_reset_btn))
            .set_enabled(cx, !is_default);
        self.view
            .label(cx, ids!(select_status))
            .set_text(cx, &format!("Status: {status_label}. City: {city_label}."));
    }
}

impl ScriptHook for GallerySelectPage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| self.sync_ui(cx));
    }
}

impl Widget for GallerySelectPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let status = self.view.drop_down(cx, ids!(status_select));
            let city = self.view.drop_down(cx, ids!(city_select));

            if self
                .view
                .button(cx, ids!(select_done_btn))
                .clicked(actions)
            {
                status.set_selected_by_label("Done", cx);
                self.sync_ui(cx);
                return;
            }

            if self
                .view
                .button(cx, ids!(select_montreal_btn))
                .clicked(actions)
            {
                city.set_selected_by_label("Montreal", cx);
                self.sync_ui(cx);
                return;
            }

            if self
                .view
                .button(cx, ids!(select_reset_btn))
                .clicked(actions)
            {
                status.set_selected_item(cx, 0);
                city.set_selected_item(cx, 0);
                self.sync_ui(cx);
                return;
            }

            if status.changed(actions).is_some()
                || status.changed_label(actions).is_some()
                || city.changed(actions).is_some()
                || city.changed_label(actions).is_some()
            {
                self.sync_ui(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
