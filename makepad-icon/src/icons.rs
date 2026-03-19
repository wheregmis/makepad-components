use makepad_widgets::*;

const ALL_ICONS_REGISTRY_MODULE: LiveId = live_id!(makepad_icon_all_registered);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconModule {
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    Info,
    Search,
    X,
    ButtonChevronLeft,
    ButtonChevronRight,
    ButtonMenu,
    ButtonMoon,
    ButtonMoreHorizontal,
    ButtonSun,
    ButtonX,
}

// Expands to a script_mod with one icon widget per Lucide SVG binding.
// Input format: IconTypeName => "self://resources/icons/name.svg"
macro_rules! define_lucide_icons {
    ($($icon_name:ident => $icon_path:literal,)*) => {
        script_mod! {
            use mod.prelude.widgets.*
            use mod.widgets.*

            mod.widgets.MakepadIconBase = mod.widgets.Icon{
                icon_walk: Walk{width: 16, height: 16}
                draw_icon.color: #d4d4d8
            }

            $(mod.widgets.$icon_name = mod.widgets.MakepadIconBase{
                draw_icon.svg: crate_resource($icon_path)
            })*

            // Icon-button types: ButtonFlatIcon with SVG pre-bound.
            // Callers apply draw_bg / draw_icon.color styling on top.
            mod.widgets.IconButtonX = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/x.svg")
                }
            }

            mod.widgets.IconButtonChevronLeft = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/chevron-left.svg")
                }
            }

            mod.widgets.IconButtonChevronRight = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/chevron-right.svg")
                }
            }

            mod.widgets.IconButtonMenu = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/menu.svg")
                }
            }

            mod.widgets.IconButtonMoon = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/moon.svg")
                }
            }

            mod.widgets.IconButtonSun = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/sun.svg")
                }
            }

            mod.widgets.IconButtonMoreHorizontal = mod.widgets.ButtonFlatIcon{
                icon_walk: Walk{width: 14, height: 14}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/ellipsis.svg")
                }
            }
        }
    };
}

mod generated {
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/lucide_icon_bindings.rs"));
}

fn widgets_module(vm: &mut ScriptVm) -> Option<ScriptObject> {
    vm.bx.heap
        .value(vm.bx.heap.modules, id!(widgets).into(), NoTrap)
        .as_object()
}

fn widget_registered(vm: &mut ScriptVm, widget_id: LiveId) -> bool {
    let Some(widgets) = widgets_module(vm) else {
        return false;
    };
    vm.bx
        .heap
        .value(widgets, widget_id.into(), NoTrap)
        .as_object()
        .is_some()
}

fn all_icons_registered(vm: &mut ScriptVm) -> bool {
    vm.bx
        .heap
        .value(vm.bx.heap.modules, ALL_ICONS_REGISTRY_MODULE.into(), NoTrap)
        .as_object()
        .is_some()
}

fn mark_all_icons_registered(vm: &mut ScriptVm) {
    if !all_icons_registered(vm) {
        vm.new_module(ALL_ICONS_REGISTRY_MODULE);
    }
}

mod makepad_icon_base {
    use super::*;

    script_mod! {
        use mod.prelude.widgets.*
        use mod.widgets.*

        mod.widgets.MakepadIconBase = mod.widgets.Icon{
            icon_walk: Walk{width: 16, height: 16}
            draw_icon.color: #d4d4d8
        }
    }

    pub(super) fn register(vm: &mut ScriptVm) {
        script_mod(vm);
    }
}

fn ensure_makepad_icon_base(vm: &mut ScriptVm) {
    if widget_registered(vm, id!(MakepadIconBase)) {
        return;
    }
    makepad_icon_base::register(vm);
}

macro_rules! define_selective_lucide_registrars {
    ($(
        $module_name:ident => {
            register: $register_fn:ident,
            widget: $widget:ident,
            path: $path:literal,
        }
    )*) => {
        $(
            mod $module_name {
                use super::*;

                script_mod! {
                    use mod.prelude.widgets.*
                    use mod.widgets.*

                    mod.widgets.$widget = mod.widgets.MakepadIconBase{
                        draw_icon.svg: crate_resource($path)
                    }
                }

                pub(super) fn register(vm: &mut ScriptVm) {
                    script_mod(vm);
                }
            }

            fn $register_fn(vm: &mut ScriptVm) {
                if all_icons_registered(vm) || widget_registered(vm, id!($widget)) {
                    return;
                }
                ensure_makepad_icon_base(vm);
                $module_name::register(vm);
            }
        )*
    };
}

macro_rules! define_selective_button_registrars {
    ($(
        $module_name:ident => {
            register: $register_fn:ident,
            widget: $widget:ident,
            path: $path:literal,
        }
    )*) => {
        $(
            mod $module_name {
                use super::*;

                script_mod! {
                    use mod.prelude.widgets.*
                    use mod.widgets.*

                    mod.widgets.$widget = mod.widgets.ButtonFlatIcon{
                        icon_walk: Walk{width: 14, height: 14}
                        draw_icon +: {
                            svg: crate_resource($path)
                        }
                    }
                }

                pub(super) fn register(vm: &mut ScriptVm) {
                    script_mod(vm);
                }
            }

            fn $register_fn(vm: &mut ScriptVm) {
                if all_icons_registered(vm) || widget_registered(vm, id!($widget)) {
                    return;
                }
                $module_name::register(vm);
            }
        )*
    };
}

