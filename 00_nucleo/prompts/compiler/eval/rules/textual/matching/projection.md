# L0 — compiler/eval/rules/textual/matching/projection
Hash do Código: 5d053bcf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/matching/projection.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the working tree captured in `00_nucleo/diagnosticos/atomizacao-textual-projecao-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual/matching.rs:186-223` contains unmatched-range projection and flattening of replacement sequences. The existing `p1494` filter passed 17 tests after the space/matching movement, before this projection movement. These helpers use content and ranges only; engine, selector and style state are absent.

## Decision and behavior

Own reconstruction of an unmatched textual range from its original content nodes. Preserve complete nodes, including explicit and inferred spaces and style, when wholly included. Slice only partially included Text into a new Text node. Flatten replacement sequences recursively in their original order. Keep UTF-8 byte ranges supplied by the matching owner and do not change selector matching or error behavior. The matching owner calls these helpers; it retains rule precedence, guard state, engine restoration and transform invocation. No public signature, phase, default or compatibility changes.

The existing unmatched-range projection test belongs to this module. No dynamic dispatch or additional I/O is introduced.

## Acceptance

The projection test, existing `p1494` filter and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.
