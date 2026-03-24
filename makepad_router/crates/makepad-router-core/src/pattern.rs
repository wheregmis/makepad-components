//! Route pattern matching types and logic.
//!
//! This module provides types for defining and matching URL patterns with support for:
//! - Static segments (e.g., `/user/profile`)
//! - Dynamic segments (e.g., `/user/:id`)
//! - Single-segment wildcards (e.g., `/admin/*`)
//! - Multi-segment wildcards (e.g., `/admin/**`)

use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::ops::Deref;
use std::sync::Arc;

fn next_non_empty_segment<'a>(segments: &mut std::str::Split<'a, char>) -> Option<&'a str> {
    segments.by_ref().find(|seg| !seg.is_empty())
}

fn next_non_empty_segment_with_rest(path: &str) -> Option<(&str, &str)> {
    let bytes = path.as_bytes();
    let mut start = 0;
    while start < bytes.len() && bytes[start] == b'/' {
        start += 1;
    }
    if start >= bytes.len() {
        return None;
    }

    let end = bytes[start..]
        .iter()
        .position(|&b| b == b'/')
        .map(|offset| start + offset)
        .unwrap_or(bytes.len());
    Some((&path[start..end], &path[end..]))
}

fn normalize_remaining_tail(rest: &str) -> String {
    if rest.is_empty() || rest.bytes().all(|b| b == b'/') {
        return String::new();
    }
    if !rest.contains("//") && !rest.ends_with('/') {
        // Optimization: root-prefix matches leave `remaining` without a leading slash.
        // Preserve the documented contract here by returning either `""` or a slash-prefixed
        // tail, while still avoiding the slower split/rebuild path for normalized suffixes.
        let mut out = String::with_capacity(rest.len() + usize::from(!rest.starts_with('/')));
        if !rest.starts_with('/') {
            out.push('/');
        }
        out.push_str(rest);
        return out;
    }

    let mut out = String::with_capacity(rest.len());
    for segment in rest.split('/').filter(|segment| !segment.is_empty()) {
        out.push('/');
        out.push_str(segment);
    }
    out
}

fn normalize_tail_with_head(first: &str, rest: &str) -> String {
    if rest.is_empty() {
        let mut out = String::with_capacity(first.len() + 1);
        out.push('/');
        out.push_str(first);
        return out;
    }
    if !rest.contains("//") && !rest.ends_with('/') {
        let mut out = String::with_capacity(first.len() + rest.len() + 1);
        // Optimization: nested routers call `matches_prefix_with_tail` during path resolution.
        // For already-normalized paths, copy the remaining suffix in one shot instead of
        // rebuilding it segment-by-segment through `split()` and repeated `push_str()` calls.
        out.push('/');
        out.push_str(first);
        out.push_str(rest);
        return out;
    }

    let mut out = String::with_capacity(first.len() + rest.len() + 1);
    out.push('/');
    out.push_str(first);
    for segment in rest.split('/').filter(|segment| !segment.is_empty()) {
        out.push('/');
        out.push_str(segment);
    }
    out
}

/// Represents a route segment type in a pattern
#[derive(Clone, Debug, PartialEq, Eq, Hash, SerBin, DeBin, SerRon, DeRon)]
pub enum RouteSegment {
    /// Static segment (e.g., "user", "profile")
    Static(String),
    /// Dynamic segment with parameter name (e.g., ":id", ":postId")
    Dynamic { name: String, key: LiveId },
    /// Single-segment wildcard
    WildcardSingle,
    /// Multi-segment wildcard
    WildcardMulti,
}

/// Route pattern for matching paths with dynamic segments and wildcards
#[derive(Clone, Debug, PartialEq, Eq, Hash, SerBin, DeBin, SerRon, DeRon)]
pub struct RoutePattern {
    /// Segments of the pattern
    pub segments: Vec<RouteSegment>,
}

/// Shared route pattern reference (Arc-backed) for cheap cloning.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoutePatternRef(Arc<RoutePattern>);

impl RoutePatternRef {
    pub fn new(pattern: RoutePattern) -> Self {
        Self(Arc::new(pattern))
    }
}

