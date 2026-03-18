use crate::ui::registry::gallery_page_entries;
use makepad_components::makepad_widgets::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GallerySnippetKey {
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
    CommandPalette,
    ContextMenu,
    DatePicker,
    Dialog,
    Input,
    IconGallery,
    InputOtp,
    Kbd,
    Label,
    Menubar,
    NavigationMenu,
    Pagination,
    Popover,
    Progress,
    RadioGroup,
    Resizable,
    ScrollArea,
    Select,
    Separator,
    Sheet,
    Sidebar,
    Skeleton,
    Slider,
    Sonner,
    Spinner,
    Switch,
    Table,
    Tabs,
    Textarea,
    Toggle,
}

#[derive(Clone, Copy, Debug)]
pub struct GalleryCatalogEntry {
    pub title: &'static str,
    #[cfg_attr(not(test), allow(dead_code))]
    pub route: &'static str,
    pub page: LiveId,
    #[cfg_attr(not(test), allow(dead_code))]
    pub bundle_id: Option<&'static str>,
    pub sidebar_id: LiveId,
    #[cfg_attr(not(test), allow(dead_code))]
    pub sidebar_label: &'static str,
    pub section: &'static str,
    pub shortcut: &'static str,
    pub snippet: GallerySnippetKey,
}

macro_rules! build_gallery_catalog {
    (
        $(
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
        )*
    ) => {
        pub const GALLERY_CATALOG: &[GalleryCatalogEntry] = &[
            $(
                GalleryCatalogEntry {
                    title: $title,
                    route: $route,
                    page: live_id!($page),
                    bundle_id: catalog_bundle_id!($bundle, $page),
                    sidebar_id: live_id!($sidebar_id),
                    sidebar_label: $sidebar_label,
                    section: $section,
                    shortcut: $shortcut,
                    snippet: GallerySnippetKey::$snippet,
                },
            )*
        ];
    };
}

macro_rules! catalog_bundle_id {
    (base, $page:ident) => {
        None
    };
    (page, $page:ident) => {
        Some(stringify!($page))
    };
}

gallery_page_entries!(build_gallery_catalog);

pub fn entries() -> &'static [GalleryCatalogEntry] {
    GALLERY_CATALOG
}

pub fn entry_for_page(page: LiveId) -> Option<&'static GalleryCatalogEntry> {
    GALLERY_CATALOG.iter().find(|entry| entry.page == page)
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn entry_for_sidebar(sidebar_id: LiveId) -> Option<&'static GalleryCatalogEntry> {
    GALLERY_CATALOG
        .iter()
        .find(|entry| entry.sidebar_id == sidebar_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{root, snippets, ROUTE_BUNDLE_DESCRIPTORS};
    use std::collections::HashSet;

    #[test]
    fn catalog_entries_have_unique_ids_and_routes() {
        let mut pages = HashSet::new();
        let mut sidebar_ids = HashSet::new();
        let mut routes = HashSet::new();
        let mut shortcuts = HashSet::new();
        let mut bundle_ids = HashSet::new();

        for entry in GALLERY_CATALOG {
            assert!(pages.insert(entry.page));
            assert!(sidebar_ids.insert(entry.sidebar_id));
            assert!(routes.insert(entry.route));
            assert!(shortcuts.insert(entry.shortcut));
            if let Some(bundle_id) = entry.bundle_id {
                assert!(bundle_ids.insert(bundle_id));
            }
            assert!(!entry.title.is_empty());
            assert!(!entry.sidebar_label.is_empty());
            assert!(!entry.section.is_empty());
            assert!(!snippets::snippet_code(entry.snippet).is_empty());
        }
    }

    #[test]
    fn catalog_matches_router_bindings() {
        let bindings: std::collections::HashMap<LiveId, &str> =
            root::ROUTER_BINDINGS.iter().copied().collect();

        assert_eq!(GALLERY_CATALOG.len(), root::ROUTER_BINDINGS.len());
        for entry in GALLERY_CATALOG {
            assert_eq!(bindings.get(&entry.page), Some(&entry.route));
        }
    }

    #[test]
    fn sidebar_lookup_round_trips() {
        for entry in GALLERY_CATALOG {
            let looked_up = entry_for_sidebar(entry.sidebar_id).unwrap();
            assert_eq!(looked_up.page, entry.page);
            assert_eq!(looked_up.title, entry.title);
        }
    }

    #[test]
    fn bundled_catalog_entries_match_bundle_descriptors() {
        let bundled_pages: HashSet<LiveId> = ROUTE_BUNDLE_DESCRIPTORS
            .iter()
            .map(|item| item.page)
            .collect();

        for entry in GALLERY_CATALOG {
            assert_eq!(
                entry.bundle_id.is_some(),
                bundled_pages.contains(&entry.page)
            );
        }
    }

    #[test]
    fn route_bundle_manifest_matches_registry() {
        let manifest = include_str!("../../route-bundles.toml");

        for descriptor in ROUTE_BUNDLE_DESCRIPTORS {
            assert!(
                manifest.contains(&format!("[bundles.{}]", descriptor.bundle_id)),
                "missing bundle entry for {}",
                descriptor.bundle_id
            );
            assert!(
                manifest.contains(&format!("functions = [\"{}\"]", descriptor.marker_symbol)),
                "missing marker symbol for {}",
                descriptor.bundle_id
            );
        }

        assert_eq!(
            manifest.matches("[bundles.").count(),
            ROUTE_BUNDLE_DESCRIPTORS.len()
        );
    }
}
