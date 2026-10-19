# Independent structural witness review

PASS: all 23 selected candidate decisions emit a primary E0004 at their exact match header after enum evolution. All three isolated HAlign tuple classes also emit the expected primary E0004. The original 20 baseline decisions and all three baseline tuple probes have no primary E0004 at those headers.

Reviewed at 2026-10-19T16:28:50.068062+00:00; HEAD a9f03073809fa15ff11d86c532f2c803ee892e47; uncommitted working tree. The JSON records the exact diffstat, source hashes, archive/log hashes, measurement UTC, command, environment, full primary spans, and independently recomputed byte positions. No compiler/build/lint was run by this reviewer.

I reparsed raw cargo compiler-message records, independently located function and match-header anchors in the actual archived mutated sources, and excluded enum-definition and help-child spans. Every archived original source matches its input hash and snapshot base; candidate originals match the current repository. Frozen owner hashes match their respective snapshot bases; candidate frozen owners match the current files. All mutations are insert-only: one sentinel enum member in each declared domain, plus exactly one tuple-covering arm for each isolated tuple probe. There are no productive-source edits or suppression attributes in these mutations.

| Target | Source line | Baseline primary E0004 | Candidate primary E0004 |
|---|---|---:|---:|
| V16-029 | `01_core/src/compiler/layout/grid/borders.rs:43` | 0 | 1 |
| V16-030 | `01_core/src/compiler/layout/grid/cell_body.rs:72` | 0 | 1 |
| V16-031 | `01_core/src/compiler/layout/grid/measurement_cache.rs:42` | 0 | 1 |
| V16-032 | `01_core/src/compiler/layout/grid/measurement_cache.rs:182` | 0 | 1 |
| V16-033 | `01_core/src/compiler/layout/grid/tracks.rs:77` | 0 | 1 |
| V16-034 | `01_core/src/compiler/layout/grid/tracks.rs:406` | 0 | 1 |
| V16-035 | `01_core/src/compiler/layout/grid/tracks.rs:697` | 0 | 1 |
| V16-036 | `01_core/src/compiler/layout/grid_placement.rs:98` | 0 | 1 |
| V16-037 | `01_core/src/compiler/layout/helpers.rs:451` | 0 | 1 |
| V16-038 | `01_core/src/compiler/layout/image.rs:130` | 0 | 1 |
| V16-039 | `01_core/src/compiler/layout/line_commit.rs:706` | 0 | 1 |
| V16-040 | `01_core/src/compiler/layout/measurement.rs:278` | 0 | 1 |
| V16-041 | `01_core/src/compiler/layout/mod.rs:1227` | 0 | 1 |
| V16-042 | `01_core/src/compiler/layout/mod.rs:1384` | 0 | 1 |
| V16-043 | `01_core/src/compiler/layout/par_line_callbacks.rs:135` | 0 | 1 |
| V16-044 | `01_core/src/compiler/layout/place.rs:305` | 0 | 1 |
| V16-045 | `01_core/src/compiler/layout/references.rs:353` | 0 | 1 |
| V16-046 | `01_core/src/compiler/layout/text/style.rs:86` | 0 | 1 |
| V16-047 | `01_core/src/compiler/layout/text/style.rs:79` | 0 | 1 |
| V16-077 | `01_core/src/entities/elements/table_cell.rs:158` | 0 | 1 |
| V16-045-selector | `01_core/src/compiler/layout/references.rs:354` | new nested decision | 1 |
| V16-045-kind | `01_core/src/compiler/layout/references.rs:355` | new nested decision | 1 |
| V16-040-certificate | `01_core/src/compiler/layout/measurement.rs:55` | new nested decision | 1 |

Tuple isolation was checked against actual mutation bytes: new-old covers every new-right pair and leaves five new-left/old-right pairs; old-new covers every new-left pair and leaves five old-left/new-right pairs; new-new covers the two mixed classes and leaves only new/new. The raw compiler messages identify those exact residual classes. Current nominal HAlign coverage is the complete five-by-five product (two left cases, two right cases, six same-axis neutral cases, and fifteen Center/Start/End rows).

All eight probe builds intentionally exit 101. Unrelated exhaustive matches account for additional E0004s; these were not mistaken for target witnesses. No other compiler error code appears. Structural closure does not certify language behavior or parity; those remain the separate semantic/control/replay gates. The three newly introduced nested decisions have no baseline match and are not falsely included in the 20 baseline-absence claims.

Audit implementation: `agents/grid/review_structural.py`. Detailed reproducible evidence: `structural-witness-review.json`.
