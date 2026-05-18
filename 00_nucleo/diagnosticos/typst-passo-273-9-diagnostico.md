# Diagnóstico Fase A P273.9.A — Containers estendidos (Grid + Stack + Pad — escopo 1γ)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.9.A.
**Magnitude**: S documental (~30 min — análise de 3 arms heterogéneos).
**Cluster**: Visualize / Gradient (encerra refino estrutural ampliado).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Vigésimo consumo directo de fonte** (cristalino post-P273.7+P273.8;
padrão DEBT-37 reused N=4 cumulativo; template Block save/restore
reused N=2 cumulativo + adaptação layout duplo para Stack/Pad
emergente).

---

## §A.1 — Inventário arm `Content::Grid` (caso fácil)

`01_core/src/rules/layout/grid.rs`:

- **Linha 31** — `pub(super) fn layout_grid(...)` ponto de entrada.
- **Linhas 318-385** — loop sobre cells; cada cell tem:
  - **Linha 355-356** — `body_w = (cell_w - inset_l - inset_r).max(0.0)`;
    `body_h = (cell_h - inset_t - inset_b).max(0.0)`. **Bbox da cell
    inner: sempre conhecido a 4 dimensions literais antes do body
    layout**.
  - **Linhas 364-365** — `saved_cell_ox = self.cell_origin_x;
    saved_cell_oy = self.cell_origin_y;` (DEBT-37 P84.6 save/restore).
  - **Linhas 366-368** — `enter_cell(Region::new(body_w, body_h))`
    cria sub-region para body.
  - **Linhas 369-370** — `self.cell_origin_x = Some(body_x);
    self.cell_origin_y = Some(body_y);` set legacy fields.
  - **Linhas 376-377** — `(cell_h_measured, cell_items) =
    self.layout_sub_frame_with_width(cell, body_x, body_w);` —
    **body layout duplo já existe via `layout_sub_frame_with_width`**.
  - **Linhas 382-385** — exit_cell + restore legacy fields.

**`parent_bbox` para Grid cell**: bbox = inner body rectangle =
`Rect { x: body_x, y: body_y, w: body_w, h: body_h }`. Todos 4
disponíveis pré-body. **Não precisa de layout duplo adicional** —
dimensões cell sempre literais pós-track resolution.

---

## §A.2 — Inventário arm `Content::Stack` (caso médio — layout duplo)

`01_core/src/rules/layout/mod.rs:1280-1322`:

- **Linha 1280** — `Content::Stack { children, dir, spacing }` arm.
- **Linhas 1284-1287** — flush_line antes (Stack é structural).
- **Linhas 1295-1299** — iter forward/reverse consoante `dir`.
- **Linhas 1301-1321** — loop:
  - **Vertical (TTB/BTT)**: cada child em "linha" própria;
    `cursor_y += space_pt` entre.
  - **Horizontal (LTR/RTL)**: layout inline; `cursor_x += space_pt`
    entre.

**Dimensions disponíveis**: **nenhuma literal**. Stack é content-based.

**Bbox medido via `measure_content_constrained`**:
- Existe handler Stack em `measure_content_constrained` (linha 2095):
  - Vertical: `max_w = max(child_w); sum_h = sum(child_h) + (n-1)*space_pt`.
  - Horizontal: `sum_w = sum(child_w) + (n-1)*space_pt; max_h = max(child_h)`.
- `&self` puro (sem mut) — pode ser invocado pré-layout para conhecer
  bbox antes de iterar children.

**`parent_bbox` para Stack**: medir via inline replicação do handler
Stack do `measure_content_constrained` (ou helper local) com
`max_width = self.available_width()`. Bbox =
`Rect { x: cursor_x, y: cursor_y, w: measured_w, h: measured_h }`
**após o flush_line inicial** (cursor já alinhado a line_start).

---

## §A.3 — Inventário arm `Content::Pad` (caso médio — layout duplo)

`01_core/src/rules/layout/mod.rs:1205-1237`:

- **Linha 1205** — `Content::Pad { body, sides }` arm.
- **Linhas 1212-1215** — resolve insets (left/top/right/bottom).
- **Linha 1218** — flush_line se cursor_x > line_start (structural).
- **Linha 1220** — `cursor_y += top` (avança INSET top).
- **Linhas 1222-1227** — save line_start_x/width; new line_start_x
  += left; cursor_x = new line_start_x; width -= right.
