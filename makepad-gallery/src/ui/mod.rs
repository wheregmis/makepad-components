use makepad_components::makepad_widgets::*;

pub mod catalog;
pub mod command_palette;
mod page_macros;
pub mod root;
pub mod sidebar;
pub mod snippets;
pub mod themed_widgets;
mod registry;

use crate::ui::registry::gallery_page_entries;

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
            $(transition: $transition:ident,)?
        }
    )*) => {
        $(pub mod $page;)*
    };
}

gallery_page_entries!(declare_gallery_page_modules);

pub fn script_mod(vm: &mut ScriptVm) {
    crate::ui::themed_widgets::script_mod(vm);
    crate::ui::sidebar::script_mod(vm);
    crate::ui::command_palette::script_mod(vm);
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
                $(transition: $transition:ident,)?
            }
        )*) => {
            $(crate::ui::$page::script_mod(vm);)*
        };
    }

    gallery_page_entries!(register_gallery_pages);
    crate::ui::root::script_mod(vm);
}
