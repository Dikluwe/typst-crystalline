# P512 — Spec de Materialização — Table/Grid HLine/VLine e Curve Elements

> **Passo:** 512-spec
> **Data:** 2026-06-30
> **Tipo:** Spec de materialização (gate duro ADR-0114 satisfeito pela sonda A.0)
> **Dependências:** `00_nucleo/diagnosticos/sonda-p512-table-grid-curve.md`
> **ADRs:** ADR-0107 (paridade de linguagem), ADR-0109 (atomização), ADR-0026 (Content enum fechado)

---

## 1. Resumo Executivo

A sonda A.0 do P512 concluiu que **9/9 elementos estão completamente ausentes** no cristalino: não existem variants em `Content`, construtores nativos, nem layout handlers. O parser falha para todos (acesso a campos inexistentes ou função desconhecida).

Esta spec define o trabalho de materialização necessário para fechar a brecha.

---

## 2. Elementos a Materializar

### 2.1 `table.hline` / `table.vline`

- **Semântica:** Linhas horizontais/verticais entre células de uma tabela.
- **Sintaxe alvo:**
  ```typst
  #table(columns: 2, table.hline(), [A], [B])
  #table(columns: 2, table.vline(), [A], [B])
  ```
- **API:**
  - `table.hline(start: int = 0, end: int = auto, stroke: stroke = 1pt, position: auto | top | bottom = auto)`
  - `table.vline(start: int = 0, end: int = auto, stroke: stroke = 1pt, position: auto | left | right = auto)`
- **Mecânica proposta:** Adicionar campos `hlines: Vec<TableLine>` e `vlines: Vec<TableLine>` a `TableElem`. Os construtores `table.hline()`/`table.vline()` devolvem `Content` especial que o construtor `table()` reconhece e coleta, ou então `Value::Content(Content::TableHLine(...))` integrado diretamente na célula.
- **Decisão arquitetural pendente:** Usar linhas como células especiais no `children` do `table()` vs campos separados no `TableElem`.
- **L0 necessário:**
  - `00_nucleo/prompts/entities/elements/table_hline.md`
  - `00_nucleo/prompts/entities/elements/table_vline.md`
  - `00_nucleo/prompts/rules/stdlib/table_hline.md`
  - `00_nucleo/prompts/rules/stdlib/table_vline.md`

### 2.2 `grid.hline` / `grid.vline`

- **Semântica:** Idem `table.hline`/`table.vline`, mas para `grid()` (sem cabeçalho/rodapé semântico).
- **Sintaxe alvo:**
  ```typst
  #grid(columns: 2, grid.hline(), [A], [B])
  #grid(columns: 2, grid.vline(), [A], [B])
  ```
- **API:** Igual a `table.hline`/`table.vline`.
- **Mecânica proposta:** API idêntica às linhas de `table`; pode reutilizar as mesmas estruturas internas com um flag `is_table: bool`.
- **L0 necessário:**
  - `00_nucleo/prompts/entities/elements/grid_hline.md`
  - `00_nucleo/prompts/entities/elements/grid_vline.md`
  - `00_nucleo/prompts/rules/stdlib/grid_hline.md`
  - `00_nucleo/prompts/rules/stdlib/grid_vline.md`

### 2.3 `curve.move`

- **Semântica:** Mover a caneta para um ponto (início de um path).
- **Sintaxe alvo:** `#curve.move((0, 0))`
- **API:** `curve.move(pos: array<length>) -> CurveCommand`
- **Variant `Content`:** `CurveMove { to: Point }`.

### 2.4 `curve.line`

- **Semântica:** Linha reta até um ponto.
- **Sintaxe alvo:** `#curve.line((100pt, 0pt))`
- **API:** `curve.line(to: array<length>) -> CurveCommand`
- **Variant `Content`:** `CurveLine { to: Point }`.

### 2.5 `curve.cubic`

- **Semântica:** Curva cúbica de Bézier.
- **Sintaxe alvo:** `#curve.cubic((50pt, 50pt), (100pt, 0pt))` (com controlo 1 e fim; controlo 0 implícito) ou `#curve.cubic((0,0), (50,50), (100,0))` (controlo 0, controlo 1, fim).
- **API:** `curve.cubic(c0: array<length>, c1: array<length>, to: array<length>) -> CurveCommand` (ou 2-arg variant).
- **Variant `Content`:** `CurveCubic { c0: Point, c1: Point, to: Point }`.

