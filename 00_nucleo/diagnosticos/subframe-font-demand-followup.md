# Remove unused cap-height demand in ordinary subframes

Image and shape block adapters previously requested cap-height before ordinary subframe positioning, even though that branch uses the local cursor directly. The correction avoids this unused query in ordinary subframes. Main-flow paths continue to acquire typography when required. Shapes still resolve cap-height if their actual extent triggers the existing page-transition branch; the query stays before new_page so it uses the original style. The subframe origin and shared rebase protocol remain unchanged.

Mandatory tests inject a FontMetrics provider that rejects typography. Both ordinary-subframe controls failed specifically at the cap-height query before implementation. After the correction the full layout suite passed: 938 tests, zero failures. The previous first-block/edge tests remain active. Build and global lint passed with zero error-level findings; preexisting warnings remain. Owned consumers have no V5/V6/V15/V26 findings. No public API, default mode, pipeline phase, global cache, formatting pass or global hash rewrite was added.

The two L0 owners were read and updated before implementation. Only these owner/consumer pairs changed:

- `00_nucleo/prompts/compiler/layout-image.md` ↔ `01_core/src/compiler/layout/image.rs`
- `00_nucleo/prompts/compiler/layout/shape_block_behaviour.md` ↔ `01_core/src/compiler/layout/shape.rs`

HEAD at preparation: `f1ff689fe` (full identity, exact dirty/untracked status, diff-stat and clock in provenance.json). before-sources.json fingerprints current consumers/prompts/benchmarks including untracked sources. before/ and change.patch preserve the exact scoped transition. The final audit verifies all inventoried files outside these four paths remain byte-identical. Scoped lineage hashes are recorded in scoped-seals.json; no older sealed evidence was rewritten.

Commands: `cargo test -p typst-core --lib ordinary_subframe_block_ -- --test-threads=1` for behavioral RED; `cargo test -p typst-core --lib compiler::layout:: -- --test-threads=1` for final validation; `cargo build`; `crystalline-lint . --format sarif`. Logs retain compiler output and test results. These are correctness checks, not timing measurements.

This correction removes unused accessor demand but **does not eliminate the full catalog's first acquisition in Place**. layout_sub_frame still establishes its typography-based origin and callers compensate it. Earlier six book accessor calls in the placed-shape control are not six scans. No new latency, post-correction accessor-count total or speedup is claimed. A zero-origin/lazy-origin redesign requires explicit text/non-text/nested/float/alignment geometry controls before changes to the shared coordinate convention.