- **Linha 1229** — `self.layout_content(body)`.
- **Linha 1230** — flush_line.
- **Linha 1232** — `cursor_y += bottom`.
- **Linhas 1233-1236** — restore line_start_x/cursor_x/width.

**Bbox medido via `measure_content_constrained`**:
- Existe handler Pad em `measure_content_constrained` (linha 2061):
  - body_w_max = max_width - left - right;
  - `(body_w, body_h) = measure_content_constrained(body, body_w_max)`;
  - retorna `(body_w + left + right, body_h + top + bottom)`.
- Alternativa preferida P273.9: chamar
  `self.measure_content_constrained(body, available_w_inner)` directo
  obtendo body_w/body_h, então construir bbox.

**`parent_bbox` para Pad** (decisão semantic: bbox INNER ou OUTER?):
- **Decisão sub α — INNER (body region, sem insets)**: bbox =
  `Rect { x: cursor_x_inner, y: cursor_y_inner, w: body_w, h: body_h }`.
  Análogo a Block (que captura inner body via width+height literais).
  Gradient `relative=parent` recebe o body interno, paralela a vanilla.
- **Decisão sub β — OUTER (full pad rectangle)**: bbox =
  `Rect { x: line_start_x_orig, y: cursor_y_pre_top, w: saved_width -
  line_start_x_orig, h: body_h + top + bottom }`. Inclui insets.

**Recomendação spec**: **sub α (INNER)** — paridade Block; consistente
com convenção "parent_bbox = região onde o body é renderizado".

---

## §A.4 — Decisão 1 fixada: escopo 1γ (Grid + Stack + Pad)

**Decisão final (utilizador)**: **1γ** — extensão a Grid cell + Stack
+ Pad.

**Justificativa do utilizador**: ambição de cobertura máxima de
containers structural; aceitação consciente do risco regressão alto
e magnitude M; cluster Gradient encerrado com folga sem deixar
pendência específica para containers estruturais comuns.

**Trade-offs aceites**:
- Magnitude M (~80-150 LOC L1 esperado).
- Risco regressão tests P262-P273.8 alto — mitigado via defaults
  rigorosos (bbox populated apenas se measured_w/h > 0).
- Layout duplo via `measure_content_constrained` — custo de
  performance ~1.5-2× para Stack/Pad em pipelines que usem gradient
  relative=parent (não regression bit-exact).

---

## §A.5 — Decisão 2 fixada: bbox Grid cell

**Fixada**: **2α — bbox exacto cell** =
`Rect { x: body_x, y: body_y, w: body_w, h: body_h }`.

Razões:
1. `body_w` / `body_h` são `f64` literais já calculados (linhas 355-356).
2. Não precisa de `cell_origin_*` fields legacy (DEBT-37 P84.6 reused
   no espírito mas sem dependência directa).
3. Paridade vanilla — gradient `relative=parent` em cell Grid vê a
   inner cell rectangle.

Defaults: se `body_w <= 0.0 || body_h <= 0.0`, **não popular**
`parent_bbox` (cae no fallback page_bbox; análogo Decisão 3γ.2.γ).

---

## §A.6 — Decisão 3 fixada: bbox Stack + Pad

### Stack — bbox via measurement

`measured_stack_bbox`:
- Vertical: `(max_w, sum_h + (n-1)*space_pt)`.
- Horizontal: `(sum_w + (n-1)*space_pt, max_h)`.
- `cursor_x/y` no momento do save são `line_start + 0` / `cursor_y`
  (pós flush_line — Stack é structural).

Bbox = `Rect { x: cursor_x, y: cursor_y, w: measured_w, h: measured_h }`.

Defaults: se `measured_w <= 0.0 || measured_h <= 0.0`, não popular
(n=0 ou stack vazio).

### Pad — bbox INNER (sub α)

`measured_pad_inner_bbox`:
- `(body_w, body_h) = measure_content_constrained(body, available_inner)`.
- `cursor_x_inner = line_start_x + left`; `cursor_y_inner = cursor_y_pre_top + top`.

Bbox = `Rect { x: cursor_x_inner, y: cursor_y_inner, w: body_w, h: body_h }`.

Defaults: se `body_w <= 0.0 || body_h <= 0.0`, não popular.

