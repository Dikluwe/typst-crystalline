# L0 — compiler/eval/rules/textual/matching
Hash do Código: 44421380

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/matching.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the working tree captured in `00_nucleo/diagnosticos/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual.rs:96-333` contains realized group traversal, rule precedence, matching, transformation and projection of unmatched original ranges. The `p1494` filter passed 17 tests on that state. This is a structural baseline, not a timing or new vanilla measurement.

## Decision and behavior

Own the complete realized textual group algorithm, from grouping by causal snapshot and style boundary through match selection and replacement, delegating preservation of unmatched
content to `compiler/eval/rules/textual/matching/projection.md`. Search literal/regex over the largest eligible realized group of the same lexical snapshot and style chain, including separated Text/space and dynamic text nodes. Each recipe receives only its matched slice; preserve leftmost order, unmatched prefix/suffix, precedence, guard/revocation and depth limit. A replacement is not fed back into the same revoked rule. Empty regex matches remain rejected and literal '.' stays distinct from regex '.'.

While a ContentBlock is only a value, preserve raw Space/SpaceFromNewline. Its local textual show operates provisionally before normalization: replace only the matched slice, preserving unmatched separators for later insertion. Root realization normalizes the inserted value and applies its eligible snapshot. The combined local CJK show/normalization case and sequential-identical-rule override remain explicit unresolved parity obligations, not presumed successes.

The coordinator passes its existing inputs after optional normalization and early return when show is disabled. This module may access the parent snapshot and call the existing selector matcher/call dispatcher, but must not duplicate space normalization, alter the engine restoration order or introduce dynamic dispatch. The descendant projection owner preserves unmatched nodes and flattened
replacement sequences; its unmatched-slice test lives with that algorithm. No phase, public signature or default changes.

## Projection refinement evidence

At the working tree captured in
`00_nucleo/diagnosticos/atomizacao-textual-projecao-2026-09-30-before.json.gz`,
this consumer contains `project_original_range` and `append_flat_sequence` at
lines 189-223, followed by the matching transform. The slice projection test
passed in the `p1494` filter after the first movement. Their only input is
content and range/sequence structure, so projection can be a pure descendant
without access to engine state.

## Acceptance

The existing literal/regex slicing, lexical snapshot and guard tests and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.
