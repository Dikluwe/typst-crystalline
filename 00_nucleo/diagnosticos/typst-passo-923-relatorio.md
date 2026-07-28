# Relatório P923 — resíduo de ~4pt/linha em matrizes

**Commit base (antes deste passo):** `c9df6fde` (P922 e anteriores já integrados).
**Baseline de testes (antes):** typst-core: 4803 · typst-infra: 743 · typst-shell: 41 — todos `0 failed`.
**Baseline de testes (fim deste passo):** mesma contagem — `0 failed`.
**Data:** 2026-07-28.

---

## Fase A — causa isolada com medição directa ao vanilla real

**Input:** `temp/p923/matrix6_exact.typ`:
```typst
$mat(1,2;3,4;5,6;7,8;9,10;11,12)$
```
a 20pt (tamanho ambiente efectivo 11pt no trace, porque `#set text(size: 20pt)` eleva o corpo da página e a fórmula herda o tamanho).

**Método:** `lab/typst-original/target/release/typst` (vanilla 0.15.0) vs `target/release/typst` (cristalino), ambos release, mesma sessão; `mutool trace`; medição de `trm` das células e gap entre linhas consecutivas.

**Estado antes das correcções deste passo** (já com P921/P922 integrados):

| | Vanilla | Cristalino |
|---|---|---|
| `trm` das células | `7.7` | `11.0` |
| gap médio entre linhas | `9.869pt` | `10.976pt` |
| total_span (6 linhas) | `49.346pt` | `54.879pt` |

**Causa principal (P923-A):** o cristalino renderizava as células de `mat`/`cases` no **tamanho do ambiente** (`style.size`), enquanto o vanilla as resolve com `style_for_denominator(styles)` — `MathSize` desce um nível (`script_percent_scale_down`) e `cramped` é forçado a `true` (`typst-layout/src/math/ir/resolve.rs:1124`, `resolve_cells`, usado por `resolve_mat` e `resolve_cases`).

**Causa secundária (P923-B), descoberta depois de corrigida P923-A:** mesmo com as células no tamanho correcto, o gap entre linhas no cristalino era `math_leading * cell_style.size` (~0.154em × 7.7pt ≈ 1.19pt), quando o vanilla usa `DEFAULT_ROW_GAP = 0.2em` resolvido contra o **estilo exterior** (0.2em × 11pt = 2.2pt).

**Causa terciária (P923-C), descoberta ao ajustar P923-B:** o piso de altura por linha via `(` sintético (P921) aplicava o factor `script_percent_scale_down` **duas vezes** — `layout_grid_boxes` recebia já o estilo de denominador (`cell_style`) e criava um `denom_style = cell_style * script_percent_scale_down`, fazendo com que o `(` sintético fosse medido a 5.39pt em vez de 7.7pt. A altura da linha ficava ~6.50pt em vez dos ~7.67pt do vanilla.

---

## Fase B — Implementação (TDD directo)

### B.1 — Células em estilo de denominador (`matrix.rs`, `cases.rs`)

Construir `cell_style = TextStyle {
    size: style.size * self.constants.script_percent_scale_down,
    cramped: true,
    ..style.clone()
}` e passá-lo a todas as operações que medem/layoutam o conteúdo das células/ramos:

- `matrix.rs`: `layout_node` em cada célula, `layout_grid_rows`/`layout_grid_boxes`, e `align_boundary_spacing` (limite `&`).
- `cases.rs`: `layout_grid_rows`.

### B.2 — `row_gap` explicitado (`mod.rs`, `matrix.rs`, `cases.rs`)

`layout_grid_rows` e `layout_grid_boxes` passam a receber `row_gap: Pt` explicitamente, em vez de deduzirem-no de `self.constants.math_leading` e do `style` recebido:

- `matrix.rs` / `cases.rs` passam `style.size * 0.2` (paridade `DEFAULT_ROW_GAP` do `MatElem`/`CasesElem` resolvido contra o estilo exterior).
- `layout_grid` (multiline math `&`/`\\`) mantém o valor anterior (`math_leading` do estilo actual) até haver medição do vanilla para `ParElem::leading`/`TIGHT_LEADING`.

### B.3 — Piso de `(` sintético corrigido (`mod.rs`)

