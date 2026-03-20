use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryInputPage,
    page: input_page,
    title: "Input",
    subtitle: "Inputs are page-owned draft state: keep a visible label, use placeholder text as a hint, and use TextInputRef methods for live changes or submit-on-return.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Field with persistent label" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldLabel{ text: "Email address" }
            ShadInput{
                empty_text: "you@example.com"
            }
            ShadFieldDescription{
                text: "Keep the label visible so the field purpose stays clear after someone starts typing."
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "Read only" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldLabel{ text: "Workspace slug" }
            workspace_slug_input := ShadInput{
                is_read_only: true
            }
            ShadFieldDescription{
                text: "Use a nearby label for read-only values too, so the locked field still reads clearly."
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "With Label" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadLabel{ text: "Email" }
            ShadInput{ empty_text: "Email" }
        }

        ShadSectionHeader{ text: "With Leading Icon" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldLabel{ text: "Search components" }
            ShadInputWithIcon{
                input +: {
                    empty_text: "Search components"
                }
            }
            ShadFieldDescription{
                text: "Leading icons help with recognition, but the visible label keeps the search scope explicit."
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give any input you need to drive an id, like email_input := ShadInput{...}."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read live edits with view.text_input(cx, ids!(email_input)).changed(actions)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use returned(actions) when Enter should submit or confirm the current draft."}
        mod.widgets.GalleryActionFlowStep{text: "4. When external state changes, push it back into the field with set_text(cx, ...)."}
    },
}

#[derive(Script, Widget)]
pub struct GalleryInputPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl ScriptHook for GalleryInputPage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.view
                .text_input(cx, ids!(workspace_slug_input))
                .set_text(cx, "northwind-revamp");
        });
    }
}

impl Widget for GalleryInputPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