impl Deref for RoutePatternRef {
    type Target = RoutePattern;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<RoutePattern> for RoutePatternRef {
    fn as_ref(&self) -> &RoutePattern {
        &self.0
    }
}

impl SerBin for RoutePatternRef {
    fn ser_bin(&self, s: &mut Vec<u8>) {
        self.0.ser_bin(s);
    }
}

impl DeBin for RoutePatternRef {
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let pattern = RoutePattern::de_bin(o, d)?;
        Ok(Self(Arc::new(pattern)))
    }
}

impl SerRon for RoutePatternRef {
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        self.0.ser_ron(d, s);
    }
}

impl DeRon for RoutePatternRef {
    fn de_ron(s: &mut DeRonState, i: &mut std::str::Chars) -> Result<Self, DeRonErr> {
        let pattern = RoutePattern::de_ron(s, i)?;
        Ok(Self(Arc::new(pattern)))
    }
}

/// Route parameters - optimized for small param counts.
#[derive(Clone, Debug, Default)]
pub struct RouteParams {
    /// Generic parameters stored as LiveId key-value pairs.
    pub data: RouteParamStore,
}

#[derive(Clone, Debug)]
pub enum RouteParamStore {
    Small(Vec<(LiveId, LiveId)>),
    Map(HashMap<LiveId, LiveId>),
}

impl RoutePattern {
    fn push_live_id_value(out: &mut String, value: LiveId) {
        if value == LiveId::empty() {
            out.push('0');
        } else {
            write!(out, "{:016x}", value.get_value())
                .expect("writing into a String should be infallible");
        }
    }

    fn push_formatted_path(
        &self,
        out: &mut String,
        params: &RouteParams,
        stop_before_wildcards: bool,
    ) -> Option<()> {
        for segment in &self.segments {
            match segment {
                RouteSegment::Static(segment) => {
                    out.push('/');
                    out.push_str(segment);
                }
                RouteSegment::Dynamic { key, .. } => {
                    let value = match params.get(*key) {
                        Some(value) => value,
                        None if stop_before_wildcards => break,
                        None => return None,
                    };
                    out.push('/');
                    // Optimization: write directly into the output string instead of first
                    // building a Vec<String> and joining it, which cloned every static segment.
                    // Keep the non-interned fallback outside of LiveId::as_string's mutex to avoid
                    // recursively formatting the same LiveId and deadlocking.
                    value.as_string(|value_str| match value_str {
                        Some(value_str) => out.push_str(value_str),
                        None => Self::push_live_id_value(out, value),
                    });
                }
                RouteSegment::WildcardSingle | RouteSegment::WildcardMulti => {
                    if stop_before_wildcards {
                        break;
                    }
                    return None;
                }
            }
        }
        if out.is_empty() {
            out.push('/');
        }
        Some(())
    }

    /// Parse a route pattern string (e.g., "/user/:id" or "/admin/*")
    pub fn parse(pattern: &str) -> Result<Self, String> {
        let pattern = pattern.trim();
        if pattern.is_empty() {
            return Err("Pattern cannot be empty".to_string());
        }

        // Normalize: ensure it starts with /
        let pattern = pattern.strip_prefix('/').unwrap_or(pattern);

        // Optimization: parse route patterns in a single streaming pass.
        // Previously: collected every non-empty segment into a temporary `Vec<&str>` just to
        // check whether `**` was trailing, adding an extra allocation during route registration
        // and live-reload rebuilds. Now: walk a `Peekable` iterator and only allocate the final
        // owned segment strings that must live in the parsed pattern.
        let mut segments =
            Vec::with_capacity(pattern.as_bytes().iter().filter(|&&b| b == b'/').count() + 1);
        let mut parts = pattern.split('/').filter(|s| !s.is_empty()).peekable();

        while let Some(part) = parts.next() {
            if part == "**" {
                // Multi-segment wildcard must be the last segment
                if parts.peek().is_some() {
                    return Err("Multi-segment wildcard (**) must be the last segment".to_string());
                }
                segments.push(RouteSegment::WildcardMulti);
                break;
            } else if part == "*" {
                segments.push(RouteSegment::WildcardSingle);
            } else if let Some(param_name) = part.strip_prefix(':') {
                if param_name.is_empty() {
                    return Err("Dynamic segment parameter name cannot be empty".to_string());
                }
                let name = param_name.to_string();
                let key = LiveId::from_str(param_name);
                segments.push(RouteSegment::Dynamic { name, key });
            } else {
                segments.push(RouteSegment::Static(part.to_string()));
            }
        }

        Ok(RoutePattern { segments })
    }

