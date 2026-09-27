# L0 — infra/pipeline/paged
Hash do Código: d6b0f8b1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/paged.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43
```

## Current evidence

- `03_infra/src/pipeline/paged.rs:188` — `compile_to_paged_document_full_error_and_features_with_introspector`.
- `03_infra/src/pipeline/paged.rs:686` — `bind_logical_page_numbers`.
- `03_infra/src/pipeline/paged.rs:712` — `realize_page_numberings`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/paged.rs`.

## Owned contract

- Own paged compilation, runtime introspection, outline realization, callback coordination, PageStore construction and final logical-number binding. Only complete validated candidates may reach shaping and export.
- Forward explicit features unchanged to evaluation; compatibility wrappers use the empty set. Features, target, PDF tagging and stream modes remain independent axes.
- Delegate selective contextual preparation to its child. A selected result already supplies final content, introspector, document and warnings: do not run a second contextual/page-numbering loop. Without selection, preserve the ordinary path and its errors.
- Realize outlines through L1 using each occurrence's lexical snapshot. After realization, discover newly produced context blocks with causal Locations and expand before layout. Remove only markers of already executed causal producers from the discovery view, not sibling results; do not replay old producers or identify them solely by numeric id.
- Coordinate independent math and `par.line` stores outside L1 layout. Pending requests are realized with their captured style/span and Engine, then replayed under the existing finite eight-attempt budget. Math angle requests receive the default Angle and must return Angle; line numbering receives exactly one integer and uses canonical `value_to_content`.
- New transcripts or partially consumed stores invalidate the candidate. Errors, invalid casts or exhausted callback budgets prevent export. Rebuild stores for every external relayout; never cache them across compilations or merge their budget with introspection fixpoints.
- Realize visible page numbering with `[current,total]`; realize the unary reference view `[current]` only for actual resolved page-reference targets. Preserve raw numbering objects separately from realized content and propagate callback diagnostics.
- The page-numbering cycle is bounded to five passes and checks page counts, logical snapshots, raw objects and realized content, not page count alone. Reconstruct from original input; never accumulate markers, callbacks or warnings.
- Preserve zero-size causal context markers for layout position sealing. Convert layout-warning strings to detached `SourceDiagnostic::warning` in L3 and retain them through errors.
- Associate L1-collected format options with each complete candidate. Return its realized PageStore, not a pre-layout clone; inject final positions after shaping. Bind every page's logical number from the aligned final store, failing on mismatch.
- Keep semantic wrappers and all page layers intact. Resource collection and shaping must include background, items and foreground. HTML does not enter this global paged path.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
