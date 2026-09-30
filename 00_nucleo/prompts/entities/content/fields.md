# L0 — entities/content/fields
Hash do Código: bfc0858d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/fields.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own `Content::get_field` and its transparent styled-equation lookup. Preserve
field presence, omission, style precedence, and delegation to element owners.
Unsupported fields return `None`.

## Acceptance

Existing field-access and show-rule tests pass. The implementation remains a
closed static match and does not introduce a property registry.
