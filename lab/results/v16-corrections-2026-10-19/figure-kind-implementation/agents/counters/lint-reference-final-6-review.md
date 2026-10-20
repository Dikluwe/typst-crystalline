# Diagnostic differential inventory

Evidence-only draft for `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/lint-final-6/after.sarif.gz` with 63 manifested pairs. Manual causal reconciliation is required before a final conclusion.

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

Accepted for diagnostic integrity: frozen final6 has18 V16,1315 V19 and905 V20, with no new error or warning against the original baseline. All1606 outside-scope complete objects are exactly identical, including messages/levels/coordinates. All63 current source/owner pairs match the sealed inputs; changed_inputs=[] at2026-10-20T02:03:21.820969–02:03:27.289760Z. The partition contains one fewer outside diagnostic than the62 review because the declared invoke pair is now in scope; no diagnostic was lost.

Exactly original **V16-055** (Figure constructor inference) and incidental **V16-100** (pipeline.rs original529 substitution callback) are removed. The original inventory identifies V16-100 as batch G and separately identifies still-open **V16-099** (collector fallback at original488). Their exact diagnostic identities and original105 index hash remain in JSON. Final3's rejected ordering wildcard addition is preserved in its historical failed review; it does not reappear in final6 and is not hidden by count compensation.

Every added/removed note message and multiplicity is retained in the companion inventory:58 added V19,30 added V20;5 removed V19,2 removed V20. The prior final4 substantive reconciliation still applies to nominal Figure kinds, supplement maps, actual Ref eligibility/origin, explicit generic substitution complements and unchanged-node callbacks. Its full review remains preserved and hash-linked.

The only message-inventory change since final4 is the measured Block-original correction. The old117-alternative ordinary-original remainder loses Block and remains an explicit116-alternative remainder with the same delegated full-original behavior. The new `Table | Block` original-root-only decision has its own V19 note. This is a concrete actual-view decision: frozen language controls show hidden child Heading count0 versus product1 and identity child1 versus product2. It does not reclassify other containers as empty, weaken alias topology or claim every remainder alternative is dynamically exercised. The new shared root arm contains two explicitly named alternatives and remains visible to the linter.

The full added-note responsibility inventory from final4 remains authoritative with two more walk notes: one changed116-alternative remainder and the new two-alternative root-only group. SourceSpan argument mapping introduces no suppressed or downgraded diagnostic. Fixed actual error controls establish their measured source locations, not universal diagnostic coverage.

Runtime execution of one shared arm cannot prove all grouped alternatives. V19 still marks that uncertainty for each named Content/Value/Func/selector exclusion and delegation group. V20 retains the deeper nominal Option/kind/supplement/origin patterns. Real enum-extension compile probes and independently measured language tests complement these notes; neither erases them or substitutes for the other.

Global18 V16 and separate language debts remain. This review makes no whole-parity claim and does not certify the still-running workspace gates or close F. Historical55, failed final3 and accepted final4 reviews are unchanged.
