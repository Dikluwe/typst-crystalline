# Relatório de Paridade — P647

**Passo:** 647  
**Data:** 2026-07-09  
**Foco:** Grid inválida (conflito de células / colspan fora dos limites) deve produzir erro, em vez de renderizar vazio em silêncio.  
**Dependências:** P633 (caso 7 confirmado), P638 (comportamento do vanilla confirmado — não seguido aqui).

---

## 1. Nota de enquadramento

Este passo é uma **melhoria além do vanilla**, não uma correcção de paridade. P638 confirmou que o vanilla também não reporta este problema, mas por um mecanismo diferente (não detecta o conflito, em vez de detectar e descartar). O critério de aceitação do cristalino é, portanto, produzir um erro claro — não bater byte-a-byte com o vanilla.

---

## 2. Sonda

### 2.1 Tipos de conflito detectados por `place_cells`

`place_cells` (`01_core/src/rules/layout/grid_placement.rs:64`) já detecta três situações de erro:

1. **`num_cols == 0`** (`grid_placement.rs:68`) — erro interno, improvável no uso normal.
2. **Colspan excede `num_cols`** (`grid_placement.rs:99-108` para explicit; `grid_placement.rs:162-170` para auto) — mensagem indica `colspan` e `num_cols`.
3. **Overlap de células explicit** (`grid_placement.rs:119-132`) — mensagem indica posição da célula e coordenada já ocupada.

### 2.2 Decisão: erro que pára a compilação

Decisão: propagar o erro como **erro fatal de layout** (via `layout_errors`), não como aviso.

Razão:

- Grid inválida é um erro estrutural do documento; continuar a renderizar vazio oculta o problema.
- Consistente com os outros seis casos de P633, todos convertidos para erro.
- `place_cells` já devolve `SourceResult`; usar o mecanismo `layout_errors` (P644/P645) é a forma menos invasiva de propagar o diagnóstico até à pipeline.
- Aviso que continua renderizando vazio seria menos útil para o utilizador.

---

## 3. Implementação

### 3.1 `01_core/src/rules/layout/grid.rs:322`

O código anterior usava `place_cells(cells, num_cols).unwrap_or_default()`, descartando qualquer erro e renderizando a grid vazia.

Alterado para:

```rust
let placed_cells: Vec<PlacedCell> = match place_cells(cells, num_cols) {
    Ok(placed) => placed,
    Err(diagnostics) => {
        self.layout_errors.extend(diagnostics);
        self.cell_align = saved_cell_align;
        return;
    }
};
```

Os diagnósticos são adicionados a `Layouter.layout_errors` (já `Vec<SourceDiagnostic>` desde P645), `cell_align` é restaurado, e a função retorna cedo. A pipeline em L3 propagará o erro como falha de compilação.

### 3.2 Testes

Adicionados em `01_core/src/rules/layout/tests.rs`:

- `p647_grid_explicit_overlap_produz_layout_error`: duas células explicitas em `(0,0)` geram `layout_errors` com mensagem de conflito.
- `p647_grid_colspan_maior_que_num_cols_produz_layout_error`: célula auto com `colspan=3` numa grid de 2 colunas gera `layout_errors` com mensagem de excesso.

---

## 4. Validação

### 4.1 Testes automáticos

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram**.

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

### 4.2 CLI — overlap explicito

```typst
#grid(
  columns: 2,
  grid.cell(x: 0, y: 0, colspan: 2)[A],
  grid.cell(x: 0, y: 0)[B],
)
```

Saída:

```text
p647-explicit-overlap.typ:<detached>: error: grid placement: conflito — célula explicit (x=0, y=0) ocupa posição já ocupada (0, 0)
```

### 4.3 CLI — colspan excede num_cols

```typst
#grid(
  columns: 2,
  grid.cell(colspan: 3)[A],
)
```

Saída:

```text
p647-colspan-excess.typ:<detached>: error: grid placement: cell colspan=3 excede num_cols=2
```

### 4.4 Grid válida sem regressão

Grids válidas (incluindo o exemplo do passo com duas células `colspan: 2` numa grid de 2 colunas, que não entram em conflito pelo algoritmo de auto-placement) continuam a renderizar normalmente.

---

## 5. Decisão

- Conflitos de grid e colspan excessivo propagam-se como erros de layout.
- A grid não renderiza vazia em silêncio.
- A mensagem identifica o tipo de conflito e a posição envolvida.
- Registado explicitamente como melhoria além do vanilla, não correcção de paridade.

---

## 6. Estado de fecho

- [x] Sonda completa; decisão erro vs. aviso registada com razão.
- [x] Conflitos de grid produzem diagnóstico claro.
- [x] Grid válida sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p647.md`.
- [x] Registado como melhoria além do vanilla.

---

## 7. Hash do commit

`HASH_A_PREENCHER`
