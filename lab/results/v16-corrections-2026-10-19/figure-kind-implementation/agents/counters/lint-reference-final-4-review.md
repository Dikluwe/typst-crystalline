# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-4/after.sarif.gz` with 62 manifested pairs. Manual causal reconciliation is required before a final conclusion.

| Snapshot | Counts |
|---|---|
| Original baseline | {'V16:error': 20, 'V19:note': 1262, 'V20:note': 877} |
| Candidate | {'V16:error': 18, 'V19:note': 1314, 'V20:note': 905} |

Outside scope, exact complete diagnostic objects: 0 removed, 0 added. Snapshot changed inputs: []; current scope mismatches: [].

## Removed message occurrences

- `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`: V19 × 1, V20 × 1
- `01_core/src/compiler/eval/context_reads.rs`: V19 × 1
- `01_core/src/compiler/eval/selector_matching.rs`: V19 × 1
- `01_core/src/compiler/layout/figure.rs`: V20 × 1
- `01_core/src/compiler/realize/recipes.rs`: V19 × 1
- `01_core/src/compiler/stdlib/figure_image.rs`: V16 × 1
- `03_infra/src/pipeline.rs`: V16 × 1

## Added message occurrences

- `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`: V19 × 2, V20 × 1
- `01_core/src/compiler/eval/context_reads.rs`: V19 × 1
- `01_core/src/compiler/eval/observation/elements.rs`: V19 × 4
- `01_core/src/compiler/eval/observation/style.rs`: V19 × 1
- `01_core/src/compiler/eval/operators/equality.rs`: V19 × 5
- `01_core/src/compiler/eval/selector_matching.rs`: V19 × 1
- `01_core/src/compiler/introspect.rs`: V20 × 3
- `01_core/src/compiler/introspect/figure_demands.rs`: V19 × 3
- `01_core/src/compiler/introspect/labelled.rs`: V19 × 1, V20 × 2
- `01_core/src/compiler/introspect/materialize_time.rs`: V19 × 1, V20 × 2
- `01_core/src/compiler/layout/figure.rs`: V19 × 3, V20 × 4
- `01_core/src/compiler/layout/references.rs`: V19 × 1
- `01_core/src/compiler/layout/tests.rs`: V20 × 2
- `01_core/src/compiler/realize/figure_kind.rs`: V19 × 7
- `01_core/src/compiler/realize/recipes.rs`: V19 × 1
- `01_core/src/compiler/realize/reference_supplements.rs`: V19 × 1, V20 × 1
- `01_core/src/compiler/stdlib/figure_image.rs`: V19 × 5, V20 × 6
- `01_core/src/entities/content/traversal.rs`: V19 × 1, V20 × 2
- `01_core/src/entities/elements/figure.rs`: V19 × 10, V20 × 6
- `02_shell/src/cli.rs`: V19 × 1, V20 × 1
- `03_infra/src/pipeline.rs`: V19 × 4
- `03_infra/src/pipeline/context_stabilization.rs`: V19 × 3

The companion JSON retains every full message, multiplicity and location, exact outside-scope objects and hashes. V19 runtime alternative coverage and V20 nested identity notes remain visible; no test can establish every alternative merely by executing a shared arm. No completion or clean-global-lint claim is made.

## Completed independent review

Accepted for diagnostic integrity at this frozen snapshot: exactly two original V16 errors removed, no new error or warning. All1607 outside-scope complete objects remain identical. All62 source/owner pairs matched the sealed inputs, and changed_inputs=[] at 2026-10-20T01:29:14.778279–01:29:20.405351Z. Global18 V16 remain; this is not zero-violation or F closure.

The exact V16 identities matter. Original `stdlib/figure_image.rs:69` is replaced by the approved explicit kind/constructor decisions. Original `pipeline.rs:529` is the generic substitution callback's `_=>None`, now the explicit118-form unchanged-node complement; descendants still use ordinary exhaustive generic mapping. It is distinct from the still-open collector wildcard nearby, which this scope did not erase. Final3's newly introduced `context_stabilization.rs:129` wildcard-false generic-delegation decision was correctly rejected despite the same total19. Final4 removes that exact new error through a closed complement and retains its grouped-alternative V19. The failed final3 inventory is preserved in lint-reference-final-review.*, not overwritten.

