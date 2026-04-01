use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadBreadcrumb = mod.widgets.View{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 4.0
    }

    mod.widgets.ShadBreadcrumbLink = mod.widgets.LinkLabel{
        width: Fit
        height: Fit
        margin: Inset{}
        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_disabled_foreground)
            text_style.font_size: 11
        }
        draw_bg +: {
            color: (shad_theme.color_clear)
            color_hover: (shad_theme.color_clear)
            color_down: (shad_theme.color_clear)
            color_focus: (shad_theme.color_clear)
            color_disabled: (shad_theme.color_clear)
        }
    }

    mod.widgets.ShadBreadcrumbPage = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadBreadcrumbSeparator = mod.widgets.IconChevronRight{
        icon_walk: Walk{width: 10, height: 10}
        draw_icon.color: (shad_theme.color_outline_border_hover)
    }

    mod.widgets.ShadBreadcrumbEllipsis = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 11
        text: "..."
    }
}
