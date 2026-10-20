# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-2/after.sarif.gz` with 55 manifested pairs. Manual causal reconciliation is required before a final conclusion.

| Snapshot | Counts |
|---|---|
| Original baseline | {'V16:error': 20, 'V19:note': 1262, 'V20:note': 877} |
| Candidate | {'V16:error': 19, 'V19:note': 1306, 'V20:note': 904} |

Outside scope, exact complete diagnostic objects: 0 removed, 0 added. Snapshot changed inputs: []; current scope mismatches: ['00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md', '00_nucleo/prompts/core/compiler/eval/mod.md', '00_nucleo/prompts/core/compiler/eval/tests.md', '00_nucleo/prompts/core/compiler/introspect.md', '00_nucleo/prompts/core/compiler/introspect/extract_payload.md', '00_nucleo/prompts/core/compiler/introspect/figure_demands.md', '00_nucleo/prompts/core/compiler/introspect/labelled.md', '00_nucleo/prompts/core/compiler/introspect/locatable.md', '00_nucleo/prompts/core/compiler/introspect/walk.md', '00_nucleo/prompts/core/compiler/layout/references.md', '00_nucleo/prompts/core/compiler/layout/tests.md', '00_nucleo/prompts/core/compiler/realize/figure.md', '00_nucleo/prompts/core/compiler/realize/mod.md', '00_nucleo/prompts/core/compiler/realize/recipes.md', '00_nucleo/prompts/core/compiler/stdlib/figure_image.md', '00_nucleo/prompts/core/entities/content/traversal.md', '00_nucleo/prompts/infra/pipeline/context_stabilization.md', '01_core/src/compiler/eval/bindings/field_access/fields.rs', '01_core/src/compiler/eval/mod.rs', '01_core/src/compiler/eval/tests.rs', '01_core/src/compiler/introspect.rs', '01_core/src/compiler/introspect/convergence.rs', '01_core/src/compiler/introspect/extract_payload.rs', '01_core/src/compiler/introspect/figure_demands.rs', '01_core/src/compiler/introspect/labelled.rs', '01_core/src/compiler/introspect/locatable.rs', '01_core/src/compiler/introspect/walk.rs', '01_core/src/compiler/layout/tests.rs', '01_core/src/compiler/realize/figure.rs', '01_core/src/compiler/realize/mod.rs', '01_core/src/compiler/realize/recipes.rs', '01_core/src/compiler/stdlib/figure_image.rs', '01_core/src/entities/content/traversal.rs', '03_infra/src/pipeline/bundle.rs', '03_infra/src/pipeline/context_stabilization.rs', '03_infra/src/pipeline/paged.rs'].

## Removed message occurrences

- `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`: V19 × 1, V20 × 1
- `01_core/src/compiler/eval/selector_matching.rs`: V19 × 1
- `01_core/src/compiler/layout/figure.rs`: V20 × 1
- `01_core/src/compiler/realize/recipes.rs`: V19 × 1
- `01_core/src/compiler/stdlib/figure_image.rs`: V16 × 1

## Added message occurrences

- `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`: V19 × 2, V20 × 1
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
- `01_core/src/compiler/stdlib/figure_image.rs`: V19 × 5, V20 × 6
- `01_core/src/entities/content/traversal.rs`: V19 × 1, V20 × 2
- `01_core/src/entities/elements/figure.rs`: V19 × 10, V20 × 6
- `02_shell/src/cli.rs`: V19 × 1, V20 × 1

The companion JSON retains every full message, multiplicity and location, exact outside-scope objects and hashes. V19 runtime alternative coverage and V20 nested identity notes remain visible; no test can establish every alternative merely by executing a shared arm. No completion or clean-global-lint claim is made.
