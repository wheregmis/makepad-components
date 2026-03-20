use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

/// Parsed URL parts for router navigation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouterUrl {
    /// Normalized path (always starts with `/`).
    pub path: String,
    /// Raw query string including the leading `?` (or empty).
    pub query: String,
    /// Raw hash including the leading `#` (or empty).
    pub hash: String,
}

impl RouterUrl {
    /// Parse a URL or path into normalized path/query/hash parts.
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Self {
                path: "/".to_string(),
                query: String::new(),
                hash: String::new(),
            };
        }

        // Optimization: keep parsing on borrowed slices until the final result.
        // Previously: built an eager scratch String, then reformatted query/hash/path pieces.
        // Now: slice the input first and allocate only the three output strings.
        let normalized = if let Some((_, after_scheme)) = trimmed.split_once("://") {
            after_scheme
                .split_once('/')
                .map(|(_, after_host_slash)| after_host_slash)
                .unwrap_or("")
        } else {
            trimmed
        };

        let (before_hash, hash) = match normalized.split_once('#') {
            Some((head, tail)) => (head, tail),
            None => (normalized, ""),
        };
        let (path, query) = match before_hash.split_once('?') {
            Some((head, tail)) => (head, tail),
            None => (before_hash, ""),
        };

        let path = match path.trim() {
            "" => "/".to_string(),
            trimmed_path if trimmed_path.starts_with('/') => trimmed_path.to_string(),
            trimmed_path => {
                let mut normalized_path = String::with_capacity(trimmed_path.len() + 1);
                normalized_path.push('/');
                normalized_path.push_str(trimmed_path);
                normalized_path
            }
        };

        Self {
            path,
            query: if query.is_empty() {
                String::new()
            } else {
                let mut query_string = String::with_capacity(query.len() + 1);
                query_string.push('?');
                query_string.push_str(query);
                query_string
            },
            hash: if hash.is_empty() {
                String::new()
            } else {
                let mut hash_string = String::with_capacity(hash.len() + 1);
                hash_string.push('#');
                hash_string.push_str(hash);
                hash_string
            },
        }
    }

    /// Parse the query string into a string map.
    pub fn parse_query_map(&self) -> HashMap<String, String> {
        parse_query_map(&self.query)
    }
}

impl fmt::Display for RouterUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.path, self.query, self.hash)
    }
}

/// Normalize a path or URL for matching (strip scheme/host, query/hash, ensure leading `/`,
/// and collapse trailing slashes).
pub fn normalize_path(input: &str) -> String {
    normalize_path_cow(input).into_owned()
}

/// Borrowing variant of `normalize_path`.
///
/// Returns a borrowed slice whenever normalization can be represented as a subslice of `input`,
/// avoiding an allocation in the common case where the input is already a normalized path.
pub fn normalize_path_cow(input: &str) -> Cow<'_, str> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Cow::Borrowed("/");
    }

    let mut core = trimmed;
    if let Some((_, after_scheme)) = trimmed.split_once("://") {
        if let Some((_, after_host_slash)) = after_scheme.split_once('/') {
            core = after_host_slash;
        } else {
            return Cow::Borrowed("/");
        }
    }

    if let Some(pos) = core.find(['?', '#']) {
        core = &core[..pos];
    }

    if core.is_empty() {
        return Cow::Borrowed("/");
    }

    if core.starts_with('/') {
        let mut end = core.len();
        while end > 1 && core.as_bytes()[end - 1] == b'/' {
            end -= 1;
        }
        return Cow::Borrowed(&core[..end]);
    }

    let mut out = String::with_capacity(core.len() + 1);
    out.push('/');
    out.push_str(core);
    while out.len() > 1 && out.ends_with('/') {
        out.pop();
    }
    Cow::Owned(out)
}

/// Parse a query string (`?a=1&b=2`) into a string map.
pub fn parse_query_map(query: &str) -> HashMap<String, String> {
    let q = query.trim();
    let q = q.strip_prefix('?').unwrap_or(q);
    if q.is_empty() {
        return HashMap::new();
    }
    let mut out = HashMap::new();
    for pair in q.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (k, v) = match pair.split_once('=') {
            Some((k, v)) => (k, v),
            None => (pair, ""),
        };
        let key = decode_www_form_component(k);
        if key.is_empty() {
            continue;
        }
        let val = decode_www_form_component(v);
        out.insert(key, val);
    }
    out
}

/// Build a stable, sorted query string from a map.
pub fn build_query_string(map: &HashMap<String, String>) -> String {
    if map.is_empty() {
        return String::new();
    }
    let mut entries: Vec<(&str, &str)> = map
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    entries.sort_unstable_by_key(|(key, _)| *key);

    let estimated_len = 1 + entries
        .iter()
        .map(|(key, value)| key.len() + value.len() + usize::from(!value.is_empty()) + 1)
        .sum::<usize>();
    let mut out = String::with_capacity(estimated_len);
    out.push('?');
    for (i, (key, value)) in entries.iter().enumerate() {
        if i > 0 {
            out.push('&');
        }
        // Optimization: write percent-encoded bytes directly into the final query buffer.
        // Previously: each key/value called `encode_www_form_component`, which built a
        // temporary String before copying it into `out`, multiplying heap churn for hot
        // router URL updates. Streaming keeps the same output with one destination buffer.
        encode_www_form_component_into(&mut out, key);
        if !value.is_empty() {
            out.push('=');
            encode_www_form_component_into(&mut out, value);
        }
    }
    out
}

