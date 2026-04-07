use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSurface = RoundedView{
        width: Fill
        height: Fit

        draw_bg +: {
            color: (shad_theme.color_background)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadSurfaceTransparent = mod.widgets.ShadSurface{
        draw_bg.color: #0000
    }

    mod.widgets.ShadSurfacePanel = mod.widgets.ShadSurface{
        width: Fill
        height: Fit
        flow: Down
        spacing: 0.0
        padding: 0.
    }

    mod.widgets.ShadSurfaceMutedPanel = mod.widgets.ShadSurfacePanel{
        draw_bg.color: (shad_theme.color_muted)
    }

    mod.widgets.ShadSurfaceSecondary = mod.widgets.ShadSurfacePanel{
        draw_bg.color: (shad_theme.color_secondary)
    }

    mod.widgets.ShadSurfacePopover = mod.widgets.ShadSurfacePanel{
        new_batch: true
        draw_bg.color: (shad_theme.color_popover)
    }

    mod.widgets.ShadSurfaceSection = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
    }

    mod.widgets.ShadSurfaceHeader = mod.widgets.ShadSurfaceSection{
        spacing: 6.0
        padding: Inset{left: 20, right: 20, top: 20, bottom: 12}
    }

    mod.widgets.ShadSurfaceContent = mod.widgets.ShadSurfaceSection{
        spacing: 12.0
        padding: Inset{left: 20, right: 20, top: 0, bottom: 16}
    }

    mod.widgets.ShadSurfaceFooter = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{x: 1.0, y: 0.5}
        spacing: 8.0
        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
    }

    mod.widgets.ShadSurfaceMuted = mod.widgets.ShadSurface{
        draw_bg.color: (shad_theme.color_muted)
    }

    mod.widgets.ShadMediaFrame = mod.widgets.ShadSurface{
        width: Fill
        height: Fill
        flow: Overlay
        clip_x: true
        clip_y: true
        draw_bg.color: (shad_theme.color_secondary)
    }
}
