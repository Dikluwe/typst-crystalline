# Prompt L0 — `text`
Hash do Código: 6a5949ed

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

## P1293 — preservar proveniência no merge de `TextStyle` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O consumer constrói o estilo efetivo por literal exaustivo em
`01_core/src/compiler/layout/text.rs:151-200`. Ele já herda do
`layouter.style` os eixos internos `math`, `math_script`, `cramped` e
`math_size` (`:185-196`). O campo público `math_text_item` proposto em
`entities/layout_types.md` fará esse literal deixar de compilar e, se fosse
reposto como `false` durante um merge textual, apagaria a proveniência antes de
`FrameItem::Text` alcançar a medição final.

O vanilla conserva os styles do `TextItem` ao chamar o layout inline
(`lab/typst-original/crates/typst-layout/src/math/text.rs:15-40`). A
proveniência não é uma propriedade configurável de texto, mas também não pode
ser descartada por um merge que já ocorre dentro da via selecionada.

Inferência: o merge deve herdar `layouter.style.math_text_item`, sem sintetizar
`true`. Refutador: o consumer criar por si uma nova morfologia `TextItem`; a
fonte atual não mostra esse caminho e exigiria nova decisão.

### Decisão proposta após confirmação humana

O literal de `effective` preserva
`math_text_item: layouter.style.math_text_item`, no mesmo padrão de `math`,
`math_script`, `cramped` e `math_size`. Nenhum namespace `text.*`,
`StyleDelta`, parsing, default ou regra de utilizador pode definir o bit.

Ownership 1:1: este prompt legitima somente
`01_core/src/compiler/layout/text.rs`. A futura adaptação é compatibilidade
default-preserving do novo campo público e integra o gate humano ADR-0127
categoria 1; nenhum código é autorizado antes da confirmação.
