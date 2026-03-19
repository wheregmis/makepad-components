pub use makepad_icon;
pub use makepad_widgets;

use makepad_icon::IconModule;
use makepad_widgets::*;

mod internal;
mod models;

const COMPONENT_REGISTRY_MODULE: LiveId = live_id!(makepad_components_registered);

pub mod prelude {
    pub use crate::calendar::ShadDate;
    pub use crate::makepad_widgets::*;
}

pub mod accordion;
pub mod alert;
mod animation;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod chart;
pub mod checkbox;
pub mod collapsible;
pub mod context_menu;
pub mod date_picker;
pub mod dialog;
pub mod hr;
pub mod image;
pub mod input;
pub mod input_otp;
pub mod kbd;
pub mod label;
pub mod menubar;
pub mod navigation_menu;
pub mod pagination;
pub mod panel;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod resizable;
pub mod scroll;
pub mod select;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod sonner;
pub mod spinner;
pub mod surface;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod theme;
pub mod toggle;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComponentModule {
    Accordion,
    Alert,
    AspectRatio,
    Avatar,
    Badge,
    Breadcrumb,
    Button,
    ButtonGroup,
    Calendar,
    Card,
    Carousel,
    Chart,
    Checkbox,
    Collapsible,
    ContextMenu,
    DatePicker,
    Dialog,
    Hr,
    Image,
    Input,
    InputOtp,
    Kbd,
    Label,
    Menubar,
    NavigationMenu,
    Pagination,
    Panel,
    Popover,
    Progress,
    RadioGroup,
    Resizable,
    Scroll,
    Select,
    Sheet,
    Sidebar,
    Skeleton,
    Slider,
    Sonner,
    Spinner,
    Surface,
    Switch,
    Table,
    Tabs,
    Textarea,
    Toggle,
}

pub const ALL_COMPONENT_MODULES: &[ComponentModule] = &[
    ComponentModule::Accordion,
    ComponentModule::Alert,
    ComponentModule::AspectRatio,
    ComponentModule::Avatar,
    ComponentModule::Badge,
    ComponentModule::Breadcrumb,
    ComponentModule::Button,
    ComponentModule::ButtonGroup,
    ComponentModule::Calendar,
    ComponentModule::Card,
    ComponentModule::Carousel,
    ComponentModule::Chart,
    ComponentModule::Checkbox,
    ComponentModule::Collapsible,
    ComponentModule::ContextMenu,
    ComponentModule::DatePicker,
    ComponentModule::Dialog,
    ComponentModule::Hr,
    ComponentModule::Image,
    ComponentModule::Input,
    ComponentModule::InputOtp,
    ComponentModule::Kbd,
    ComponentModule::Label,
    ComponentModule::Menubar,
    ComponentModule::NavigationMenu,
    ComponentModule::Pagination,
    ComponentModule::Panel,
    ComponentModule::Popover,
    ComponentModule::Progress,
    ComponentModule::RadioGroup,
    ComponentModule::Resizable,
    ComponentModule::Scroll,
    ComponentModule::Select,
    ComponentModule::Sheet,
    ComponentModule::Sidebar,
    ComponentModule::Skeleton,
    ComponentModule::Slider,
    ComponentModule::Sonner,
    ComponentModule::Spinner,
    ComponentModule::Surface,
    ComponentModule::Switch,
    ComponentModule::Table,
    ComponentModule::Tabs,
    ComponentModule::Textarea,
    ComponentModule::Toggle,
];