define_selective_lucide_registrars! {
    icon_check_module => {
        register: register_icon_check,
        widget: IconCheck,
        path: "self://resources/icons/check.svg",
    }
    icon_chevron_down_module => {
        register: register_icon_chevron_down,
        widget: IconChevronDown,
        path: "self://resources/icons/chevron-down.svg",
    }
    icon_chevron_left_module => {
        register: register_icon_chevron_left,
        widget: IconChevronLeft,
        path: "self://resources/icons/chevron-left.svg",
    }
    icon_chevron_right_module => {
        register: register_icon_chevron_right,
        widget: IconChevronRight,
        path: "self://resources/icons/chevron-right.svg",
    }
    icon_info_module => {
        register: register_icon_info,
        widget: IconInfo,
        path: "self://resources/icons/info.svg",
    }
    icon_search_module => {
        register: register_icon_search,
        widget: IconSearch,
        path: "self://resources/icons/search.svg",
    }
    icon_x_module => {
        register: register_icon_x,
        widget: IconX,
        path: "self://resources/icons/x.svg",
    }
}

define_selective_button_registrars! {
    icon_button_chevron_left_module => {
        register: register_button_chevron_left,
        widget: IconButtonChevronLeft,
        path: "self://resources/icons/chevron-left.svg",
    }
    icon_button_chevron_right_module => {
        register: register_button_chevron_right,
        widget: IconButtonChevronRight,
        path: "self://resources/icons/chevron-right.svg",
    }
    icon_button_menu_module => {
        register: register_button_menu,
        widget: IconButtonMenu,
        path: "self://resources/icons/menu.svg",
    }
    icon_button_moon_module => {
        register: register_button_moon,
        widget: IconButtonMoon,
        path: "self://resources/icons/moon.svg",
    }
    icon_button_more_horizontal_module => {
        register: register_button_more_horizontal,
        widget: IconButtonMoreHorizontal,
        path: "self://resources/icons/ellipsis.svg",
    }
    icon_button_sun_module => {
        register: register_button_sun,
        widget: IconButtonSun,
        path: "self://resources/icons/sun.svg",
    }
    icon_button_x_module => {
        register: register_button_x,
        widget: IconButtonX,
        path: "self://resources/icons/x.svg",
    }
}

pub fn register_icons(vm: &mut ScriptVm, modules: &[IconModule]) {
    if all_icons_registered(vm) {
        return;
    }

    for module in modules {
        match module {
            IconModule::Check => register_icon_check(vm),
            IconModule::ChevronDown => register_icon_chevron_down(vm),
            IconModule::ChevronLeft => register_icon_chevron_left(vm),
            IconModule::ChevronRight => register_icon_chevron_right(vm),
            IconModule::Info => register_icon_info(vm),
            IconModule::Search => register_icon_search(vm),
            IconModule::X => register_icon_x(vm),
            IconModule::ButtonChevronLeft => register_button_chevron_left(vm),
            IconModule::ButtonChevronRight => register_button_chevron_right(vm),
            IconModule::ButtonMenu => register_button_menu(vm),
            IconModule::ButtonMoon => register_button_moon(vm),
            IconModule::ButtonMoreHorizontal => register_button_more_horizontal(vm),
            IconModule::ButtonSun => register_button_sun(vm),
            IconModule::ButtonX => register_button_x(vm),
        }
    }
}

pub fn register_all_icons(vm: &mut ScriptVm) {
    if all_icons_registered(vm) {
        return;
    }
    generated::script_mod(vm);
    mark_all_icons_registered(vm);
}

#[cfg(test)]
pub(crate) fn all_icons_registered_for_test(vm: &mut ScriptVm) -> bool {
    all_icons_registered(vm)
}

#[cfg(test)]
pub(crate) fn icon_registered_for_test(vm: &mut ScriptVm, widget_id: LiveId) -> bool {
    widget_registered(vm, widget_id)
}

#[cfg(test)]
pub(crate) fn selective_shell_icons() -> &'static [IconModule] {
    &[
        IconModule::ButtonMenu,
        IconModule::ButtonMoon,
        IconModule::ButtonSun,
        IconModule::ButtonX,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn bootstrap_vm() -> Cx {
        let mut cx = Cx::new(Box::new(|_, _| {}));
        cx.with_vm(|vm| {
            makepad_widgets::script_mod(vm);
        });
        cx
    }

    #[test]
    fn selective_icon_registration_is_targeted() {
        let mut cx = bootstrap_vm();
        cx.with_vm(|vm| {
            register_icons(vm, selective_shell_icons());
            assert!(icon_registered_for_test(vm, id!(IconButtonMenu)));
            assert!(icon_registered_for_test(vm, id!(IconButtonMoon)));
            assert!(!icon_registered_for_test(vm, id!(IconSearch)));
            assert!(!all_icons_registered_for_test(vm));
        });
    }

    #[test]
    fn full_icon_registration_marks_vm_once() {
        let mut cx = bootstrap_vm();
        cx.with_vm(|vm| {
            register_all_icons(vm);
            assert!(all_icons_registered_for_test(vm));
            assert!(icon_registered_for_test(vm, id!(IconSearch)));
            assert!(icon_registered_for_test(vm, id!(IconButtonMoreHorizontal)));
        });
    }

    #[test]
    fn icon_button_more_horizontal_uses_existing_asset() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(manifest_dir.join("resources/icons/ellipsis.svg").exists());
        assert!(!manifest_dir.join("resources/icons/more-horizontal.svg").exists());
    }
}
