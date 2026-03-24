use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""

        container := ShadSurfaceMuted{
            width: Fill
            height: Fit
            padding: Inset{top: 12, right: 12, bottom: 12, left: 12}
            draw_bg +: {
                color: (shad_theme.color_muted)
                border_radius: (shad_theme.radius)
                border_size: 0.0
            }

            code_view := CodeView{
                text: ""
                editor +: {
                    width: Fill
                    height: Fit
                }
            }
        }
    }

    mod.widgets.GalleryActionFlowStep = ShadFieldDescription{
        width: Fill
    }

    mod.widgets.GalleryButtonDestructiveSm = ShadButtonDestructive{
        height: 28
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        draw_text.text_style.font_size: 10
    }

    mod.widgets.GalleryButtonDestructiveLg = ShadButtonDestructive{
        height: 44
        padding: Inset{left: 32, right: 32, top: 0, bottom: 0}
        draw_text.text_style.font_size: 13
    }

    mod.widgets.GalleryButtonGroupItemIcon = ShadButtonGroupItem{
        width: 36
        spacing: 0.0
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
    }

    mod.widgets.GalleryButtonIconChevronLeft = IconButtonChevronLeft{
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

    mod.widgets.GalleryButtonIconChevronRight = IconButtonChevronRight{
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

    mod.widgets.GalleryButtonIconX = IconButtonX{
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

    mod.widgets.GalleryButtonIconMoreHorizontal = IconButtonMoreHorizontal{
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

    mod.widgets.GalleryActionFlow = ShadSurfaceMuted{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
        draw_bg +: {
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        title := ShadSectionHeader{
            text: "Action Flow"
        }

        body := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 8.0
        }
    }

    mod.widgets.GalleryPreviewStackNavigation = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 16.0

        preview_title := ShadSectionHeader{
            text: "Preview"
        }

        root_view := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0
        }

        code_page := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            code_title := ShadSectionHeader{
                text: "Code"
            }

            body := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0
            }
        }
    }

    mod.widgets.GalleryPreviewSection = View{
        width: Fill
        height: Fit
        flow: Down

        preview_panel := mod.widgets.ShadPanel{
            preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                width: Fill
                height: Fit

                root_view +: {
                    preview_content := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                    }

                    action_flow := View{
                        visible: false
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0
                    }
                }

                code_page +: {
                    body +: {
                        code_snippet := mod.widgets.GalleryCodeSnippet{}
                    }
                }
            }
        }
    }
}

#[derive(Script, Widget)]
pub struct GalleryCodeSnippet {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    code: ArcStringMut,
}

impl ScriptHook for GalleryCodeSnippet {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.view
                .widget(cx, ids!(container.code_view))
                .set_text(cx, self.code.as_ref());
        });
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
