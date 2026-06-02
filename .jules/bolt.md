## 2026-04-23 - `replace` beat manual `+` decoding
**Learning:** In `makepad_router` query decoding, a hand-rolled single-scan `+` fast path looked cheaper on paper but regressed the release benchmark versus the existing `String::replace` branch. The extra per-byte `push` work outweighed the saved scans.
**Action:** For short router query strings, benchmark standard-library string transforms before replacing them with manual byte loops. Keep the benchmark harness and revert quickly when the numbers move the wrong way.

## 2025-05-08 – Eliminating Redundant Route Cloning in Makepad Router Action Dispatch
**Learning:** In the Makepad router widgets library (`makepad-router-widgets`), `queue_route_actions` previously accepted a full `&Route` reference to extract the ID, while callers simultaneously created `RouterAction` enum variants (like `Navigate`, `Replace`, `Reset`) by calling `.clone()` on the `Route`. Because `Route` contains nested objects (like `String` identifiers and `HashMap` query parameters), this `.clone()` caused significant heap allocation and churn during every route transition.
**Action:** By modifying `queue_route_actions` to strictly accept `new_route_id: LiveId` (a lightweight `Copy` type), callers can extract the `LiveId` upfront, dispatch lifecycle events using `&Route`, and finally *move* ownership of the original `Route` into the `RouterAction` enum variant. This safely and idiomatically eliminates the `.clone()`, reducing memory allocations in the hot path.

## 2025-05-11 - Zero-allocation case-insensitive substring search
**Learning:** Calling `to_ascii_lowercase()` in a loop for case-insensitive `contains` checks (like `command.title.to_ascii_lowercase().contains(&query)`) forces heap allocations per iteration per string.
**Action:** Use `.as_bytes().windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle.as_bytes()))` to perform a zero-allocation case-insensitive substring search.

## $(date +%Y-%m-%d) - Avoiding Unnecessary Allocations during HashMap Insertions
**Learning:** Using `HashMap::entry(key.clone()).or_insert(...)` or `.or_default()` allocates a `String` unconditionally. This creates significant heap churn on a hot path if the keys frequently already exist in the map (e.g., when indexing shared paths in `RouteRegistry`).
**Action:** Use a check-then-insert approach, looking up with a borrowed slice (`contains_key` or `get_mut`), and only calling `clone()` to allocate the owned `String` when an insertion actually needs to happen.