impl ComponentModule {
    fn marker_id(self) -> LiveId {
        match self {
            ComponentModule::Accordion => live_id!(accordion),
            ComponentModule::Alert => live_id!(alert),
            ComponentModule::AspectRatio => live_id!(aspect_ratio),
            ComponentModule::Avatar => live_id!(avatar),
            ComponentModule::Badge => live_id!(badge),
            ComponentModule::Breadcrumb => live_id!(breadcrumb),
            ComponentModule::Button => live_id!(button),
            ComponentModule::ButtonGroup => live_id!(button_group),
            ComponentModule::Calendar => live_id!(calendar),
            ComponentModule::Card => live_id!(card),
            ComponentModule::Carousel => live_id!(carousel),
            ComponentModule::Chart => live_id!(chart),
            ComponentModule::Checkbox => live_id!(checkbox),
            ComponentModule::Collapsible => live_id!(collapsible),
            ComponentModule::ContextMenu => live_id!(context_menu),
            ComponentModule::DatePicker => live_id!(date_picker),
            ComponentModule::Dialog => live_id!(dialog),
            ComponentModule::Hr => live_id!(hr),
            ComponentModule::Image => live_id!(image),
            ComponentModule::Input => live_id!(input),
            ComponentModule::InputOtp => live_id!(input_otp),
            ComponentModule::Kbd => live_id!(kbd),
            ComponentModule::Label => live_id!(label),
            ComponentModule::Menubar => live_id!(menubar),
            ComponentModule::NavigationMenu => live_id!(navigation_menu),
            ComponentModule::Pagination => live_id!(pagination),
            ComponentModule::Panel => live_id!(panel),
            ComponentModule::Popover => live_id!(popover),
            ComponentModule::Progress => live_id!(progress),
            ComponentModule::RadioGroup => live_id!(radio_group),
            ComponentModule::Resizable => live_id!(resizable),
            ComponentModule::Scroll => live_id!(scroll),
            ComponentModule::Select => live_id!(select),
            ComponentModule::Sheet => live_id!(sheet),
            ComponentModule::Sidebar => live_id!(sidebar),
            ComponentModule::Skeleton => live_id!(skeleton),
            ComponentModule::Slider => live_id!(slider),
            ComponentModule::Sonner => live_id!(sonner),
            ComponentModule::Spinner => live_id!(spinner),
            ComponentModule::Surface => live_id!(surface),
            ComponentModule::Switch => live_id!(switch),
            ComponentModule::Table => live_id!(table),
            ComponentModule::Tabs => live_id!(tabs),
            ComponentModule::Textarea => live_id!(textarea),
            ComponentModule::Toggle => live_id!(toggle),
        }
    }

