use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.draw.KeyCode
    use mod.widgets.*

    let GalleryPageStackView = StackNavigationView{
        full_screen: false
        draw_bg +: {
            color: instance(shad_theme.color_background)
        }
        header +: {
            visible: false
            height: 0
        }
        body +: {
            width: Fill
            height: Fill
            flow: Overlay
            margin: Inset{top: 0, right: 0, bottom: 0, left: 0}
        }
    }

    mod.widgets.GalleryContentFlip = RouterWidget{
        width: Fill
        height: Fill
        default_route: @accordion_page
        not_found_route: @accordion_page
        sync_browser_url: true
        browser_base_path: "/makepad-components"

        accordion_page := RouterRoute{
            route_pattern: "/"
            route_transition: @none
            mod.widgets.GalleryAccordionPage{}
        }
        alert_page := RouterRoute{
            route_pattern: "/alert"
            mod.widgets.GalleryAlertPage{}
        }
        aspect_ratio_page := RouterRoute{
            route_pattern: "/aspect-ratio"
            mod.widgets.GalleryAspectRatioPage{}
        }
        avatar_page := RouterRoute{
            route_pattern: "/avatar"
            mod.widgets.GalleryAvatarPage{}
        }
        badge_page := RouterRoute{
            route_pattern: "/badge"
            mod.widgets.GalleryBadgePage{}
        }
        breadcrumb_page := RouterRoute{
            route_pattern: "/breadcrumb"
            mod.widgets.GalleryBreadcrumbPage{}
        }
        button_page := RouterRoute{
            route_pattern: "/button"
            mod.widgets.GalleryButtonPage{}
        }
        button_group_page := RouterRoute{
            route_pattern: "/button-group"
            mod.widgets.GalleryButtonGroupPage{}
        }
        card_page := RouterRoute{
            route_pattern: "/card"
            mod.widgets.GalleryCardPage{}
        }
        carousel_page := RouterRoute{
            route_pattern: "/carousel"
            mod.widgets.GalleryCarouselPage{}
        }
        checkbox_page := RouterRoute{
            route_pattern: "/checkbox"
            mod.widgets.GalleryCheckboxPage{}
        }
        collapsible_page := RouterRoute{
            route_pattern: "/collapsible"
            mod.widgets.GalleryCollapsiblePage{}
        }
        command_palette_page := RouterRoute{
            route_pattern: "/command-palette"
            mod.widgets.GalleryCommandPalettePage{}
        }
        context_menu_page := RouterRoute{
            route_pattern: "/context-menu"
            mod.widgets.GalleryContextMenuPage{}
        }
        dialog_page := RouterRoute{
            route_pattern: "/dialog"
            mod.widgets.GalleryDialogPage{}
        }
        input_page := RouterRoute{
            route_pattern: "/input"
            mod.widgets.GalleryInputPage{}
        }
        input_otp_page := RouterRoute{
            route_pattern: "/input-otp"
            mod.widgets.GalleryInputOtpPage{}
        }
        pagination_page := RouterRoute{
            route_pattern: "/pagination"
            mod.widgets.GalleryPaginationPage{}
        }
        popover_page := RouterRoute{
            route_pattern: "/popover"
            mod.widgets.GalleryPopoverPage{}
        }
        radio_group_page := RouterRoute{
            route_pattern: "/radio-group"
            mod.widgets.GalleryRadioGroupPage{}
        }
        resizable_page := RouterRoute{
            route_pattern: "/resizable"
            mod.widgets.GalleryResizablePage{}
        }
        scroll_area_page := RouterRoute{
            route_pattern: "/scroll-area"
            mod.widgets.GalleryScrollAreaPage{}
        }
        select_page := RouterRoute{
            route_pattern: "/select"
            mod.widgets.GallerySelectPage{}
        }
        separator_page := RouterRoute{
            route_pattern: "/separator"
            mod.widgets.GallerySeparatorPage{}
        }
        sheet_page := RouterRoute{
            route_pattern: "/sheet"
            mod.widgets.GallerySheetPage{}
        }
        skeleton_page := RouterRoute{
            route_pattern: "/skeleton"
            mod.widgets.GallerySkeletonPage{}
        }
        switch_page := RouterRoute{
            route_pattern: "/switch"
            mod.widgets.GallerySwitchPage{}
        }
        tabs_page := RouterRoute{
            route_pattern: "/tabs"
            mod.widgets.GalleryTabsPage{}
        }
        textarea_page := RouterRoute{
            route_pattern: "/textarea"
            mod.widgets.GalleryTextareaPage{}
        }
        toggle_page := RouterRoute{
            route_pattern: "/toggle"
            mod.widgets.GalleryTogglePage{}
        }
        kbd_page := RouterRoute{
            route_pattern: "/kbd"
            mod.widgets.GalleryKbdPage{}
        }
        label_page := RouterRoute{
            route_pattern: "/label"
            mod.widgets.GalleryLabelPage{}
        }
        progress_page := RouterRoute{
            route_pattern: "/progress"
            mod.widgets.GalleryProgressPage{}
        }
        sidebar_page := RouterRoute{
            route_pattern: "/sidebar"
            mod.widgets.GallerySidebarPage{}
        }
        slider_page := RouterRoute{
            route_pattern: "/slider"
            mod.widgets.GallerySliderPage{}
        }
        sonner_page := RouterRoute{
            route_pattern: "/sonner"
            mod.widgets.GallerySonnerPage{}
        }
        spinner_page := RouterRoute{
            route_pattern: "/spinner"
            mod.widgets.GallerySpinnerPage{}
        }
    }

    mod.widgets.GalleryMobileHeader = View{
        width: Fill
        height: Fit
        visible: false
        flow: Right
        align: Align{y: 0.5}
        padding: Inset{left: 16, right: 16, top: 12, bottom: 12}
        spacing: 12.0
        draw_bg.color: (shad_theme.color_background)

        mobile_sidebar_button := ShadButtonGhost{
            width: 36
            height: 36
            padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
            text: "☰"
            draw_text.text_style.font_size: 16
        }

        ShadLabel{
            text: "Components"
            draw_text.text_style.font_size: 11
            draw_text.color: (shad_theme.color_muted_foreground)
        }

        View{
            width: Fill
            height: Fit
        }

        mobile_theme_toggle := ShadButtonOutline{
            text: "Light mode"
        }
    }

    mod.widgets.GalleryMainContent = View{
        width: Fill
        height: Fill
        flow: Down

        mobile_header := mod.widgets.GalleryMobileHeader{}
        content_flip := mod.widgets.GalleryContentFlip{}
    }

    mod.widgets.GalleryAppShell = View{
        width: Fill
        height: Fill
        flow: Right
        sidebar := mod.widgets.GallerySidebar{}
        main_content := mod.widgets.GalleryMainContent{}
    }

    mod.widgets.GalleryAppUi = Root{
        main_window := Window{
            window.inner_size: vec2(1400 900)
            window.title: "Makepad Components Gallery"
            pass +: { clear_color: (shad_theme.color_background) }
            window_menu +: {
                command_palette_menu := MenuItem.Item {
                    name: "Command Palette"
                    key: KeyCode.KeyK
                    enabled: true
                }
                view_menu := MenuItem.Sub {
                    name: "View"
                    items: [@zoom_in, @zoom_out, @line9, @command_palette_menu, @fullscreen]
                }
            }
            body +: {
                width: Fill
                height: Fill
                flow: Overlay
                draw_bg.color: (shad_theme.color_background)

                app_shell := mod.widgets.GalleryAppShell{}
                command_palette := mod.widgets.GalleryCommandPalette{}
            }
        }
    }
}
