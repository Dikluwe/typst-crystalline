# L0 — infra/pipeline
Hash do Código: 246d49ec

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
```

## Current evidence

- `03_infra/src/pipeline.rs:174` — `eval_expression_with_document_features`.
- `03_infra/src/pipeline.rs:244` — `eval_to_module_with_sink_target_features`.
- `03_infra/src/pipeline.rs:355` — `collect_context_blocks_into`.
- `03_infra/src/pipeline.rs:409` — `substitute_context_blocks`.
- `03_infra/src/pipeline.rs:438` — `Timings`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline.rs`.

## Owned contract

- Own the evaluation adapters, comemo setup (`Routines`, `Traced`, `Sink`, `Route`), causal context walkers, public expansion facades and `Timings` serialization. Return raw diagnostics; never format them or write output.
- Isolated expression evaluation must not manufacture markup content. Document-context evaluation compiles through the paged owner, evaluates against its final introspector, and appends expression warnings after compilation warnings without exporting.
- Keep compatibility defaults (`full_error=false`, empty features, Paged evaluation) and explicit target/features unchanged. Reject assets outside Bundle with `assets are only supported in the bundle target` and hint `try enabling the bundle target`.
- Discover context blocks through the exhaustive L1 content traversal, accumulating the effective ancestor `StyleChain`. Substitute through the same topology; delegate attempt coordination to its child rather than implement a second stabilization algorithm.
- Preserve public import paths through explicit reexports. Private helper access stays within this module family. Target compilation, resources and stabilization belong to the child owners listed below; this prompt does not own their bodies.
- Timing fields describe existing production phases, including shaping and subsetting. Do not add boundary callbacks, an observation ledger, extra replay or an instrumented default.

## Child owners

- `00_nucleo/prompts/infra/pipeline/bundle.md` owns only `03_infra/src/pipeline/bundle.rs`.
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md` owns only `03_infra/src/pipeline/context_stabilization.rs`.
- `00_nucleo/prompts/infra/pipeline/html.md` owns only `03_infra/src/pipeline/html.rs`.
- `00_nucleo/prompts/infra/pipeline/paged.md` owns only `03_infra/src/pipeline/paged.rs`.
- `00_nucleo/prompts/infra/pipeline/pdf.md` owns only `03_infra/src/pipeline/pdf.rs`.
- `00_nucleo/prompts/infra/pipeline/raster.md` owns only `03_infra/src/pipeline/raster.rs`.
- `00_nucleo/prompts/infra/pipeline/resources.md` owns only `03_infra/src/pipeline/resources.rs`.
- `00_nucleo/prompts/infra/pipeline/svg.md` owns only `03_infra/src/pipeline/svg.rs`.
- `00_nucleo/prompts/infra/pipeline/tests.md` owns only `03_infra/src/pipeline/tests.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
