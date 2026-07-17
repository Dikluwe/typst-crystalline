---
# Diagnóstico — P772f: varredura de `typst_library::layout::grid::resolve`

> **Passo:** 772f (série P765a→P772e→P772f)
> **Data:** 2026-07-16
> **Commit-base:** `2c7025a9a949ecafda8a2550c8c3692db3c9ad0b` (working tree com alterações
> não commitadas — ver `git diff HEAD --stat` no fecho do passo).
> **Tipo:** Sonda + achado real corrigido + achado real registado para passo dedicado.

---

## 1. Sonda — classificação dos 24 itens de `layout::grid::resolve`

Varredura item-a-item contra
`lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs` (2421 linhas) e
`01_core/src/engine/layout/grid.rs` + `01_core/src/engine/layout/grid_placement.rs` +
`01_core/src/entities/elements/{grid,table}_{header,footer,cell}.rs`.

| # | Item | Vanilla (file:line) | Status | Justificação (crystalline) |
|---|------|---------------------|--------|------------------------------|
| 1 | `Cell` (struct) | resolve.rs:572-613 | mecânica-diverge | `PlacedCell` (`grid_placement.rs:42-53`, body+row+col+colspan+rowspan) — mesmo papel; `stroke_overridden`/`breakable` lidos à parte em `grid.rs:422-453`, não fundidos na struct. |
| 2 | `CellGrid` (struct) | resolve.rs:657-899 | mecânica-diverge | Substituída por `Vec<PlacedCell>` + variáveis locais `resolved_widths`/`row_heights`/`col_starts` em `grid.rs` — mesma informação, sem struct única. |
| 3 | `CellGridResolver` (struct) | resolve.rs:940-950 | mecânica-diverge | Sem struct-resolver; estado equivalente são os parâmetros/locais de `layout_grid` (`grid.rs:104-118`). |
| 4 | `check_for_conflicting_cell_row` (fn) | resolve.rs:2112-2149 | scope-out (parcial) | Conflito básico célula-célula existe (`grid_placement.rs:119-132,181-195`, bail em posição ocupada); a parte header/footer-aware não existe (decorrência do #16). |
| 5 | `Entry` (enum) | resolve.rs:628-647 | mecânica-diverge | Sem `Entry::Merged`; `cell_bounds` (`grid.rs:765-781`) soma tracks via `colspan`/`rowspan` directo em `PlacedCell`, sem posições-fantasma. |
| 6 | `expand_row_group` (fn) | resolve.rs:1998-2109 | scope-out | Sem equivalente — decorre da ausência de row-groups (#16 e correlatos). |
| 7 | `find_next_available_position` (fn) | resolve.rs:2314-2373 | mecânica-diverge | Coberto pelo loop `cursor_col`/wrap em `grid_placement.rs:172-195` — mesmo comportamento observável sem header/footer. |
| 8 | `find_next_empty_row` (fn) | resolve.rs:2378-2393 | scope-out | Só usado por headers/footers no vanilla; sem uso — decorre do #16. |
| 9 | `Footer` (struct) | resolve.rs:444-463 | scope-out | `GridFooterElem` existe (`entities/elements/grid_footer.rs`) mas só renderiza 1 vez (`engine/layout/table_footer.rs:14`, DEBT-56) — sem `range`/`level`, sem repeat-across-páginas. |
| 10 | `grid_item_to_resolvable` (fn) | resolve.rs:129-164 | mecânica-diverge | Sem conversão intermédia — `Content::GridHLine/GridVLine/GridCell` casados directo no loop de `grid()` (`stdlib/layout.rs:270-307`). |
| 11 | `grid_to_cellgrid` (fn) | resolve.rs:27-75 | mecânica-diverge | `grid.rs:84-93` (`layout()`→`layout_grid`) — mesmo papel (converter `GridElem` p/ layout), sem `CellGrid` intermédio. |
| 12 | `Header` (struct) | resolve.rs:427-442 | scope-out | `GridHeaderElem{body,repeat}` existe mas só suporta 1 render único (`engine/layout/grid_header.rs:19`) — sem `range`/`level`/`short_lived`; DEBT-56 documenta repeat diferido. |
| 13 | `LinePosition` (enum) | resolve.rs:619-625 | mecânica-diverge | `EcoString` `"top"/"bottom"`, `"left"/"right"` em `grid_hline.rs:28`/`grid_vline.rs:27` — mesma semântica, tipo diferente. |
| 14 | `Repeatable<T>` (struct) | resolve.rs:465-498 | scope-out | Zero ocorrências no repo — repeat-across-páginas não implementado (consistente com #9/#12). |
| 15 | `ResolvableCell` (trait) | resolve.rs:502-535 | mecânica-diverge | Sem trait — campos (`x`,`y`,`colspan`,`rowspan`,`align`,`inset`,`stroke`) lidos directo de `GridCellElem`/`TableCellElem` em `grid.rs:422-453`. |
| 16 | `ResolvableGridChild` (enum) | resolve.rs:650-654 | scope-out | Sem conceito equivalente: `Content::GridHeader`/`GridFooter` caem no braço genérico `other => cells.push(...)` no loop do `grid()` — não tratados como row-group distinto. Esta é a lacuna-raiz de que #4/#6/#8/#9/#12/#14/#20/#21/#22 são sintomas. |
| 17 | `ResolvableGridItem` (enum) | resolve.rs:539-568 | mecânica-diverge | Papel coberto pelas variantes já existentes `Content::GridHLine/GridVLine/GridCell`, sem enum próprio. |
| 18 | `resolve_cellgrid` (fn) | resolve.rs:908-938 | mecânica-diverge | `layout_grid` (`grid.rs:104-`) cobre o mesmo papel ponta-a-ponta, chamando `place_cells` (`grid_placement.rs:64`) em vez de `CellGridResolver::resolve`. |
| 19 | `resolve_cell_position` (fn) | resolve.rs:2160-2305 | mecânica-diverge | `place_cells` (`grid_placement.rs:64-217`) implementa o comportamento observável equivalente (posição explícita fixa; auto-placement varre linha-a-linha) via cursor+matriz `occupied`; sem consciência de header/footer (ligado ao #16). |
| 20 | `RowGroupData` (struct) | resolve.rs:967-1007 | scope-out | Zero ocorrências — decorrência do #16. |
| 21 | `RowGroupKind` (enum) | resolve.rs:952-965 | scope-out | Zero ocorrências — mesma razão. |
| 22 | `skip_auto_index_through_fully_merged_rows` (fn) | resolve.rs:2402-2421 | scope-out (não totalmente verificado) | Zero ocorrências; refina posição de hline auto sob rowspans-cheios. **Não verificado** se a emissão de hline/vline em `grid.rs` (linhas ~580-760, fora do que esta sonda leu) tem tratamento equivalente ou simplificado — marcar para verificação futura. |
| 23 | `table_item_to_resolvable` (fn) | resolve.rs:166-201 | mecânica-diverge | Mesmo padrão do #10, variante Table. |
| 24 | `table_to_cellgrid` (fn) | resolve.rs:79-127 | mecânica-diverge | `table.rs` + `layout_grid` partilhado (`grid.rs:81`, "cluster Grid+Table"). |

**Achado adicional da varredura (fora dos 24 itens, mas no mesmo ficheiro):**
`layout_grid` (`grid.rs:104-118`) recebe `e.header.as_ref()`/`e.footer.as_ref()`
(`grid.rs:91`) como `_header: Option<&Content>`/`_footer: Option<&Content>` —
**nunca lidos no corpo da função** (confirmado: `grep -n "_header\|_footer"
01_core/src/engine/layout/grid.rs` só encontra as duas linhas da assinatura). Verifiquei
adicionalmente que **o vanilla real não tem** `header:`/`footer:` como argumentos
nomeados de `grid()`/`table()` — no vanilla, cabeçalhos/rodapés são elementos-filho
(`grid.header(...)`/`grid.footer(...)`, `lab/typst-original/.../grid/mod.rs:580,608`),
nunca argumentos nomeados da função `grid()`. O stdlib cristalino
(`01_core/src/engine/stdlib/layout.rs:191,251,259`) **inventou** um parâmetro nomeado
`header:`/`footer:` sem equivalente vanilla, e o conteúdo passado por ele é
**silenciosamente descartado** — nem erro (vanilla rejeitaria `#grid(header: ..)` com
"unexpected argument"), nem render. Isto é distinto do scope-out #16 (que é sobre
`grid.header(...)` como filho, que pelo menos renderiza uma vez). Não corrigido neste
passo — decisão de desenho (remover o parâmetro nomeado para bater com vanilla, ou
implementá-lo) cabe ao humano; registo aqui por ser achado real de perda silenciosa de
conteúdo.

**Cobertura desta sonda:** `grid.rs` linhas ~580-760 (emissão de hline/vline) não foram
lidas linha-a-linha nesta varredura — item #22 fica como "não totalmente verificado"
até essa leitura. `fixup_cells`/`collect_lines`/`finalize_headers_and_footers` (só
relevantes no vanilla por causa de row-groups) não foram comparados função-a-função,
dado que a ausência de row-groups no cristalino (#16) torna boa parte não-aplicável;
o comportamento de preencher posições vazias com células default (papel de
`fixup_cells`) não foi verificado directamente.

---

## 2. Achado P763g — investigado por coordenadas (`mutool trace`)

### 2.1 O que P763g tinha registado

P763g mediu AE=4093 (grid com `place`) vs AE=2020 (grid sem `place`) e concluiu, **sem
confirmar por coordenadas**: "o grid cristalino já não renderiza a segunda célula (ou
posiciona-a fora da página) mesmo sem place. A divergência principal é estrutural do
próprio grid." Esta frase nunca foi verificada com `mutool trace`.

### 2.2 Reprodução e medição

Repro usado (`/tmp/.../p772f-grid-repro.typ`):

```typst
#set page(width: 8cm, height: 6cm)
#grid(
  columns: 2, gutter: 5pt,
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
)
```

`mutool trace` no PDF cristalino (**antes** de qualquer correcção, binário compilado a
partir do working tree no início deste passo) mostrou **dois** `<stroke_path>` — a
segunda célula **não está ausente**. Mas ambos os círculos aparecem nas **coordenadas
absolutas idênticas** (centro ≈ `(55.494, 35.25)` em ambos), i.e. sobrepostos — o que
visualmente parece "uma célula em falta" (P763g leu correctamente o *sintoma* visual mas
inferiu a *causa* errada sem medir coordenadas, exactamente o padrão que ADR-0108 pede
para desconfiar).

**Conclusão sobre P763g: refutado como enunciado ("estrutural, sem causa
identificável"), mas o sintoma visual (parece faltar uma célula) é real — causa raiz
identificada e são duas, distintas, medidas abaixo.**

### 2.3 Causa raiz A — `measure_content_constrained` media 0pt para `Content::Align`
e ignorava a largura explícita de `Content::Block` (CONFIRMADO, CORRIGIDO)

Instrumentação directa (`eprintln!` temporário, removido) em
`01_core/src/engine/layout/grid.rs` mostrou, para as duas colunas Auto do repro:

```
cell col=0 row=0 cell_x=20.247142857142858 cell_y=... cell_w=0 cell_h=56.69...
cell col=1 row=0 cell_x=20.247142857142858 cell_y=... cell_w=0 cell_h=56.69...
```

**As duas colunas mediram largura 0 e ficaram na mesma posição x** — daí a sobreposição.
Root cause em `01_core/src/engine/layout/mod.rs::measure_content_constrained`:

1. `Content::Align` não tinha braço próprio → caía no catch-all `_ => (0.0, 0.0)`
   (linha 1566, numeração pré-fix). Confirmado isoladamente com um repro **sem**
   `block()`, apenas `align(center, [Hello])`/`align(center, [World])` em colunas Auto:
   ambos mediram 0 e o texto das duas células colidiu em `x=20.247` nos dois casos.
2. `Content::Block` com `width: Some(w)` calculava `body_max` correctamente a partir de
   `w`, mas depois **reportava `bw` (a largura medida do corpo)** como `total_w`, em vez
   da largura explícita do bloco. Como o corpo (`align(...)`) media 0 (bug #1), o bloco
   também reportava 0, mesmo tendo `width: 3cm` explícito.

**Correcção aplicada** (`01_core/src/engine/layout/mod.rs`):
- Novo braço `Content::Align(e) => self.measure_content_constrained(&e.body, max_width)`
  — `align` não tem tamanho intrínseco próprio (o wrapper só reposiciona dentro do
  espaço disponível; a medição para efeitos de auto-sizing de coluna deve ser a do
  corpo). Nota: `Content::Place` foi deliberadamente **deixado** no catch-all
  `(0.0, 0.0)` — no vanilla, conteúdo `place()`d está fora do fluxo e não deveria
  informar o dimensionamento automático do container ancestral; alterar esse braço
  seria uma divergência nova, não uma correcção.
- `Content::Block`: `total_w` passa a usar `w.resolve_pt(font).min(max_width)` quando
  `width: Some(w)`, em vez de `bw + inset_l + inset_r`.

**Verificação pós-fix** (mesmo repro, `mutool trace`):
- `block(width:3cm, align(top,place(...)))`: as duas colunas passaram a medir
  `85.038pt` cada (3cm), `col_starts=[20.247, 105.285]` — deixaram de colidir (a
  posição final ainda diverge do vanilla por causa da causa raiz B, §2.4 — mas já não
  há sobreposição/colisão de coluna).
- `align(center,[Hello])`/`align(center,[World])` sem bloco e **sem** `align:` a nível
  de grid (para não exercitar o código descrito em §3.3, que é um mecanismo diferente):
  texto das duas colunas em `x` distintos (`20.247` vs `49.546`), já não colidem.

**Testes de regressão** (`01_core/src/engine/layout/tests.rs`, prefixo `p772f_`):
- `p772f_measure_content_constrained_align_reporta_largura_do_corpo`
- `p772f_measure_content_constrained_block_largura_explicita_nao_colapsa`
- `p772f_grid_auto_colunas_com_align_nao_colidem` (E2E via `layout_test`)

Todos os 3 passam; `cargo test --workspace --release` = **4160 + 644 testes verdes, 0
falhas** (ver §4).

### 2.4 Causa raiz B — `layout_place` (PlaceScope::Column) duplica a origem da
célula quando aninhado dentro de `Content::Align` (CONFIRMADO, **NÃO CORRIGIDO** —
achado grande, ver §3)

Mesmo depois da correcção da causa raiz A, o repro completo
(`block(width:3cm, align(top, place(...)))`) ainda **não bate com o vanilla**: o
círculo da célula 1 (coluna 2) fica centrado em `x≈225.57pt` numa página de
`226.77pt` de largura — ou seja, **quase inteiramente fora da página**, contra o
vanilla (`x≈125.29pt`, dentro da página). A célula 0 também diverge (x≈55.49 vs
vanilla x≈35.25), só que por coincidência ainda cabe na página, o que a tornou menos
óbvia visualmente.

Isolado com 3 reproduções (`mutool trace` em todas, binário release recompilado a cada
alteração):

| Conteúdo da célula | x medido (célula 0 / célula 1) | vanilla (referência) |
|---|---|---|
| `place(...)` directo (sem `align`) dentro de `block()` | 35.247 / 120.285 | 35.247 / 125.287 (diff = gutter, gap conhecido — ver §3.2) |
| `align(top, place(...))` dentro de `block()` | 55.494 / 225.57 | 35.247 / 125.287 |

Instrumentação directa em `layout_place`/`layout_align` confirmou o mecanismo: quando
`regions.cell`/`cell_origin_x` estão `Some` (dentro de uma célula de grid),
`layout_place` calcula coordenadas **absolutas** (`target_x = cx + alinhamento + dx`) e
empurra os itens para `self.regions.current.current_items` — mas, quando `place()` está
aninhado dentro de `layout_align` (que cria o seu próprio sub-frame com
`origin_x: 0.0` e depois **soma o seu próprio `target_x` absoluto** aos itens que
recebe de volta), a origem da célula é somada **duas vezes**. `layout_place` já tem uma
correcção parcial para este problema (comentário "P763f" no código, braço `_ =>` que
usa `(0.0, 0.0)` quando `in_sub_frame`) — mas essa correcção só cobre o ramo de
fallback (`regions.cell` ausente); o ramo principal
(`(Some(cx), Some(cy), Some(cell))`) não verifica `in_sub_frame` e sempre usa
coordenadas absolutas, mesmo quando aninhado.

**Por que não corrigido neste passo:** o mecanismo correcto exige decidir como
`layout_place` deve reconhecer "estou a emitir directamente para o frame que o meu
chamador vai tratar como absoluto" vs "estou aninhado dentro de outro wrapper
(align/pad/box/stack/columns) que vai re-traduzir as minhas coordenadas como locais" —
isto é uma questão de modelo de coordenadas que atravessa `layout_place`,
`layout_align`, e potencialmente qualquer outro wrapper que chame `layout_sub_frame`
com `origin_x: 0.0` e depois recomponha `target_x` a partir de `line_start_x`. Não é
específico de `grid::resolve` (o bug está em `placement.rs`, não em `grid.rs`), e
requer o mesmo tipo de investigação faseada (sonda de localização → decisão →
implementação → validação por coordenadas) da cadeia P763–P767. Registo aqui a decisão
explícita: **não forçar a correcção neste passo**; ver §3 para proposta de passo
dedicado.

---

## 3. Achados adicionais e decisões

### 3.1 Passo dedicado proposto para a causa raiz B

Título sugerido: **P772g — `layout_place` duplica origem de célula quando aninhado em
`Content::Align`/outros wrappers de sub-frame**.

Escopo: generalizar a correcção já parcial (comentário "P763f" em
`01_core/src/engine/layout/placement.rs`) para o ramo `(Some(cx), Some(cy), Some(cell))`
do `match scope` em `layout_place`, cobrindo o caso em que `place()` está aninhado
dentro de `align()` (confirmado) e auditando os outros wrappers que chamam
`layout_sub_frame` com `origin_x: 0.0` (pad, boxed, stack, columns) para o mesmo
padrão antes de declarar a correcção completa. Critério de fecho: os 3 repros de
`mutool trace` usados neste passo (`place` directo, `align`+`place`, e o caso completo
com `block`) devem bater com o vanilla dentro de tolerância sub-pt.

### 3.2 Gap já documentado, não nesta linha de investigação: gutter de colunas

`layout_grid` (`01_core/src/engine/layout/grid.rs:111`) recebe `_gutter: Option<Length>`
com underscore — não está a ser aplicado ao espaçamento horizontal entre colunas em
`col_starts`. Isto já estava assim antes deste passo (comentário no código já assinala
"graded"/incompleto) — não é um achado novo, só relevante para explicar o gap de
~5pt entre o cristalino (sem gutter aplicado) e o vanilla no repro "place directo" de
§2.4. Não fechado neste passo — fora do escopo de `grid::resolve`.

### 3.3 Código não commitado, pré-existente, não coberto por L0 (achado incidental)

`git status` no início deste passo já mostrava `01_core/src/engine/layout/grid.rs`
modificado (não commitado) com um bloco `P772f — aplicar align efectivo da célula...`
que envolve o corpo da célula num `Content::Place` (`scope: Column`) quando
`effective_align` é `Some`. Este código **não foi escrito nesta sessão** e não
corresponde ao que o L0 actual (`00_nucleo/prompts/engine/layout.md`) especifica para
grid+align (grep não encontrou nenhuma menção a este mecanismo no L0). Testado
isoladamente (`#grid(columns:2, align: center, [Hello], [World])`):
cristalino coloca "Hello" em `x=33.772`; vanilla em `x=20.247`
— **diverge do vanilla**, um deslocamento de ~13.5pt não explicado pelas causas raiz A/B
(mecanismo de posicionamento diferente, `Place` como wrapper único directo, não
aninhado em `Align`). Não tentei corrigir nem reverter este código — é órfão
(sem L0 correspondente) e fora do âmbito deste passo; **decisão do humano**: reverter
(descartar esta tentativa incompleta) ou formalizar com L0 próprio + passo dedicado.

---

## 4. Validação

- `cargo test --workspace --release`: **4160 passed (typst-core lib) + 644 passed
  (integration) + 33 + 29 + 2 + 2, 0 failed** em todo o workspace. Medido no working
  tree deste passo (commit-base `2c7025a9a`, ver `git diff HEAD --stat` no topo).
- `crystalline-lint .`: **0 violations** (1 warning V7 pré-existente, não relacionado —
  prompt órfão `package_version_resolution.md`, já existia antes deste passo).

---

## 5. Critério de fecho — checklist

- [x] Os 24 itens de `layout::grid::resolve` classificados item a item (§1).
- [x] Achado de P763g confirmado/desmentido por coordenadas, não por inspecção visual
      (§2.2) — desmentido como "célula ausente"; confirmado como duas causas raiz
      distintas medidas por `mutool trace`.
- [x] Achado grande (causa raiz B) registado com decisão explícita (§2.4, §3.1) —
      passo dedicado proposto, não forçado neste passo.
- [x] Achado pequeno (causa raiz A) corrigido com teste e coordenadas antes/depois
      (§2.3).
- [x] Outros itens do módulo classificados (§1).
- [x] `cargo test --workspace` verde (§4).
- [x] `crystalline-lint .` zero violações (§4).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772f.md` (este ficheiro).

---

## 6. Próximo passo

Conforme P772e: `visualize::image::svg` (7 itens), `foundations::scope` (7 itens),
`text::font::*` (~22 itens). Mas a gravidade do achado desta linha sugere abrir
**P772g** (§3.1) antes de avançar para os módulos seguintes, dado que `place()`
aninhado em wrappers de layout é um padrão comum (qualquer combinação de
`align`/`pad`/`box` com `place()` dentro, não só em grid) e o sintoma (conteúdo
empurrado para fora da página) é severo.
