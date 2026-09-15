# Prompt L0 — `entities/elements/outline_entry` — `OutlineEntryElem`
Hash do Código: da6bdf9d

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
