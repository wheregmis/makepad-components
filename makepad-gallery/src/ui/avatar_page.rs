use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAvatarPage,
    page: avatar_page,
    title: "Avatar",
    subtitle: "Circular avatar surfaces for photos, fallback initials, and presence states. Configure each avatar with `size:` and optional `status:`.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Anatomy" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "ShadAvatar is a compositional circular surface. Set `size: ShadAvatarSize.Small/Default/Large` and keep the fallback text or image children inside the same avatar."}
            ShadFieldDescription{text: "Use ShadAvatarImage for a fill-sized, cover-cropped photo. Keep ShadAvatarFallback in the same avatar so empty or loading states still have a readable identity."}
            ShadFieldDescription{text: "Set `status: ShadAvatarPresence.Online/Away/Busy` when you need a small presence marker anchored to the avatar edge."}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Photo Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{y: 0.0}

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Small
                    fallback := ShadAvatarFallback{text: "ML"}
                    image := ShadAvatarImage{
                        src: crate_resource("self://resources/avatar/portrait-a.jpg")
                    }
                }

                ShadFieldLabel{text: "Small"}
                ShadFieldDescription{
                    width: 110
                    text: "Compact lists, mentions, and dense menus."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Default
                    fallback := ShadAvatarFallback{text: "CN"}
                    image := ShadAvatarImage{
                        src: crate_resource("self://resources/avatar/portrait-a.jpg")
                    }
                }

                ShadFieldLabel{text: "Default"}
                ShadFieldDescription{
                    width: 120
                    text: "Standard profile surfaces in app chrome."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Large
                    fallback := ShadAvatarFallback{text: "AB"}
                    image := ShadAvatarImage{
                        src: crate_resource("self://resources/avatar/portrait-b.jpg")
                    }
                }

                ShadFieldLabel{text: "Large"}
                ShadFieldDescription{
                    width: 120
                    text: "Profile cards, account switchers, and headers."
                }
            }
        }

        ShadSectionHeader{ text: "Fallbacks" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{y: 0.0}

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Default
                    fallback := ShadAvatarFallback{text: "JD"}
                }

                ShadFieldLabel{text: "Initials"}
                ShadFieldDescription{
                    width: 110
                    text: "Use when the user has no uploaded photo."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Default
                    fallback := ShadAvatarFallback{text: "AB"}
                }

                ShadFieldLabel{text: "Team Member"}
                ShadFieldDescription{
                    width: 110
                    text: "Letter pairs keep list rows identifiable."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Default
                    fallback := ShadAvatarFallback{text: "?"}
                }

                ShadFieldLabel{text: "Unknown"}
                ShadFieldDescription{
                    width: 110
                    text: "Fallback for anonymous or unresolved people."
                }
            }
        }

        ShadSectionHeader{ text: "Presence" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{y: 0.0}

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Default
                    fallback := ShadAvatarFallback{text: "ML"}
                    image := ShadAvatarImage{
                        src: crate_resource("self://resources/avatar/portrait-a.jpg")
                    }
                    status: ShadAvatarPresence.Online
                }

                ShadFieldLabel{text: "Online"}
                ShadFieldDescription{
                    width: 110
                    text: "Green status dot for active presence."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Default
                    fallback := ShadAvatarFallback{text: "AB"}
                    image := ShadAvatarImage{
                        src: crate_resource("self://resources/avatar/portrait-b.jpg")
                    }
                    status: ShadAvatarPresence.Away
                }

                ShadFieldLabel{text: "Away"}
                ShadFieldDescription{
                    width: 110
                    text: "Muted dot for idle, away, or offline states."
                }
            }

            View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 8.0
                align: Align{x: 0.5, y: 0.0}

                ShadAvatar{
                    size: ShadAvatarSize.Default
                    fallback := ShadAvatarFallback{text: "CN"}
                    image := ShadAvatarImage{
                        src: crate_resource("self://resources/avatar/portrait-a.jpg")
                    }
                    status: ShadAvatarPresence.Busy
                }

                ShadFieldLabel{text: "Busy"}
                ShadFieldDescription{
                    width: 110
                    text: "Destructive dot for do-not-disturb or busy presence."
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Start with ShadAvatar and set `size: ShadAvatarSize.Small/Default/Large` depending on the layout density you need."}
        mod.widgets.GalleryActionFlowStep{text: "2. Keep ShadAvatarFallback in the avatar for identity-safe empty states, then add ShadAvatarImage when a real profile photo is available."}
        mod.widgets.GalleryActionFlowStep{text: "3. Add `status: ShadAvatarPresence.Online/Away/Busy` only when presence matters to the workflow."}
        mod.widgets.GalleryActionFlowStep{text: "4. Keep avatar groups, notification counts, and richer profile metadata outside the primitive so the base avatar stays small and composable."}
    },
}
