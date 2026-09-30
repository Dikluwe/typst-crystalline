# L0 — entities/content/morphology
Hash do Código: 05fe993b

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/morphology.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own `Content::morph_canon`, the canonical form used for language-level
morphology. Preserve ADR-0107: rendering mechanics and baked style details do
not become language identity. The implementation must preserve every
canonical form and equality outcome.

## Acceptance

Existing morphology and language-equality tests pass and the module contains
simultaneous verification.