    /// Match a path against this pattern and extract parameters
    pub fn matches(&self, path: &str) -> Option<RouteParams> {
        let path = path.trim();
        // Normalize: ensure it starts with /
        let path = path.strip_prefix('/').unwrap_or(path);
        let mut params = RouteParams::new();

        let mut path_segments = path.split('/');
        for segment in &self.segments {
            match segment {
                RouteSegment::Static(expected) => {
                    let actual = next_non_empty_segment(&mut path_segments)?;
                    if actual != expected {
                        return None;
                    }
                }
                RouteSegment::Dynamic { key, .. } => {
                    let value = next_non_empty_segment(&mut path_segments)?;
                    // Use from_str_with_intern to store the string so it can be retrieved later
                    use makepad_live_id::InternLiveId;
                    let param_value = LiveId::from_str_with_intern(value, InternLiveId::Yes);
                    params.add(*key, param_value);
                }
                RouteSegment::WildcardSingle => {
                    // Match exactly one segment
                    next_non_empty_segment(&mut path_segments)?;
                }
                RouteSegment::WildcardMulti => {
                    // Match remaining segments (zero or more)
                    // This is the last segment, so we're done
                    return Some(params);
                }
            }
        }

        if next_non_empty_segment(&mut path_segments).is_some() {
            // Path has more segments than pattern
            return None;
        }

        Some(params)
    }

    /// Match a path prefix against this pattern and return both extracted params and a "tail" path.
    ///
    /// This is used for nested routing: if a parent route pattern matches the beginning of a path,
    /// the remaining part (or captured wildcard part) can be delegated to a child router.
    ///
    /// Tail rules:
    /// - If the pattern ends before the path, the tail is the remaining unmatched segments.
    /// - If the pattern ends with `*` or `**`, the tail is the segment(s) matched by that wildcard.
    /// - The returned tail is `""` if there is nothing to delegate, otherwise it starts with `/`.
    pub fn matches_prefix_with_tail(&self, path: &str) -> Option<(RouteParams, String)> {
        let path = path.trim();
        let path = path.strip_prefix('/').unwrap_or(path);
        let mut remaining = path;

        let mut params = RouteParams::new();
        let last_idx = self.segments.len().saturating_sub(1);

        for (pattern_idx, segment) in self.segments.iter().enumerate() {
            match segment {
                RouteSegment::Static(expected) => {
                    let (actual, rest) = next_non_empty_segment_with_rest(remaining)?;
                    if actual != expected {
                        return None;
                    }
                    remaining = rest;
                }
                RouteSegment::Dynamic { key, .. } => {
                    let (value, rest) = next_non_empty_segment_with_rest(remaining)?;
                    use makepad_live_id::InternLiveId;
                    let param_value = LiveId::from_str_with_intern(value, InternLiveId::Yes);
                    params.add(*key, param_value);
                    remaining = rest;
                }
                RouteSegment::WildcardSingle => {
                    let (matched, rest) = next_non_empty_segment_with_rest(remaining)?;
                    // For nested routing we capture the tail if wildcard is trailing.
                    if pattern_idx == last_idx {
                        return Some((params, normalize_tail_with_head(matched, rest)));
                    }
                    remaining = rest;
                }
                RouteSegment::WildcardMulti => {
                    // Must be last (enforced by parser). Capture the rest (could be empty).
                    return Some((params, normalize_remaining_tail(remaining)));
                }
            }
        }

        Some((params, normalize_remaining_tail(remaining)))
    }

