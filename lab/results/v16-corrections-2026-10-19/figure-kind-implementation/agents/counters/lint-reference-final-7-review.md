# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-7/after.sarif.gz` with 63 manifested pairs. Manual causal reconciliation is required before a final conclusion.

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

Accepted for diagnostic integrity: frozen final7 has18 V16,1315 V19 and905 V20, with no new error or warning against the original baseline. All1606 outside-scope complete diagnostic objects are exactly identical, including messages, levels and coordinates. All63 current source/owner pairs match the sealed snapshot; changed_inputs=[] during the recorded lint execution. The JSON binds the exact linter SHA, command, HEAD, dirty-file stat and UTC interval.

Exactly original **V16-055** (Figure inference) and incidental **V16-100** (pipeline substitution callback, batch G) are removed. **V16-099**, the distinct original collector fallback, remains open. The original105 inventory identity proof and the historical failed final3 review remain hash-linked; no aggregate-count compensation is accepted.

Every note occurrence remains inventoried:58 added V19,30 added V20,5 removed V19 and2 removed V20. The accepted final6 responsibility reconciliation is retained in the JSON. Final7 versus final6 has no source/rule/level/full-message/multiplicity change. Exactly13 complete diagnostic objects in walk.rs move14 lines earlier after the exact predicate-preserving common ChildrenOnly body factoring; all other complete objects remain identical. The consolidated Figure L0 changes no diagnostic inventory.

The Block decision remains explicit: original Table|Block roots are observed once, actual Block children are reached only under the existing same-type/same-Location guard, and the116-alternative ordinary-original remainder retains its prior full-original delegation. No other container is declared empty. V19 continues to disclose grouped-alternative coverage uncertainty; V20 retains nested nominal patterns. Actual language controls and enum-extension compile probes do not establish every grouped alternative dynamically.

Global18 V16 and bounded language debts remain, including V16-099, SetPage, full captioned default Block and ordinary Block own-label membership. The narrow captionless Block and actual-child corrections have separate measured controls. This review makes no global zero-lint, all-parity, F-closure or unfinished workspace-gate claim. All prior reviews are preserved.
