## 2026-04-23 - `replace` beat manual `+` decoding
**Learning:** In `makepad_router` query decoding, a hand-rolled single-scan `+` fast path looked cheaper on paper but regressed the release benchmark versus the existing `String::replace` branch. The extra per-byte `push` work outweighed the saved scans.
**Action:** For short router query strings, benchmark standard-library string transforms before replacing them with manual byte loops. Keep the benchmark harness and revert quickly when the numbers move the wrong way.

## 2025-05-08 – Eliminating Redundant Route Cloning in Makepad Router Action Dispatch
**Learning:** In the Makepad router widgets library (`makepad-router-widgets`), `queue_route_actions` previously accepted a full `&Route` reference to extract the ID, while callers simultaneously created `RouterAction` enum variants (like `Navigate`, `Replace`, `Reset`) by calling `.clone()` on the `Route`. Because `Route` contains nested objects (like `String` identifiers and `HashMap` query parameters), this `.clone()` caused significant heap allocation and churn during every route transition.
**Action:** By modifying `queue_route_actions` to strictly accept `new_route_id: LiveId` (a lightweight `Copy` type), callers can extract the `LiveId` upfront, dispatch lifecycle events using `&Route`, and finally *move* ownership of the original `Route` into the `RouterAction` enum variant. This safely and idiomatically eliminates the `.clone()`, reducing memory allocations in the hot path.

## 2025-05-11 - Zero-allocation case-insensitive substring search
**Learning:** Calling `to_ascii_lowercase()` in a loop for case-insensitive `contains` checks (like `command.title.to_ascii_lowercase().contains(&query)`) forces heap allocations per iteration per string.
**Action:** Use `.as_bytes().windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle.as_bytes()))` to perform a zero-allocation case-insensitive substring search.
## 2024-05-14 - Zero-Allocation Case-Insensitive String Search
**Learning:** In UI search filtering loops, standard `.to_ascii_lowercase()` calls on strings inside `for` loops cause rapid and excessive heap allocations (creating new Strings). A case-insensitive comparison using `String::contains()` after a `.to_ascii_lowercase()` acts as a major bottleneck in frame rendering.
**Action:** Extract reusable zero-allocation string search helpers like `contains_ignore_ascii_case` to shared modules, leveraging byte slices (`haystack.as_bytes().windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle.as_bytes()))`) to prevent allocation. Eliminate `Vec<String>` caches meant merely for holding lowercased string data.
