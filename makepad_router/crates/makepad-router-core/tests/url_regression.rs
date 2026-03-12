use makepad_router_core::url::{
    build_query_string, normalize_path_cow, parse_query_map, RouterUrl,
};
use std::collections::HashMap;

#[test]
fn router_url_parses_path_query_hash() {
    let u = RouterUrl::parse("/admin/dashboard?tab=settings#section");
    assert_eq!(u.path, "/admin/dashboard");
    assert_eq!(u.query, "?tab=settings");
    assert_eq!(u.hash, "#section");
    assert_eq!(u.to_string(), "/admin/dashboard?tab=settings#section");
}

#[test]
fn router_url_parses_full_url() {
    let u = RouterUrl::parse("https://example.com/admin/dashboard?tab=settings#section");
    assert_eq!(u.to_string(), "/admin/dashboard?tab=settings#section");
}

#[test]
fn router_url_normalizes_empty_and_missing_slash() {
    assert_eq!(RouterUrl::parse("").to_string(), "/");
    assert_eq!(RouterUrl::parse("home").to_string(), "/home");
    assert_eq!(RouterUrl::parse("http://example.com").to_string(), "/");
}

#[test]
fn query_map_decodes_and_builds() {
    let map = parse_query_map("?q=hello+world&x=%2F&empty=&flag");
    let mut expected = HashMap::new();
    expected.insert("q".to_string(), "hello world".to_string());
    expected.insert("x".to_string(), "/".to_string());
    expected.insert("empty".to_string(), "".to_string());
    expected.insert("flag".to_string(), "".to_string());
    assert_eq!(map, expected);

    let rebuilt = build_query_string(&map);
    assert_eq!(rebuilt, "?empty&flag&q=hello+world&x=%2F");
}

#[test]
fn normalize_path_cow_borrows_when_possible() {
    let borrowed = normalize_path_cow("/already/normalized");
    assert!(matches!(borrowed, std::borrow::Cow::Borrowed(_)));
    assert_eq!(borrowed.as_ref(), "/already/normalized");
}

#[test]
fn normalize_path_cow_owns_when_rewrite_needed() {
    let owned = normalize_path_cow("https://example.com/a/b/?q=1#x");
    assert!(matches!(owned, std::borrow::Cow::Owned(_)));
    assert_eq!(owned.as_ref(), "/a/b");
}
