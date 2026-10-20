# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-9/after.sarif.gz` with 63 manifested pairs. Manual causal reconciliation is required before a final conclusion.

| Snapshot | Counts |
|---|---|
| Original baseline | {'V16:error': 20, 'V19:note': 1262, 'V20:note': 877} |
| Candidate | {'V16:error': 18, 'V19:note': 1315, 'V20:note': 905} |

Outside scope, exact complete diagnostic objects: 0 removed, 0 added. Snapshot changed inputs: []; current scope mismatches: [].

## Removed message occurrences

- `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`: V19 × 1, V20 × 1
- `01_core/src/compiler/eval/context_reads.rs`: V19 × 1
- `01_core/src/compiler/eval/selector_matching.rs`: V19 × 1
- `01_core/src/compiler/introspect/walk.rs`: V19 × 1
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
- `01_core/src/compiler/introspect/walk.rs`: V19 × 2
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

## Completed independent causal review

Accepted for diagnostic integrity: frozen final9 has18 V16,1315 V19 and905 V20. No new error or warning appears against the original baseline. All1606 outside-scope complete diagnostic objects are exactly identical, including messages, levels and coordinates. All63 current source/owner pairs match the sealed snapshot; changed_inputs=[] during the recorded execution. Exact HEAD/dirty-file state, UTC, linter SHA and command remain in JSON.

Exactly original **V16-055** (Figure inference) and incidental **V16-100** (batch-G substitution callback) are removed. The distinct original collector **V16-099** remains open. Original105 identity proof and the historical rejected final3 addition remain preserved; no aggregate count compensation is accepted.

Every note occurrence remains inventoried:58 added V19,30 added V20,5 removed V19 and2 removed V20 against the original baseline. Final9 versus final8 has no message/rule/level/source/multiplicity change. Exactly139 existing core test diagnostics shift36 lines later and13 existing infra test diagnostics shift16 lines later; columns and all other object fields match. These are the phase-aware fixture migrations, with exact sources and semantic assertions preserved and focused3/3 GREEN separately bound. No productive helper, callback admission or diagnostic policy changed.

The previous nominal-identity, actual-view, closed-complement and Block-original note responsibility reviews remain linked in the companion history. V19 does not certify every grouped alternative merely because a shared arm executes; V20 nested nominal patterns remain visible. Enum-extension compile controls and measured language regressions complement these notes, without erasing runtime coverage uncertainty.

Global18 V16 and separate language debts remain. This review does not claim zero global lint, full parity, F closure or completion of the still-running final workspace gates. All earlier reviews and failed receipts remain preserved.
