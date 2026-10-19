# L0 — compiler/eval/observation/page
Hash do Código: c801226a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/page.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/page.rs:11` — `impl ObservationEq for crate::entities::elements::page_run::PageRunElem {`.
- `01_core/src/compiler/eval/observation/page.rs:56` — `impl ObservationEq for crate::entities::page_canvas::PageBleedSpec {`.
- `01_core/src/compiler/eval/observation/page.rs:67` — `impl ObservationEq for crate::entities::page_running::PageNumberAlign {`.
- `01_core/src/compiler/eval/observation/page.rs:75` — `impl ObservationEq for crate::entities::page_running::PageNumberVAlign {`.

## Contract

Own observation of paper, dimensions, margins, binding, bleed, fill, supplement,
numbering, alignment, marginals, page runs, columns and break/parity carriers
by variant and causal fields. Do not paginate, realize headers/footers or
serialize output. Content's SetPage match is outside this owner and delegates
to the types covered here. Equal configurations are Same; covered differences
in presence, order or payload are Different.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
