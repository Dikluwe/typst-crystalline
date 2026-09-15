# Prompt L0 — `entities/elements/outline_entry` — `OutlineEntryElem`
Hash do Código: 8077e570

**Camada:** L1 · **Alvo:** `01_core/src/entities/elements/outline_entry.rs`

## P1402 — entrada pública de outline

### Medição antes da decisão

No vanila ratificado `586e1bd43`, `outline.entry(1, heading[H])` constrói
`entry(level: 1, element: heading(body: [H]))`. Uma ocorrência realizada por
`#outline()` expõe `level`, o heading completo em `element` e o fill default
`repeat(body: [.], gap: 0.15em)`. `body()` devolve `[H]`; um `strong` produz
`cannot outline strong`. Fonte: `typst-library/src/model/outline.rs:468-522,
668-674`.

### Obrigação

`OutlineEntryElem` é unidade nativa estática, nunca `Dynamic`. Guarda `level:
usize`, `element: Content` e `fill: Option<Content>`. O nível é maior que zero.
`get_field` expõe os três campos. `body()` aceita Heading nesta fatia e devolve
seu Content sem converter para texto; outras variantes falham explicitamente no
consumer de chamada. `map_content` e `map_text` recursam em `element` e `fill`.
`plain_text` segue o body do elemento outlinable. A identidade pública é
`outline.entry`, mas a entidade não conhece scopes nem avaliação.

Como elemento locatável, `element_kind()` devolve
`ElementKind::OutlineEntry`. Essa identidade alimenta query sem transformar o
tipo em função dinâmica.

Não implementar `indented`, `inner`, `page` ou `prefix` em P1402.

## P1403 — dados causais privados para `prefix()` e `page()`

Após a confirmação humana do contrato em
`compiler/stdlib/structural/outline.md` §P1403, a unidade guarda também, sem os
expor em `get_field`, o prefixo semântico já realizado do elemento fonte e sua
`Location` causal. Entries construídas diretamente inicializam ambos como
ausentes; entries sintetizadas pelo introspector recebem os dois. `prefix()`
devolve o Content preservado ou `none`; `page()` pertence ao consumer de
stdlib, que usa a Location. Walkers preservam esses metadados e só transformam
o Content do prefixo. P1403 não implementa `inner` nem `indented`.
