# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-11/after.sarif.gz` with 63 manifested pairs. Manual causal reconciliation is required before a final conclusion.

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

Accepted for diagnostic integrity: frozen final11 has18 V16,1315 V19 and905 V20. No new error or warning appears versus original baseline. All1606 outside-scope complete diagnostic objects remain exactly identical, including coordinates. All63 current source/owner pairs match the reseal14 snapshot; changed_inputs=[] during the recorded UTC interval. Exact source state, command, linter SHA and provenance are bound in JSON.

Exactly original **V16-055** and incidental **V16-100** are removed; the separate collector **V16-099** remains open. Original105 identity evidence, the historical rejected final3 wildcard and all accepted differential reviews remain preserved and hash-linked.

Every58 added V19,30 added V20,5 removed V19 and2 removed V20 occurrence retains its full message, multiplicity and coordinates. The nominal identity, actual-view, exhaustive delegation and Block-original causal reconciliations remain authoritative through the linked history. Final11 versus final10 has zero complete diagnostic-object changes. The helper-local known-style/no-Location math context correction adds no diagnostic and hides no loss.

V19 shared-arm execution cannot prove each named alternative dynamically. V20 nested nominal patterns remain visible. Real-enum compile probes and actual language regressions complement these notes without removing runtime-coverage uncertainty.

Global18 V16 and separate language debts remain. In particular, the existing Tekt math script14 contract differs from the pinned target lexical20 observation and remains recorded independently; the preserved explicit Context unwrap is an existing Tekt extension. SetPage, full captioned default Block and ordinary Block own-label membership are also open. This review makes no zero-global-lint, complete-parity, F-closure or still-running final-gate claim.
