# L0 — infra/pipeline/tests
Hash do Código: c3fbebaf

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/tests.rs`

## Current evidence

- `03_infra/src/pipeline/tests.rs:18` — `p1247_contexto_svg_filtra_pagina_e_posicao_homologa`.
- `03_infra/src/pipeline/tests.rs:41` — `MockWorld`.
- `03_infra/src/pipeline/tests.rs:47` — `library`.
- `03_infra/src/pipeline/tests.rs:50` — `book`.
- `03_infra/src/pipeline/tests.rs:53` — `main`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/tests.rs`.

## Owned contract

- Own only the integrated pipeline suite and fixtures, using the existing public facades and descendant helpers. Tests do not authorize production behavior or a new pipeline phase.
- Preserve integrated format precedence, feature separation, contextual snapshots, callbacks, bundle, resources and diagnostics oracles. Use structure, text, DOM, dimensions and positions rather than complete serialized byte hashes.
- Characterize fractional boxes on finite lines: suffix position for 1fr, proportional positive shares, zero share, isolated negative share and safe zero-sum/nonfinite ratios. Preserve repeat ceiling 1000, link scope and strictly-more-copies-at-greater-width relation.
- Keep repeat controls with/without prefix at equal width, distinct sibling 1fr repeats/suffixes, a later block body and nested boxes. Observe paged output, not private repeat/fr indices; do not adapt expectations to the candidate.

Verify that resource collection and resolution for a paged document containing only shapes succeed with a World whose catalog and font accessors reject any access. Construct layout before applying the rejecting World: shape layout can legitimately require baseline and cap-height metrics. This checks export resource demand independently of installed host fonts or wall-clock timing.

Verify with the real Ubuntu Sans variable fixture that unknown-axis-only resource requests retain source bytes even with an unavailable configured Python. Keep a matching `wght` control: it must still fail with the existing dependency diagnostic when the tool is absent. Do not mutate process-global environment in tests; the isolated test command supplies the missing interpreter.

## Verification and boundary

Run `cargo test -p typst-infra --lib pipeline::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