    fn component_dependencies(self) -> &'static [ComponentModule] {
        match self {
            ComponentModule::Button => &[ComponentModule::Tabs],
            ComponentModule::DatePicker => &[
                ComponentModule::Button,
                ComponentModule::Calendar,
                ComponentModule::Popover,
            ],
            ComponentModule::Dialog => &[ComponentModule::Alert, ComponentModule::Button],
            ComponentModule::Menubar => &[
                ComponentModule::Button,
                ComponentModule::Hr,
                ComponentModule::Popover,
            ],
            ComponentModule::NavigationMenu => &[ComponentModule::Popover],
            ComponentModule::Pagination => &[ComponentModule::Button],
            ComponentModule::Popover => &[ComponentModule::Button],
            ComponentModule::Sheet => &[ComponentModule::Alert],
            ComponentModule::Sidebar => &[ComponentModule::Button],
            ComponentModule::Sonner => &[ComponentModule::Alert],
            ComponentModule::Table => &[ComponentModule::Input],
            ComponentModule::Textarea => &[ComponentModule::Input],
            _ => &[],
        }
    }

    fn icon_dependencies(self) -> &'static [IconModule] {
        match self {
            ComponentModule::Alert => &[IconModule::Info, IconModule::X],
            ComponentModule::Breadcrumb => &[IconModule::ChevronRight],
            ComponentModule::Carousel => &[
                IconModule::ButtonChevronLeft,
                IconModule::ButtonChevronRight,
            ],
            ComponentModule::Input => &[IconModule::Search],
            ComponentModule::Sonner => &[IconModule::Check, IconModule::ButtonX],
            _ => &[],
        }
    }

    fn widget_dependencies(self) -> &'static [WidgetModule] {
        const COMMON: &[WidgetModule] = &[WidgetModule::ViewUi, WidgetModule::Label];

        match self {
            ComponentModule::AspectRatio => &[WidgetModule::ViewUi],
            ComponentModule::Avatar => {
                &[WidgetModule::View, WidgetModule::ViewUi, WidgetModule::Label]
            }
            ComponentModule::Breadcrumb => &[
                WidgetModule::LinkLabel,
                WidgetModule::ViewUi,
                WidgetModule::Label,
            ],
            ComponentModule::Button => &[WidgetModule::Button],
            ComponentModule::ButtonGroup => {
                &[WidgetModule::Button, WidgetModule::ViewUi, WidgetModule::Label]
            }
            ComponentModule::Chart => &[WidgetModule::Chart],
            ComponentModule::ContextMenu => &[WidgetModule::PopupMenu],
            ComponentModule::Image => &[WidgetModule::Image],
            ComponentModule::Input => &[
                WidgetModule::TextInput,
                WidgetModule::ViewUi,
                WidgetModule::Label,
            ],
            ComponentModule::Label => &[WidgetModule::Label],
            ComponentModule::Scroll => &[WidgetModule::ViewUi],
            ComponentModule::Select => &[WidgetModule::DropDown, WidgetModule::PopupMenu],
            ComponentModule::Slider => &[WidgetModule::Slider],
            ComponentModule::Table => &[
                WidgetModule::PortalList,
                WidgetModule::ViewUi,
                WidgetModule::Label,
            ],
            ComponentModule::Toggle => &[
                WidgetModule::CheckBox,
                WidgetModule::ViewUi,
                WidgetModule::Label,
            ],
            _ => COMMON,
        }
    }

    fn register_script_mod(self, vm: &mut ScriptVm) {
        match self {
            ComponentModule::Accordion => crate::accordion::script_mod(vm),
            ComponentModule::Alert => crate::alert::script_mod(vm),
            ComponentModule::AspectRatio => crate::aspect_ratio::script_mod(vm),
            ComponentModule::Avatar => crate::avatar::script_mod(vm),
            ComponentModule::Badge => crate::badge::script_mod(vm),
            ComponentModule::Breadcrumb => crate::breadcrumb::script_mod(vm),
            ComponentModule::Button => crate::button::script_mod(vm),
            ComponentModule::ButtonGroup => crate::button_group::script_mod(vm),
            ComponentModule::Calendar => crate::calendar::script_mod(vm),
            ComponentModule::Card => crate::card::script_mod(vm),
            ComponentModule::Carousel => crate::carousel::script_mod(vm),
            ComponentModule::Chart => crate::chart::script_mod(vm),
            ComponentModule::Checkbox => crate::checkbox::script_mod(vm),
            ComponentModule::Collapsible => crate::collapsible::script_mod(vm),
            ComponentModule::ContextMenu => crate::context_menu::script_mod(vm),
            ComponentModule::DatePicker => crate::date_picker::script_mod(vm),
            ComponentModule::Dialog => crate::dialog::script_mod(vm),
            ComponentModule::Hr => crate::hr::script_mod(vm),
            ComponentModule::Image => crate::image::script_mod(vm),
            ComponentModule::Input => crate::input::script_mod(vm),
            ComponentModule::InputOtp => crate::input_otp::script_mod(vm),
            ComponentModule::Kbd => crate::kbd::script_mod(vm),
            ComponentModule::Label => crate::label::script_mod(vm),
            ComponentModule::Menubar => crate::menubar::script_mod(vm),
            ComponentModule::NavigationMenu => crate::navigation_menu::script_mod(vm),
            ComponentModule::Pagination => crate::pagination::script_mod(vm),
            ComponentModule::Panel => crate::panel::script_mod(vm),
            ComponentModule::Popover => crate::popover::script_mod(vm),
            ComponentModule::Progress => crate::progress::script_mod(vm),
            ComponentModule::RadioGroup => crate::radio_group::script_mod(vm),
            ComponentModule::Resizable => crate::resizable::script_mod(vm),
            ComponentModule::Scroll => crate::scroll::script_mod(vm),
            ComponentModule::Select => crate::select::script_mod(vm),
            ComponentModule::Sheet => crate::sheet::script_mod(vm),
            ComponentModule::Sidebar => crate::sidebar::script_mod(vm),
            ComponentModule::Skeleton => crate::skeleton::script_mod(vm),
            ComponentModule::Slider => crate::slider::script_mod(vm),
            ComponentModule::Sonner => crate::sonner::script_mod(vm),
            ComponentModule::Spinner => crate::spinner::script_mod(vm),
            ComponentModule::Surface => crate::surface::script_mod(vm),
            ComponentModule::Switch => crate::switch::script_mod(vm),
            ComponentModule::Table => crate::table::script_mod(vm),
            ComponentModule::Tabs => crate::tabs::script_mod(vm),
            ComponentModule::Textarea => crate::textarea::script_mod(vm),
            ComponentModule::Toggle => crate::toggle::script_mod(vm),
        };
    }
}