    /// Get the priority for route matching (lower = higher priority)
    pub fn priority(&self) -> usize {
        let mut priority = 0;
        for segment in &self.segments {
            match segment {
                RouteSegment::Static(_) => priority += 1,
                RouteSegment::Dynamic { .. } => priority += 100,
                RouteSegment::WildcardSingle => priority += 10000,
                RouteSegment::WildcardMulti => priority += 100000,
            }
        }
        priority
    }

    /// Format a concrete path (no wildcards) from this pattern and params.
    pub fn format_path(&self, params: &RouteParams) -> Option<String> {
        let mut out = String::with_capacity(
            1 + self
                .segments
                .iter()
                .map(|segment| match segment {
                    RouteSegment::Static(segment) => segment.len() + 1,
                    _ => 1,
                })
                .sum::<usize>(),
        );
        self.push_formatted_path(&mut out, params, false)?;
        Some(out)
    }

    /// Format the "base" part of a pattern, stopping before wildcards.
    ///
    /// This is useful for nested routing patterns like `/admin/**`, where the base is `/admin`.
    pub fn format_base_path(&self, params: &RouteParams) -> String {
        let mut out = String::with_capacity(
            1 + self
                .segments
                .iter()
                .map(|segment| match segment {
                    RouteSegment::Static(segment) => segment.len() + 1,
                    _ => 1,
                })
                .sum::<usize>(),
        );
        self.push_formatted_path(&mut out, params, true)
            .expect("base path formatting should only stop early, not fail");
        out
    }
}

impl RouteParams {
    /// Create empty route parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a parameter
    pub fn add(&mut self, key: LiveId, value: LiveId) {
        match &mut self.data {
            RouteParamStore::Small(entries) => {
                for (k, v) in entries.iter_mut() {
                    if *k == key {
                        *v = value;
                        return;
                    }
                }
                if entries.len() < 4 {
                    entries.push((key, value));
                } else {
                    let mut map: HashMap<LiveId, LiveId> = entries.iter().copied().collect();
                    map.insert(key, value);
                    self.data = RouteParamStore::Map(map);
                }
            }
            RouteParamStore::Map(map) => {
                map.insert(key, value);
            }
        }
    }

    /// Get a parameter value by key
    pub fn get(&self, key: LiveId) -> Option<LiveId> {
        match &self.data {
            RouteParamStore::Small(entries) => {
                entries.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
            }
            RouteParamStore::Map(map) => map.get(&key).copied(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.data {
            RouteParamStore::Small(entries) => entries.is_empty(),
            RouteParamStore::Map(map) => map.is_empty(),
        }
    }

    pub fn iter(&self) -> RouteParamIter<'_> {
        match &self.data {
            RouteParamStore::Small(entries) => RouteParamIter::Small(entries.iter()),
            RouteParamStore::Map(map) => RouteParamIter::Map(map.iter()),
        }
    }
}

impl Default for RouteParamStore {
    fn default() -> Self {
        RouteParamStore::Small(Vec::new())
    }
}

impl PartialEq for RouteParams {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().all(|(k, v)| other.get(*k) == Some(*v))
    }
}

impl Eq for RouteParams {}

impl RouteParams {
    fn len(&self) -> usize {
        match &self.data {
            RouteParamStore::Small(entries) => entries.len(),
            RouteParamStore::Map(map) => map.len(),
        }
    }

    fn to_hash_map(&self) -> HashMap<LiveId, LiveId> {
        match &self.data {
            RouteParamStore::Small(entries) => entries.iter().copied().collect(),
            RouteParamStore::Map(map) => map.clone(),
        }
    }

    fn from_hash_map(map: HashMap<LiveId, LiveId>) -> Self {
        if map.len() <= 4 {
            let entries = map.into_iter().collect();
            Self {
                data: RouteParamStore::Small(entries),
            }
        } else {
            Self {
                data: RouteParamStore::Map(map),
            }
        }
    }
}

pub enum RouteParamIter<'a> {
    Small(std::slice::Iter<'a, (LiveId, LiveId)>),
    Map(std::collections::hash_map::Iter<'a, LiveId, LiveId>),
}

impl<'a> Iterator for RouteParamIter<'a> {
    type Item = (&'a LiveId, &'a LiveId);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RouteParamIter::Small(iter) => iter.next().map(|(k, v)| (k, v)),
            RouteParamIter::Map(iter) => iter.next(),
        }
    }
}

