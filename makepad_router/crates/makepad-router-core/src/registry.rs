#![allow(clippy::question_mark)]
//! Route registry for pattern-based route management.
//!
//! This module provides the route registry that handles:
//! - Route registration by ID and by pattern
//! - Path resolution with pattern matching
//! - Priority-based route ordering
//! - Optimized lookups via indexing

use crate::pattern::{RoutePattern, RoutePatternRef, RouteSegment};
use crate::route::Route;
use crate::url;
use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;

fn first_segment(path: &str) -> &str {
    let trimmed = path.strip_prefix('/').unwrap_or(path);
    if trimmed.is_empty() {
        return "";
    }
    match trimmed.find('/') {
        Some(pos) => &trimmed[..pos],
        None => trimmed,
    }
}

/// Route registry entry
#[allow(clippy::question_mark)]
#[derive(Clone, Debug, SerBin, DeBin, SerRon, DeRon)]
struct RouteEntry {
    route_id: LiveId,
    pattern: Option<RoutePatternRef>,
    priority: usize,
}

#[derive(Clone, Debug, Default)]
struct RouteEntryMeta {
    first_static_segment: Option<String>,
    segment_count: usize,
    has_wildcard: bool,
    exact_static_path: Option<String>,
}

/// Registry for pattern-based routes
#[derive(Clone, Debug, Default)]
pub struct RouteRegistry {
    /// Routes by LiveId (for exact matches)
    by_id: HashMap<LiveId, RouteEntry>,
    /// Routes by pattern (for path-based matching)
    by_pattern: Vec<RouteEntry>,
    /// Exact lookup for fully-static patterns (normalized path -> route_id).
    exact_static: HashMap<String, LiveId>,
    /// Candidate pattern indices keyed by first static segment.
    by_first_segment: HashMap<String, Vec<usize>>,
    /// Candidate pattern indices for patterns without a static first segment.
    fallback_dynamic: Vec<usize>,
    /// Candidate pattern indices that contain wildcards and no static first segment.
    fallback_wildcard: Vec<usize>,
    /// Precomputed metadata for entries in `by_pattern`.
    metas: Vec<RouteEntryMeta>,
}

