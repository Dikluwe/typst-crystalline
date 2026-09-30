# L0 — compiler/eval/observation/elements
Hash do Código: db1012b4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/elements.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/elements.rs:11` — `impl ObservationEq for crate::entities::elements::align::AlignElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:19` — `impl ObservationEq for crate::entities::elements::underline::UnderlineElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:29` — `impl ObservationEq for crate::entities::elements::v_space::VSpaceElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:38` — `impl ObservationEq for crate::entities::elements::enum_item::EnumItemElem {`.

## Contract

Own observation of the remaining non-math, non-page element carriers. Compare
variant identity and every causal field, including BlockHeight and ImageElem.alt.
Distinguish absent alt, an empty string and nonempty text. Do not compare
derived frames, execute callbacks/layout or substitute textual names for
identity. Dynamic elements without immutability guarantees remain Unproven.
Nominally exhaustive destructuring must prevent silent field omissions.

BlockHeight compares Relative/Fractional nominal variants and their payloads;
different variants are Different without resolving a reference or allocation.
Image alt remains distinct independently of identical image bytes/geometry.
Boxed observation includes its fractional width carrier.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
