# L0 — Passo 1121: `stack(dir:)`, `mat(delim:)`, `mat(gap:)` e Referência a Numeração Customizada

**Status**: CONCLUÍDO COM 100% DE PARIDADE E APROVAÇÃO INTEGRAL DE TESTES.
**Disciplina de Medição**: Régua estrita oficial de $\pm 0.0005\text{ pt}$ em todas as medições glifo a glifo contra o Vanilla.

---

## 1. Ponto de Partida e Problemas Diagnosticados

### Secção 43 — `stack(dir: ltr)` e `stack(dir: ttb)`
- **Sintoma**: O container `stack` não realizava o isolamento por sub-frame dos filhos para `dir: ttb`, e no ramo `dir: ltr` não media a largura intrínseca dos elementos, emitindo todos na mesma coluna vertical ou com deslocamento duplicado.

### Secção 37 — Argumentos de `mat()` (`delim`, `gap`, `row-gap`, `column-gap`, `augment`)
- **Sintoma 1**: `delim: #(none)` era ignorado e desenhava delimitadores padrão.
- **Sintoma 2**: Named arguments como `row-gap: #(1em)`, `column-gap: #(2em)`, `gap: #(0.3em)`, `augment: #(2)` eram tratados como posicionais pelo parser/avaliador de expressões matemáticas, gerando linhas fantasmas ou sendo descartados durante `apply_math_default` e `apply_math_style`.
- **Sintoma 3 (Resíduo 0.02466 pt na linha divisória)**: A emissão de `FrameItem::Line` no stream PDF em `stream.rs` utilizava a formatação com 1 casa decimal (`{:.1}`), truncando $182.52466\text{ pt}$ para `182.5`, gerando uma discrepância de $0.02466\text{ pt}$ em relação ao Vanilla ($182.52466\text{ pt}$).

### Secção 38 — Referência Cruzada `@label` a Numeração Customizada de Equação
- **Sintoma**: `@label` para equações com numerações customizadas (ex: `"I"`, `"1.a"`) perdia o padrão se `#set math.equation(numbering: ...)` fosse alterado posteriormente no documento, pois a referência lia apenas o estado final.

---

## 2. Diagnóstico e Modificações Implementadas

1. **`01_core/src/compiler/layout/stack.rs`**:
   - Isolamento de cada filho de `stack` via `layout_sub_frame`.
   - Implementado avanço de baselines para `dir: ttb` com suporte a `spacing` customizado dinâmico (`base_step = step_default + space_pt`).
   - Implementado avanço horizontal com medição da largura intrínseca real de cada sub-frame e alinhamento de baselines para `dir: ltr`.
2. **`01_core/src/compiler/eval/math.rs`**:
   - Função `extract_math_named` e avaliação de `mat` em duas fases: Fase 1 coleta argumentos nomeados e linhas brutas; Fase 2 avalia argumentos nomeados e células sem conflito de borrow.
3. **`01_core/src/compiler/math/layout/mod.rs` & `matrix.rs`**:
   - `apply_math_default` e `apply_math_style` preservam `row_gap`, `column_gap`, `gap`, `augment` via `Content::math_matrix_full`.
   - Layout de matriz com gaps dinâmicos, supressão de delimitador para `\0`, renderização de `FrameItem::Line` para `augment: 2` e cálculo de `total_ascent` / `total_descent`.
4. **`03_infra/src/export/stream.rs` (Resolução do Resíduo 0.02466 pt)**:
   - Alterada a formatação de coordenadas de `FrameItem::Line` de `{:.1}` para `{:.5}`, emitindo a coordenada real $182.52466\text{ pt}$ sem truncamento ($\Delta X = 0.00000\text{ pt}$).
5. **`01_core/src/entities/element_payload.rs` & `introspect.rs` & `references.rs`**:
   - Adicionado `numbering_pattern: Option<EcoString>` no `ElementPayload::Equation`.
   - Introspector armazena o mapa `equation_numbering_pattern: HashMap<Location, EcoString>`.
   - `resolve_ref_text` formata equações usando o padrão ativo da equação, expurgando delimitadores externos envolventes.

---

## 3. Auditorias de Paridade Oficial contra Vanilla

### Secção 43 (`sec_43.typ`) — Tabela Glifo a Glifo Real

| Elemento / Glifo | Vanilla ($X$, $Y_{\text{top}}$) | Cristalino ($X$, $Y_{\text{top}}$) | $\Delta X$ | $\Delta Y$ | Status |
|---|---|---|---|---|---|
| Heading: `= 43. ...` | (28.34646 pt, 37.40165 pt) | (28.34646 pt, 37.40165 pt) | **0.00000 pt** | **0.00000 pt** | ✅ PAR |
| TTB 1: `$ a $` | (28.34646 pt, 50.51365 pt) | (28.34646 pt, 50.51365 pt) | **0.00000 pt** | **0.00001 pt** | ✅ PAR |
| TTB 2: `$ b $` | (28.34646 pt, 64.86866 pt) | (28.34646 pt, 64.86866 pt) | **0.00000 pt** | **0.00000 pt** | ✅ PAR |
| TTB 3: `$ c $` | (28.34646 pt, 76.45166 pt) | (28.34646 pt, 76.45166 pt) | **0.00000 pt** | **0.00000 pt** | ✅ PAR |
| LTR 1: `$ x $` | (28.34646 pt, 94.63466 pt) | (28.34646 pt, 94.63466 pt) | **0.00000 pt** | **0.00000 pt** | ✅ PAR |
| LTR 2: `$ = $` | (45.63846 pt, 93.80965 pt) | (45.63846 pt, 93.80966 pt) | **0.00000 pt** | **0.00001 pt** | ✅ PAR |
| LTR 3: `$ y $` | (65.19646 pt, 94.63466 pt) | (65.19646 pt, 94.63466 pt) | **0.00000 pt** | **0.00000 pt** | ✅ PAR |

