use makepad_components::makepad_widgets::*;
use std::{cell::Cell, ptr};

mod bundled_page_host;
pub mod catalog;
pub mod command_palette;
mod page_macros;
mod registry;
pub mod root;
pub mod sidebar;
pub mod themed_widgets;

use crate::ui::registry::gallery_page_entries;

#[derive(Clone, Debug, Default)]
pub enum GalleryShellAction {
    OpenCommandPalette,
    #[default]
    None,
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GalleryRouteBundleDescriptor {
    pub page: LiveId,
    pub bundle_id: &'static str,
    pub marker_symbol: &'static str,
}

thread_local! {
    static GALLERY_BUNDLE_CX: Cell<*mut Cx> = const { Cell::new(ptr::null_mut()) };
}

macro_rules! declare_gallery_page_modules {
    ($(
        {
            title: $title:literal,
            route: $route:literal,
            page: $page:ident,
            widget: $widget:ident,
            sidebar_id: $sidebar_id:ident,
            sidebar_label: $sidebar_label:literal,
            section: $section:literal,
            shortcut: $shortcut:literal,
            snippet: $snippet:ident,
            bundle: $bundle:ident,
            components: [$($components:ident),* $(,)?],
            icons: [$($icons:ident),* $(,)?],
            icon_policy: $icon_policy:ident,
            $(transition: $transition:ident,)?
        }
    )*) => {
        $(pub mod $page;)*
    };
}

gallery_page_entries!(declare_gallery_page_modules);

macro_rules! build_gallery_route_bundle_descriptors {
    (@items
        [
            $($items:tt)*
        ]
        {
            title: $title:literal,
            route: $route:literal,
            page: $page:ident,
            widget: $widget:ident,
            sidebar_id: $sidebar_id:ident,
            sidebar_label: $sidebar_label:literal,
            section: $section:literal,
            shortcut: $shortcut:literal,
            snippet: $snippet:ident,
            bundle: base,
            components: [$($components:ident),* $(,)?],
            icons: [$($icons:ident),* $(,)?],
            icon_policy: $icon_policy:ident,
            $(transition: $transition:ident,)?
        }
        $($rest:tt)*
    ) => {
        build_gallery_route_bundle_descriptors!(@items [$($items)*] $($rest)*);
    };

    (@items
        [
            $($items:tt)*
        ]
        {
            title: $title:literal,
            route: $route:literal,
            page: $page:ident,
            widget: $widget:ident,
            sidebar_id: $sidebar_id:ident,
            sidebar_label: $sidebar_label:literal,
            section: $section:literal,
            shortcut: $shortcut:literal,
            snippet: $snippet:ident,
            bundle: page,
            components: [$($components:ident),* $(,)?],
            icons: [$($icons:ident),* $(,)?],
            icon_policy: $icon_policy:ident,
            $(transition: $transition:ident,)?
        }
        $($rest:tt)*
    ) => {
        build_gallery_route_bundle_descriptors!(
            @items
            [
                $($items)*
                GalleryRouteBundleDescriptor {
                    page: live_id!($page),
                    bundle_id: stringify!($page),
                    marker_symbol: concat!("gallery_bundle_mark_", stringify!($page)),
                },
            ]
            $($rest)*
        );
    };

    (@items
        [
            $($items:tt)*
        ]
    ) => {
        #[cfg_attr(not(test), allow(dead_code))]
        pub const ROUTE_BUNDLE_DESCRIPTORS: &[GalleryRouteBundleDescriptor] = &[
            $($items)*
        ];
    };

    ($($entries:tt)*) => {
        build_gallery_route_bundle_descriptors!(@items [] $($entries)*);
    };
}

gallery_page_entries!(build_gallery_route_bundle_descriptors);

fn gallery_pages_module(vm: &mut ScriptVm) -> ScriptObject {
    let existing = vm
        .bx
        .heap
        .value(vm.bx.heap.modules, id!(gallery_pages).into(), NoTrap);
    if let Some(module) = existing.as_object() {
        module
    } else {
        vm.new_module(id!(gallery_pages))
    }
}

pub(crate) fn publish_gallery_page_template(
    vm: &mut ScriptVm,
    page_id: LiveId,
    template: ScriptValue,
) {
    let Some(template_obj) = template.as_object() else {
        error!(
            "Gallery route bundle registration for {:?} did not produce a widget template",
            page_id
        );
        return;
    };

    let gallery_pages = gallery_pages_module(vm);
    vm.bx
        .heap
        .set_value_def(gallery_pages, page_id.into(), template_obj.into());
}

pub(crate) fn gallery_snippet_resource(vm: &mut ScriptVm, page_name: &str) -> ScriptValue {
    let resource_path = format!("self://resources/snippets/{page_name}.txt");
    script_eval!(vm, {
        mod.res.crate_resource(#(resource_path))
    })
}

pub(crate) fn gallery_page_template_value_vm(
    vm: &mut ScriptVm,
    page_id: LiveId,
) -> Option<ScriptValue> {
    let gallery_pages = gallery_pages_module(vm);
    let value = vm.bx.heap.value(gallery_pages, page_id.into(), NoTrap);
    value.as_object().map(|_| value)
}

pub(crate) fn set_gallery_bundle_vm(vm: &mut ScriptVm) {
    let Some(cx) = vm.host.downcast_mut::<Cx>() else {
        error!("Gallery bundle registration requires a Cx host");
        return;
    };
    GALLERY_BUNDLE_CX.with(|slot| slot.set(cx as *mut Cx));
}

pub(crate) fn with_gallery_bundle_vm<R>(f: impl FnOnce(&mut ScriptVm) -> R) -> Option<R> {
    GALLERY_BUNDLE_CX.with(|slot| {
        let cx_ptr = slot.get();
        if cx_ptr.is_null() {
            None
        } else {
            Some(unsafe { (&mut *cx_ptr).with_vm(f) })
        }
    })
}

pub fn ensure_gallery_page_registered(cx: &mut Cx, page_id: LiveId) -> Result<(), String> {
    cx.with_vm(|vm| ensure_gallery_page_registered_vm(vm, page_id))
}

pub(crate) fn ensure_gallery_page_registered_vm(
    vm: &mut ScriptVm,
    page_id: LiveId,
) -> Result<(), String> {
    if gallery_page_template_value_vm(vm, page_id).is_some() {
        return Ok(());
    }

    macro_rules! register_gallery_page_by_id {
        (@match
            $page_id:expr;
        ) => {
            return Err(format!("Unknown gallery page {:?}", $page_id));
        };

        (@match
            $page_id:expr;
            {
                title: $title:literal,
                route: $route:literal,
                page: $page:ident,
                widget: $widget:ident,
                sidebar_id: $sidebar_id:ident,
                sidebar_label: $sidebar_label:literal,
                section: $section:literal,
                shortcut: $shortcut:literal,
                snippet: $snippet:ident,
                bundle: base,
                components: [$($components:ident),* $(,)?],
                icons: [$($icons:ident),* $(,)?],
                icon_policy: $icon_policy:ident,
                $(transition: $transition:ident,)?
            }
            $($rest:tt)*
        ) => {
            if $page_id == live_id!($page) {
                crate::ui::$page::register_gallery_route_bundle(vm);
            } else {
                register_gallery_page_by_id!(@match $page_id; $($rest)*);
            }
        };

        (@match
            $page_id:expr;
            {
                title: $title:literal,
                route: $route:literal,
                page: $page:ident,
                widget: $widget:ident,
                sidebar_id: $sidebar_id:ident,
                sidebar_label: $sidebar_label:literal,
                section: $section:literal,
                shortcut: $shortcut:literal,
                snippet: $snippet:ident,
                bundle: page,
                components: [$($components:ident),* $(,)?],
                icons: [$($icons:ident),* $(,)?],
                icon_policy: $icon_policy:ident,
                $(transition: $transition:ident,)?
            }
            $($rest:tt)*
        ) => {
            register_gallery_page_by_id!(@match $page_id; $($rest)*);
        };

        ($(
            {
                title: $title:literal,
                route: $route:literal,
                page: $page:ident,
                widget: $widget:ident,
                sidebar_id: $sidebar_id:ident,
                sidebar_label: $sidebar_label:literal,
                section: $section:literal,
                shortcut: $shortcut:literal,
                snippet: $snippet:ident,
                bundle: $bundle:ident,
                components: [$($components:ident),* $(,)?],
                icons: [$($icons:ident),* $(,)?],
                icon_policy: $icon_policy:ident,
                $(transition: $transition:ident,)?
            }
        )*) => {
            register_gallery_page_by_id!(@match page_id; $({
                title: $title,
                route: $route,
                page: $page,
                widget: $widget,
                sidebar_id: $sidebar_id,
                sidebar_label: $sidebar_label,
                section: $section,
                shortcut: $shortcut,
                snippet: $snippet,
                bundle: $bundle,
                components: [$($components),*],
                icons: [$($icons),*],
                icon_policy: $icon_policy,
                $(transition: $transition,)?
            })*);
        };
    }

    gallery_page_entries!(register_gallery_page_by_id);

    if gallery_page_template_value_vm(vm, page_id).is_some() {
        Ok(())
    } else {
        Err(format!("Gallery page template missing for {:?}", page_id))
    }
}

pub fn script_mod(vm: &mut ScriptVm) {
    crate::ui::themed_widgets::script_mod(vm);
    crate::ui::sidebar::script_mod(vm);
    crate::ui::command_palette::script_mod(vm);
    crate::ui::bundled_page_host::script_mod(vm);

    macro_rules! register_gallery_pages {
        ($(
            {
                title: $title:literal,
                route: $route:literal,
                page: $page:ident,
                widget: $widget:ident,
                sidebar_id: $sidebar_id:ident,
                sidebar_label: $sidebar_label:literal,
                section: $section:literal,
                shortcut: $shortcut:literal,
                snippet: $snippet:ident,
                bundle: $bundle:ident,
                components: [$($components:ident),* $(,)?],
                icons: [$($icons:ident),* $(,)?],
                icon_policy: $icon_policy:ident,
                $(transition: $transition:ident,)?
            }
        )*) => {
            $(
                register_gallery_page!($bundle, vm, $page);
            )*
        };
    }

    macro_rules! register_gallery_page {
        (base, $vm:ident, $page:ident) => {
            crate::ui::$page::register_gallery_route_bundle($vm);
        };
        (page, $vm:ident, $page:ident) => {};
    }

    gallery_page_entries!(register_gallery_pages);
    crate::ui::root::script_mod(vm);
}

const GALLERY_SHELL_ICONS: &[makepad_components::makepad_icon::IconModule] = &[
    makepad_components::makepad_icon::IconModule::ButtonMenu,
    makepad_components::makepad_icon::IconModule::ButtonMoon,
    makepad_components::makepad_icon::IconModule::ButtonSun,
    makepad_components::makepad_icon::IconModule::ButtonX,
];

const GALLERY_SHELL_WIDGETS: &[WidgetModule] = &[
    WidgetModule::Root,
    WidgetModule::Window,
    WidgetModule::View,
    WidgetModule::ViewUi,
    WidgetModule::ScrollBar,
    WidgetModule::ScrollBars,
    WidgetModule::Label,
    WidgetModule::LinkLabel,
    WidgetModule::Button,
    WidgetModule::CheckBox,
    WidgetModule::RadioButton,
    WidgetModule::Image,
    WidgetModule::Icon,
    WidgetModule::DesktopButton,
    WidgetModule::KeyboardView,
    WidgetModule::WindowMenu,
    WidgetModule::NavControl,
    WidgetModule::PopupMenu,
    WidgetModule::DropDown,
    WidgetModule::TextInput,
    WidgetModule::Slider,
    WidgetModule::Splitter,
    WidgetModule::FoldButton,
    WidgetModule::FoldHeader,
    WidgetModule::LoadingSpinner,
    WidgetModule::GlassPanel,
    WidgetModule::BareStep,
    WidgetModule::TurtleStep,
    WidgetModule::PortalList,
    WidgetModule::CachedWidget,
    WidgetModule::TabCloseButton,
    WidgetModule::Tab,
    WidgetModule::TabBar,
    WidgetModule::ScrollShadow,
    WidgetModule::StackNavigation,
    WidgetModule::ExpandablePanel,
    WidgetModule::Modal,
    WidgetModule::Tooltip,
    WidgetModule::PopupNotification,
    WidgetModule::PageFlip,
    WidgetModule::FlatList,
    WidgetModule::SlidePanel,
    WidgetModule::Svg,
    WidgetModule::Vector,
];

pub fn register_gallery_shell_widgets(vm: &mut ScriptVm) {
    makepad_widgets::register_widgets(vm, GALLERY_SHELL_WIDGETS);
}

pub fn register_gallery_shell_dependencies(vm: &mut ScriptVm) {
    makepad_components::register_component_set!(
        vm,
        [Button, Hr, Input, Kbd, Label, Panel, Scroll, Sidebar, Surface]
    );
    makepad_components::makepad_icon::register_icons(vm, GALLERY_SHELL_ICONS);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bootstrap_gallery_vm(vm: &mut ScriptVm) {
        register_gallery_shell_widgets(vm);
        makepad_components::theme::script_mod(vm);
        script_eval!(vm, {
            mod.widgets.shad_theme = mod.widgets.shad_themes.dark
        });
        set_gallery_bundle_vm(vm);
        register_gallery_shell_dependencies(vm);
        makepad_router::script_mod(vm);
        crate::ui::script_mod(vm);
    }

    #[test]
    fn bundled_pages_can_all_be_registered() {
        let mut cx = Cx::new(Box::new(|_, _| {}));

        cx.with_vm(|vm| {
            bootstrap_gallery_vm(vm);
        });

        for descriptor in ROUTE_BUNDLE_DESCRIPTORS {
            invoke_gallery_bundle_marker_for_test(descriptor.page);
            cx.with_vm(|vm| {
                assert!(gallery_page_template_value_vm(vm, descriptor.page).is_some());
            });
        }
    }

    fn invoke_gallery_bundle_marker_for_test(page_id: LiveId) {
        macro_rules! match_gallery_bundle_marker {
            (@match
                $page_id:expr;
            ) => {
                panic!("missing bundled marker for {:?}", $page_id);
            };

            (@match
                $page_id:expr;
                {
                    title: $title:literal,
                    route: $route:literal,
                    page: $page:ident,
                    widget: $widget:ident,
                    sidebar_id: $sidebar_id:ident,
                    sidebar_label: $sidebar_label:literal,
                    section: $section:literal,
                    shortcut: $shortcut:literal,
                    snippet: $snippet:ident,
                    bundle: base,
                    components: [$($components:ident),* $(,)?],
                    icons: [$($icons:ident),* $(,)?],
                    icon_policy: $icon_policy:ident,
                    $(transition: $transition:ident,)?
                }
                $($rest:tt)*
            ) => {
                match_gallery_bundle_marker!(@match $page_id; $($rest)*);
            };

            (@match
                $page_id:expr;
                {
                    title: $title:literal,
                    route: $route:literal,
                    page: $page:ident,
                    widget: $widget:ident,
                    sidebar_id: $sidebar_id:ident,
                    sidebar_label: $sidebar_label:literal,
                    section: $section:literal,
                    shortcut: $shortcut:literal,
                    snippet: $snippet:ident,
                    bundle: page,
                    components: [$($components:ident),* $(,)?],
                    icons: [$($icons:ident),* $(,)?],
                    icon_policy: $icon_policy:ident,
                    $(transition: $transition:ident,)?
                }
                $($rest:tt)*
            ) => {
                if $page_id == live_id!($page) {
                    crate::ui::$page::gallery_bundle_mark();
                } else {
                    match_gallery_bundle_marker!(@match $page_id; $($rest)*);
                }
            };

            ($(
                {
                    title: $title:literal,
                    route: $route:literal,
                    page: $page:ident,
                    widget: $widget:ident,
                    sidebar_id: $sidebar_id:ident,
                    sidebar_label: $sidebar_label:literal,
                    section: $section:literal,
                    shortcut: $shortcut:literal,
                    snippet: $snippet:ident,
                    bundle: $bundle:ident,
                    components: [$($components:ident),* $(,)?],
                    icons: [$($icons:ident),* $(,)?],
                    icon_policy: $icon_policy:ident,
                    $(transition: $transition:ident,)?
                }
            )*) => {
                match_gallery_bundle_marker!(@match page_id; $({
                    title: $title,
                    route: $route,
                    page: $page,
                    widget: $widget,
                    sidebar_id: $sidebar_id,
                    sidebar_label: $sidebar_label,
                    section: $section,
                    shortcut: $shortcut,
                    snippet: $snippet,
                    bundle: $bundle,
                    components: [$($components),*],
                    icons: [$($icons),*],
                    icon_policy: $icon_policy,
                    $(transition: $transition,)?
                })*);
            };
        }

        gallery_page_entries!(match_gallery_bundle_marker);
    }
}
