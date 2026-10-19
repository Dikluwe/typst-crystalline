# Relatório — typst-passo-822: `layout::grid::resolve` — mensagens divergentes + footer fora do fim aceite (achado #9 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-822.md`; único ficheiro acedido em `materialization/`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado** nas medições ANTES e DEPOIS (`git diff HEAD --stat` no início: 41 ficheiros, +3262/-367 — passos P823…P818; ao fechar, 2026-07-22T04:46Z, os ficheiros deste passo somam +362/-64 adicionais, listados em §Implementação). Binário cristalino rebuildado (`cargo build --release`, 20.99s) após a implementação e antes da medição DEPOIS.
**Binários:** `./target/release/typst` (cristalino — `typst <input> <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Fixtures em `temp/p822/`.

**Âmbito (do prompt):** só os dois pontos novos do achado #9 de P810 — (b) mensagens de erro divergentes e (c) `footer` fora do fim aceite. **SCOPE-OUT explícito respeitado:** o débito "header/footer não repetem entre páginas" (P772i/P789, achado #9(a)) **não foi tocado** — continua registado como débito grande; ver §Scope-outs com o controlo de não-regressão.

---

## Passo 1 — Sonda (ANTES, literal)

Comandos exactos (em `temp/p822/`):
`../../lab/typst-original/target/release/typst compile <caso>.typ <caso>.vanilla.pdf` e
`../../target/release/typst <caso>.typ <caso>.crist.pdf`.

### Caso (a) — `a_footer_mid_table.typ`: footer no meio de `table`

```typ
#table(
  columns: 2,
  table.footer[F1][F2],
  [a], [b],
)
```

```text
VANILLA (exit=1):
error: footer must end at the last row
  ┌─ a_footer_mid_table.typ:3:2
  │
3 │   table.footer[F1][F2],
  │   ^^^^^^^^^^^^^^^^^^^^

CRISTALINO ANTES (exit=0):  — aceite em silêncio, PDF gerado
```

### Caso (b) — `b_footer_mid_grid.typ`: footer no meio de `grid`

Mesma forma com `grid.footer[F1][F2]` → VANILLA `error: footer must end at the last row` (exit=1, span 3:2); CRISTALINO ANTES exit=0 (aceite em silêncio).

### Caso (c) — `c_conflict_cell_cell.typ`: conflito célula↔célula

```typ
#grid(columns: 2, grid.cell(x: 0, y: 0)[A], grid.cell(x: 0, y: 0)[B])
```

```text
VANILLA (exit=1):
error: attempted to place a second cell at column 0, row 0
  ...
  = hint: try specifying your cells in a different order

CRISTALINO ANTES (exit=1):
error: grid placement: conflito — célula explicit (x=0, y=0) ocupa posição já ocupada (0, 0)   [<detached>, sem hint]
```

### Caso (d) — `d_colspan_overflow_auto.typ`: colspan overflow (auto)

`#grid(columns: 2, grid.cell(colspan: 5)[WIDE])`

```text
VANILLA (exit=1):
error: cell's colspan would cause it to exceed the available column(s)
  = hint: try placing the cell in another position or reducing its colspan

CRISTALINO ANTES (exit=1):
error: grid placement: cell colspan=5 excede num_cols=2   [<detached>, sem hint]
```

### Caso (e) — `e_invalid_column.typ`: coluna inválida

`#grid(columns: 2, grid.cell(x: 5, y: 0)[X])`

```text
VANILLA (exit=1):
error: cell could not be placed at invalid column 5   [sem hint]

CRISTALINO ANTES (exit=1):
error: grid placement: cell em x=5 colspan=1 excede num_cols=2
  → confirma o achado de P810: o cristalino confunde "invalid column" com "exceed num_cols".
```

### Caso (f) — `f_colspan_overflow_explicit.typ`: colspan overflow (x explícito)

`#grid(columns: 2, grid.cell(x: 1, colspan: 2)[W])`

```text
VANILLA (exit=1): error: cell's colspan would cause it to exceed the available column(s)
                  = hint: try placing the cell in another position or reducing its colspan
CRISTALINO ANTES (exit=1): error: grid placement: cell em x=1 colspan=2 excede num_cols=2
```

### Casos-limite medidos (definem a fronteira da validação de footer)

