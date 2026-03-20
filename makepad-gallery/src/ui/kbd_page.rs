use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryKbdPage,
    page: kbd_page,
    title: "Kbd",
    subtitle: "Keyboard shortcut key caps for displaying shortcuts (e.g. ⌘ ⇧ ⌥ ⌃ or Ctrl + B).",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        kbd_demo_shell := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadFieldDescription{
                width: Fill
                text: "Use Kbd chips anywhere you need shortcut copy: menu rows, dialog footers, onboarding hints, or command overlays."
            }

            ShadSurface{
                width: Fill
                height: Fit
                flow: Down
                spacing: 14.0
                padding: Inset{left: 18, right: 18, top: 18, bottom: 18}
                draw_bg +: {
                    color: (shad_theme.color_secondary)
                    border_size: 1.0
                    border_color: (shad_theme.color_outline_border)
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 8.0

                    ShadSectionHeader{ text: "Modifier keys" }
                    ShadFieldDescription{
                        width: Fill
                        text: "Keep single keys as separate caps so layouts can mix Mac and Windows copy without changing the component."
                    }

                    View{
                        width: Fit
                        height: Fit
                        flow: Right{wrap: true}
                        spacing: 6.0
                        align: Align{y: 0.5}

                        ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
                        ShadKbd{ label := ShadKbdLabel{text: "Shift"} }
                        ShadKbd{ label := ShadKbdLabel{text: "Option"} }
                        ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
                    }
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 10.0

                    ShadSectionHeader{ text: "Shortcut examples" }
                    ShadFieldDescription{
                        width: Fill
                        text: "Compose the caps inline with labels so the shortcut reads like product copy instead of a standalone control."
                    }

                    View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        View{
                            width: Fill
                            height: Fit
                            flow: Right{wrap: true}
                            spacing: 16.0
                            align: Align{y: 0.5}

                            ShadLabel{
                                width: 140
                                text: "Open command palette"
                            }

                            View{
                                width: Fit
                                height: Fit
                                flow: Right{wrap: true}
                                spacing: 6.0
                                align: Align{y: 0.5}

                                ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
                                ShadKbdSeparator{}
                                ShadKbd{ label := ShadKbdLabel{text: "K"} }
                            }

                            View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 6.0
                                align: Align{y: 0.5}

                                ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
                                ShadKbdSeparator{}
                                ShadKbd{ label := ShadKbdLabel{text: "K"} }
                            }
                        }

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 16.0
                            align: Align{y: 0.5}

                            ShadLabel{
                                width: 140
                                text: "Duplicate current row"
                            }

                            View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 6.0
                                align: Align{y: 0.5}

                                ShadKbd{ label := ShadKbdLabel{text: "Shift"} }
                                ShadKbdSeparator{}
                                ShadKbd{ label := ShadKbdLabel{text: "Alt"} }
                                ShadKbdSeparator{}
                                ShadKbd{ label := ShadKbdLabel{text: "Down"} }
                            }
                        }
                    }
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Use ShadKbd as display-only shortcut copy next to buttons, menus, or helper text; it does not own the shortcut behavior."}
        mod.widgets.GalleryActionFlowStep{text: "2. Put each key in its own ShadKbd and set the visible text through ShadKbdLabel so platform-specific labels stay easy to swap."}
        mod.widgets.GalleryActionFlowStep{text: "3. Use ShadKbdSeparator between keys inside a combo, and skip it when you are showing standalone modifier chips."}
        mod.widgets.GalleryActionFlowStep{text: "4. Keep the real key handling in the surrounding page or app shell, then render the shortcut string here as part of the product copy."}
    },
}
