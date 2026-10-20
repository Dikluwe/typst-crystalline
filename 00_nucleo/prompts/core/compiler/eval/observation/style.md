# L0 — compiler/eval/observation/style
Hash do Código: 8dac7d35

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

## Captured raw style-chain observation

Measured 2026-10-19: StyleChain's private StyleNode stores delta plus parent
(entities/style_chain.rs:275), and this owner already compares all StyleDelta
fields at style.rs:11. RED8's actual own-hidden Figure query exposes recipes
whose captured styles require a storage proof; resolving them with collapse
would discard shadowed causal values and violate the observation nucleus.

Compare StyleChain through its crate-private borrowed raw_deltas traversal.
Preserve exact head-to-parent node order, length, empty-node presence, and every
StyleDelta causal slot through the existing full observer. Never collapse,
resolve inheritance, skip shadowed values, or accept shared Arc identity as
proof of an opaque slot. Fully comparable equal chains are Same, a proven
node/value/order/presence difference is Different, and otherwise opaque data
remains Unproven. Test independent equal chains, reversed nodes, empty nodes,
changed shadowed values, and a shared opaque dynamic value hidden by an inner
custom declaration. No change to global Func observation or query contracts.