```text
h_footer_then_hline.typ  (footer seguido de grid.hline()):  VANILLA exit=0 · CRISTALINO exit=0
i_footer_last_ok.typ     (footer como último child):        VANILLA exit=0 · CRISTALINO exit=0
j_footer_mid_table_cells (footer seguido de table.cell):    VANILLA exit=1 "footer must end at the last row" · CRISTALINO ANTES exit=0
```

**Conclusão da sonda:** a regra vanilla é sobre **células do corpo depois do footer** — linhas (hline/vline) depois do footer são aceites. A validação foi implementada exactamente com essa fronteira.

### Pontos localizados (registados antes de tocar em código)

| Caso | Vanilla | Cristalino (ANTES) |
|---|---|---|
| footer fora do fim | `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:1889` (`bail!(footer_span, "footer must end at the last row")`) | sem validação — `native_grid` (`01_core/src/engine/stdlib/layout.rs:361-369`) e `native_table` (`01_core/src/engine/stdlib/structural.rs:839-847`) extraíam o footer de qualquer posição |
| conflito célula↔célula | `resolve.rs:1500-1504` (msg+hint) | `01_core/src/engine/layout/grid_placement.rs:123-130` (msg PT, sem hint) |
| span sobre célula prévia | `resolve.rs:1522-1530` (msg+hint distintos) | não distinguido (caía na mesma msg de conflito) |
| coluna inválida | `resolve.rs:2221-2225` (`cell could not be placed at invalid column {x}`, sem hint) | confundido com colspan overflow (`grid_placement.rs:100-108`) |
| colspan overflow | `resolve.rs:1413-1419` (msg+hint) | `grid_placement.rs:100-108` (explicit) e `:162-169` (auto), msgs PT sem hint |

## Passo 2 — Implementação

Ficheiros alterados (`git diff HEAD --stat` destes ficheiros, 2026-07-22T04:46Z: 6 ficheiros, +362/-64 — inclui testes):

- **`01_core/src/engine/layout/grid_placement.rs`** — mensagens em paridade verbatim:
  - Pass 1: nova guarda de coluna inválida **antes** da de colspan (`x.is_some() && col_start >= num_cols` → `cell could not be placed at invalid column {col_start}`, sem hint — distingue os dois erros que o cristalino confundia);
  - Pass 1: colspan overflow → `cell's colspan would cause it to exceed the available column(s)` + hint `try placing the cell in another position or reducing its colspan`;
  - Pass 1: conflito na posição de origem → `attempted to place a second cell at column {c}, row {r}` + hint `try specifying your cells in a different order`; conflito em posição spanned → `cell would span a previously placed cell at column {c}, row {r}` + hint `try specifying your cells in a different order or reducing the cell's rowspan or colspan` (distinção origem/span replicada do vanilla);
  - Pass 2 (auto): colspan overflow → mesma mensagem + hint do vanilla;
  - 5 testes novos (`p822_*`) com mensagem e hint exactos.
- **`01_core/src/engine/stdlib/layout.rs`** (`native_grid`) — célula do corpo depois do footer → `footer must end at the last row` (verbatim, span `args.span`); hline/vline depois do footer continuam aceites (fronteira medida); comentário de scope-out P772i actualizado (footer no meio deixou de ser scope-out silencioso).
- **`01_core/src/engine/stdlib/structural.rs`** (`native_table`) — mesma validação nos braços `Value::Content` e `Value::Str` (span `detached`, consistente com os erros existentes do ficheiro).
- **`01_core/src/engine/stdlib/mod.rs`** — 4 testes novos (`p822_native_grid/table_footer_fora_do_fim_rejeita`, `p822_native_grid_footer_no_fim_ok`, `p822_native_grid_hline_depois_do_footer_ok`).
- **`01_core/src/engine/layout/tests.rs`** — 2 testes existentes (P647) actualizados para as mensagens novas (`contains("conflito")` → msg vanilla; `contains("excede num_cols")` → msg vanilla). Sem alteração de lógica — só asserções de texto afectadas pela mudança de mensagem.
- **`01_core/src/engine/layout/grid.rs`** — comentário do error path actualizado (menciona as três condições de erro).

L0: nenhum prompt editado (não foi necessário `--fix-hashes`). O scope-out documentado no L0 (`layout.md` §P789) cobre o **conflito célula↔footer** (range absoluto do footer pós-placement) — ortogonal à validação de posição do footer implementada aqui; sem conflito com o L0 vigente. Pureza L1 mantida: só `format!`/strings; sem I/O, relógio, env ou estado global.

## Passo 3 — Validação (DEPOIS, literal)

### Testes novos confirmados a falhar ANTES da implementação

