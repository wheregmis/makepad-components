use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::button::{ShadButtonRef, ShadButtonWidgetExt};
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryButtonPage,
    page: button_page,
    title: "Button",
    subtitle: "Buttons are prop-driven leaf actions: set `variant:` and `size:` on the instance you need, then listen for clicked(actions) in the page or feature controller.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Interaction Probe" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 8.0

            button_probe_btn := ShadButton{
                text: "Probe button"
            }

            button_probe_state := ShadFieldDescription{
                text: "hovered=false down=false focused=false active=false enabled=true visible=true"
            }
        }

        ShadSectionHeader{ text: "Variants" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0

            ShadButton{text: "Default"}
            ShadButton{variant: ShadButtonVariant.Destructive text: "Destructive"}
            ShadButton{variant: ShadButtonVariant.Outline text: "Outline"}
            ShadButton{variant: ShadButtonVariant.Secondary text: "Secondary"}
            ShadButton{variant: ShadButtonVariant.Ghost text: "Ghost"}
            ShadButton{variant: ShadButtonVariant.Link text: "Link"}
        }

        ShadSectionHeader{ text: "Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButton{size: ShadControlSize.Small text: "Small"}
            ShadButton{text: "Default"}
            ShadButton{size: ShadControlSize.Large text: "Large"}
        }

        ShadSectionHeader{ text: "Destructive Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButton{
                variant: ShadButtonVariant.Destructive
                size: ShadControlSize.Small
                text: "Small"
            }
            ShadButton{variant: ShadButtonVariant.Destructive text: "Default"}
            ShadButton{
                variant: ShadButtonVariant.Destructive
                size: ShadControlSize.Large
                text: "Large"
            }
        }

        ShadSectionHeader{ text: "Outline Variations" }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButton{variant: ShadButtonVariant.Outline text: "Outline"}
            ShadButton{variant: ShadButtonVariant.Ghost text: "Ghost"}
            ShadButton{variant: ShadButtonVariant.Link text: "Link"}
        }

        ShadSectionHeader{ text: "Icons" }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 14.0

            IconCheck{}
            IconX{}
            IconSearch{}
            IconInfo{}
            IconChevronLeft{}
            IconChevronRight{}
            IconChevronDown{}
        }

        ShadSectionHeader{ text: "Icon Buttons" }

        View{
            width: Fill
            height: Fit
            flow: Right{wrap: true}
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButtonIconSm{text: "✓"}
            ShadButtonIcon{text: "✓"}
            ShadButtonIconLg{text: "✓"}

            IconButtonChevronLeft{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                }
                draw_icon.color: (shad_theme.color_primary)
            }

            IconButtonChevronRight{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                }
                draw_icon.color: (shad_theme.color_primary)
            }

            IconButtonX{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 0.0
                    border_radius: (shad_theme.radius)
                    border_color: #0000
                }
                draw_icon.color: (shad_theme.color_muted_foreground)
            }

            IconButtonMoreHorizontal{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 0.0
                    border_radius: (shad_theme.radius)
                    border_color: #0000
                }
                draw_icon.color: (shad_theme.color_primary)
            }
        }

    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give the button an id and read clicks with `ui.shad_button(cx, ids!(probe_btn)).clicked(actions)`."}
        mod.widgets.GalleryActionFlowStep{text: "2. Button refs also expose hover, down, focus, active, enabled, and visible queries for interaction tests."}
        mod.widgets.GalleryActionFlowStep{text: "3. Keep state in the page or feature owner so the button remains a leaf widget."}
        mod.widgets.GalleryActionFlowStep{text: "4. Use a small probe section to verify hover, focus, and press behavior without coupling tests to layout internals."}
    },
}

#[derive(Script, Widget)]
pub struct GalleryButtonPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryButtonPage {
    fn probe_button(&self, cx: &Cx) -> ShadButtonRef {
        self.view.shad_button(cx, ids!(button_probe_btn))
    }

    fn sync_probe_state(&self, cx: &mut Cx) {
        let button = self.probe_button(cx);
        self.view.label(cx, ids!(button_probe_state)).set_text(
            cx,
            &format!(
                "hovered={} down={} focused={} active={} enabled={} visible={}",
                button.is_hovered(cx),
                button.is_down(cx),
                button.is_focused(cx),
                button.is_active(),
                button.is_enabled(),
                button.is_visible()
            ),
        );
    }
}

impl ScriptHook for GalleryButtonPage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| self.sync_probe_state(cx));
    }
}

impl Widget for GalleryButtonPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.sync_probe_state(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
