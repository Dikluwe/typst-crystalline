# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-12/after.sarif.gz` with 64 manifested pairs. Manual causal reconciliation is required before a final conclusion.

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

Accepted for diagnostic integrity: frozen final12 has18 V16,1315 V19 and905 V20, with no new error or warning versus the original baseline. All1606 outside-scope complete objects are exactly identical, including coordinates. All64 current source/owner pairs match reseal15/current inputs; changed_inputs=[] during the recorded UTC interval. Exact source state, command, linter SHA and provenance remain in JSON. The newly manifested filtered_reads pair has no baseline/final diagnostics, so the outside partition remains1606.

Exactly original **V16-055** and incidental **V16-100** are removed; the distinct original collector **V16-099** remains open. Original105 identity proof, rejected final3 wildcard addition and all preceding accepted reviews are preserved and hash-linked.

Every58 added V19,30 added V20,5 removed V19 and2 removed V20 occurrence remains inventoried in full. Final12 versus final11 has zero complete diagnostic-object changes. The new private modern-final capability guard and six local controls therefore introduce no diagnostic loss or hidden count compensation. The guard retains filtered demand marking before error and prevents delegation/fold effects without known context plus actual Location; the public no-Engine materialized helper remains separate and unchanged. Its focused language proofs remain separate receipts.

The existing nominal identity, actual-view, explicit delegation and Block-original note responsibility reviews remain linked. V19 grouped-arm execution cannot prove every named alternative dynamically; V20 nested nominal patterns stay visible. Real-enum compile controls and measured language tests complement these notes without removing runtime-coverage uncertainty.

Global18 V16 and bounded language debts remain. This review does not certify every introspection capability, full math callback parity, SetPage, full captioned default Block, ordinary Block label membership, global zero lint, F closure or the outcome of the still-running final workspace gates.