fn decode_www_form_component(input: &str) -> String {
    let mut bytes = Vec::<u8>::with_capacity(input.len());
    let mut iter = input.as_bytes().iter().copied().peekable();
    while let Some(b) = iter.next() {
        match b {
            b'+' => bytes.push(b' '),
            b'%' => {
                let hi = iter.next();
                let lo = iter.next();
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    if let (Some(hi), Some(lo)) = (hex_val(hi), hex_val(lo)) {
                        bytes.push((hi << 4) | lo);
                    }
                }
            }
            _ => bytes.push(b),
        }
    }
    String::from_utf8(bytes).unwrap_or_else(|_| input.to_string())
}

fn encode_www_form_component_into(out: &mut String, input: &str) {
    for &b in input.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => {
                out.push('%');
                out.push(hex_char((b >> 4) & 0x0f));
                out.push(hex_char(b & 0x0f));
            }
        }
    }
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn hex_char(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'A' + (n - 10)) as char,
        _ => '0',
    }
}

#[cfg(test)]
mod tests {
    use super::RouterUrl;
    use std::collections::HashMap;
    use std::hint::black_box;
    use std::time::Instant;

    #[test]
    fn parse_url_handles_full_url_and_path_inputs() {
        let parsed = RouterUrl::parse("https://example.com/dashboard/jobs?tab=active#row-7");
        assert_eq!(parsed.path, "/dashboard/jobs");
        assert_eq!(parsed.query, "?tab=active");
        assert_eq!(parsed.hash, "#row-7");

        let parsed = RouterUrl::parse("reports?sort=desc");
        assert_eq!(parsed.path, "/reports");
        assert_eq!(parsed.query, "?sort=desc");
        assert_eq!(parsed.hash, "");
    }

    #[test]
    #[ignore = "micro-benchmark; run explicitly in release mode for stable numbers"]
    fn parse_url_performance_comparison() {
        fn old_parse(input: &str) -> RouterUrl {
            let mut s = input.trim().to_string();
            if s.is_empty() {
                return RouterUrl {
                    path: "/".to_string(),
                    query: String::new(),
                    hash: String::new(),
                };
            }
            if let Some((_, after_scheme)) = s.split_once("://") {
                if let Some((_, after_host_slash)) = after_scheme.split_once('/') {
                    s = format!("/{}", after_host_slash);
                } else {
                    s = "/".to_string();
                }
            }
            let s_trim = s.trim();
            let (before_hash, hash) = match s_trim.split_once('#') {
                Some((a, b)) => (a, format!("#{}", b)),
                None => (s_trim, String::new()),
            };
            let (path, query) = match before_hash.split_once('?') {
                Some((a, b)) => (a, format!("?{}", b)),
                None => (before_hash, String::new()),
            };
            let mut path = path.trim().to_string();
            if path.is_empty() {
                path = "/".to_string();
            } else if !path.starts_with('/') {
                path.insert(0, '/');
            }
            RouterUrl { path, query, hash }
        }

        const ITERATIONS: usize = 20_000;
        let input = "/dashboard/jobs?tab=active#row-7";

        let old_start = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(old_parse(input));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(RouterUrl::parse(input));
        }
        let new_elapsed = new_start.elapsed();

        println!(
            "old_parse={:?} new_parse={:?} improvement={:.2}%",
            old_elapsed,
            new_elapsed,
            (1.0 - (new_elapsed.as_secs_f64() / old_elapsed.as_secs_f64())) * 100.0
        );
    }

    #[test]
    #[ignore = "micro-benchmark; run explicitly in release mode for stable numbers"]
    fn build_query_string_direct_encode_benchmark() {
        fn old_encode_www_form_component(input: &str) -> String {
            let mut out = String::new();
            for &b in input.as_bytes() {
                match b {
                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                        out.push(b as char)
                    }
                    b' ' => out.push('+'),
                    _ => {
                        out.push('%');
                        out.push(super::hex_char((b >> 4) & 0x0f));
                        out.push(super::hex_char(b & 0x0f));
                    }
                }
            }
            out
        }

        fn old_build_query_string(map: &HashMap<String, String>) -> String {
            let mut out = String::new();
            out.push('?');
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for (i, k) in keys.iter().enumerate() {
                if i > 0 {
                    out.push('&');
                }
                let v = map.get(*k).map(|s| s.as_str()).unwrap_or("");
                out.push_str(&old_encode_www_form_component(k));
                if !v.is_empty() {
                    out.push('=');
                    out.push_str(&old_encode_www_form_component(v));
                }
            }
            out
        }

        let mut query = HashMap::new();
        query.insert("tab".to_string(), "team members".to_string());
        query.insert("filter".to_string(), "role=admin".to_string());
        query.insert("page".to_string(), "42".to_string());
        query.insert("empty".to_string(), String::new());
        query.insert("redirect".to_string(), "/settings/profile".to_string());
        const ITERATIONS: usize = 200_000;

        let old_start = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(old_build_query_string(&query));
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(super::build_query_string(&query));
        }
        let new_elapsed = new_start.elapsed();

        println!(
            "old_build_query_string={:?} new_build_query_string={:?} improvement={:.2}%",
            old_elapsed,
            new_elapsed,
            (1.0 - (new_elapsed.as_secs_f64() / old_elapsed.as_secs_f64())) * 100.0
        );
    }
}