fn component_registry_module(vm: &mut ScriptVm) -> ScriptObject {
    let existing = vm
        .bx
        .heap
        .value(vm.bx.heap.modules, COMPONENT_REGISTRY_MODULE.into(), NoTrap);
    if let Some(module) = existing.as_object() {
        module
    } else {
        vm.new_module(COMPONENT_REGISTRY_MODULE)
    }
}

fn component_registered(vm: &mut ScriptVm, component: ComponentModule) -> bool {
    let registry = component_registry_module(vm);
    vm.bx
        .heap
        .value(registry, component.marker_id().into(), NoTrap)
        .as_object()
        .is_some()
}

fn mark_component_registered(vm: &mut ScriptVm, component: ComponentModule) {
    let registry = component_registry_module(vm);
    vm.bx
        .heap
        .set_value_def(registry, component.marker_id().into(), registry.into());
}

fn register_component_recursive(vm: &mut ScriptVm, component: ComponentModule) {
    if component_registered(vm, component) {
        return;
    }

    for dependency in component.component_dependencies() {
        register_component_recursive(vm, *dependency);
    }

    let icon_dependencies = component.icon_dependencies();
    if !icon_dependencies.is_empty() {
        makepad_icon::register_icons(vm, icon_dependencies);
    }

    let widget_dependencies = component.widget_dependencies();
    if !widget_dependencies.is_empty() {
        makepad_widgets::register_widgets(vm, widget_dependencies);
    }

    component.register_script_mod(vm);
    mark_component_registered(vm, component);
}

pub fn register_components(vm: &mut ScriptVm, modules: &[ComponentModule]) {
    for module in modules {
        register_component_recursive(vm, *module);
    }
}

macro_rules! define_direct_component_registrars {
    ($(
        $fn_name:ident => {
            variant: $variant:ident,
            deps: [$($deps:ident),* $(,)?],
            icons: [$($icons:ident),* $(,)?],
            script: $script:path,
        }
    )*) => {
        #[doc(hidden)]
        pub mod direct_component_registrars {
            use super::*;

            $(
                pub fn $fn_name(vm: &mut ScriptVm) {
                    if component_registered(vm, ComponentModule::$variant) {
                        return;
                    }

                    $(self::$deps(vm);)*

                    let icon_dependencies = &[$(IconModule::$icons),*];
                    if !icon_dependencies.is_empty() {
                        makepad_icon::register_icons(vm, icon_dependencies);
                    }

                    let widget_dependencies = ComponentModule::$variant.widget_dependencies();
                    if !widget_dependencies.is_empty() {
                        makepad_widgets::register_widgets(vm, widget_dependencies);
                    }

                    $script(vm);
                    mark_component_registered(vm, ComponentModule::$variant);
                }
            )*
        }

    };
}

