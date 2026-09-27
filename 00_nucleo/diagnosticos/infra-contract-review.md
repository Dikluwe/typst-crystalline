# Infra contract and implementation review

This review corrects six demonstrated defects in non-export infrastructure through five regression tests. It extends the preceding shaper review, without establishing that all infrastructure is correct or that these defects explain the measured vanilla performance gap. No new performance measurements were taken.

## Provenance and boundaries

HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, with existing uncommitted changes. Review started at the UTC timestamp in `infra-contract-review/provenance.json`; completion time and toolchain are in `completion-audit.json`. The exact incoming working-tree status and tracked patch are preserved, alongside before-images of each edited file. `change.patch` isolates this task from those incoming changes; `working-tree-after.patch`, `status-after.txt` and `diff-stat.txt` describe the resulting tree.

The earlier exclusion of `infra/export/` was retained while a scope clarification remained unanswered. Export source and prompts were not edited. No L1/L2/L4, ADR, shared nucleus, benchmark or global hash repair was changed. Formatting was limited to the six modified Rust files, with `skip_children=true`. Existing shaper corrections and ten preceding sealed datasets/reports were preserved and checked by SHA-256. The current consumer/input fingerprint differs from the preceding shaper record only in the twelve files listed below. Tracked patch sections outside this task's scope are byte-identical to their incoming versions.

The repository instructions and ADR-0107/0108/0127/0129 govern this work. Each owned L0 was updated before its correction; RED evidence preceded production changes. These are internal correctness repairs to existing contracts, not new public methods, defaults, pipeline phases or compatibility modes. Hashes were updated only for the six owner/consumer pairs, including the existing effective nucleus pin of the fallback owner.

## Evidence before conclusions

| Defect | Discriminating observation before correction | Correction and owner |
|---|---|---|
| Math mode absent from shaped-width cache identity | A shared metrics instance reused a prose width for a math fallback chain, differing from a fresh instance. Both request orders are checked. | Include explicit math mode in `ShapedWidthKey`; `infra/font_metrics/caches.md`. |
| Explicit measurement size absent from advance cache identity | A 24pt request with an unchanged 12pt style reused the cached 12pt width: `16.164pt` instead of `32.328pt`. | Pass the actual size to the private advance cache helper/key; caches owner and `infra/font_metrics/fallback.md`. |
| Shaped advance ignored explicit measurement size | Independent fresh instances returned `14.82pt` at both sizes, rather than `29.64pt` at 24pt. This RED was obtained after correcting the separate advance-cache defect. | Supply a local effective style with the requested size to both shaped caching and shaping, preserving the caller's style. Borrow the original style when the size bits already match; fallback owner. |
| File identity publication raced | Concurrent registration of one path produced different FileIds. | Recheck under allocation synchronization and publish the path/ID/slot association while holding the identity lock; `infra/system-world/cache.md`. |
| Malformed JPEG segment metadata panicked | APP0/APP1 lengths zero or one produced an inverted slice instead of an absent metadata result. | Reject lengths below two before slicing in JFIF DPI, EXIF DPI and orientation scans; `infra/image-sizer.md`. |
| Embedded slots accepted an invalid face index | An embedded single-face font with `index = 1` before its first load returned a font. | Validate the requested source index before collection reconstruction/index-zero parsing; `infra/fonts.md`. |

The numeric widths above are regression outputs from the exact working states represented by the recorded RED logs, before-images and sequential changes; they are not timing measurements or upstream parity assertions. The authoritative artifacts are `size-red.stdout` and `shaped-size-red.stdout`. The remaining RED evidence is in `red.stdout` and `fonts-red.stdout`.

The math-cache fixture uses real embedded Libertinus Serif, DejaVu Sans Mono and New Computer Modern Math bytes. It selects a codepoint absent from the primary face and with distinct fallback advances, then uses a synthetic catalog with controlled names and flags to isolate the explicit math chain from global similarity scoring. Earlier nondiscriminating preparations are retained under `preparation/`, not counted as positive evidence. The explicit-size control uses embedded Libertinus Serif and tests fresh/shared instances in both orders, shaped and unshaped. The concurrency control uses a barrier and repeated same-path registration without sleeps or filesystem writes; it tests an invariant, not a schedule or throughput claim. The embedded-index control changes the index before the first load and preserves existing OnceLock idempotence afterward.

## Prompt versus implementation

The font-size interface already specified an explicit per-call size to support mixed sizes; the implementation violated that contract in both paths. Invalid face indices and malformed metadata already had failure-return contracts, but the consumers had gaps. Canonical file identity was intended, but publication was not synchronized as one association. The shaped cache specification mirrored an incomplete key and omitted the explicit math dependency. The repaired L0s make these invariants reviewable at their owners rather than duplicating them in hubs. These findings do not justify blaming all defects on either prompts or code alone.

