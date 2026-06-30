---

# P512 — Materialização de Grid HLine/VLine (Prova de Conceito)

> **Passo:** 512
> **Data:** 2026-06-30
> **Foco:** Fechar empiricamente os 4 elementos de linha em grids/tables, começando por `grid.hline`/`grid.vline` como prova de conceito (menos complexidade semântica que `table.hline`/`table.vline`). Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação M-size com validação via sonda P512.
> **Tamanho:** M (~45 min de implementação + 15 min de validação).
> **ADR-0107 ACEITE** — paridade é linguagem, não mecânica.
> **ADR-0108 ACEITE** — medir antes de decidir; sonda A.0 satisfeita.
> **ADR-0109 ACEITE** — atomização de código.
> **ADR-0114 ACEITE** — gate duro satisfeito (9/9 ausentes); spec de materialização redigida.
> **Dependências:** Sonda P512 (gate duro satisfeito), P496 (table.header/table.footer com namespace anexado), P511 (math elements granulares fechados).

---

## 1. Contexto

A sonda P512 confirmou que **9/9 elementos** (table/grid HLine/VLine + curve elements) estão **completamente ausentes** no cristalino:

| Elemento | Parser | Variants | Construtores | Layout | Estado |
|----------|--------|----------|--------------|--------|--------|
| `table.hline` | FAIL | AUSENTE | AUSENTE | AUSENTE | Materializar |
| `table.vline` | FAIL | AUSENTE | AUSENTE | AUSENTE | Materializar |
| `grid.hline` | FAIL | AUSENTE | AUSENTE | AUSENTE | **Prova de conceito** |
| `grid.vline` | FAIL | AUSENTE | AUSENTE | AUSENTE | **Prova de conceito** |
| `curve.move` | FAIL | AUSENTE | AUSENTE | AUSENTE | P513 |
| `curve.line` | FAIL | AUSENTE | AUSENTE | AUSENTE | P513 |
| `curve.cubic` | FAIL | AUSENTE | AUSENTE | AUSENTE | P513 |
| `curve.quad` | FAIL | AUSENTE | AUSENTE | AUSENTE | P513 |
| `curve.close` | FAIL | AUSENTE | AUSENTE | AUSENTE | P513 |

**Recomendação da sonda:** Implementar `grid.hline`/`grid.vline` primeiro como prova de conceito. A API é idêntica à `table.hline`/`table.vline`, mas sem a complexidade semântica de headers/footers/cells.

---

## 2. Metodologia

Para cada sub-tarefa, implementar o elemento, correr a sonda P512, e comparar contra vanilla 0.15.0.

```bash
# Baseline sonda P512 (antes de tocar código):
for elem in grid.hline grid.vline table.hline table.vline; do
  echo "=== Baseline: $elem ==="
  case $elem in
    grid.hline) echo '#grid(columns: 2, grid.hline(), [A], [B])' > /tmp/sonda.typ ;;
    grid.vline) echo '#grid(columns: 2, grid.vline(), [A], [B])' > /tmp/sonda.typ ;;
    table.hline) echo '#table(columns: 2, table.hline(), [A], [B])' > /tmp/sonda.typ ;;
    table.vline) echo '#table(columns: 2, table.vline(), [A], [B])' > /tmp/sonda.typ ;;
  esac
  cargo run --release -p typst-wiring -- compile /tmp/sonda.typ /tmp/out.pdf 2>&1 | head -3
  echo "---"
done
# Esperado: 4/4 FAIL (parser não reconhece)

# Após cada sub-tarefa:
# Re-executar sonda e verificar MATCH
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## 3. Categorias

### 3.1 — `grid.hline` (512a)

**Estado baseline:** Parser falha com `esta função não tem campos`.

**Semântica:** Linha horizontal entre células de grid.

**Sintaxe vanilla 0.15.0:**
```typst
#grid(
  columns: 2,
  grid.hline(start: 0, end: 2, stroke: 1pt, position: top),
  [A], [B],
  [C], [D],
)
```

**Implementação:**

**Passo 1:** Adicionar `grid.hline` e `grid.vline` ao namespace anexado de `grid` (padrão P496 para `table.header`/`table.footer`).

```rust
// stdlib/structural.rs (ou onde grid é definido)
let mut grid_namespace = Scope::new();
grid_namespace.define("hline", Value::Func(native_grid_hline));
grid_namespace.define("vline", Value::Func(native_grid_vline));

