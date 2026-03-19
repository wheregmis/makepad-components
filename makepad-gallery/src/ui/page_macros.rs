macro_rules! gallery_static_page {
    (
        root: $root:ident,
        $($rest:tt)*
    ) => {
        gallery_static_page!(@impl $root, $($rest)*);
    };

    (
        @impl $root:ident,
        $(shell: { $($shell:tt)* },)?
        widget: $widget:ident,
        page: $page:ident,
        title: $title:literal,
        subtitle: $subtitle:literal,
        divider: { $($divider:tt)* },
        preview_spacing: $preview_spacing:literal,
        preview: { $($preview:tt)* }
        $(,
            action_flow: { $($action_flow:tt)* }
        )?
        $(,
            after_root: { $($after_root:tt)* }
        )?
        $(,)?
    ) => {
        script_mod! {
            use mod.prelude.widgets.*
            use mod.widgets.*

            mod.widgets.$widget = $root{
                width: Fill
                height: Fill

                ShadPageTitle{
                    text: $title
                }

                ShadPageSubtitle{
                    text: $subtitle
                }

                $($divider)*

                preview_section := mod.widgets.GalleryPreviewSection{
                    width: Fill
                    height: Fit

                    preview_panel +: {
                        preview_flip +: {
                            root_view +: {
                                preview_content +: {
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: $preview_spacing

                                    $($preview)*
                                }

                                $(
                                    action_flow +: {
                                        visible: true
                                        mod.widgets.GalleryActionFlow{
                                            body +: {
                                                $($action_flow)*
                                            }
                                        }
                                    }
                                )?
                            }

                            code_page +: {
                                body +: {
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12.0

                                    code_snippet +: {
                                        code_resource: #(crate::ui::gallery_snippet_resource(vm, stringify!($page)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        pub fn register_gallery_route_bundle(vm: &mut ScriptVm) {
            crate::ui::registry::$page::register_bundle_dependencies(vm);
            script_mod(vm);
            let template = script_eval!(vm, {
                mod.widgets.$widget{}
            });
            crate::ui::publish_gallery_page_template(
                vm,
                live_id!($page),
                template,
            );
        }

        #[unsafe(export_name = concat!("gallery_bundle_mark_", stringify!($page)))]
        pub extern "C" fn gallery_bundle_mark() -> usize {
            crate::ui::with_gallery_bundle_vm(|vm| {
                register_gallery_route_bundle(vm);
                1usize
            })
            .unwrap_or(0)
        }
    };

    (
        $($rest:tt)*
    ) => {
        gallery_static_page!(@impl ShadScrollYView, $($rest)*);
    };
}
pub(crate) use gallery_static_page;

macro_rules! gallery_stateful_page_shell {
    (
        root: $root:ident,
        shell: { $($shell:tt)* },
        widget: $widget:ident,
        page: $page:ident,
        title: $title:literal,
        subtitle: $subtitle:literal,
        divider: { $($divider:tt)* },
        preview_spacing: $preview_spacing:literal,
        preview: { $($preview:tt)* }
        $(,
            action_flow: { $($action_flow:tt)* }
        )?
        $(,)?
    ) => {
        gallery_stateful_page_shell!(
            @impl
            $root,
            { $($shell)* },
            {},
            widget: $widget,
            page: $page,
            title: $title,
            subtitle: $subtitle,
            divider: { $($divider)* },
            preview_spacing: $preview_spacing,
            preview: { $($preview)* }
            $(,
                action_flow: { $($action_flow)* }
            )?
        );
    };

    (
        root: $root:ident,
        shell: { $($shell:tt)* },
        widget: $widget:ident,
        page: $page:ident,
        title: $title:literal,
        subtitle: $subtitle:literal,
        divider: { $($divider:tt)* },
        preview_spacing: $preview_spacing:literal,
        preview: { $($preview:tt)* }
        $(,
            action_flow: { $($action_flow:tt)* }
        )?
        ,
        after_root: { $($after_root:tt)* }
        $(,)?
    ) => {
        gallery_stateful_page_shell!(
            @impl
            $root,
            { $($shell)* },
            { $($after_root)* },
            widget: $widget,
            page: $page,
            title: $title,
            subtitle: $subtitle,
            divider: { $($divider)* },
            preview_spacing: $preview_spacing,
            preview: { $($preview)* }
            $(,
                action_flow: { $($action_flow)* }
            )?
        );
    };

    (
        root: $root:ident,
        $($rest:tt)*
    ) => {
        gallery_stateful_page_shell!(@impl $root, {}, {}, $($rest)*);
    };

    (
        @impl $root:ident,
        { $($shell:tt)* },
        { $($after_root:tt)* },
        widget: $widget:ident,
        page: $page:ident,
        title: $title:literal,
        subtitle: $subtitle:literal,
        divider: { $($divider:tt)* },
        preview_spacing: $preview_spacing:literal,
        preview: { $($preview:tt)* }
        $(,
            action_flow: { $($action_flow:tt)* }
        )?
        $(,)?
    ) => {
        script_mod! {
            use mod.prelude.widgets.*
            use mod.widgets.*

            mod.widgets.$widget = set_type_default() do #( $widget::register_widget(vm)){
                width: Fill
                height: Fill
                $($shell)*

                page_root := $root{
                    width: Fill
                    height: Fill

                    ShadPageTitle{
                        text: $title
                    }

                    ShadPageSubtitle{
                        text: $subtitle
                    }

                    $($divider)*

                    preview_section := mod.widgets.GalleryPreviewSection{
                        width: Fill
                        height: Fit

                        preview_panel +: {
                            preview_flip +: {
                                root_view +: {
                                    preview_content +: {
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: $preview_spacing

                                        $($preview)*
                                    }

                                    $(
                                        action_flow +: {
                                            visible: true
                                            mod.widgets.GalleryActionFlow{
                                                body +: {
                                                    $($action_flow)*
                                                }
                                            }
                                        }
                                    )?
                                }

                                code_page +: {
                                    body +: {
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 12.0

                                        code_snippet +: {
                                            code_resource: #(crate::ui::gallery_snippet_resource(vm, stringify!($page)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                $($after_root)*
            }
        }

        pub fn register_gallery_route_bundle(vm: &mut ScriptVm) {
            crate::ui::registry::$page::register_bundle_dependencies(vm);
            script_mod(vm);
            let template = script_eval!(vm, {
                mod.widgets.$widget{}
            });
            crate::ui::publish_gallery_page_template(
                vm,
                live_id!($page),
                template,
            );
        }

        #[unsafe(export_name = concat!("gallery_bundle_mark_", stringify!($page)))]
        pub extern "C" fn gallery_bundle_mark() -> usize {
            crate::ui::with_gallery_bundle_vm(|vm| {
                register_gallery_route_bundle(vm);
                1usize
            })
            .unwrap_or(0)
        }
    };

    (
        $($rest:tt)*
    ) => {
        gallery_stateful_page_shell!(@impl ShadScrollYView, {}, {}, $($rest)*);
    };
}

pub(crate) use gallery_stateful_page_shell;
