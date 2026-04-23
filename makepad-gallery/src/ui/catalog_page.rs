use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCatalogPage = GalleryPageRoot{
        width: Fill
        height: Fill

        ShadPageTitle{
            text: "Component Catalog"
            margin: Inset{top: 0}
        }

        ShadPageSubtitle{
            text: "A single-page preview of the gallery's components, grouped by family."
        }

        ShadHr{}

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 32.0

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadSectionHeader{text: "Buttons & Badges"}

                ShadScrollAreaX{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 12.0
                    ShadButton{text: "Default"}
                    ShadButtonSecondary{text: "Secondary"}
                    ShadButtonOutline{text: "Outline"}
                    ShadButtonGhost{text: "Ghost"}
                    ShadButtonDestructive{text: "Destructive"}
                    ShadBadge{ ShadBadgeLabel{text: "Badge"} }
                    ShadBadgeSecondary{ ShadBadgeSecondaryLabel{text: "Secondary"} }
                    ShadBadgeOutline{ ShadBadgeOutlineLabel{text: "Outline"} }
                    ShadBadgeDestructive{ ShadBadgeDestructiveLabel{text: "Destructive"} }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadSectionHeader{text: "Forms"}

                ShadScrollAreaX{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 16.0

                    ShadSurfaceMuted{
                        width: 280
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        ShadLabel{text: "Search"}
                        ShadInput{text: "Search..."}
                        ShadTextarea{text: "Write your message here."}
                    }

                    ShadSurfaceMuted{
                        width: 280
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        ShadLabel{text: "Selection"}
                        View{
                            width: Fit
                            height: Fit
                            flow: Right
                            spacing: 16.0
                            ShadCheckbox{label: "Check me"}
                            ShadSwitch{text: "Toggle me"}
                        }
                        ShadSlider{default: 0.5}
                    }

                    ShadSurfaceMuted{
                        width: 280
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        ShadLabel{text: "Pickers"}
                        ShadCalendar{value: "2026-03-13"}
                        ShadDatePicker{width: Fill value: "2026-03-13"}
                        ShadInputOtp{}
                        ShadRadioGroup{
                            starter_plan := ShadRadioItem{text: "Starter"}
                            pro_plan := ShadRadioItem{text: "Pro"}
                            enterprise_plan := ShadRadioItem{text: "Enterprise"}
                        }
                        ShadSelect{labels: ["Pending" "In Progress" "Done"]}
                    }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadSectionHeader{text: "Feedback & Text"}

                ShadScrollAreaX{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 16.0

                    ShadSurfaceMuted{
                        width: 320
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        ShadAlert{
                            ShadAlertIcon{}
                            ShadAlertContent{
                                ShadAlertTitle{text: "Heads up!"}
                                ShadAlertDescription{text: "You can add components to your app with a single copy-paste."}
                            }
                        }
                    }

                    ShadSurfaceMuted{
                        width: 240
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        ShadLabel{text: "Loading"}
                        ShadSkeleton{width: 180 height: 18}
                        ShadSkeleton{width: 240 height: 14}
                        ShadSpinner{}
                    }

                    ShadSurfaceMuted{
                        width: 240
                        height: Fit
                        flow: Down
                        spacing: 8.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        ShadLabel{text: "Toast"}
                        ShadSonner{open: false}
                    }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadSectionHeader{text: "Layout & Surface"}

                ShadScrollAreaX{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 16.0

                    ShadSurface{
                        width: 220
                        height: Fit
                        flow: Down
                        spacing: 6.0
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        ShadLabel{text: "Surface"}
                        ShadFieldDescription{text: "Default framed container."}
                    }

                    ShadPanel{
                        width: 220
                        height: Fit
                        flow: Down
                        spacing: 6.0
                        ShadLabel{text: "Panel"}
                        ShadFieldDescription{text: "Transparent inset wrapper."}
                    }

                    ShadAspectRatio{
                        width: 240
                        ratio: 1.7777777778
                        ShadMediaFrame{
                            ShadImage{
                                src: crate_resource("self://resources/aspect-ratio/royal-esplanade.jpg")
                            }
                        }
                    }

                    ShadResizable{
                        width: 240
                        align: SplitterAlign.FromA(120.0)
                        a: ShadSurfaceMuted{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 4.0
                            padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
                            ShadLabel{text: "Pane A"}
                        }
                        b: ShadSurfaceMuted{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 4.0
                            padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
                            ShadLabel{text: "Pane B"}
                        }
                    }

                    ShadScrollArea{
                        width: 240
                        height: 160
                        View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 6.0
                            ShadLabel{text: "Row 1"}
                            ShadLabel{text: "Row 2"}
                            ShadLabel{text: "Row 3"}
                            ShadLabel{text: "Row 4"}
                            ShadLabel{text: "Row 5"}
                        }
                    }

                    ShadScrollAreaX{
                        width: 240
                        height: 120
                        flow: Right
                        spacing: 8.0
                        View{
                            width: 520
                            height: Fit
                            flow: Right
                            spacing: 8.0
                            ShadLabel{text: "A"}
                            ShadLabel{text: "B"}
                            ShadLabel{text: "C"}
                            ShadLabel{text: "D"}
                        }
                    }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadSectionHeader{text: "Navigation & Overlay"}

                ShadScrollAreaX{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 16.0

                    ShadBreadcrumb{
                        ShadBreadcrumbLink{text: "Home"}
                        ShadBreadcrumbSeparator{}
                        ShadBreadcrumbLink{text: "Components"}
                        ShadBreadcrumbSeparator{}
                        ShadBreadcrumbPage{text: "Catalog"}
                    }

                    ShadTabs{
                        width: 360
                        ShadTabsList{
                            ShadTabsTrigger{text: "Account"}
                            ShadTabsTrigger{text: "Password"}
                        }
                    }

                    ShadPagination{
                        current_page: 5
                        page_count: 12
                    }

                    ShadSidebar{
                        nav_playground := ShadSidebarItem{text: "Playground"}
                        nav_history := ShadSidebarItem{text: "History"}
                        nav_settings := ShadSidebarItem{text: "Settings"}
                    }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadSectionHeader{text: "Collections & Data"}

                ShadScrollAreaX{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 16.0

                    ShadCard{
                        width: 360
                        ShadCardHeader{
                            ShadCardTitle{text: "Team Access"}
                            ShadCardDescription{text: "Review seats and pending invites before applying changes."}
                        }
                        ShadCardContent{
                            ShadFieldDescription{text: "Seats in use: 18 of 25"}
                            ShadFieldDescription{text: "Pending invites: 3 awaiting acceptance"}
                        }
                        ShadCardFooter{
                            ShadButtonGhost{text: "Cancel"}
                            ShadButton{text: "Review changes"}
                        }
                    }

                    ShadCarousel{width: 320}

                    ShadAccordion{
                        item_accessible := ShadAccordionItem{
                            title: "Is it accessible?"
                            is_open: true
                            body: ShadLabel{text: "Yes. It supports keyboard interaction and semantic structure."}
                        }
                    }

                    ShadButtonGroup{
                        archive_btn := ShadButtonGroupItem{text: "Archive"}
                        ShadButtonGroupSeparator{}
                        report_btn := ShadButtonGroupItem{text: "Report"}
                    }

                    ShadCollapsible{
                        title: "Order #4189"
                        is_open: true
                        body: View{
                            flow: Down
                            spacing: 6.0
                            ShadLabel{text: "Status: Shipped"}
                            ShadLabel{text: "Shipping Address: 123 Main St, New York, NY"}
                            ShadLabel{text: "Items: 2 x T-shirt, 1 x Hoodie"}
                        }
                    }

                    ShadKbd{
                        label := ShadKbdLabel{text: "Cmd"}
                    }
                }
            }

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 16.0

                ShadSectionHeader{text: "Charts & Tables"}

                ShadScrollAreaX{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 16.0

                    ShadLineChart{}
                    ShadAreaChart{}
                    ShadBarChart{}

                    ShadTable{
                        caption: "Team roster"
                        headers: ["Name" "Role" "Location" "Status"]
                        rows: []
                    }
                }
            }

        }
    }
}
