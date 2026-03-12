#![allow(clippy::question_mark)]

use crate::pattern::{RouteParams, RoutePattern, RoutePatternRef};
use crate::url;
use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;

/// Represents a route in the application
#[allow(clippy::question_mark)]
#[derive(Clone, Debug, PartialEq, Eq, SerBin, DeBin, SerRon, DeRon)]
pub struct Route {
    /// The unique identifier for this route
    pub id: LiveId,
    /// Optional parameters for the route
    pub params: RouteParams,
    /// Optional query parameters for the route.
    pub query: RouteQuery,
    /// Optional hash fragment for the route (including the leading `#`).
    pub hash: String,
    /// Optional route pattern for path-based matching
    pub pattern: Option<RoutePatternRef>,
}

/// Query parameters stored as a string map.
#[allow(clippy::question_mark)]
#[derive(Clone, Debug, Default, PartialEq, Eq, SerBin, DeBin, SerRon, DeRon)]
pub struct RouteQuery {
    /// Query parameters stored as key-value string pairs.
    pub data: HashMap<String, String>,
}

impl Route {
    /// Create a new route with the given ID
    pub fn new(id: LiveId) -> Self {
        Self {
            id,
            params: RouteParams::default(),
            query: RouteQuery::default(),
            hash: String::new(),
            pattern: None,
        }
    }

    /// Create a new route with parameters
    pub fn with_params(id: LiveId, params: RouteParams) -> Self {
        Self {
            id,
            params,
            query: RouteQuery::default(),
            hash: String::new(),
            pattern: None,
        }
    }

    /// Create a route from a pattern string
    pub fn from_pattern(pattern: &str, id: LiveId) -> Result<Self, String> {
        let route_pattern = RoutePattern::parse(pattern)?;
        Ok(Self {
            id,
            params: RouteParams::default(),
            query: RouteQuery::default(),
            hash: String::new(),
            pattern: Some(RoutePatternRef::new(route_pattern)),
        })
    }

    /// Add a parameter to the route
    pub fn param(mut self, key: LiveId, value: LiveId) -> Self {
        self.params.add(key, value);
        self
    }

    /// Get a parameter value by key
    pub fn get_param(&self, key: LiveId) -> Option<LiveId> {
        self.params.get(key)
    }

    /// Get a parameter value as a `String`.
    pub fn get_param_string(&self, key: LiveId) -> Option<String> {
        self.get_param(key)?
            .as_string(|id_str| id_str.map(|s| s.to_string()))
    }

    /// Get a parameter value as `i64` (parsed).
    pub fn get_param_i64(&self, key: LiveId) -> Option<i64> {
        self.get_param_string(key)?.parse().ok()
    }

    /// Get a parameter value as `u64` (parsed).
    pub fn get_param_u64(&self, key: LiveId) -> Option<u64> {
        self.get_param_string(key)?.parse().ok()
    }

    /// Get a parameter value as `bool` (accepts 1/0, true/false, yes/no, on/off).
    pub fn get_param_bool(&self, key: LiveId) -> Option<bool> {
        match self.get_param_string(key)?.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        }
    }

    /// Get a parameter value as `f64` (parsed).
    pub fn get_param_f64(&self, key: LiveId) -> Option<f64> {
        self.get_param_string(key)?.parse().ok()
    }

    /// Build a query string from the stored query map.
    pub fn query_string(&self) -> String {
        url::build_query_string(&self.query.data)
    }

    /// Get a query value by key.
    pub fn query_get(&self, key: &str) -> Option<&str> {
        self.query.get(key)
    }

    /// Get a query value as a `String`.
    pub fn query_get_string(&self, key: &str) -> Option<String> {
        Some(self.query_get(key)?.to_string())
    }

    /// Get a query value as `i64` (parsed).
    pub fn query_get_i64(&self, key: &str) -> Option<i64> {
        self.query_get(key)?.parse().ok()
    }

    /// Get a query value as `u64` (parsed).
    pub fn query_get_u64(&self, key: &str) -> Option<u64> {
        self.query_get(key)?.parse().ok()
    }

    /// Get a query value as `bool` (accepts 1/0, true/false, yes/no, on/off).
    pub fn query_get_bool(&self, key: &str) -> Option<bool> {
        match self.query_get(key)?.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        }
    }

    /// Get a query value as `f64` (parsed).
    pub fn query_get_f64(&self, key: &str) -> Option<f64> {
        self.query_get(key)?.parse().ok()
    }
}

impl RouteQuery {
    /// Create an empty query map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a query value by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|v| v.as_str())
    }

    /// Set or replace a query key.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    /// Remove a query key, returning true if present.
    pub fn remove(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    /// Clear all query keys.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Build a query map from a raw query string (`?a=1&b=2`).
    pub fn from_query_string(query: &str) -> Self {
        Self {
            data: url::parse_query_map(query),
        }
    }
}

/// Macro to create routes easily
#[macro_export]
macro_rules! route {
    ($id:ident) => {
        Route::new(live_id!($id))
    };
    ($id:ident, $($key:ident = $value:ident),+) => {
        {
            let mut route = Route::new(live_id!($id));
            $(
                route = route.param(live_id!($key), live_id!($value));
            )+
            route
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use makepad_live_id::live_id;

    #[test]
    fn test_route_from_pattern() {
        let route = Route::from_pattern("/user/:id", live_id!(user_profile)).unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert!(route.pattern.is_some());
    }
}
