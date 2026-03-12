use crate::url::RouterUrl;

use super::RouterWidget;

impl RouterWidget {
    pub(super) fn parse_url_cached(&mut self, input: &str) -> RouterUrl {
        if let Some(parsed) = self.caches.url_parse_cache.get(input) {
            return parsed;
        }

        let parsed = RouterUrl::parse(input);
        self.caches
            .url_parse_cache
            .insert(input.to_string(), parsed.clone());
        parsed
    }
}
