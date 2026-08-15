# Relatório de Execução — Passo 1054

**Data**: 2026-08-14
**Passo**: 1054 — V21: Triar e Fechar por Categoria (Não Item a Item Cego)
**Gate**: `ADR-0127` (Classificação: Refinamento de Proveniência e Auditoria de Citações)
**Status**: CONCLUÍDO (4 citações obsoletas da Categoria A corrigidas e zeradas, constantes da Categoria B materializadas, escopo da Categoria C categorizado e zero regressão em 5.930 testes)

---

## 1. Fase 0 — Diagnóstico Geral da Triagem V21

Com o linter atualizado pós-Passo 0067 (`tekt-linter`), a regra **V21** foi segregada em duas frentes de auditoria:

1. **Citação Obsoleta** (`warning: Citação obsoleta`): 4 ocorrências iniciais $\longrightarrow$ **0 ocorrências pós-correção**.
2. **Escalar Contextual Fixo** (`warning: Escalar contextual fixo`): 72 ocorrências iniciais segregadas em 3 classes (Centragem Geométrica, Defaults de Especificação e Fatores de Layout).

---

## 2. Categoria A — Correção de Citações Obsoletas (Zeradas)

As 4 citações obsoletas identificadas decorriam de caminhos de arquivos desatualizados pós-fatiamento arquitetural:

| Arquivo e Linha | Citação Anterior (Desatualizada) | Citação Canônica Corrigida | Diagnóstico Causal |
| :--- | :--- | :--- | :--- |
| **`01_core/.../math/layout/frac.rs:173`** | `fraction.rs:60` | `lab/typst-original/crates/typst-layout/src/math/fraction.rs:60` | O arquivo `fraction.rs` pertence à crate `typst-layout` no Vanilla. |
| **`01_core/.../math/layout/tests.rs:3783`** | `compiler/layout/metrics.rs:59` | `01_core/src/compiler/layout/metrics.rs:59` | Caminho pré-fatiamento do Crystalline atualizado para a camada `01_core`. |
| **`01_core/.../math/layout/underover.rs:117`** | `scripts.rs:300` | `lab/typst-original/crates/typst-layout/src/math/scripts.rs:300` | `compute_limit_shifts` reside em `typst-layout/src/math/scripts.rs`. |
| **`01_core/.../math/layout/underover.rs:146`** | `scripts.rs:306` | `lab/typst-original/crates/typst-layout/src/math/scripts.rs:306` | `compute_limit_shifts` inferior em `typst-layout/src/math/scripts.rs`. |

> **Lição para o checklist de fatiamento (`auditar-fatiamento.md`)**:
> Ao renomear, mover ou fatiar arquivos de código ou documentação, é obrigatório executar um *grep* de validação em todos os comentários de proveniência (`// ref:`) para garantir que os caminhos relativos continuem apontando para arquivos existentes.

---

## 3. Categoria B — Materialização de Constantes Canônicas Confirmadas

Foram materializadas no código as citações canônicas de valores já confirmados como legítimos nos passos anteriores:

1. **`par.leading = 0.65em`**:
   - **Proveniência**: `lab/typst-original/crates/typst-library/src/model/par.rs:210` (`#[default(Em::new(0.65).into())]`).
   - **Propagação**: Comentários `// ref:` adicionados nos 8 pontos de fallback de leading (`cursor.rs:352, 361`, `enum_item.rs:44`, `list_item.rs:43`, `sequence.rs:132`, `sub_frame.rs:205, 213`).
2. **`matrix.column_gap` (`0.5em`) e `matrix.row_gap` (`0.2em`)**:
   - **Proveniência**: `lab/typst-original/crates/typst-library/src/math/matrix.rs:15-16` (`DEFAULT_ROW_GAP` e `DEFAULT_COL_GAP`).
   - **Materialização**: Adicionados comentários explícitos em `01_core/src/compiler/math/layout/matrix.rs:26, 29`.
3. **`block.spacing = 1.2em`**:
   - **Proveniência**: `lab/typst-original/crates/typst-library/src/layout/container.rs:342` (confirmado em `equation.rs:118, 334`).

---

## 4. Categoria C — Escopo Remanescente de Escalares Contextuais

Os 64 avisos remanescentes de escalares contextuais dividem-se em:

1. **Centragem e Margens Simétricas (30 casos)**: Operações canônicas de divisão `(total - width) / 2.0` e margem dupla `2.0 * margin` (em `columns.rs`, `cursor.rs`, `equation.rs`, `footnote_flush.rs`, etc.).
2. **Testes e Mock Metrics (17 casos)**: Asserções em `tests.rs` utilizando `FixedMetrics` (escala monospaced `0.6` e `0.7`).
3. **Achados do P1053 (6 casos)**: Itens catalogados com medição empírica no Passo 1053 para alinhamento em passos dedicados (`raw.rs:111`, `divider.rs:44`, `quote.rs:37`, `term_item.rs:27`).
4. **Outros Escalares (11 casos)**: Espaçamentos de frações (`frac.rs:53` com `2.0 * padding`) e espessuras mínimas (`mod.rs:993` com `0.05 * font_pt`).

---

## 5. Fase B — Convenção de Reutilização e Proveniência

Adotada a seguinte convenção padronizada:
- **Primeira ocorrência**: Citação canônica completa `// ref: lab/typst-original/.../file.rs:line` (ou `// P<NNN> — ...`).
- **Ocorrências subsequentes**: Remissão curta `// ref: lab/typst-original/.../file.rs:line` ou `// mesmo valor que <arquivo>:<linha>`.

---

## 6. Validação do Workspace

- **Citações Obsoletas**: **`0` (eliminadas)**.
- **Crystalline Linter**: `0 erros`.
- **Cargo Test Workspace**: **`5.930 aprovados, 0 falhas (100% PASS)`**.
