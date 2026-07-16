---
# Diagnóstico — P772i: `grid.header`/`grid.footer` como row-groups reais

> **Passo:** 772i
> **Data:** 2026-07-16
> **Commit-base:** `2c7025a9a949ecafda8a2550c8c3692db3c9ad0b` (working tree com
> alterações não commitadas).
> **Tipo:** Implementação directa (causa e solução especificadas no plano original
> de P224, verificadas contra o vanilla real, não copiadas cegamente).

---

## Sonda

### API real do vanilla

`grep -n "#\[elem(name = \"header\"\|#\[elem(name = \"footer\"" grid/mod.rs`:
confirma `grid.header(...)`/`grid.footer(...)` como elementos-filho (`#[elem]`),
nunca argumentos nomeados de `grid()`. Consistente com a arqueologia de P772h.

### Bug adicional descoberto durante a sonda (não estava em nenhum relatório anterior)

Repro directo (`grid.header[Nome][Idade]`, `mutool draw -F txt` antes de qualquer
correcção): **"Idade" estava completamente ausente** do output. Causa:
`native_grid_header`/`native_grid_footer` (e os pares `table_header`/`table_footer`)
usavam `args.items.first()` — só o primeiro argumento posicional. A sintaxe
`grid.header[A][B]` (múltiplos blocos de conteúdo trailing) produz múltiplos itens
em `args.items`; o segundo em diante era descartado silenciosamente. Confirmado
antes de decidir corrigir (ADR-0108), não assumido a partir do nome da função.

---

## Implementação

### 1. `header:`/`footer:` removidos como argumentos nomeados