impl RouteRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_pattern: Vec::new(),
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_dynamic: Vec::new(),
            fallback_wildcard: Vec::new(),
            metas: Vec::new(),
        }
    }

    /// Register a route by LiveId
    pub fn register_by_id(&mut self, route_id: LiveId) {
        let entry = RouteEntry {
            route_id,
            pattern: None,
            priority: 0, // Highest priority
        };
        self.by_id.insert(route_id, entry);
    }

    /// Register a route pattern
    pub fn register_pattern(&mut self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        let route_pattern = RoutePattern::parse(pattern)?;
        let priority = route_pattern.priority();
        let entry = RouteEntry {
            route_id,
            pattern: Some(RoutePatternRef::new(route_pattern)),
            priority,
        };

        let mut requires_full_rebuild = false;
        if let Some(existing_pos) = self.by_pattern.iter().position(|e| e.route_id == route_id) {
            self.by_pattern.remove(existing_pos);
            requires_full_rebuild = true;
        }

        self.by_id.insert(route_id, entry.clone());

        // Insert in sorted order by priority (lower value = higher priority)
        let pos = self
            .by_pattern
            .iter()
            .position(|e| e.priority > priority)
            .unwrap_or(self.by_pattern.len());
        if pos == self.by_pattern.len() {
            self.by_pattern.push(entry);
            if requires_full_rebuild {
                self.rebuild_indices();
            } else {
                self.index_last_pattern_entry();
            }
        } else {
            self.by_pattern.insert(pos, entry);
            self.rebuild_indices();
        }
        Ok(())
    }

    /// Resolve a path to a route (exact static match first, then pattern match).
    pub fn resolve_path(&self, path: &str) -> Option<Route> {
        let normalized = url::normalize_path_cow(path);
        let normalized = normalized.as_ref();

        if let Some(route_id) = self.exact_static.get(normalized).copied() {
            let pattern = self.by_id.get(&route_id).and_then(|e| e.pattern.clone());
            return Some(Route {
                id: route_id,
                params: Default::default(),
                query: Default::default(),
                hash: String::new(),
                pattern,
            });
        }

        let first = first_segment(normalized);

        if let Some(candidates) = self.by_first_segment.get(first) {
            for &idx in candidates {
                let Some(entry) = self.by_pattern.get(idx) else {
                    continue;
                };
                if let Some(meta) = self.metas.get(idx) {
                    debug_assert_eq!(meta.first_static_segment.as_deref(), Some(first));
                    let _ = (
                        meta.segment_count,
                        meta.has_wildcard,
                        &meta.exact_static_path,
                    );
                }
                let Some(ref pattern) = entry.pattern else {
                    continue;
                };
                if let Some(params) = pattern.matches(normalized) {
                    return Some(Route {
                        id: entry.route_id,
                        params,
                        query: Default::default(),
                        hash: String::new(),
                        pattern: Some(pattern.clone()),
                    });
                }
            }
        }

        for &idx in &self.fallback_dynamic {
            let Some(entry) = self.by_pattern.get(idx) else {
                continue;
            };
            if let Some(meta) = self.metas.get(idx) {
                let _ = (
                    meta.segment_count,
                    meta.has_wildcard,
                    &meta.exact_static_path,
                );
            }
            let Some(ref pattern) = entry.pattern else {
                continue;
            };
            if let Some(params) = pattern.matches(normalized) {
                return Some(Route {
                    id: entry.route_id,
                    params,
                    query: Default::default(),
                    hash: String::new(),
                    pattern: Some(pattern.clone()),
                });
            }
        }

        for &idx in &self.fallback_wildcard {
            let Some(entry) = self.by_pattern.get(idx) else {
                continue;
            };
            if let Some(meta) = self.metas.get(idx) {
                let _ = (
                    meta.segment_count,
                    meta.has_wildcard,
                    &meta.exact_static_path,
                );
            }
            let Some(ref pattern) = entry.pattern else {
                continue;
            };
            if let Some(params) = pattern.matches(normalized) {
                return Some(Route {
                    id: entry.route_id,
                    params,
                    query: Default::default(),
                    hash: String::new(),
                    pattern: Some(pattern.clone()),
                });
            }
        }

        None
    }

    /// Check if a route ID is registered
    pub fn has_route(&self, route_id: LiveId) -> bool {
        self.by_id.contains_key(&route_id)
    }

    /// Get pattern for a route ID (if registered with a pattern).
    pub fn get_pattern(&self, route_id: LiveId) -> Option<&RoutePatternRef> {
        self.by_id.get(&route_id).and_then(|e| e.pattern.as_ref())
    }

    fn index_pattern_entry(entry: &RouteEntry) -> Option<RouteEntryMeta> {
        let pattern = entry.pattern.as_ref()?;

        let segment_count = pattern.segments.len();
        let has_wildcard = pattern.segments.iter().any(|s| {
            matches!(
                s,
                RouteSegment::WildcardSingle | RouteSegment::WildcardMulti
            )
        });
        let first_static_segment = match pattern.segments.first() {
            Some(RouteSegment::Static(first)) => Some(first.clone()),
            _ => None,
        };

        let mut meta = RouteEntryMeta {
            first_static_segment,
            segment_count,
            has_wildcard,
            exact_static_path: None,
        };

        if pattern
            .segments
            .iter()
            .all(|s| matches!(s, RouteSegment::Static(_)))
        {
            let mut path = String::new();
            path.push('/');
            for (seg_i, segment) in pattern.segments.iter().enumerate() {
                if seg_i > 0 {
                    path.push('/');
                }
                if let RouteSegment::Static(v) = segment {
                    path.push_str(v);
                }
            }
            meta.exact_static_path = Some(path);
        }

        Some(meta)
    }

    fn index_last_pattern_entry(&mut self) {
        let idx = self.by_pattern.len().saturating_sub(1);
        let Some(entry) = self.by_pattern.get(idx) else {
            return;
        };
        let Some(meta) = Self::index_pattern_entry(entry) else {
            return;
        };

        if let Some(path) = meta.exact_static_path.as_ref() {
            self.exact_static
                .entry(path.clone())
                .or_insert(entry.route_id);
        }

        match meta.first_static_segment.as_ref() {
            Some(first) => self
                .by_first_segment
                .entry(first.clone())
                .or_default()
                .push(idx),
            None if meta.has_wildcard => self.fallback_wildcard.push(idx),
            None => self.fallback_dynamic.push(idx),
        }
        self.metas.push(meta);
    }

    fn rebuild_indices(&mut self) {
        self.exact_static.clear();
        self.by_first_segment.clear();
        self.fallback_dynamic.clear();
        self.fallback_wildcard.clear();
        self.metas.clear();

        self.by_pattern.sort_by_key(|entry| entry.priority);
        for (idx, entry) in self.by_pattern.iter().enumerate() {
            let Some(meta) = Self::index_pattern_entry(entry) else {
                continue;
            };
            if let Some(path) = meta.exact_static_path.as_ref() {
                self.exact_static
                    .entry(path.clone())
                    .or_insert(entry.route_id);
            }
            match meta.first_static_segment.as_ref() {
                Some(first) => self
                    .by_first_segment
                    .entry(first.clone())
                    .or_default()
                    .push(idx),
                None if meta.has_wildcard => self.fallback_wildcard.push(idx),
                None => self.fallback_dynamic.push(idx),
            }
            self.metas.push(meta);
        }
    }
}