---

## §A.7 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.8 | Save/restore 3 arms simultâneos amplo | Defaults rigorosos (popular apenas se w/h > 0); LIFO restore preserva contexto outer |
| Regressão DEBT-37 P246 | grid.rs ganha 1 statement adicional próximo de `cell_origin_*` save/restore | Padrão paralelo (independent state slot); tests `cell_origin_*` consumption preserved bit-exact |
| Custo perf layout duplo Stack/Pad | `measure_content_constrained` invocado em cada Stack/Pad | Acessível apenas em pipelines com gradient relative=parent (default Self_/None ignora) — custo zero no caso comum |
| Stack vazio populates bbox 0×0 | Edge case n=0 | Default `popular apenas se measured_w > 0 && measured_h > 0` |
| Pad com body vazio | measure retorna 0×0 | Idêntico — default protect |
| L1 cap soft 25 estourado | 1γ é M magnitude | Spec §A.5 1γ reconhece "M; ~80-150 LOC"; ADR-0094 Pattern 1 estouro hard cap registado |
| Tests novos disparam P273.6/P273.7 testes existentes | Defaults preservam P273.6/P273.7 bit-exact | Tests cobrem APENAS novos arms; Block/Boxed tests preserved literal |

---

## §A.8 — Critério de aceitação Fase A

- ✓ §A.1 cita arm Grid literal (`grid.rs:31` + body_w/body_h linhas
  355-356).
- ✓ §A.2 cita arm Stack literal (`mod.rs:1280-1322`) + handler
  Stack em `measure_content_constrained` (`mod.rs:2095`).
- ✓ §A.3 cita arm Pad literal (`mod.rs:1205-1237`) + handler Pad
  em `measure_content_constrained` (`mod.rs:2061`).
- ✓ §A.4 Decisão 1 fixada: **1γ** (Grid + Stack + Pad).
- ✓ §A.5 Decisão 2 fixada: **2α — bbox exacto cell**.
- ✓ §A.6 Decisão 3 fixada: bbox medido para Stack + Pad INNER.
- ✓ §A.7 risco "regressão P246 DEBT-37" mitigado por padrão paralelo
  independente.

**Fase A produzida — critério §A.8 cumprido absoluto.**

---

## §A.9 — Plano de implementação (Fase C)

### Cap LOC (ADR-0094 Pattern 1 — 1γ M magnitude reconhecido)

- **L1 hard cap**: ≤ 80 LOC (estouro hard cap P273.9.A 1α/1β
  registado; magnitude M ⇒ caps recalibrados acima).
- **L1 soft cap**: ≤ 60 LOC.
- **L3 hard cap**: 0 (não tocar export.rs).
- **Tests hard cap**: ≤ 12.
- **Tests soft cap**: ≤ 8.

### Estimativa por arm

| Arm | LOC esperado | Mecanismo |
|---|---|---|
| Grid cell | ~10 | Save + set + restore análogo Block |
| Stack | ~25 | Inline replicação handler `measure_content_constrained` Stack arm + bbox compose |
| Pad | ~15 | Single `measure_content_constrained` call + bbox compose |
| **Total L1** | **~50** | Folga vs cap soft 60 |

### Ordem literal

1. Fase A (este documento).
2. ADR-0091 anotação cumulativa décima.
3. L0 `entities/gradient.md` anotação P273.9.
4. `crystalline-lint --fix-hashes`.
5. Tests-first (~8 testes — cap soft 8 respeitado).
6. Implementação:
   - 6a. Grid cell (grid.rs).
   - 6b. Stack (mod.rs).
   - 6c. Pad (mod.rs).
7. Verificação final.

### Sub-padrões esperados

- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=3 → **N=4
  cumulativo** (Grid cell `parent_bbox` paralelo a `cell_origin_*`).
- **"Template-passo replicado literal"** N=1 → **N=2 cumulativo**
  (Stack + Pad replicam template P273.6/P273.7 com adaptação layout
  duplo).
- **"Sub-passos consecutivos do mesmo cluster"** N=4 → **N=5
  cumulativo emergente**.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo consumo
directo de fonte. Decisões 1γ + 2α + 3 fixadas; pronto para Fase C
(~50 LOC L1; ~8 testes; cleanup mecânico residual e DEBT-37 N=4).*