`01_core/src/rules/stdlib/layout.rs::native_grid` — removidos da whitelist e da
extracção. `#grid(header: [Nome])` confirmado a dar erro ("argumento nomeado
inesperado em grid(): 'header'"), paridade vanilla.

### 2. `native_grid_header`/`native_grid_footer`/`native_table_header`/`native_table_footer`
corrigidos para colectar todos os argumentos posicionais

`args.items.first()` → loop sobre `args.items`, colectando todos os
`Value::Content`/`Value::Str` em `Content::sequence(..)` (que colapsa para o
`Content` bare se só houver 1 elemento — sem mudança de forma para o caso comum de
1 célula).

### 3. `split_header_footer` — `Content::GridHeader`/`GridFooter` como row-group

No loop de resolução de `grid()` (`stdlib/layout.rs`): `Content::GridHeader(_)`/
`Content::GridFooter(_)` deixam de cair no braço genérico (que os tratava como
célula normal, desalinhando `col`/`row` das células seguintes — scope-out #16 de
P772f). São guardados separadamente em `header`/`footer: Option<Content>`
(populando os campos já existentes de `GridElem`, agora alimentados por children
em vez de named args). Erro explícito se houver mais do que um header ou footer.

No motor de layout (`grid.rs::layout_grid`): as células do header/footer são
extraídas do seu `body` (`Content::Sequence` ou `Content` bare), preenchidas com
`Content::Empty` até múltiplo de `num_cols` (headers/footers ocupam linhas
inteiras, paridade vanilla), e coladas antes/depois das células normais
(`header_cells ++ cells ++ footer_cells`) antes de todo o resto do algoritmo de
posicionamento/dimensionamento correr — sem mecanismo de renderização novo, o motor
existente trata-as como quaisquer outras células a partir daí.

### 4. Repeat-across-páginas — scope-out explícito, registado (não silencioso)

**Decisão explícita:** não implementado neste passo. Requer estruturas
`Header`/`Footer`/`Repeatable<T>` com `range`/`level`/`short_lived` e lógica de
re-emissão consciente de paginação no motor de grid — esforço equivalente a um
passo dedicado próprio (a própria `GridHeaderElem`/`GridFooterElem` actuais só têm
`body`+`repeat`, sem `level`; adicionar isso é parte do esforço, não só a lógica de
repetição). Header e footer renderizam **uma única vez**: header sempre no topo,
footer sempre a seguir aos dados (não necessariamente ancorado ao fundo da última
página se a tabela quebrar página). Documentado no L0
(`00_nucleo/prompts/rules/layout.md` §"grid.header(...)/grid.footer(...) como
row-groups") para não ser "descoberto por acidente" outra vez.

### 5. `table()` — fora do âmbito, registado explicitamente

`table()` nunca teve `header:`/`footer:` como argumentos nomeados (não é o mesmo
bug), mas `TableElem` também não tem campos `header`/`footer`, e o loop de `table()`
trata `Content::TableHeader`/`TableFooter` como célula normal (mesmo scope-out #16,
variante table). Corrigir exige adicionar campos a `TableElem` (mudança estrutural,
não só de fluxo) — maior do que o âmbito deste passo; registado no L0 como gap
conhecido para passo dedicado futuro.

---

## Validação

### `grid.header[Nome][Idade]` + `[Ana][30]` — antes/depois, por coordenadas

| | Antes | Depois | Vanilla |
|---|---|---|---|
| "Nome" | presente, x=? (célula única, desalinhado) | x=70.867 | x=70.866 |
| "Idade" | **ausente** (bug do `.first()`) | x=100.457 | x=97.706 (diff residual ~2.75pt, mesma classe de divergência mecânica documentada em P772j) |
| "Ana" (linha de dados, col0) | — | x=70.867 (mesma coluna que "Nome") | x=70.866 |
| "30" (linha de dados, col1) | — | x=100.457 (mesma coluna que "Idade") | x=97.706 |

Confirmado por `mutool trace`: as 4 células aparecem, nas colunas correctas
(header e linha de dados alinhados na mesma coluna), estrutura 2×2 correcta.

### Footer

`[Ana][30]` + `grid.footer[Total][30]`: `mutool draw -F txt` confirma "Total"
aparece depois de "Ana" (ordem de leitura), ambas as células do footer presentes.

### `#grid(header: [Nome])` agora erra

Confirmado: `error: argumento nomeado inesperado em grid(): 'header'`, exit code 1 —
paridade vanilla (que rejeitaria da mesma forma).

### Testes de regressão novos

- `p772i_native_grid_header_footer_nomeados_rejeitados` — `header:`/`footer:`
  nomeados devem falhar.
- `p772i_native_grid_header_footer_como_children_populam_grid_elem` — via children,
  `GridElem.header`/`.footer` ficam populados como `Content::GridHeader`/`GridFooter`;
  `cells` não inclui as células do header/footer.
- `p772i_grid_header_multi_celula_preserva_todas_as_celulas` — regressão do bug
  `.first()`.
- `p772i_grid_header_nao_desalinha_colunas_seguintes` — regressão do scope-out #16
  (colunas da linha de dados devem coincidir com as colunas do header).
- `p772i_grid_footer_aparece_apos_dados` — footer emitido depois dos dados.
- `p772i_grid_header_footer_nomeados_dao_erro` — `#grid(header: ..)` deve falhar
  (via `layout_test` + `catch_unwind`).

Todos os 6 passam.

### Suite completa

- `cargo test --workspace --release`: **4170 passed (typst-core lib, +6 novos +
  1 teste substituído por 2) + 644 + 33 + 29 + 2 + 2, 0 failed**.
- `crystalline-lint .`: **0 violations** (mesmo warning V7 pré-existente).

### Reavaliação dos itens de P772f à luz desta implementação

| Item P772f | Classificação anterior | Reavaliação |
|---|---|---|
| #16 `ResolvableGridChild` | scope-out (lacuna-raiz) | **mecânica-diverge** — o papel funcional (distinguir header/footer/cell) está coberto, via splice directo em vez de um enum próprio; sem repeat-across-páginas. |
| #9 `Header` (struct) | scope-out (só renderiza 1 vez) | **inalterado** — ainda scope-out; renderiza 1 vez continua a ser a limitação real (não há `range`/`level`). |
| #12 `Footer` (struct) | scope-out (só renderiza 1 vez) | **inalterado** — idem. |
| #14 `Repeatable<T>` | scope-out (zero ocorrências) | **inalterado** — repeat-across-páginas não implementado. |
| #4 `check_for_conflicting_cell_row` | scope-out (parcial) | **inalterado** — conflito com posições explícitas dentro de header/footer não verificado por esta implementação (splice simples, sem detecção de colisão row-group-aware). |
| #6 `expand_row_group` | scope-out | **inalterado** — não há expansão dinâmica de row-group (header/footer têm bounds fixos ao momento da resolução). |
| #8 `find_next_empty_row` | scope-out | **inalterado** — só relevante para repeat-across-páginas. |
| #20/#21 `RowGroupData`/`RowGroupKind` | scope-out | **inalterado** — não implementadas como estruturas; papel coberto informalmente pela ordem de splice. |

---

## Critério de fecho — checklist

- [x] `header:`/`footer:` removidos como argumentos nomeados; `#grid(header: ...)`
      dá erro consistente com o vanilla.
- [x] `split_header_footer` implementado, distinguindo row-groups no loop de
      resolução.
- [x] `grid.header(...)`/`grid.footer(...)` como filhos renderizam correctamente,
      confirmado por coordenadas.
- [x] Decisão registada sobre repeat-across-páginas — **scope-out explícito**,
      documentado no L0, não implementado.
- [x] Itens #4, #6, #8, #16, #20, #21 de P772f reavaliados (tabela acima) — só #16
      passa a "mecânica-diverge"; os restantes permanecem scope-out por dependerem
      especificamente de repeat-across-páginas.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] L0 de `grid`/`stdlib/layout` actualizado (`00_nucleo/prompts/rules/layout.md`
      §"grid.header(...)/grid.footer(...) como row-groups", hash `9631382f`),
      removendo a menção implícita aos argumentos nomeados inventados (que nunca
      chegaram a estar documentados no L0 — confirma a arqueologia de P772h) e
      documentando o mecanismo novo + o scope-out de repeat + o gap de `table()`.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772i.md`.

---

## Próximo passo

Com P772g, P772i e P772j fechados, a linha de achados extra de P772f está
resolvida (as duas decisões pendentes de P772f — header/footer inventados, código
órfão — foram tratadas: a primeira corrigida de raiz neste passo; a segunda
revertida e reimplementada em P772j). Retomar a varredura da stdlib:
`visualize::image::svg` (7 itens), `foundations::scope` (7 itens), `text::font::*`
(~22 itens).

Gaps registados para passos dedicados futuros (não silenciosos):
- Repeat-across-páginas de header/footer (este passo, §4).
- `table()` header/footer via row-group (este passo, §5) — requer adicionar campos
  a `TableElem`.