## Files changed

- `00_nucleo/prompts/infra/font_metrics/caches.md` and `03_infra/src/font_metrics/caches.rs`.
- `00_nucleo/prompts/infra/font_metrics/fallback.md` and `03_infra/src/font_metrics/fallback.rs`.
- `00_nucleo/prompts/infra/system-world/cache.md` and `03_infra/src/world/cache.rs`.
- `00_nucleo/prompts/infra/system-world/tests.md` and `03_infra/src/world/tests.rs`.
- `00_nucleo/prompts/infra/image-sizer.md` and `03_infra/src/image_sizer.rs`.
- `00_nucleo/prompts/infra/fonts.md` and `03_infra/src/fonts.rs`.
- This report and the new `infra-contract-review/` evidence directory.

## Review coverage and preserved contracts

`review-inventory.json` maps non-export Rust consumers to their actual L0 owners and distinguishes corrected units, preceding sealed shaper work, contract/consumer inspection, ownership-only inventory and partial review. It is not a claim of exhaustive line-by-line or upstream behavioral verification. Font metrics, font loading/catalogs, world identity/cache/paths/packages, image metadata, plugin host, resources and pipeline contracts were inspected; larger stabilization/bundle contracts need deeper discriminating cases. Watch/project initialization have shell-owned prompts despite their L3 consumer locations. The infra-named bibliography specification owns a pure L1 parser and was not treated as permission to change L1.

Preserved behavior includes instance-local face and width caches, lazy loading, successful byte sharing, source-error memoization versus byte-error retry, canonicalization when available, lexical root/package policies, font catalog ordering and coverage, fallback selection, variation handling, math/ssty/TextItem predicates, tracking bypass, caller-style immutability, source index normalization and collection support boundaries, image density/orientation precedence, plugin store/snapshot policies, existing target/options/diagnostics and pipeline composition. I/O remains in L3; L4 composition boundaries were not changed. No hub contract was broadened to take over a child's behavior.

## Validation

- `cargo test -p typst-infra --lib infra_review --offline -- --test-threads=1`: **5 passed**, no failures, after all behavioral corrections and scoped formatting (`final-green.stdout`).
- `cargo test -p typst-infra --lib --offline -- --test-threads=1 --skip export::`: **614 passed**, no failures (`non-export-suite.stdout`). Some integration tests still exercise unchanged exporters transitively; the filter excludes tests named under `export::`, not every export call.
- `cargo build --offline`: passed with existing warnings (`build.stderr`).
- `crystalline-lint . --format sarif`: no errors and no diagnostics on the edited pairs. Global warnings remain under V5/V12/V16/V17/V18; this is not a zero-violations claim. The final scoped metadata refresh is covered by the final SARIF run.
- The unfiltered infra library suite was interrupted with SIGTERM during `export::p1431_line_cap_tests::p1431_raster_mapeia_butt_round_square_e_auto_square`, after several minutes of sustained CPU use. Its partial logs are retained. It is not reported as a passed full suite, and the interruption does not establish the cause of the raster test's duration.

Tests/builds used the development/test profile and served correctness verification only. No release-speed or performance benefit is inferred. Raw commands, toolchain, snapshots, source hashes, logs, lint results and scope preservation checks are retained in the evidence directory. `MANIFEST.sha256` seals the evidence; `completion-audit.json` also seals this report.

## Remaining questions and controlled follow-ups

- Revisit export only under an explicit expanded scope, including an isolated, bounded investigation of the slow raster regression. Preserve its current partial evidence.
- Retained shaper questions remain: explicit language configuration in the Unicode buffer, script/math shaping identity, glyph confirmation versus coverage, and the documented cached-face mechanism. Passing current probes does not resolve nondiscriminating cases.
- JPEG/PNG/TIFF zero-density and TIFF rational/signed-rational conversion differences remain documented. Use table-driven malformed and signed-value fixtures plus pinned upstream probes before changing the policy; this patch only closes the demonstrated JPEG slicing panic.
- Font instantiation resource collection may invoke the instancer when requested axes do not match a variable face. Compare identical documents under matched versus unknown axes while recording actual instantiations and output semantics; no optimization or cause claim is made here.
- World FileId exhaustion/wrapping and collection support branches preserve existing contracts. They need separate boundary tests and classification before a compatibility change.
- Pipeline stabilization/bundle lifetime, observations and generation boundaries need tests that distinguish fresh execution from retained contributions; a broad passing suite is insufficient proof of complete reuse correctness.
- Re-measure the performance baseline with stable release code, equivalent inputs/fonts/options and compatible phase intervals to assess any speed effect. Do not add widths, microbenchmarks or test durations to explain total compilation time. These repairs alone do not establish the cause of the vanilla gap.

This closes the demonstrated repairs and their scoped validation, not global infra sanitization or performance attribution.