`cargo test -p typst-core p822` → `2 passed; 7 failed; 4451 filtered out` — os 7 falhados são exactamente os testes de mensagem/footer novos (os 2 controlos já passavam).

### DEPOIS — binário rebuildado, mesmos comandos da sonda

```text
a_footer_mid_table   CRISTALINO DEPOIS (exit=1): error: footer must end at the last row  ✓ verbatim vanilla
b_footer_mid_grid    CRISTALINO DEPOIS (exit=1): error: footer must end at the last row  ✓ verbatim vanilla (span real 1:5 via args.span)
c_conflict_cell_cell CRISTALINO DEPOIS (exit=1): error: attempted to place a second cell at column 0, row 0
                     hint: try specifying your cells in a different order                ✓ verbatim + hint
d_colspan_overflow_auto    DEPOIS (exit=1): error: cell's colspan would cause it to exceed the available column(s)
                     hint: try placing the cell in another position or reducing its colspan ✓ verbatim + hint
e_invalid_column     DEPOIS (exit=1): error: cell could not be placed at invalid column 5 ✓ verbatim, sem hint (como o vanilla)
f_colspan_overflow_explicit DEPOIS (exit=1): idem (d)                                        ✓ verbatim + hint
j_footer_mid_table_cells   DEPOIS (exit=1): error: footer must end at the last row         ✓ verbatim vanilla
```

### Controlos (sem regressão)

```text
g_control_ok (header+footer bem posicionados em table):  VANILLA exit=0 · CRISTALINO DEPOIS exit=0 ✓
h_footer_then_hline (hline depois do footer):            VANILLA exit=0 · CRISTALINO DEPOIS exit=0 ✓
i_footer_last_ok (footer no fim de grid):                VANILLA exit=0 · CRISTALINO DEPOIS exit=0 ✓
k_multipage_repeat (table multi-página com header/footer repeat: true):
                                                         VANILLA exit=0 · CRISTALINO DEPOIS exit=0 ✓
                                                         (comportamento de repetição inalterado — débito intacto, não regredido)
```

### Suítes (comando + contagem ANTES/DEPOIS)

| Suíte | ANTES | DEPOIS |
|---|---|---|
| `cargo test -p typst-core` | 4451 passed; **7 failed** (os testes P822 novos); 2 ignored — i.e. baseline pré-P822 4449 passed; 0 failed; 2 ignored + 9 testes novos | **4458 passed; 0 failed; 2 ignored** |
| `cargo test -p typst-infra` | 667 passed; 0 failed; 5 ignored (estado declarado) | **667 passed; 0 failed; 5 ignored** (inalterado) |

4449 + 9 testes novos = 4458 ✓ — a contagem bate com os testes declarados.

### Lint

`~/.cargo/bin/crystalline-lint .` → 6 warnings, todos **V7 "prompt órfão"** pré-existentes da working tree (`eval/field-access.md`, `layout/enum_item.md`, `model/document.md`, `stdlib/layout.md`, `stdlib/structural.md`, `infra/package_version_resolution.md`) — nenhum em ficheiro tocado por este passo, nenhum V3/V4/V5/V13/V14. **Zero violations novas.**

## Scope-outs e débitos (confirmados, não tentados)

- **Repeat-across-páginas (achado #9(a))**: header/footer continuam a renderizar uma única vez (scope-out `grid.rs:188-192`); débito grande registado desde P772i/P789 — **não corrigido neste passo, não regredido** (controlo `k_multipage_repeat` exit=0).
- **Conflito célula↔footer**: scope-out documentado no L0 (`layout.md` §P789) — range absoluto do footer só existe pós-placement no modelo splice; mantido.
- **Múltiplos headers por `level` / `repeat: false` / múltiplos footers**: scope-outs P772i mantidos. Nota: as mensagens de "mais do que um header/footer" do cristalino estão em PT (`grid: não pode haver mais do que um footer`) vs vanilla `cannot have more than one footer` (`resolve.rs:1246`) — divergência de texto **registada**, fora dos casos medidos em P810 §9(b); candidata a passo futuro de mensagens.
- **Spans `<detached>`**: mantidos nos erros de placement (trade-off P647 — elementos não carregam span) e nos erros de `native_table`; o vanilla anexa span da célula/footer. Registado como apresentação (P810 já o notava para o conflito célula↔header).
- **Larguras de coluna auto divergentes** (nota lateral de P810 §9): fora do âmbito; registada como pista do módulo de sizing.
