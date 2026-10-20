# Frozen final lint differential review

Independent evidence-only review of `lint-final-1/after.sarif.gz` against the original Figure proposal baseline. Full exact messages/multiplicities/locations, input/receipt hashes and outside-scope equality proof are retained in `lint-final-review.json`. The preliminary `lint-notes-review.*` remains historical.

The final lint ran 2026-10-20T00:20:39.171564–00:20:45.720251Z at HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`, uncommitted working tree, with `changed_inputs=[]`. All 55 source/owner pairs still match that frozen run at review time.

| Check | Original baseline | Frozen final | Result |
|---|---:|---:|---|
| Global V16 errors | 20 | 19 | Exactly old Figure wildcard-neutral error removed; no new error |
| Global V19 notes | 1262 | 1306 | +47 added, −3 removed; every message retained in inventory |
| Global V20 notes | 877 | 904 | +29 added, −2 removed; every message retained in inventory |
| Outside-scope complete diagnostic objects | 1649 | 1649 | Exactly identical, including lines/columns/levels/messages |
| New warning-level diagnostics | 0 | 0 | None added |

## Every removal reconciled

- Old `compiler/stdlib/figure_image.rs:69` V16: the neutral inference wildcard is replaced by explicit approved semantic decisions. No error severity downgrade or new default suppresses it.
- Old `content_methods.rs:239` V19/V20: Figure leaves the 98-alternative field fallback; the 97-alternative remainder still emits both notes at final249. Explicit body/caption/kind/supplement dispatch adds V19.
- Old `selector_matching.rs:449` V19: 15 variants gain concrete native matching; the remaining32 still emit V19 at final516. The moved variants are HtmlElem, HtmlFrame, Title, ListItem, EnumItem, FigureCaption, Curve, Flush, Divider, TermItem, Document, Asset, PdfAttach, PdfArtifact and FootnoteEntry.
- Old `layout/figure.rs:30` V20: `Some(Value::Str(s))` becomes `Some(Value::Str(pattern))`, retaining V20 at final27.
- Old `realize/recipes.rs:62` V19: Figure moves into its actual-child realization owner; the exhaustive remainder still emits V19 at final65.

## Every added note inventoried

There are 76 added note occurrences (47 V19, 29 V20), five removed notes, and495 unchanged in-scope notes. The exact-message inventory covers repeated identical messages with multiplicity and preserves all source locations. Grouped exclusions, terminal arms and nested nominal patterns remain visible in final SARIF; no suppression or neutral wildcard replaces them.

| Source | Added notes | Review |
|---|---:|---|
| `01_core/src/compiler/eval/bindings/field_access/content_methods.rs` | 3 | Figure field dispatch extracted from the old grouped fallback; retained fallback is 97 instead of 98 alternatives. |
| `01_core/src/compiler/eval/observation/elements.rs` | 4 | Different FigureSupplement variants remain explicit Different observations; grouped mismatches are not evidence of testing every pair. |
| `01_core/src/compiler/eval/observation/style.rs` | 1 | Different raw StyleChain lengths explicitly return Different in either direction; all paired deltas retain strict per-value observation, including shadowed opaque Unproven values. |
| `01_core/src/compiler/eval/operators/equality.rs` | 5 | Nominal Func/String separation and recursive Figure-selector classification; grouped selectors retain their explicit owning recursion. |
| `01_core/src/compiler/eval/selector_matching.rs` | 1 | Fifteen Content variants moved from the false group into concrete native-constructor matching; remaining 32 alternatives still emit V19. |
| `01_core/src/compiler/introspect.rs` | 3 | Nested Option/FigureKind assertion in regression fixtures, not a new production default. |
| `01_core/src/compiler/introspect/figure_demands.rs` | 3 | Exhaustive actual-view collector groups structurally shared children and explicit opaque/terminal variants; runtime does not certify every grouped variant. |
| `01_core/src/compiler/introspect/labelled.rs` | 3 | Explicit legacy/synthetic FigureKind Name versus Element-or-absent projection; no productive typed counter string shadow. |
| `01_core/src/compiler/introspect/materialize_time.rs` | 3 | Content-only FigureSupplement mapping; Auto/None/Func/absent remain opaque and callbacks are not executed by the map. |
| `01_core/src/compiler/layout/figure.rs` | 7 | Numbering style gates, rich Content prefix absence and explicit unresolved synthetic FigureKind boundary; deep nominal patterns remain visible as notes. |
| `01_core/src/compiler/layout/references.rs` | 1 | Rich reference supplement empty/absent compose the same number-only result; no neutral missing-history fallback introduced. |
| `01_core/src/compiler/layout/tests.rs` | 2 | Nested Option/FigureKind assertion in regression fixtures, not a new production default. |
| `01_core/src/compiler/realize/figure_kind.rs` | 7 | Closed semantic search catalogue and explicit Type/Value exclusions, shared child forms and Grid header/footer ordering; compile exhaustiveness is not runtime alternative coverage. |
| `01_core/src/compiler/realize/recipes.rs` | 1 | Figure removed from the generic child fallback and dispatched to its owner-specific actual emitted children realization. The large remainder still emits V19; no wildcard/suppression replaces it. |
| `01_core/src/compiler/stdlib/figure_image.rs` | 11 | Explicit constructor Value/FigureKind/FigureSupplement validation groups, preserving actual error paths and Function/Content distinctions. |
| `01_core/src/entities/content/traversal.rs` | 3 | FigureSupplement Content is mapped structurally; Auto/None/Func/absent are not interpreted or executed. |
| `01_core/src/entities/elements/figure.rs` | 16 | Nominal FuncRepr cross-kind false cases, pointer-kind hashing, Content-only maps and typed field projections; enum identity depth is deliberate, not suppressed. |
| `02_shell/src/cli.rs` | 2 | Query serialization recognizes both Func and String Figure kind language values without collapsing their identity. |

## Honest limits

V19 remains a warning about unproven runtime alternative coverage: one execution of a shared arm does not exercise every Content, Value, Type, FuncRepr or supplement mismatch. Real-enum extension probes certify structural exhaustiveness; fixed runtime language controls complement them and cannot certify every grouped alternative. V20 reports explicit nested Option/FigureKind/FigureSupplement identity patterns, including test assertions; the review does not hide them to lower a note count.

No hidden diagnostic loss is evidenced by this complete differential. This does not assert a clean global lint or close all Figure functionality:19 pre-existing V16 errors remain, and the separate rich-reference phase still requires its own approval and executable acceptance.
