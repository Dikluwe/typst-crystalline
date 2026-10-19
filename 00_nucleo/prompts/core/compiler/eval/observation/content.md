# L0 — compiler/eval/observation/content
Hash do Código: 18356a47

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:6f6d86b0ee72d314a84d86b12f9524b2959e075b29f99fcf6d2de9f30a4b338a


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


## Single-evaluation causal observation

The observation relation includes Location and origin/product, independently
of language equality. A recipe not proven causally equal returns Unproven and
must not suppress contextual re-execution.

## Ratified transparent source-origin boundary

Prior evidence is
`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`,
recording HEAD/working tree/UTC/sources/argv and binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. This unit has the corresponding
transparent operation in existing ShowRecipe/Located/Realized branches; Source
must not leak as an observable element. Human approval was `confirmo`,
2026-10-07. Final approval/execution record:
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
the nested contract.md is the pre-approval candidate retained there.

ObservationEq preserves causal span and body comparison. A distinct Source or
span must not authorize caching, regardless of language equality.

Preserve exclusive ownership and static dispatch, without I/O, new
signature/trait/flag or eval↔layout phase change. C2/C/macro tests and
collections, builds and lineage are required before certification.
