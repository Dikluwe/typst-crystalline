# Prompt L0 — `text`
Hash do Código: c4327da6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/text.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Resolver TextStyle efetivo da chain e emitir runs de texto por palavras/espaços, preservando math, math_script, idioma e propriedades tipográficas.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

## Preservação da proveniência `TextItem`

Ao construir `TextStyle`, o layout copia `layouter.style.math_text_item`
junto dos demais campos. O merge não deriva nem zera essa proveniência.

O campo continua ortogonal a `math`: texto inline matemático preserva
`math=true` e `math_text_item=true`; prosa e rotas de glifo mantêm o default
`false`. Família, tamanho, variações, direção e idioma não mudam por esta
cópia.

## P1481 — preservação da origem de Strong/Em no texto (IMPLEMENTADO; confirmado em 2026-09-23)

`text.rs:158` cria um `TextStyle` efetivo a partir do estilo do layouter
e de opções textuais. `StyleDelta` já distingue `from_strong` e
`from_emph`; o QDF vanilla do corpus P1481 contém Strong e Em.
Com os campos públicos confirmados em `entities/layout_types.md`,
o merge preserva `semantic_strong/semantic_emph` somente quando a
origem morfológica correspondente está ativa. Bold/italic genéricos
continuam visuais e não se tornam Strong/Em. O fragmento resultante
pode ser moldado por L3 sem reavaliar a aparência.
