# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-3/after.sarif.gz` with 62 manifested pairs. Manual causal reconciliation is required before a final conclusion.

| Snapshot | Counts |
|---|---|
| Original baseline | {'V16:error': 20, 'V19:note': 1262, 'V20:note': 877} |
| Candidate | {'V16:error': 19, 'V19:note': 1313, 'V20:note': 905} |

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
- `03_infra/src/pipeline/context_stabilization.rs`: V16 × 1, V19 × 2

The companion JSON retains every full message, multiplicity and location, exact outside-scope objects and hashes. V19 runtime alternative coverage and V20 nested identity notes remain visible; no test can establish every alternative merely by executing a shared arm. No completion or clean-global-lint claim is made.

## Independent causal reconciliation of final3

This snapshot is **not accepted** as a no-new-error result. Its total19 V16 consists of two removed original errors (old Figure and original pipeline node-callback wildcard) plus one newly introduced ordering wildcard in context_stabilization.rs:129. The new error was identified by complete differential comparison and separately released for an exhaustive complement correction. A later sealed run must verify that correction; this inventory does not pre-certify it.

All1607 outside-scope complete objects are identical, including source locations and full messages. During this review all62 source/owner pairs matched final3 inputs, and the lint state records changed_inputs=[] (2026-10-20T01:26:43.900365–01:26:48.976371Z).

Every note removal is a concrete changed decision group or pattern: Figure leaves content_methods'98-alternative fallback (retained97 group); private actual reference-target eligibility joins context_reads'14-alternative group (retained15 group); concrete native matching reduces selector_matching's47 group; recipes'116 group excludes feature-owned Figure/Ref paths (retained114 group); layout Figure's Str binding changes spelling while retaining the nested-pattern note. The full removed messages and locations remain in JSON. No note is reclassified as an error-free test proof.

All55 added V19 and30 added V20 occurrences remain individually inventoried with multiplicity. Relative to historical final2, new reference work adds the actual eligibility decision note, one fewer generic recipes alternative, exact reference token Value exclusions, the explicit retained origin Args pattern, pipeline generic node/delegation complements, and shared Sequence/MathSequence and Par/SmallCaps child-form groups. Existing Figure kind, supplement and nominal identity notes remain as reconciled in the historical55 review. These changes describe explicit closed decisions and deeper nominal carriers; they do not certify runtime coverage of every shared alternative.

No warning-level diagnostic or suppression is introduced. The new V16 remains visible and must be removed by a causal closed-world source repair, not total-count compensation. The original pre-Ref55 review files remain historical and unchanged.
