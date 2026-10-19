# L0 — compiler/eval/observation/style
Hash do Código: cc216460

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:6f6d86b0ee72d314a84d86b12f9524b2959e075b29f99fcf6d2de9f30a4b338a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/style.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/style.rs:11` — `impl ObservationEq for crate::entities::style_chain::StyleDelta {`.
- `01_core/src/compiler/eval/observation/style.rs:62` — `impl ObservationEq for crate::entities::font_list::FontFamily {`.
- `01_core/src/compiler/eval/observation/style.rs:81` — `impl ObservationEq for crate::entities::font_list::FontNamePattern {`.
- `01_core/src/compiler/eval/observation/style.rs:99` — `impl ObservationEq for crate::entities::font_list::Covers {`.

## Contract

Own observation of styles, font lists/names, language, direction and associated
typographic carriers. Preserve order, presence, explicit values and deltas.
Do not resolve the cascade, font fallback or rendering. Do not invent derived
fields outside causal carriers. Equal styles are Same; a covered causal
difference is Different; opaque unproven state must not become Same.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