Every note removal is accounted: content_methods'98-group loses Figure, retaining97; context_reads'14-group gains the actual private reference-target eligibility read, retaining15; selector_matching's47-group loses concretely matched native variants, retaining its remainder; recipes'116-group loses feature-owned Figure/Ref paths, retaining114; layout Figure's nested Str binding changes spelling but remains explicitly nested. Exact full old/new messages, repeated occurrence counts and all locations are in JSON.

All86 added notes (56 V19,30 V20) are visible and inventoried. Their responsibilities are:

| Source responsibility | Added V19 | Added V20 | Causal review |
|---|---:|---:|---|
| Figure field access | 2 | 1 | Concrete Figure fields leave grouped fallback; nominal Option patterns remain. |
| Context read replay | 1 | 0 | Actual reference-target eligibility joins explicit read classification. |
| Element observation | 4 | 0 | Distinct supplement forms retain explicit mismatch observations. |
| Style observation | 1 | 0 | Different raw chain lengths remain observable; opaque values stay unproven. |
| Selector/value equality | 5 | 0 | Nominal Func/String Figure kinds and owning recursive selector comparisons. |
| Native selector matching | 1 | 0 | Concrete native constructors leave the remaining explicit exclusion group. |
| Introspection fixtures | 0 | 3 | Assertions retain deep Option/FigureKind structure. |
| Figure demand traversal | 3 | 0 | Actual emitted-child and terminal/opaque catalogue groups remain explicit. |
| Label projections | 1 | 2 | Typed kinds versus synthetic name/absence remain separate. |
| Time materialization | 1 | 2 | Content supplements map; callback/Auto/None stay opaque. |
| Figure layout | 3 | 4 | Prepared history, numbering and rich-prefix boundary forms stay visible. |
| Reference layout | 1 | 0 | Explicit empty/absent rich supplement composition. |
| Layout fixtures | 0 | 2 | Deep nominal assertions retain their original guarantee. |
| Figure kind semantics | 7 | 0 | Closed search/value/type exclusions and shared child forms. |
| Recipe realization | 1 | 0 | Reduced generic group delegates feature-owned Figure/Ref behavior. |
| Reference producer | 1 | 1 | Exact private token/value exclusions and captured origin Args form. |
| Figure constructors | 5 | 6 | Explicit argument/error partitions and nominal kinds/supplements. |
| Generic Content traversal | 1 | 2 | Content supplement mapping is explicit; opaque forms stay distinct. |
| Figure entity identity/maps | 10 | 6 | Nominal function identity and Content-only mapping/projection. |
| CLI Figure serialization | 1 | 1 | Func and String kinds remain distinct language values. |
| Pipeline substitution | 4 | 0 | Actual shared child forms plus two closed generic delegation/node complements. |
| Producer ordering | 3 | 0 | Shared child forms plus the closed generic-delegation complement replacing final3's rejected error. |

Executing one shared arm cannot prove that every alternative in it has run. Runtime controls certify their measured language cases; enum-extension probes certify structural completeness. V19 continues to mark the remaining runtime-coverage uncertainty for each grouped pattern, including the96/118-form delegation complements. V20 continues to expose nested identity/origin forms. This review records those limits and does not translate a successful control into universal alternative coverage.

The historical55 reviews and failed final3 review remain preserved. The generated default Figure Block gap, SetPage gap and pending workspace validation are separate obligations; no language-parity claim follows from this lint comparison.

Original105 inventory mapping is now proven: the incidental callback removal is **V16-100, batch G**, indexed in `v16-cause-analysis-2026-10-19/case-index.md:106` and analyzed at `entities-boundaries-analysis.md:705–727`. The still-open collector is **V16-099** (index105, original pipeline.rs488). The selected Figure correction remains V16-055. Exact index hash and mappings are retained in JSON; no active progress ledger was edited by this review.
