# L0 — compiler/eval/observation/geometry
Hash do Código: bec7b6b0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/geometry.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/geometry.rs:11` — `impl ObservationEq for crate::entities::layout_types::Align2D {`.
- `01_core/src/compiler/eval/observation/geometry.rs:19` — `impl ObservationEq for crate::entities::layout_types::HAlign {`.
- `01_core/src/compiler/eval/observation/geometry.rs:45` — `impl ObservationEq for crate::entities::layout_types::VAlign {`.
- `01_core/src/compiler/eval/observation/geometry.rs:63` — `impl ObservationEq for crate::entities::color::SpotColorant {`.

## Contract

Own observation of geometry, alignment, relative/absolute measures, colors,
paints, strokes, paths, gradients and tilings by variant and causal fields.
Do not rasterize, normalize units, resolve layout or replace language equality.
Delegate IEEE-bit float comparison to the observation hub. Early and late
covered differences produce Different without rendering.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
