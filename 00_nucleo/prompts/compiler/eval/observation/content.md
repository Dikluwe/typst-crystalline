# L0 — compiler/eval/observation/content
Hash do Código: b13768c9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
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


## Realização com avaliação única

Relação observacional inclui Location e origem/produto, independente da igualdade de linguagem. Recipe não provada causalmente igual retorna Unproven e não suprime reexecução contextual.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

ObservationEq conserva span causal e comparação do body; Source distinto ou span distinto não autoriza cache, independentemente da igualdade de língua.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.
