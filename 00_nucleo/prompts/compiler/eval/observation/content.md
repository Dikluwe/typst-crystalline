# L0 — compiler/eval/observation/content
Hash do Código: 80e35408

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/content.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/content.rs:12` — `impl ObservationEq for crate::entities::content::Content {`.
- `01_core/src/compiler/eval/observation/content.rs:753` — `#[test]`.

## Contract

Own the exhaustive ObservationEq match for Content. Distinct variants are
Different; matching variants compare every causal carrier. Keep Space distinct
from SpaceFromNewline, explicit presence distinct from absence, and distinct
mathematical identities distinct. Do not execute show rules, layout, callbacks
or public language equality. New variants must require an explicit match arm.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
