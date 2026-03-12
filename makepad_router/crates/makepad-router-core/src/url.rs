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
        let mut s = input.trim().to_string();
        if s.is_empty() {
            return Self {
                path: "/".to_string(),
                query: String::new(),
                hash: String::new(),
            };
        }

        // Accept full URLs (e.g. https://host/path?query#hash) and path-only inputs.
        if let Some((_, after_scheme)) = s.split_once("://") {
            let mut rest = after_scheme;
            if let Some((_, after_host_slash)) = rest.split_once('/') {
                rest = after_host_slash;
                s = format!("/{}", rest);
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

        Self { path, query, hash }
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
    let mut out = String::new();
    out.push('?');
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    for (i, k) in keys.iter().enumerate() {
        if i > 0 {
            out.push('&');
        }
        let v = map.get(*k).map(|s| s.as_str()).unwrap_or("");
        out.push_str(&encode_www_form_component(k));
        if !v.is_empty() {
            out.push('=');
            out.push_str(&encode_www_form_component(v));
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

fn encode_www_form_component(input: &str) -> String {
    let mut out = String::new();
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
    out
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
