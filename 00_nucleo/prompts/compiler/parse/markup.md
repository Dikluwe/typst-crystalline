# Prompt L0 — gramática de markup
Hash do Código: f2acf0f7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/markup.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer possui markup, strong/emph, headings, listas, refs e equações. A
regra P786a emite warning para `**`/`__` vazios fechados; delimitador sem fecho
continua erro. O lexer, não este owner, governa a agregação P1137 de espaços.

## Contrato

- Interpretar elementos dependentes de início de linha somente quando
  `at_start` é verdadeiro e preservar trivia conforme `wrap_trivia`.
- Tratar brackets aninhados como texto até o nível correspondente.
- Entrar em Math entre `$...$` e em Code após `#`, com restauração de modo.
- Preservar exatamente warnings, hints, delimitadores e morfologia vigentes.

## Aceitação

Cobrir markup normal, strong/emph vazios/abertos, estruturas de linha, refs e
equações sem mudança de outputs públicos.
