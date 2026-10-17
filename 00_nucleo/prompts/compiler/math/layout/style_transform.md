# L0 — compiler/math/layout/style_transform
Hash do Código: 97f68ce8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/math/layout/style_transform.rs`. A lógica original estava em `01_core/src/compiler/math/layout/mod.rs:2263-2732` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own recursive application of default mathematical italics and explicit glyph styles. Preserve MathStyled axis precedence, size wrappers, attachment slot presence, content morphology and immutable input. This unit performs no geometric layout or callback execution. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-eighth/p966 demonstra phi/psi crus sob Source. Transformação de glifos matemáticos default/explicit atravessa body de Source conservando origem, eixos/tamanhos/slots e precedência MathStyled; não achatar containers em texto ou alterar fase/layout.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.