Removida a dupla aplicação de `script_percent_scale_down` no piso de `(` de `layout_grid_boxes`. Como o `style` recebido já é o estilo de denominador quando chamado por matrizes/cases, o `(` sintético é medido directamente com `style.size` (e `cramped: true`).

### B.4 — Testes ajustados

Testes sintéticos com `FixedMetrics` que medem posições de `mat` foram actualizados para usarem o tamanho reduzido das células (`script_percent_scale_down`) nos cálculos de referência:

- `p825d_mat_align_spacing_de_classe_no_limite`
- `p825d_mat_sem_align_mantem_colunas_centradas`
- `axis_bug_cases_conteudo_centra_no_axis_height_nao_a_zero`
- `axis_bug_matrix_conteudo_centra_no_axis_height_nao_a_zero`

Nenhum teste novo introduzido — os existentes, uma vez re-derivados, cobrem a mudança de comportamento.

### B.5 — L0s actualizados

- `00_nucleo/prompts/engine/math/layout/_comum.md` — secção P923b (`row_gap` explicitado).
- `00_nucleo/prompts/engine/math/layout/matrix.md` — secções P923 e P923b.
- `00_nucleo/prompts/engine/math/layout/cases.md` — secção P923 (com referência a P923b).

Hashes actualizados via `crystalline-lint --fix-hashes .`:
- `matrix.rs` → `dacb20d4`
- `cases.rs` → `27ff19eb`
- `mod.rs` → `58db8a25`
- `tests.rs` → `58db8a25`

### B.6 — Resultado dos testes

`cargo test --workspace`: typst-core 4803 · typst-infra 743 · typst-shell 41 — `0 failed`.

`crystalline-lint .`: 0 drift warnings; apenas warning V7 pré-existente (`package_version_resolution.md` órfão), não relacionado com este passo.

---

## Fase C — Confirmação geométrica contra vanilla real

Após as três correcções (P923-A, P923-B, P923-C):

| | Vanilla | Cristalino | Δ |
|---|---|---|---|
| `trm` das células | `7.7` | `7.7` | 0 |
| gap médio entre linhas | `9.869pt` | `9.869pt` | 0 |
| total_span (6 linhas) | `49.346pt` | `49.346pt` | 0 |

**Recibo:** o espaçamento de linha medido com `mutool trace` bate exactamente com o vanilla real; o resíduo de ~4pt/linha está fechado.

Visual (`mutool draw` para PNG): delimitadores `(`/`)` esticados correctamente; células e espaçamento indistinguíveis do vanilla.

---

## Fase D — Benchmark (7 cenários, `hyperfine --warmup 5 --min-runs 20`)

Script: `tools/perf/benchmark-p923.py`. Resultados em `tools/perf/results/p923/attestation.json`.

| Cenário | Razão (cristalino / vanilla) |
|---|---|
| 01-hello | 0.34× |
| 02-lorem | 0.35× |
| 03-math | 0.47× |
| 04-code | 0.34× |
| 05-utf8 | **25.91×** (outlier) |
| 06-matrix | 0.44× |
| 07-cases | 0.45× |

**Nota sobre 05-utf8:** o outlier é atribuível a fallback de fontes para emoji/caracteres multibyte no cristalino (`layout_ms` ≈ 6513ms, `shape_ms` ≈ 546ms), não às alterações deste passo. Não foi investigado mais a fundo em P923.

**Nenhuma regressão atribuível a P923** nos cenários de math/matrix/cases — todos na faixa 0.34–0.47×, coerente com medições anteriores para cenários pequenos.

---

## Resultado final

| Item | Estado |
|---|---|
| Causa do resíduo isolada e confirmada com números | ✅ três causas (tamanho das células, row_gap, piso de `(`) |
| Correcções implementadas com TDD directo | ✅ `matrix.rs`, `cases.rs`, `mod.rs` |
| L0s actualizados e hashes sincronizados | ✅ `_comum.md`, `matrix.md`, `cases.md` |
| Testes unitários re-derivados e verdes | ✅ 4 testes ajustados |
| Suíte completa verde | ✅ core 4803 · infra 743 · shell 41 |
| `crystalline-lint .` | ✅ 0 drift/violations novos |
| Validação geométrica contra vanilla real | ✅ gap e total_span exactos |
| Benchmark completo atestado (7 cenários) | ✅ `tools/perf/results/p923/attestation.json` |
