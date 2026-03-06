use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSelectItem = mod.widgets.ShadDropdownMenuItem{}

    mod.widgets.ShadSelectPopupMenu = mod.widgets.ShadDropdownPopupMenu{
        menu_item: mod.widgets.ShadSelectItem{}
    }

    mod.widgets.ShadSelect = mod.widgets.ShadDropdownMenu{
        width: 180
        popup_menu: mod.widgets.ShadSelectPopupMenu{}
    }
}