define_direct_component_registrars! {
    accordion => {
        variant: Accordion,
        deps: [],
        icons: [],
        script: crate::accordion::script_mod,
    }
    alert => {
        variant: Alert,
        deps: [],
        icons: [Info, X],
        script: crate::alert::script_mod,
    }
    aspect_ratio => {
        variant: AspectRatio,
        deps: [],
        icons: [],
        script: crate::aspect_ratio::script_mod,
    }
    avatar => {
        variant: Avatar,
        deps: [],
        icons: [],
        script: crate::avatar::script_mod,
    }
    badge => {
        variant: Badge,
        deps: [],
        icons: [],
        script: crate::badge::script_mod,
    }
    breadcrumb => {
        variant: Breadcrumb,
        deps: [],
        icons: [ChevronRight],
        script: crate::breadcrumb::script_mod,
    }
    button => {
        variant: Button,
        deps: [tabs],
        icons: [],
        script: crate::button::script_mod,
    }
    button_group => {
        variant: ButtonGroup,
        deps: [],
        icons: [],
        script: crate::button_group::script_mod,
    }
    calendar => {
        variant: Calendar,
        deps: [],
        icons: [],
        script: crate::calendar::script_mod,
    }
    card => {
        variant: Card,
        deps: [],
        icons: [],
        script: crate::card::script_mod,
    }
    carousel => {
        variant: Carousel,
        deps: [],
        icons: [ButtonChevronLeft, ButtonChevronRight],
        script: crate::carousel::script_mod,
    }
    chart => {
        variant: Chart,
        deps: [],
        icons: [],
        script: crate::chart::script_mod,
    }
    checkbox => {
        variant: Checkbox,
        deps: [],
        icons: [],
        script: crate::checkbox::script_mod,
    }
    collapsible => {
        variant: Collapsible,
        deps: [],
        icons: [],
        script: crate::collapsible::script_mod,
    }
    context_menu => {
        variant: ContextMenu,
        deps: [],
        icons: [],
        script: crate::context_menu::script_mod,
    }
    date_picker => {
        variant: DatePicker,
        deps: [button, calendar, popover],
        icons: [],
        script: crate::date_picker::script_mod,
    }
    dialog => {
        variant: Dialog,
        deps: [alert, button],
        icons: [],
        script: crate::dialog::script_mod,
    }
    hr => {
        variant: Hr,
        deps: [],
        icons: [],
        script: crate::hr::script_mod,
    }
    image => {
        variant: Image,
        deps: [],
        icons: [],
        script: crate::image::script_mod,
    }
    input => {
        variant: Input,
        deps: [],
        icons: [Search],
        script: crate::input::script_mod,
    }
    input_otp => {
        variant: InputOtp,
        deps: [],
        icons: [],
        script: crate::input_otp::script_mod,
    }
    kbd => {
        variant: Kbd,
        deps: [],
        icons: [],
        script: crate::kbd::script_mod,
    }
    label => {
        variant: Label,
        deps: [],
        icons: [],
        script: crate::label::script_mod,
    }
    menubar => {
        variant: Menubar,
        deps: [button, hr, popover],
        icons: [],
        script: crate::menubar::script_mod,
    }
    navigation_menu => {
        variant: NavigationMenu,
        deps: [popover],
        icons: [],
        script: crate::navigation_menu::script_mod,
    }
    pagination => {
        variant: Pagination,
        deps: [button],
        icons: [],
        script: crate::pagination::script_mod,
    }
    panel => {
        variant: Panel,
        deps: [],
        icons: [],
        script: crate::panel::script_mod,
    }
    popover => {
        variant: Popover,
        deps: [button],
        icons: [],
        script: crate::popover::script_mod,
    }
    progress => {
        variant: Progress,
        deps: [],
        icons: [],
        script: crate::progress::script_mod,
    }
    radio_group => {
        variant: RadioGroup,
        deps: [],
        icons: [],
        script: crate::radio_group::script_mod,
    }
    resizable => {
        variant: Resizable,
        deps: [],
        icons: [],
        script: crate::resizable::script_mod,
    }
    scroll => {
        variant: Scroll,
        deps: [],
        icons: [],
        script: crate::scroll::script_mod,
    }
    select => {
        variant: Select,
        deps: [],
        icons: [],
        script: crate::select::script_mod,
    }
    sheet => {
        variant: Sheet,
        deps: [alert],
        icons: [],
        script: crate::sheet::script_mod,
    }
    sidebar => {
        variant: Sidebar,
        deps: [button],
        icons: [],
        script: crate::sidebar::script_mod,
    }
    skeleton => {
        variant: Skeleton,
        deps: [],
        icons: [],
        script: crate::skeleton::script_mod,
    }
    slider => {
        variant: Slider,
        deps: [],
        icons: [],
        script: crate::slider::script_mod,
    }
    sonner => {
        variant: Sonner,
        deps: [alert],
        icons: [Check, ButtonX],
        script: crate::sonner::script_mod,
    }
    spinner => {
        variant: Spinner,
        deps: [],
        icons: [],
        script: crate::spinner::script_mod,
    }
    surface => {
        variant: Surface,
        deps: [],
        icons: [],
        script: crate::surface::script_mod,
    }
    switch => {
        variant: Switch,
        deps: [],
        icons: [],
        script: crate::switch::script_mod,
    }
    table => {
        variant: Table,
        deps: [input],
        icons: [],
        script: crate::table::script_mod,
    }
    tabs => {
        variant: Tabs,
        deps: [],
        icons: [],
        script: crate::tabs::script_mod,
    }
    textarea => {
        variant: Textarea,
        deps: [input],
        icons: [],
        script: crate::textarea::script_mod,
    }
    toggle => {
        variant: Toggle,
        deps: [],
        icons: [],
        script: crate::toggle::script_mod,
    }
}

