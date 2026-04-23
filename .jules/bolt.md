## 2026-04-23 - `replace` beat manual `+` decoding
**Learning:** In `makepad_router` query decoding, a hand-rolled single-scan `+` fast path looked cheaper on paper but regressed the release benchmark versus the existing `String::replace` branch. The extra per-byte `push` work outweighed the saved scans.
**Action:** For short router query strings, benchmark standard-library string transforms before replacing them with manual byte loops. Keep the benchmark harness and revert quickly when the numbers move the wrong way.
