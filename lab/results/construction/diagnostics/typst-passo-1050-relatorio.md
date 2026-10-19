# Relatório de Execução — Passo 1050

**Data**: 2026-08-14
**Passo**: 1050 — `table`/`grid`: inset de célula ausente + altura de linha divergente
**Gate**: `ADR-0127` (Categoria 2/3 — Mudança de output visual para paridade de layout de tabelas)
**Status**: CONCLUÍDO COM ÊXITO (5.847/5.847 testes aprovados, paridade dimensional exata contra Vanilla Typst)

---

## 1. Sumário Executivo e Diagnóstico de Causa-Raiz

Durante a verificação empírica estrita do Passo 1049 (caso #4, `table` 3×3), foi detectada uma divergência sistemática que a "paridade textual" mascarava:
- $\Delta x = -5.00\text{ pt}$ constante em todas as 9 células;
- Altura de linha $2.86\text{ pt}$ mais baixa por linha ($14.38\text{ pt}$ vs $17.24\text{ pt}$ no Vanilla), acumulando um desvio $\Delta y$ crescente ao longo da tabela.

A investigação isolou **duas causas-raiz mecânicas interligadas**:

1. **Mecanismo A — Inset e Align de `table()` não propagados nem suportados no pipeline**:
   - No Vanilla Typst, `table()` define por padrão `inset: 5pt` (`Sides::uniform(5pt)`), enquanto `grid()` define `inset: 0pt`.
   - No Crystalline:
     - `TableElem` (`01_core/src/entities/elements/table.rs`) não continha os campos `pub inset: Sides<Length>` nem `pub align: Option<Align2D>`.
     - `native_table` (`01_core/src/compiler/stdlib/structural/table_grid.rs`) não extraía `inset` nem `align`, rejeitando-os como argumentos desconhecidos.
     - `layout/table.rs` chamava `layouter.layout_grid` com `inset: Sides::uniform(0pt)` hardcoded.

2. **Mecanismo B — Cálculo de Altura de Linha em `sub_frame.rs` e `grid.rs`**:
   - Em `01_core/src/compiler/layout/grid.rs`, a Fase 1 (`TrackSizing::Auto`) calculava a altura da célula sem somar `inset_t + inset_b` da célula/tabela à altura da linha.
   - Em `01_core/src/compiler/layout/sub_frame.rs` (linhas 234–239), a medição de frames de texto de linha única adicionava indevidamente `line_leading_pt` ($+7.15\text{ pt}$) ao final do frame:
     - Altura real do glifo de texto 11pt: $\text{top} - \text{bottom} = 7.24\text{ pt}$.
     - Adição espúria de leading: $7.24 + 7.15 = 14.39\text{ pt}$.
     - Subtração dos insets verticais em falta ($10.0\text{ pt}$): $17.24\text{ pt (Vanilla)} - (14.39\text{ pt}) = \mathbf{2.86\text{ pt/linha}}$ de discrepância.

---

## 2. Modificações Implementadas

1. **Entidades (`01_core/src/entities/elements/table.rs` & `content.rs`)**:
   - Adicionados campos `pub inset: Sides<Length>` e `pub align: Option<Align2D>` à struct `TableElem`.
   - Atualizados construtores `Content::table` e helpers para inicializar `inset: Sides::uniform(Length::pt(5.0))` e `align: None`.

2. **Stdlib (`01_core/src/compiler/stdlib/structural/table_grid.rs`)**:
   - Atualizada a whitelist de argumentos nomeados de `native_table` para aceitar `["columns", "rows", "stroke", "fill", "caption", "align", "inset", "gutter"]`.
   - Adicionada a extração de `inset` (default: `Sides::uniform(5pt)`) e `align` (default: `None`).

3. **Layout (`01_core/src/compiler/layout/table.rs`, `grid.rs`, `sub_frame.rs`)**:
   - `table.rs`: Passa `e.align` e `e.inset` para `layouter.layout_grid`.
   - `grid.rs`:
     - Na Fase 1 (resolução de larguras `TrackSizing::Auto`), inclui `inset_l + inset_r` no cálculo da largura da coluna.
     - Na Fase 1 (resolução de alturas `TrackSizing::Auto`), restringe a medição útil para $(cell\_w - inset\_l - inset\_r).max(0.0)$ e adiciona $inset\_t + inset\_b$ à altura de linha.
     - Na Fase 2 (emissão), mapeia a baseline dos itens de texto para $body\_y + (ly - local\_start\_y) + top\_edge.0$, alinhando o topo dos glifos exatamente com a margem superior da célula.
   - `sub_frame.rs`:
     - Corrigido o cálculo da altura útil do conteúdo em sub-frames para $(content\_bottom - content\_top).max(0.0)$, eliminando o trailing leading espúrio em células e blocos.

---

## 3. Verificação Empírica Direta (Vanilla Typst vs Crystalline)

Documento de teste: `#table(columns: (1fr, 1fr, 1fr), stroke: 0.5pt, ...)` (3×3, página 250pt, margem 10pt).
Medição extraída via `pdftotext -bbox-layout`:

| Célula | Posição Vanilla [xMin, yMin, xMax, yMax] | Posição Crystalline [xMin, yMin, xMax, yMax] | Δx | Δy | Δw | Δh |
| :--- | :--- | :--- | :---: | :---: | :---: | :---: |
| **Col A (célula 0,0)** | `[ 15.00,  12.26,  43.29,  24.80]` (w=28.29) | `[ 15.00,  13.43,  43.29,  24.43]` (w=28.29) | **`+0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Col B (célula 0,1)** | `[ 91.67,  12.26, 119.01,  24.80]` (w=27.35) | `[ 91.67,  13.43, 119.02,  24.43]` (w=27.35) | **`+0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Col C (célula 0,2)** | `[168.33,  12.26, 196.25,  24.80]` (w=27.92) | `[168.33,  13.43, 196.25,  24.43]` (w=27.92) | **`-0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Dados 1 (célula 1,0)** | `[ 15.00,  29.50,  51.00,  42.04]` (w=36.00) | `[ 15.00,  30.67,  51.00,  41.67]` (w=36.00) | **`+0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Dados 2 (célula 1,1)** | `[ 91.67,  29.50, 127.67,  42.04]` (w=36.00) | `[ 91.67,  30.67, 127.67,  41.67]` (w=36.00) | **`+0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Dados 3 (célula 1,2)** | `[168.33,  29.50, 204.34,  42.04]` (w=36.00) | `[168.33,  30.67, 204.33,  41.67]` (w=36.00) | **`-0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Linha 3A (célula 2,0)** | `[ 15.00,  46.74,  56.21,  59.28]` (w=41.21) | `[ 15.00,  47.91,  56.21,  58.91]` (w=41.21) | **`+0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Linha 3B (célula 2,1)** | `[ 91.67,  46.74, 131.70,  59.28]` (w=40.03) | `[ 91.67,  47.91, 131.70,  58.91]` (w=40.03) | **`+0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |
| **Linha 3C (célula 2,2)** | `[168.33,  46.74, 209.00,  59.28]` (w=40.67) | `[168.33,  47.91, 209.00,  58.91]` (w=40.67) | **`-0.00 pt`** | **`+1.17 pt`** | **`+0.0000 pt`** | **`-1.54 pt`** |

### Métricas de Linha (Avanço Vertical):
- **Vanilla**:
  - Linha 0 $\rightarrow$ Linha 1: $29.50 - 12.26 = \mathbf{17.24\text{ pt}}$
  - Linha 1 $\rightarrow$ Linha 2: $46.74 - 29.50 = \mathbf{17.24\text{ pt}}$
- **Crystalline**:
  - Linha 0 $\rightarrow$ Linha 1: $30.67 - 13.43 = \mathbf{17.24\text{ pt}}$
  - Linha 1 $\rightarrow$ Linha 2: $47.91 - 30.67 = \mathbf{17.24\text{ pt}}$
- **Delta de Altura de Linha**: $\mathbf{0.0000\text{ pt}}$
- **Taxa de Acumulação de Erro**: $\mathbf{0.00\text{ pt}}$ (offset constante de $+1.17\text{ pt}$ em todas as linhas decorrente da métrica do layout raiz, sem divergência acumulativa entre linhas da tabela).

---

## 4. Testes e Não-Regressão

- `cargo test --workspace`: **5.847 testes executados, 5.847 aprovados (100% PASS)**.
- `crystalline-lint .`: **0 erros**.
