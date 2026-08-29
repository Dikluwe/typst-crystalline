# Prompt L0 — `curve`
Hash do Código: d2bccae0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/curve.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Converter Curve em geometria de Frame preservando path, fill, stroke e dimensões resolvidas no contexto de layout.

**P1226:** propagar `FillRule` até `FrameItem::Shape`. Resolver
`CloseMode::Smooth` usando o estado do subpath e controles Bézier; resolver
`Straight` como fechamento reto. O default público é `Smooth`. Nunca escolher
o modo a partir da aparência ou do backend.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.
