use makepad_components::makepad_widgets::*;

mod bundled_page_host;
pub mod catalog;
pub mod command_palette;
mod page_macros;
mod registry;
pub mod root;
pub mod sidebar;
pub mod snippets;
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

pub(crate) fn gallery_page_template_value_vm(
    vm: &mut ScriptVm,
    page_id: LiveId,
) -> Option<ScriptValue> {
    let gallery_pages = gallery_pages_module(vm);
    let value = vm.bx.heap.value(gallery_pages, page_id.into(), NoTrap);
    value.as_object().map(|_| value)
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
                $(transition: $transition:ident,)?
            }
        )*) => {
            match page_id {
                $(
                    live_id!($page) => crate::ui::$page::gallery_bundle_mark(vm as *mut ScriptVm),
                )*
                _ => return Err(format!("Unknown gallery page {:?}", page_id)),
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn bootstrap_gallery_vm(vm: &mut ScriptVm) {
        crate::makepad_widgets::script_mod(vm);
        makepad_components::theme::script_mod(vm);
        script_eval!(vm, {
            mod.widgets.shad_theme = mod.widgets.shad_themes.dark
        });
        makepad_components::script_mod_without_theme(vm);
        crate::makepad_code_editor::script_mod(vm);
        makepad_router::script_mod(vm);
        crate::ui::script_mod(vm);
    }

    #[test]
    fn bundled_pages_can_all_be_registered() {
        let mut cx = Cx::new(Box::new(|_, _| {}));

        cx.with_vm(|vm| {
            bootstrap_gallery_vm(vm);

            for descriptor in ROUTE_BUNDLE_DESCRIPTORS {
                ensure_gallery_page_registered_vm(vm, descriptor.page).unwrap();
                assert!(gallery_page_template_value_vm(vm, descriptor.page).is_some());
            }
        });
    }
}