impl SerBin for RouteParams {
    fn ser_bin(&self, s: &mut Vec<u8>) {
        self.to_hash_map().ser_bin(s);
    }
}

impl DeBin for RouteParams {
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let map = HashMap::<LiveId, LiveId>::de_bin(o, d)?;
        Ok(Self::from_hash_map(map))
    }
}

impl SerRon for RouteParams {
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        self.to_hash_map().ser_ron(d, s);
    }
}

impl DeRon for RouteParams {
    fn de_ron(s: &mut DeRonState, i: &mut std::str::Chars) -> Result<Self, DeRonErr> {
        let map = HashMap::<LiveId, LiveId>::de_ron(s, i)?;
        Ok(Self::from_hash_map(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;

    #[test]
    fn test_pattern_parse_static() {
        let pattern = RoutePattern::parse("/user/profile").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "user"));
        assert!(matches!(pattern.segments[1], RouteSegment::Static(ref s) if s == "profile"));
    }

    #[test]
    fn test_pattern_parse_dynamic() {
        let pattern = RoutePattern::parse("/user/:id").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "user"));
        assert!(
            matches!(pattern.segments[1], RouteSegment::Dynamic { ref name, .. } if name == "id")
        );
    }

    #[test]
    fn test_pattern_parse_wildcard_single() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "admin"));
        assert!(matches!(pattern.segments[1], RouteSegment::WildcardSingle));
    }

    #[test]
    fn test_pattern_parse_wildcard_multi() {
        let pattern = RoutePattern::parse("/admin/**").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "admin"));
        assert!(matches!(pattern.segments[1], RouteSegment::WildcardMulti));
    }

    #[test]
    fn test_pattern_parse_mixed() {
        let pattern = RoutePattern::parse("/user/:id/posts/*").unwrap();
        assert_eq!(pattern.segments.len(), 4);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "user"));
        assert!(
            matches!(pattern.segments[1], RouteSegment::Dynamic { ref name, .. } if name == "id")
        );
        assert!(matches!(pattern.segments[2], RouteSegment::Static(ref s) if s == "posts"));
        assert!(matches!(pattern.segments[3], RouteSegment::WildcardSingle));
    }

    #[test]
    fn test_pattern_match_static() {
        let pattern = RoutePattern::parse("/user/profile").unwrap();
        assert!(pattern.matches("/user/profile").is_some());
        assert!(pattern.matches("/user/settings").is_none());
    }

    #[test]
    fn test_pattern_match_dynamic() {
        let pattern = RoutePattern::parse("/user/:id").unwrap();
        let params = pattern.matches("/user/123").unwrap();
        assert_eq!(
            params.get(LiveId::from_str("id")),
            Some(LiveId::from_str("123"))
        );

        let params = pattern.matches("/user/john").unwrap();
        assert_eq!(
            params.get(LiveId::from_str("id")),
            Some(LiveId::from_str("john"))
        );
    }

    #[test]
    fn test_pattern_match_multiple_dynamic() {
        let pattern = RoutePattern::parse("/post/:postId/:slug").unwrap();
        let params = pattern.matches("/post/123/my-post").unwrap();
        assert_eq!(
            params.get(LiveId::from_str("postId")),
            Some(LiveId::from_str("123"))
        );
        assert_eq!(
            params.get(LiveId::from_str("slug")),
            Some(LiveId::from_str("my-post"))
        );
    }

    #[test]
    fn test_pattern_match_wildcard_single() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        assert!(pattern.matches("/admin/users").is_some());
        assert!(pattern.matches("/admin/settings").is_some());
        assert!(pattern.matches("/admin/users/123").is_none()); // Should not match multiple segments
    }

    #[test]
    fn test_pattern_match_wildcard_multi() {
        let pattern = RoutePattern::parse("/admin/**").unwrap();
        assert!(pattern.matches("/admin/users").is_some());
        assert!(pattern.matches("/admin/users/123").is_some());
        assert!(pattern.matches("/admin/users/123/edit").is_some());
        assert!(pattern.matches("/admin").is_some()); // Should match zero segments too
    }

    #[test]
    fn test_pattern_prefix_tail_static() {
        let pattern = RoutePattern::parse("/admin").unwrap();
        let (params, tail) = pattern
            .matches_prefix_with_tail("/admin/dashboard")
            .unwrap();
        assert!(params.is_empty());
        assert_eq!(tail, "/dashboard");
    }

    #[test]
    fn test_pattern_prefix_tail_static_does_not_double_prefix_slash() {
        let pattern = RoutePattern::parse("/admin").unwrap();
        let (_params, tail) = pattern
            .matches_prefix_with_tail("/admin/dashboard")
            .unwrap();
        assert_eq!(tail, "/dashboard");
    }

    #[test]
    fn test_pattern_prefix_tail_wildcard_single() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        let (_params, tail) = pattern
            .matches_prefix_with_tail("/admin/dashboard")
            .unwrap();
        assert_eq!(tail, "/dashboard");
    }

    #[test]
    fn test_pattern_prefix_tail_wildcard_single_includes_extra_suffix() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        let (_params, tail) = pattern
            .matches_prefix_with_tail("/admin/dashboard/details")
            .unwrap();
        assert_eq!(tail, "/dashboard/details");
    }

    #[test]
    fn test_pattern_prefix_tail_wildcard_multi() {
        let pattern = RoutePattern::parse("/admin/**").unwrap();
        let (_params, tail) = pattern.matches_prefix_with_tail("/admin/a/b").unwrap();
        assert_eq!(tail, "/a/b");
    }

    #[test]
    fn test_pattern_prefix_tail_dynamic() {
        let pattern = RoutePattern::parse("/user/:id/**").unwrap();
        let (params, tail) = pattern
            .matches_prefix_with_tail("/user/42/profile/settings")
            .unwrap();
        assert_eq!(
            params.get(LiveId::from_str("id")),
            Some(LiveId::from_str("42"))
        );
        assert_eq!(tail, "/profile/settings");
    }

    #[test]
    fn test_pattern_prefix_tail_non_trailing_wildcard_keeps_remainder() {
        let pattern = RoutePattern::parse("/a/*/c").unwrap();
        let (_params, tail) = pattern.matches_prefix_with_tail("/a/x/c/d").unwrap();
        assert_eq!(tail, "/d");
    }

    #[test]
    fn test_pattern_prefix_tail_root_pattern_keeps_leading_slash() {
        let pattern = RoutePattern::parse("/").unwrap();
        let (_params, tail) = pattern.matches_prefix_with_tail("/a/b").unwrap();
        assert_eq!(tail, "/a/b");
    }

    #[test]
    fn test_pattern_prefix_tail_root_wildcard_keeps_leading_slash() {
        let pattern = RoutePattern::parse("/**").unwrap();
        let (_params, tail) = pattern.matches_prefix_with_tail("/a/b").unwrap();
        assert_eq!(tail, "/a/b");
    }

    #[test]
    fn test_pattern_prefix_tail_trims_trailing_slashes() {
        let pattern = RoutePattern::parse("/admin").unwrap();
        let (_params, tail) = pattern
            .matches_prefix_with_tail("/admin/dashboard/")
            .unwrap();
        assert_eq!(tail, "/dashboard");
    }

    #[test]
    fn test_pattern_prefix_tail_normalizes_duplicate_slashes() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        let (_params, tail) = pattern
            .matches_prefix_with_tail("/admin//dashboard//details/")
            .unwrap();
        assert_eq!(tail, "/dashboard/details");
    }

    #[test]
    fn test_pattern_priority() {
        let static_pattern = RoutePattern::parse("/user/profile").unwrap();
        let dynamic_pattern = RoutePattern::parse("/user/:id").unwrap();
        let wildcard_single = RoutePattern::parse("/user/*").unwrap();
        let wildcard_multi = RoutePattern::parse("/user/**").unwrap();

        assert!(static_pattern.priority() < dynamic_pattern.priority());
        assert!(dynamic_pattern.priority() < wildcard_single.priority());
        assert!(wildcard_single.priority() < wildcard_multi.priority());
    }

    #[test]
    fn test_pattern_types_exist() {
        // Verify that RoutePattern and RouteSegment can be constructed
        let _pattern = RoutePattern::parse("/user/:id").unwrap();
    }

    #[test]
    fn test_format_path_and_base_path_outputs() {
        let pattern = RoutePattern::parse("/team/:team/member/:member/**").unwrap();
        let mut params = RouteParams::new();
        params.add(
            LiveId::from_str("team"),
            LiveId::from_str_with_intern("alpha", InternLiveId::Yes),
        );
        params.add(
            LiveId::from_str("member"),
            LiveId::from_str_with_intern("beta", InternLiveId::Yes),
        );

        assert_eq!(pattern.format_path(&params), None);
        assert_eq!(pattern.format_base_path(&params), "/team/alpha/member/beta");
        assert_eq!(
            RoutePattern::parse("/team/:team/member/:member")
                .unwrap()
                .format_path(&params),
            Some("/team/alpha/member/beta".to_string())
        );
    }

    #[test]
    fn test_format_path_with_non_interned_param_uses_hex_fallback() {
        let pattern = RoutePattern::parse("/user/:id/settings").unwrap();
        let mut params = RouteParams::new();
        let non_interned = LiveId::from_str("42");
        params.add(LiveId::from_str("id"), non_interned);

        let expected = format!("/user/{:016x}/settings", non_interned.get_value());
        assert_eq!(pattern.format_path(&params), Some(expected.clone()));
        assert_eq!(pattern.format_base_path(&params), expected);
    }

    #[test]
    #[ignore = "benchmark-style measurement for allocation reduction"]
    fn bench_format_path_direct_string_build() {
        fn legacy_format_path(pattern: &RoutePattern, params: &RouteParams) -> Option<String> {
            let mut out: Vec<String> = Vec::with_capacity(pattern.segments.len());
            for segment in &pattern.segments {
                match segment {
                    RouteSegment::Static(segment) => out.push(segment.clone()),
                    RouteSegment::Dynamic { key, .. } => out.push(params.get(*key)?.to_string()),
                    RouteSegment::WildcardSingle | RouteSegment::WildcardMulti => return None,
                }
            }
            Some(format!("/{}", out.join("/")))
        }

        let pattern = RoutePattern::parse("/team/:team/member/:member/settings").unwrap();
        let mut params = RouteParams::new();
        params.add(
            LiveId::from_str("team"),
            LiveId::from_str_with_intern("alpha", InternLiveId::Yes),
        );
        params.add(
            LiveId::from_str("member"),
            LiveId::from_str_with_intern("beta", InternLiveId::Yes),
        );
        let iterations = 200_000;

        let start = std::time::Instant::now();
        let mut sink = 0usize;
        for _ in 0..iterations {
            sink += legacy_format_path(&pattern, &params).unwrap().len();
        }
        let legacy_elapsed = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            sink += pattern.format_path(&params).unwrap().len();
        }
        let optimized_elapsed = start.elapsed();

        eprintln!("legacy={legacy_elapsed:?} optimized={optimized_elapsed:?} sink={sink}");
    }

    #[test]
    #[ignore = "benchmark-style measurement for parse allocation reduction"]
    fn bench_parse_streaming_segments() {
        fn legacy_parse(pattern: &str) -> Result<RoutePattern, String> {
            let pattern = pattern.trim();
            if pattern.is_empty() {
                return Err("Pattern cannot be empty".to_string());
            }

            let pattern = pattern.strip_prefix('/').unwrap_or(pattern);
            let mut segments = Vec::new();
            let parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();

            for (i, part) in parts.iter().enumerate() {
                if part == &"**" {
                    if i != parts.len() - 1 {
                        return Err(
                            "Multi-segment wildcard (**) must be the last segment".to_string()
                        );
                    }
                    segments.push(RouteSegment::WildcardMulti);
                    break;
                } else if part == &"*" {
                    segments.push(RouteSegment::WildcardSingle);
                } else if let Some(param_name) = part.strip_prefix(':') {
                    if param_name.is_empty() {
                        return Err("Dynamic segment parameter name cannot be empty".to_string());
                    }
                    let name = param_name.to_string();
                    let key = LiveId::from_str(param_name);
                    segments.push(RouteSegment::Dynamic { name, key });
                } else {
                    segments.push(RouteSegment::Static(part.to_string()));
                }
            }

            Ok(RoutePattern { segments })
        }

        let pattern = "/team/:team/member/:member/settings/:tab";
        let iterations = 200_000;

        let start = std::time::Instant::now();
        let mut sink = 0usize;
        for _ in 0..iterations {
            sink += legacy_parse(pattern).unwrap().segments.len();
        }
        let legacy_elapsed = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            sink += RoutePattern::parse(pattern).unwrap().segments.len();
        }
        let optimized_elapsed = start.elapsed();

        eprintln!("legacy={legacy_elapsed:?} optimized={optimized_elapsed:?} sink={sink}");
    }

    #[test]
    #[ignore = "benchmark-style measurement for nested-tail allocation reduction"]
    fn bench_matches_prefix_with_tail_suffix_fast_path() {
        fn legacy_collect_tail(
            first: Option<&str>,
            segments: &mut std::str::Split<'_, char>,
        ) -> String {
            let mut out = String::new();
            if let Some(first) = first.filter(|segment| !segment.is_empty()) {
                out.push('/');
                out.push_str(first);
            }
            for segment in segments.by_ref().filter(|segment| !segment.is_empty()) {
                out.push('/');
                out.push_str(segment);
            }
            out
        }

        fn legacy_matches_prefix_with_tail(
            pattern: &RoutePattern,
            path: &str,
        ) -> Option<(RouteParams, String)> {
            let path = path.trim();
            let path = path.strip_prefix('/').unwrap_or(path);
            let mut path_segments = path.split('/');
            let mut params = RouteParams::new();
            let mut tail = None;
            let last_idx = pattern.segments.len().saturating_sub(1);

            for (pattern_idx, segment) in pattern.segments.iter().enumerate() {
                match segment {
                    RouteSegment::Static(expected) => {
                        let actual = path_segments.by_ref().find(|segment| !segment.is_empty())?;
                        if actual != expected {
                            return None;
                        }
                    }
                    RouteSegment::Dynamic { key, .. } => {
                        let value = path_segments.by_ref().find(|segment| !segment.is_empty())?;
                        use makepad_live_id::InternLiveId;
                        params.add(*key, LiveId::from_str_with_intern(value, InternLiveId::Yes));
                    }
                    RouteSegment::WildcardSingle => {
                        let matched = path_segments.by_ref().find(|segment| !segment.is_empty())?;
                        if pattern_idx == last_idx {
                            tail = Some(legacy_collect_tail(Some(matched), &mut path_segments));
                        }
                    }
                    RouteSegment::WildcardMulti => {
                        tail = Some(legacy_collect_tail(
                            path_segments.by_ref().find(|segment| !segment.is_empty()),
                            &mut path_segments,
                        ));
                        return Some((params, tail.unwrap_or_default()));
                    }
                }
            }

            if tail.is_none() {
                tail = Some(legacy_collect_tail(
                    path_segments.by_ref().find(|segment| !segment.is_empty()),
                    &mut path_segments,
                ));
            }

            Some((params, tail.unwrap_or_default()))
        }

        let pattern = RoutePattern::parse("/team/:team/*").unwrap();
        let path = "/team/alpha/member/beta/details";
        let iterations = 200_000;

        let start = std::time::Instant::now();
        let mut sink = 0usize;
        for _ in 0..iterations {
            sink += black_box(legacy_matches_prefix_with_tail(&pattern, path))
                .unwrap()
                .1
                .len();
        }
        let legacy_elapsed = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            sink += black_box(pattern.matches_prefix_with_tail(path))
                .unwrap()
                .1
                .len();
        }
        let optimized_elapsed = start.elapsed();

        eprintln!("legacy={legacy_elapsed:?} optimized={optimized_elapsed:?} sink={sink}");
    }
}
