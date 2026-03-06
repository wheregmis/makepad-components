use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadBreadcrumb = mod.widgets.View{
        width: Fit
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 6.0
    }

    mod.widgets.ShadBreadcrumbLink = mod.widgets.LinkLabel{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 11
        draw_text.color_hover: (shad_theme.color_primary)
    }

    mod.widgets.ShadBreadcrumbPage = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadBreadcrumbSeparator = mod.widgets.IconChevronRight{
        icon_walk: Walk{width: 12, height: 12}
        draw_icon.color: (shad_theme.color_muted_foreground)
    }

    mod.widgets.ShadBreadcrumbEllipsis = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
        text: "..."
    }
}
