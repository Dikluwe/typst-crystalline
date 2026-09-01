# Prompt L0 — aspas localizadas
Hash do Código: a2383787

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/quotes.rs`
**ADRs:** ADR-0052, ADR-0060, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer usa match exato do código `Lang`: aspas primárias específicas para
`pt`, `en`, `de`, `fr`, `es` e `it`; aspas simples específicas para `en`.
Códigos fora das tabelas usam pares curly ingleses determinísticos.

## Contrato

- `localize_quotes(&Lang)` devolve o par primário; francês preserva NBSP.
- `localize_single_quotes(&Lang)` devolve o par simples.
- `DEFAULT_QUOTES` é `“`/`”`; `DEFAULT_SINGLE_QUOTES` é `‘`/`’`.
- O lookup é linear, exato e puro; não interpreta região BCP47.

## Limites e aceitação

Sem smart-apostrophes, alternância aninhada ou inferência por ambiente. Os
pares existentes, NBSP e fallback devem permanecer iguais; mudanças públicas
ficam sob ADR-0127.

## P1286 — pares alternativos e overrides explícitos

### Medição anterior à decisão

O baseline `text/smartquote.rs:233-310` escolhe quatro glifos (single/double,
open/close) por língua, região e `alternative`; depois aplica separadamente os
overrides `single` e `double`. `:335-419` converte string por grapheme, array e
dicionário. O receipt P1286 confirma a precedência no caso alemão e os erros
de cardinalidade/chave.

### Decisão

Este owner fornece operações puras de validação e resolução sobre os tipos
canônicos públicos pertencentes ao owner
`entities/elements/smartquote.md`. Para a superfície sem região representável
no cristalino, a tabela replica os ramos sem região do baseline pinado; em
particular alemão primário usa
`(‚,‘,„,“)` e alternativo usa `(›,‹,»,«)`. Quotes explícitas anulam a
escolha alternativa apenas no membro substituído; `auto` conserva o fallback.
Não ampliar `Lang` nem inventar região neste lote.

Aceitação é a morfologia dos glifos e as mensagens medidas, não a forma do
helper. Casos regionais, stack de nesting, prime/apóstrofo e profundidade 32
permanecem `Unknown` e não podem ser promovidos a paridade.