let grid_func = Value::Func(Func::with_namespace(
    native_grid,
    grid_namespace,
));
```

**Passo 2:** Implementar `native_grid_hline`:

```rust
fn native_grid_hline(args: Args) -> SourceResult<Value> {
    let start = args.named.get("start")
        .and_then(|v| v.cast::<i64>().ok())
        .unwrap_or(0) as usize;
    let end = args.named.get("end")
        .and_then(|v| v.cast::<i64>().ok());
    let stroke = args.named.get("stroke")
        .and_then(|v| v.cast::<Stroke>().ok())
        .unwrap_or(Stroke::from(1.0)); // default 1pt
    let position = args.named.get("position")
        .and_then(|v| v.cast::<Str>().ok())
        .unwrap_or_else(|| "top".into());

    if !matches!(position.as_str(), "top" | "bottom") {
        return Err("position must be 'top' or 'bottom'");
    }

    Ok(Value::Content(Content::GridHLine(GridHLineElem {
        start,
        end,
        stroke,
        position: position.into(),
    })))
}
```

**Passo 3:** Adicionar `GridHLineElem` ao `Content`:

```rust
// entities/content.rs
pub enum Content {
    // ... variants existentes
    GridHLine(GridHLineElem),
    GridVLine(GridVLineElem),
}

#[derive(Clone, Debug, PartialEq)]
pub struct GridHLineElem {
    pub start: usize,
    pub end: Option<usize>, // None = até o final
    pub stroke: Stroke,
    pub position: EcoString, // "top" ou "bottom"
}

#[derive(Clone, Debug, PartialEq)]
pub struct GridVLineElem {
    pub start: usize,
    pub end: Option<usize>,
    pub stroke: Stroke,
    pub position: EcoString, // "left" ou "right"
}
```

**Passo 4:** Layout de `GridHLine`:

```rust
// layout/grid.rs
fn layout_grid_hline(elem: &GridHLineElem, ctx: &mut LayoutContext, grid: &GridLayout) -> Vec<Frame> {
    let start_col = elem.start;
    let end_col = elem.end.unwrap_or(grid.columns.len());
    let stroke_width = elem.stroke.width;

    let x_start = grid.column_x(start_col);
    let x_end = grid.column_x(end_col);
    let y = match elem.position.as_str() {
        "top" => grid.row_y_top(grid.current_row),
        "bottom" => grid.row_y_bottom(grid.current_row),
        _ => grid.row_y_top(grid.current_row),
    };

    let mut frame = Frame::new();
    frame.add_line(
        Point::new(x_start, y),
        Point::new(x_end, y),
        elem.stroke.clone(),
    );

    vec![frame]
}
```

**Nota:** O layout de grid precisa de acesso ao `GridLayout` (posições de colunas/linhas). Se o cristalino não tem `GridLayout` explícito, adaptar ao modelo existente.

**Validação:**
```typst
#grid(
  columns: 2,
  grid.hline(),
  [A], [B],
  [C], [D],
)
```
→ **MATCH** (linha horizontal entre células)

---

### 3.2 — `grid.vline` (512b)

**Estado baseline:** Parser falha com `esta função não tem campos`.

**Semântica:** Linha vertical entre células de grid.

**Implementação:**

```rust
fn native_grid_vline(args: Args) -> SourceResult<Value> {
    let start = args.named.get("start")
        .and_then(|v| v.cast::<i64>().ok())
        .unwrap_or(0) as usize;
    let end = args.named.get("end")
        .and_then(|v| v.cast::<i64>().ok());
    let stroke = args.named.get("stroke")
        .and_then(|v| v.cast::<Stroke>().ok())
        .unwrap_or(Stroke::from(1.0));
    let position = args.named.get("position")
        .and_then(|v| v.cast::<Str>().ok())
        .unwrap_or_else(|| "left".into());

    if !matches!(position.as_str(), "left" | "right") {
        return Err("position must be 'left' or 'right'");
    }

    Ok(Value::Content(Content::GridVLine(GridVLineElem {
        start,
        end,
        stroke,
        position: position.into(),
    })))
}
```

**Layout:**

```rust
fn layout_grid_vline(elem: &GridVLineElem, ctx: &mut LayoutContext, grid: &GridLayout) -> Vec<Frame> {
    let start_row = elem.start;
    let end_row = elem.end.unwrap_or(grid.rows.len());
    let stroke_width = elem.stroke.width;

    let y_start = grid.row_y_top(start_row);
    let y_end = grid.row_y_bottom(end_row - 1);
    let x = match elem.position.as_str() {
        "left" => grid.column_x_left(grid.current_col),
        "right" => grid.column_x_right(grid.current_col),
        _ => grid.column_x_left(grid.current_col),
    };

    let mut frame = Frame::new();
    frame.add_line(
        Point::new(x, y_start),
        Point::new(x, y_end),
        elem.stroke.clone(),
    );

    vec![frame]
}
```

**Validação:**
```typst
#grid(
  columns: 2,
  grid.vline(),
  [A], [B],
  [C], [D],
)
```
→ **MATCH** (linha vertical entre células)

---

### 3.3 — `table.hline` (512c)

**Estado baseline:** Parser falha com `função não tem campo 'hline'`.

**Semântica:** Idem `grid.hline`, mas para tabelas (com headers/footers).

**Implementação:** Reutilizar o padrão de `grid.hline`, mas adaptar ao `TableElem` (que tem headers/footers/cells).

```rust
// Adicionar ao namespace anexado de table (já existe em P496)
table_namespace.define("hline", Value::Func(native_table_hline));
table_namespace.define("vline", Value::Func(native_table_vline));

