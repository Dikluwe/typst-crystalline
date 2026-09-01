# Prompt L0 — `compiler/stdlib/structural/table_grid` — estrutura de tabela e grelha
Hash do Código: d4d8c07e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/table_grid.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/table.rs` + `layout/grid/{mod,resolve}.rs`. Co-mudança confirmada: P229-230, P233-238, P327 movem `native_grid_cell` e `native_table_cell` juntos; P772f-j e P320 movem os quatro header/footer juntos.

---

## Contexto

A **estrutura** de `table` e `grid`: o contentor, as células, e os cabeçalhos/rodapés
repetíveis. As linhas (`hline`/`vline`) são o nó irmão `table_lines.md` — separadas
porque a história as move como conjunto próprio (P512/P513, P739) e nunca com as células.

`table` e `grid` são pares simétricos por decisão registada (P224.B): tudo o que existe
para um existe para o outro, com a mesma validação.

## Instrução

| Nativa | Assinatura | Emite |
|---|---|---|
| `table` | `table(columns?, rows?, ..children)` | `Content::Table` |
| `table_cell` | `table_cell(body, x:?, y:?, colspan:?, rowspan:?)` | `Content::TableCell` |
| `table_header` / `table_footer` | `(body, repeat: true)` | `Content::TableHeader` / `TableFooter` |
| `grid_cell` | `grid_cell(body, x?, y?, colspan?, rowspan?)` | `Content::GridCell` — placement algorítmico (P224.C) |
| `grid_header` / `grid_footer` | `(body, repeat: true)` | `Content::GridHeader` / `GridFooter` |

### Extractores partilhados (privados ao nó)

| Helper | Regra |
|---|---|
| `extract_usize_or_none_min` | `Auto` **ou** `None` → `None` (ADR-0064 Caso A: `None ↔ Auto` do vanilla) |
| `extract_align_value` | `Align` directo, ou `Str` via `Align2D::from_string` (P235) |
| `extract_inset_value` | `Length` uniforme → `Sides<Length>` (P235, paridade `Block.inset`) |
| `extract_bool_with_default` | `bool` com default arbitrário (ADR-0064 Caso D) |

`fill: none` e `stroke: none` são aceites em `table`/`table.cell`/`grid.cell` (P726) —
zero-thickness continua proibido (hairline em PDF).

## Restrições Estruturais

- L1 puro.
- `table` e `grid` mantêm paridade absoluta entre si — qualquer parâmetro novo num tem
  de existir no outro (P224.B).
- `default_hline_stroke` vem de `super::table_lines` (única aresta entre os dois nós).

## Critérios de Verificação

```
#table(columns: 2)[a][b]                → Table 2 colunas
#table.cell(colspan: 2)[a]              → TableCell colspan 2
#table.cell(x: auto)[a]                 → x = None
#grid.cell(x: none)[a]                  → x = None (mesma regra)
#table.header[a]                        → repeat = true por omissão
#table.footer(repeat: false)[a]         → repeat = false
#table(fill: none)[a]                   → aceite (P726)
#table()[a]                             → grelha desenhada sem stroke explícito (P887)
```

## P1288 — níveis semânticos de `table.header`

### Medição anterior à decisão

O owner produtivo rejeita hoje `level` em `native_table_header` e qualquer
segundo `TableHeader` em `native_table` (`table_grid.rs:542-581,123-129`). O
baseline vanilla P1288 mede `level` inteiro positivo, múltiplos headers de
níveis distintos e preservação de célula explicitamente Data dentro do
header. O teste histórico cristalino mede que dois headers no mesmo nível
continuam erro.

### Decisão

`table.header(level: 1, ..cells)` aceita `level` inteiro estritamente positivo.
Cada conteúdo cru vira `TableCell` Header com scope Column; célula Auto existente
é promovida da mesma forma; Header/Data explícito conserva a classificação.
`table` combina headers de níveis distintos num único grupo lógico, em ordem,
mas rejeita níveis duplicados. Esta exceção semântica PDF não se propaga a
`grid.header`: grid continua sem `level`. Geometria, repeat e diagnóstico de
dois headers default permanecem compatíveis.
