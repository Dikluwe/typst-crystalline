# L0 — entities/content/plain-text
Hash do Código: 4ce383c8

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/plain_text.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own recursive `Content::plain_text` projection. Preserve content order, textual
markers, transparent wrappers, and element-specific delegation without making
rendered bytes or Rust representation part of the language contract.

## Acceptance

Existing plain-text, representation, and morphology tests pass; the operation
remains pure and statically dispatched.