fn native_table_hline(args: Args) -> SourceResult<Value> {
    // Idem native_grid_hline, mas retorna TableHLineElem
    let start = args.named.get("start")
        .and_then(|v| v.cast::<i64>().ok())
        .unwrap_or(0) as usize;
    let end = args.named.get("end")
        .and_then(|v| v.cast::<i64>().ok());
    let stroke = args.named.get("stroke")
        .and_then(|v| v.cast::<Stroke>().ok())
        .unwrap_or(Stroke::from(1.0));
    let position = args.named.get("position")
        .and_then(|v| v.cast::<Str>().ok())
        .unwrap_or_else(|| "top".into());

    Ok(Value::Content(Content::TableHLine(TableHLineElem {
        start,
        end,
        stroke,
        position: position.into(),
    })))
}
```

**Validação:**
```typst
#table(
  columns: 2,
  table.hline(),
  [A], [B],
  [C], [D],
)
```
→ **MATCH** (linha horizontal entre células)

---

### 3.4 — `table.vline` (512d)

**Estado baseline:** Parser falha com `função não tem campo 'vline'`.

**Semântica:** Idem `grid.vline`, mas para tabelas.

**Implementação:** Reutilizar o padrão de `grid.vline`, adaptando ao `TableElem`.

**Validação:**
```typst
#table(
  columns: 2,
  table.vline(),
  [A], [B],
  [C], [D],
)
```
→ **MATCH** (linha vertical entre células)

---

## 4. Formato do Relatório de Resultados

```
| 512a grid.hline | Vanilla: ok | Cristalino: ok | MATCH | linha horizontal em grid |
| 512b grid.vline | Vanilla: ok | Cristalino: ok | MATCH | linha vertical em grid |
| 512c table.hline | Vanilla: ok | Cristalino: ok | MATCH | linha horizontal em table |
| 512d table.vline | Vanilla: ok | Cristalino: ok | MATCH | linha vertical em table |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

---

## 5. Testes de Não-Regressão

Após as 4 sub-tarefas, rodar o corpus P490+P500:

| Ficheiro | Vanilla | Cristalino pré-512 | Esperado pós-512 | Δ |
|---|---|---|---|---|
| (37 documentos) | — | MATCH | MATCH | preservados |

**Esperado:** 37/37 OK (não-regressão).

---

## 6. Critério de Fecho

- [ ] 512a implementado: `grid.hline()` funciona.
- [ ] 512b implementado: `grid.vline()` funciona.
- [ ] 512c implementado: `table.hline()` funciona.
- [ ] 512d implementado: `table.vline()` funciona.
- [ ] 4 testes unitários novos passam (`p512_grid_hline`, `p512_grid_vline`, `p512_table_hline`, `p512_table_vline`).
- [ ] Corpus P490+P500: 37/37 OK (não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (L0 prompts + L1 entities + L1 stdlib + L3 layout).
- [ ] Sentinela `p512_grid_table_lines` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p512.md` produzido.

---

## 7. Próximo Passo (P513)

Com P512 fechado, as brechas restantes de linguagem são:

| Brecha | Tamanho | Impacto |
|--------|---------|---------|
| Curve elements (`curve.move`, `curve.line`, `curve.cubic`, `curve.quad`, `curve.close`) | M | Baixo — gráficos vetoriais |

**Recomendação:** P513 = **Curve Elements** — 5 elementos de path/curva. São M-size, mas impacto baixo (gráficos vetoriais não são core de documentos textuais).

Alternativa: Iniciar **Trilha 5 (fontdb)** em paralelo.

---

## A. Apêndice — Referência Rápida

```typst
// 512a: grid.hline
#grid(
  columns: 2,
  grid.hline(start: 0, end: 2, stroke: 1pt, position: top),
  [A], [B],
  [C], [D],
)

// 512b: grid.vline
#grid(
  columns: 2,
  grid.vline(start: 0, end: 2, stroke: 1pt, position: left),
  [A], [B],
  [C], [D],
)

// 512c: table.hline
#table(
  columns: 2,
  table.hline(),
  [A], [B],
  [C], [D],
)

// 512d: table.vline
#table(
  columns: 2,
  table.vline(),
  [A], [B],
  [C], [D],
)
```