#[macro_export]
macro_rules! register_component_set {
    ($vm:expr, [$($component:ident),* $(,)?]) => {{
        $( $crate::register_component_set!(@one $vm, $component); )*
    }};
    (@one $vm:expr, Accordion) => { $crate::direct_component_registrars::accordion($vm); };
    (@one $vm:expr, Alert) => { $crate::direct_component_registrars::alert($vm); };
    (@one $vm:expr, AspectRatio) => { $crate::direct_component_registrars::aspect_ratio($vm); };
    (@one $vm:expr, Avatar) => { $crate::direct_component_registrars::avatar($vm); };
    (@one $vm:expr, Badge) => { $crate::direct_component_registrars::badge($vm); };
    (@one $vm:expr, Breadcrumb) => { $crate::direct_component_registrars::breadcrumb($vm); };
    (@one $vm:expr, Button) => { $crate::direct_component_registrars::button($vm); };
    (@one $vm:expr, ButtonGroup) => { $crate::direct_component_registrars::button_group($vm); };
    (@one $vm:expr, Calendar) => { $crate::direct_component_registrars::calendar($vm); };
    (@one $vm:expr, Card) => { $crate::direct_component_registrars::card($vm); };
    (@one $vm:expr, Carousel) => { $crate::direct_component_registrars::carousel($vm); };
    (@one $vm:expr, Chart) => { $crate::direct_component_registrars::chart($vm); };
    (@one $vm:expr, Checkbox) => { $crate::direct_component_registrars::checkbox($vm); };
    (@one $vm:expr, Collapsible) => { $crate::direct_component_registrars::collapsible($vm); };
    (@one $vm:expr, ContextMenu) => { $crate::direct_component_registrars::context_menu($vm); };
    (@one $vm:expr, DatePicker) => { $crate::direct_component_registrars::date_picker($vm); };
    (@one $vm:expr, Dialog) => { $crate::direct_component_registrars::dialog($vm); };
    (@one $vm:expr, Hr) => { $crate::direct_component_registrars::hr($vm); };
    (@one $vm:expr, Image) => { $crate::direct_component_registrars::image($vm); };
    (@one $vm:expr, Input) => { $crate::direct_component_registrars::input($vm); };
    (@one $vm:expr, InputOtp) => { $crate::direct_component_registrars::input_otp($vm); };
    (@one $vm:expr, Kbd) => { $crate::direct_component_registrars::kbd($vm); };
    (@one $vm:expr, Label) => { $crate::direct_component_registrars::label($vm); };
    (@one $vm:expr, Menubar) => { $crate::direct_component_registrars::menubar($vm); };
    (@one $vm:expr, NavigationMenu) => {
        $crate::direct_component_registrars::navigation_menu($vm);
    };
    (@one $vm:expr, Pagination) => { $crate::direct_component_registrars::pagination($vm); };
    (@one $vm:expr, Panel) => { $crate::direct_component_registrars::panel($vm); };
    (@one $vm:expr, Popover) => { $crate::direct_component_registrars::popover($vm); };
    (@one $vm:expr, Progress) => { $crate::direct_component_registrars::progress($vm); };
    (@one $vm:expr, RadioGroup) => { $crate::direct_component_registrars::radio_group($vm); };
    (@one $vm:expr, Resizable) => { $crate::direct_component_registrars::resizable($vm); };
    (@one $vm:expr, Scroll) => { $crate::direct_component_registrars::scroll($vm); };
    (@one $vm:expr, Select) => { $crate::direct_component_registrars::select($vm); };
    (@one $vm:expr, Sheet) => { $crate::direct_component_registrars::sheet($vm); };
    (@one $vm:expr, Sidebar) => { $crate::direct_component_registrars::sidebar($vm); };
    (@one $vm:expr, Skeleton) => { $crate::direct_component_registrars::skeleton($vm); };
    (@one $vm:expr, Slider) => { $crate::direct_component_registrars::slider($vm); };
    (@one $vm:expr, Sonner) => { $crate::direct_component_registrars::sonner($vm); };
    (@one $vm:expr, Spinner) => { $crate::direct_component_registrars::spinner($vm); };
    (@one $vm:expr, Surface) => { $crate::direct_component_registrars::surface($vm); };
    (@one $vm:expr, Switch) => { $crate::direct_component_registrars::switch($vm); };
    (@one $vm:expr, Table) => { $crate::direct_component_registrars::table($vm); };
    (@one $vm:expr, Tabs) => { $crate::direct_component_registrars::tabs($vm); };
    (@one $vm:expr, Textarea) => { $crate::direct_component_registrars::textarea($vm); };
    (@one $vm:expr, Toggle) => { $crate::direct_component_registrars::toggle($vm); };
}