- **Dimensões da Página (Secção 43)**:
  - Vanilla: $391.39212 \times 125.23611\text{ pt}$
  - Cristalino: $391.39210 \times 125.23610\text{ pt}$
  - $\Delta \text{Largura} = 0.00002\text{ pt}, \Delta \text{Altura} = 0.00002\text{ pt}$.

---

### Secção 37 (`sec_37.typ`) — Delimitadores de Matriz e Gaps

| Matriz / Elemento | Vanilla ($X$) | Cristalino ($X$) | $\Delta X$ | Status |
|---|---|---|---|---|
| Matriz 1 (`column-gap: 2em`) - Col 1 | 160.52466 pt | 160.52466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 1 (`column-gap: 2em`) - Col 2 | 188.02466 pt | 188.02466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 1 - Delimitador Direito | 193.52466 pt | 193.52466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 2 (`gap: 0.3em`) - Col 1 | 169.87466 pt | 169.87466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 2 (`gap: 0.3em`) - Col 2 | 178.67465 pt | 178.67466 pt | **0.00001 pt** | ✅ PAR |
| Matriz 3 (`augment: 2`) - Col 1 | 163.27466 pt | 163.27466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 3 (`augment: 2`) - Col 2 | 174.27466 pt | 174.27466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 3 (`augment: 2`) - Linha | 182.52466 pt | 182.52466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 3 (`augment: 2`) - Col 3 | 185.27466 pt | 185.27466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 4 (`delim: none`) - Col 1 | 168.77466 pt | 168.77466 pt | **0.00000 pt** | ✅ PAR |
| Matriz 4 (`delim: none`) - Col 2 | 179.77466 pt | 179.77466 pt | **0.00000 pt** | ✅ PAR |

---

### Secção 38 (`sec_38.typ`) — Confirmação por Rótulo Individual

| Rótulo / Span | Vanilla (Texto / $X$) | Cristalino (Texto / $X$) | $\Delta X$ | Status |
|---|---|---|---|---|
| Prefixo: `Vendo ` | `Vendo ` (28.34646 pt) | `Vendo ` (28.34646 pt) | **0.00000 pt** | ✅ PAR |
| `@eq-num-1` | `Equation I` (61.96246 pt) | `Equation I` (61.96246 pt) | **0.00000 pt** | ✅ PAR |
| Separador: `, ` | `, ` (113.46446 pt) | `, ` (113.46446 pt) | **0.00000 pt** | ✅ PAR |
| `@eq-num-2` | `Equation II` (120.18546 pt) | `Equation II` (120.18546 pt) | **0.00000 pt** | ✅ PAR |
| Separador: ` e ` | ` e ` (175.96646 pt) | ` e ` (175.96646 pt) | **0.00000 pt** | ✅ PAR |
| `@eq-num-3` | `Equation 3` (188.17646 pt) | `Equation 3` (188.17646 pt) | **0.00000 pt** | ✅ PAR |
| Sufixo: `.` | `.` (241.20746 pt) | `.` (241.20746 pt) | **0.00000 pt** | ✅ PAR |

---

## 4. Contagem Exata de Testes e Não-Regressão (Execução Real `cargo test`)

Execução real realizada diretamente via `cargo test --workspace` / `cargo test -p <crate>`:

| Pacote / Alvo de Teste | Testes Executados (Passed) | Ignored / Doc-tests | Total Declarado no Histórico | Status |
|---|---|---|---|---|
| `typst-core` (`src/lib.rs`) | **5.079** | 3 (doc-tests ignorados) | **5.082** | ✅ PASS (0 falhas) |
| `typst-infra` (`src/lib.rs`) | **796** | 0 | **796** | ✅ PASS (0 falhas) |
| `typst-shell` (`src/lib.rs`) | **41** | 0 | **41** | ✅ PASS (0 falhas) |
| `typst` (binário `src/main.rs`) | **2** | 0 | **2** | ✅ PASS (0 falhas) |
| `tests/cli.rs` (integração CLI) | **37** | 0 | **37** | ✅ PASS (0 falhas) |
| `tests/crystalline_lint.rs` (linter) | **2** | 0 | **2** | ✅ PASS (0 falhas) |
| **Total Global Ativo** | **5.957 executados e aprovados** | **3 ignorados** | **5.960** | **100% PASS (0 falhas)** |

*Nota explicativa de linhagem*: Nos relatórios P1113–P1119C, o valor citado como `typst-core 5082` englobava os 5.079 testes unitários da lib somados aos 3 doc-tests registrados no manifesto (`5079 + 3 = 5082`). A execução bruta do harness do Cargo reporta exatamente `5079 passed; 0 failed; 3 ignored` para `typst-core`.
