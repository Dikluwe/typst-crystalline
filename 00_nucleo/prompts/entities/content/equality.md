# L0 — entities/content/equality
Hash do Código: 7cfd61fc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/equality.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own `PartialEq` and `Eq` for the closed `Content` algebra. Preserve all existing
distinctions and intentional equivalences, including
`Space == SpaceFromNewline`. Rust hashing is owned independently by
`entities/content/rust-hash.md`; language morphology is owned separately.

## Acceptance

Existing equality and representation tests pass; matches remain static and the
module contains simultaneous verification.


## Realização com avaliação única

Igualdade da linguagem projeta body/visual; não identifica ocorrências por igualdade. Identidades Located não alteram == morfológico.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Source é transparente nesta projeção, delegando ao body; não expõe span como identidade de língua nem executa receitas. Segmentação conserva origem por segmento quando aplicável.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.