### 2.6 `curve.quad`

- **Semântica:** Curva quadrática de Bézier.
- **Sintaxe alvo:** `#curve.quad((50pt, 50pt), (100pt, 0pt))`
- **API:** `curve.quad(c0: array<length>, to: array<length>) -> CurveCommand`
- **Variant `Content`:** `CurveQuad { c0: Point, to: Point }`.

### 2.7 `curve.close`

- **Semântica:** Fechar o path atual.
- **Sintaxe alvo:** `#curve.close()`
- **API:** `curve.close() -> CurveCommand`
- **Variant `Content`:** `CurveClose`.

### 2.8 `curve()` (container)

- **Semântica:** Recebe uma sequência de `CurveCommand` e renderiza um path.
- **Sintaxe alvo:**
  ```typst
  #curve(
    curve.move((0, 0)),
    curve.line((100pt, 0pt)),
    curve.line((50pt, 100pt)),
    curve.close(),
  )
  ```
- **Variant `Content`:** `Curve { commands: Vec<CurveCommand>, fill: Option<Color>, stroke: Option<Stroke> }`.

---

## 3. Plano de Implementação

### Fase 1 — Prompts L0

Antes de qualquer código, redigir e guardar os Prompts L0:

1. `00_nucleo/prompts/entities/elements/table_hline.md`
2. `00_nucleo/prompts/entities/elements/table_vline.md`
3. `00_nucleo/prompts/entities/elements/grid_hline.md`
4. `00_nucleo/prompts/entities/elements/grid_vline.md`
5. `00_nucleo/prompts/entities/elements/curve.md`
6. `00_nucleo/prompts/rules/stdlib/table_hline.md`
7. `00_nucleo/prompts/rules/stdlib/table_vline.md`
8. `00_nucleo/prompts/rules/stdlib/grid_hline.md`
9. `00_nucleo/prompts/rules/stdlib/grid_vline.md`
10. `00_nucleo/prompts/rules/stdlib/curve.md`

### Fase 2 — Table/Grid Lines (L1/L3/L4)

1. Adicionar structs `TableHLine`, `TableVLine`, `GridHLine`, `GridVLine` (ou `TableLine` partilhado).
2. Modificar `TableElem`/`GridElem` para aceitar linhas (como células especiais ou campos).
3. Implementar construtores nativos e registá-los no módulo `table`/`grid`.
4. Atualizar layout de tabelas/grids para desenhar linhas.

### Fase 3 — Curve (L1/L3/L4)

1. Adicionar `CurveCommand` enum e `Curve` struct em `Content`.
2. Implementar `curve`, `curve.move`, `curve.line`, `curve.cubic`, `curve.quad`, `curve.close`.
3. Registar funções no scope global e no módulo `curve`.
4. Implementar layout que converte comandos para path PDF.

### Fase 4 — Testes

- Snippets canônicos para cada elemento.
- Corpus P490+P500: 37/37 OK (não-regressão).
- `cargo test`: todos passam.

---

## 4. Ordem de Implementação Recomendada

| Ordem | Elemento | Justificação |
|---|---|---|
| 1 | `grid.hline` / `grid.vline` | Menor complexidade semântica; valida API de linhas. |
| 2 | `table.hline` / `table.vline` | Reutiliza API de grid; adiciona semântica de tabela. |
| 3 | `curve()` + `curve.move`/`line`/`close` | Base para paths. |
| 4 | `curve.quad` / `curve.cubic` | Extensão de paths. |

---

## 5. Critério de Aceitação

- [ ] `grid.hline()`/`grid.vline()` funcionam dentro de `#grid()`.
- [ ] `table.hline()`/`table.vline()` funcionam dentro de `#table()`.
- [ ] `curve(...)` renderiza paths com `move`/`line`/`quad`/`cubic`/`close`.
- [ ] Corpus P490+P500: 37/37 OK.
- [ ] `cargo test`: todos passam.
- [ ] `crystalline-lint .`: 0 violations.
- [ ] Prompts L0 guardados e hashes fixados.

---

## 6. Notas

- **Não-objectivos:** Gradientes/fills complexos em `curve`; linhas com padrões; linhas em tabelas com células mescladas.
- **Paridade:** medida ao nível da linguagem (sintaxe, semântica, morfologia), não ao nível de bytes de PDF (ADR-0107).
