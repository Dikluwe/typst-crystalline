# L0 — entities/content/plain-text
Hash do Código: 185994de

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/plain_text.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own recursive `Content::plain_text` projection. Preserve content order, textual
markers, transparent wrappers, and element-specific delegation without making
rendered bytes or Rust representation part of the language contract.

## Acceptance

Existing plain-text, representation, and morphology tests pass; the operation
remains pure and statically dispatched.


## Realização com avaliação única

Texto plano usa body de ShowRecipe/Located e somente visual de Realized, sem duplicar origem consumida.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Source é transparente nesta projeção, delegando ao body; não expõe span como identidade de língua nem executa receitas. Segmentação conserva origem por segmento quando aplicável.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.