pub fn script_mod_without_theme(vm: &mut ScriptVm) {
    makepad_widgets::register_all_widgets(vm);
    makepad_icon::register_all_icons(vm);
    register_components(vm, ALL_COMPONENT_MODULES);
}

pub fn script_mod(vm: &mut ScriptVm) {
    makepad_widgets::register_all_widgets(vm);
    crate::theme::script_mod(vm);
    crate::script_mod_without_theme(vm);
}

#[cfg(test)]
fn widget_registered_for_test(vm: &mut ScriptVm, widget_id: LiveId) -> bool {
    let widgets = vm
        .bx
        .heap
        .value(vm.bx.heap.modules, id!(widgets).into(), NoTrap)
        .as_object();
    let Some(widgets) = widgets else {
        return false;
    };
    vm.bx
        .heap
        .value(widgets, widget_id.into(), NoTrap)
        .as_object()
        .is_some()
}

#[cfg(test)]
fn component_registered_for_test(vm: &mut ScriptVm, component: ComponentModule) -> bool {
    component_registered(vm, component)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bootstrap_vm() -> Cx {
        let mut cx = Cx::new(Box::new(|_, _| {}));
        cx.with_vm(|vm| {
            makepad_widgets::script_mod(vm);
            crate::theme::script_mod(vm);
            script_eval!(vm, {
                mod.widgets.shad_theme = mod.widgets.shad_themes.dark
            });
        });
        cx
    }

    #[test]
    fn dialog_registration_includes_transitive_dependencies() {
        let mut cx = bootstrap_vm();
        cx.with_vm(|vm| {
            register_components(vm, &[ComponentModule::Dialog]);
            assert!(component_registered_for_test(vm, ComponentModule::Dialog));
            assert!(component_registered_for_test(vm, ComponentModule::Alert));
            assert!(component_registered_for_test(vm, ComponentModule::Button));
            assert!(component_registered_for_test(vm, ComponentModule::Tabs));
            assert!(widget_registered_for_test(vm, id!(ShadDialog)));
            assert!(widget_registered_for_test(vm, id!(ShadAlertTitle)));
            assert!(widget_registered_for_test(vm, id!(ShadButtonOutline)));
            assert!(widget_registered_for_test(vm, id!(ShadTabsTrigger)));
        });
    }

    #[test]
    fn textarea_registration_includes_input_and_icons() {
        let mut cx = bootstrap_vm();
        cx.with_vm(|vm| {
            register_components(vm, &[ComponentModule::Textarea]);
            assert!(component_registered_for_test(vm, ComponentModule::Textarea));
            assert!(component_registered_for_test(vm, ComponentModule::Input));
            assert!(widget_registered_for_test(vm, id!(ShadTextarea)));
            assert!(widget_registered_for_test(vm, id!(ShadInput)));
            assert!(widget_registered_for_test(vm, id!(IconSearch)));
        });
    }

    #[test]
    fn menubar_registration_is_idempotent_and_transitive() {
        let mut cx = bootstrap_vm();
        cx.with_vm(|vm| {
            register_components(vm, &[ComponentModule::Menubar]);
            register_components(vm, &[ComponentModule::Menubar]);
            assert!(component_registered_for_test(vm, ComponentModule::Menubar));
            assert!(component_registered_for_test(vm, ComponentModule::Button));
            assert!(component_registered_for_test(vm, ComponentModule::Popover));
            assert!(component_registered_for_test(vm, ComponentModule::Hr));
            assert!(widget_registered_for_test(vm, id!(ShadMenubarMenu)));
            assert!(widget_registered_for_test(vm, id!(ShadPopover)));
            assert!(widget_registered_for_test(vm, id!(ShadSeparator)));
        });
    }
}