impl SerBin for RouteRegistry {
    fn ser_bin(&self, s: &mut Vec<u8>) {
        self.by_id.ser_bin(s);
        self.by_pattern.ser_bin(s);
    }
}

impl DeBin for RouteRegistry {
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let by_id = <HashMap<LiveId, RouteEntry>>::de_bin(o, d)?;
        let by_pattern = <Vec<RouteEntry>>::de_bin(o, d)?;
        let mut out = Self {
            by_id,
            by_pattern,
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_dynamic: Vec::new(),
            fallback_wildcard: Vec::new(),
            metas: Vec::new(),
        };
        out.rebuild_indices();
        Ok(out)
    }
}

impl SerRon for RouteRegistry {
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        s.st_pre();
        s.field(d + 1, "by_id");
        self.by_id.ser_ron(d + 1, s);
        s.conl();
        s.field(d + 1, "by_pattern");
        self.by_pattern.ser_ron(d + 1, s);
        s.out.push('\n');
        s.st_post(d);
    }
}

impl DeRon for RouteRegistry {
    fn de_ron(s: &mut DeRonState, i: &mut std::str::Chars) -> Result<Self, DeRonErr> {
        s.paren_open(i)?;
        let mut by_id: Option<HashMap<LiveId, RouteEntry>> = None;
        let mut by_pattern: Option<Vec<RouteEntry>> = None;
        loop {
            match s.tok {
                DeRonTok::ParenClose => {
                    s.paren_close(i)?;
                    break;
                }
                DeRonTok::Ident => {
                    let key = s.identbuf.clone();
                    s.ident(i)?;
                    s.colon(i)?;
                    match key.as_str() {
                        "by_id" => by_id = Some(HashMap::<LiveId, RouteEntry>::de_ron(s, i)?),
                        "by_pattern" => by_pattern = Some(Vec::<RouteEntry>::de_ron(s, i)?),
                        _ => return Err(s.err_token("by_id or by_pattern")),
                    }
                    s.eat_comma_paren(i)?;
                }
                _ => return Err(s.err_token("Identifier or )")),
            }
        }
        let mut out = Self {
            by_id: by_id.unwrap_or_default(),
            by_pattern: by_pattern.unwrap_or_default(),
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_dynamic: Vec::new(),
            fallback_wildcard: Vec::new(),
            metas: Vec::new(),
        };
        out.rebuild_indices();
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_registry_register_pattern() {
        let mut registry = RouteRegistry::new();
        registry
            .register_pattern("/user/:id", live_id!(user_profile))
            .unwrap();
        assert!(registry.has_route(live_id!(user_profile)));
    }

    #[test]
    fn test_route_registry_resolve_path() {
        let mut registry = RouteRegistry::new();
        registry
            .register_pattern("/user/:id", live_id!(user_profile))
            .unwrap();

        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert_eq!(
            route.get_param(LiveId::from_str("id")),
            Some(LiveId::from_str("123"))
        );
    }

    #[test]
    fn test_route_registry_priority() {
        let mut registry = RouteRegistry::new();
        // Register in reverse priority order
        registry
            .register_pattern("/user/**", live_id!(user_wildcard))
            .unwrap();
        registry
            .register_pattern("/user/*", live_id!(user_single))
            .unwrap();
        registry
            .register_pattern("/user/:id", live_id!(user_dynamic))
            .unwrap();
        registry
            .register_pattern("/user/profile", live_id!(user_static))
            .unwrap();

        // Most specific should match first
        let route = registry.resolve_path("/user/profile").unwrap();
        assert_eq!(route.id, live_id!(user_static));

        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_dynamic));

        let route = registry.resolve_path("/user/other").unwrap();
        assert_eq!(route.id, live_id!(user_dynamic));

        let route = registry.resolve_path("/user/123/posts").unwrap();
        assert_eq!(route.id, live_id!(user_wildcard));
    }

    #[test]
    fn test_route_registry_re_register_route_id_replaces_pattern() {
        let mut registry = RouteRegistry::new();
        registry
            .register_pattern("/first", live_id!(shared_route))
            .unwrap();
        registry
            .register_pattern("/second", live_id!(shared_route))
            .unwrap();

        assert_eq!(registry.by_pattern.len(), 1);
        assert!(registry.resolve_path("/first").is_none());
        assert_eq!(
            registry.resolve_path("/second").map(|route| route.id),
            Some(live_id!(shared_route))
        );
    }
}
