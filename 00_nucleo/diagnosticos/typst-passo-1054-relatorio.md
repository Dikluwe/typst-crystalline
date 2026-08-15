# Relatório de Execução — Passo 1054 (Revisado com Reconciliação Exata)

**Data**: 2026-08-14
**Passo**: 1054 — V21: Triar e Fechar por Categoria (Não Item a Item Cego)
**Gate**: `ADR-0127` (Classificação: Refinamento de Proveniência e Auditoria de Citações)
**Status**: CONCLUÍDO (4 citações obsoletas corrigidas, 10 constantes da Categoria B materializadas com citação auto-contida, 62 escalares remanescentes reconciliados e 5.930 testes aprovados)

---

## 1. Fase 0 — Diagnóstico Geral da Triagem V21

Com o linter atualizado pós-Passo 0067 (`tekt-linter`), a regra **V21** foi segregada em duas frentes de auditoria:

1. **Citação Obsoleta** (`warning: Citação obsoleta`): **4 ocorrências iniciais** $\longrightarrow$ **0 ocorrências pós-correção** (100% zeradas).
2. **Escalar Contextual Fixo** (`warning: Escalar contextual fixo`): **72 ocorrências iniciais** $\longrightarrow$ **62 ocorrências remanescentes** após a materialização de 10 proveniências canônicas da Categoria B ($72 - 10 = 62$).

---

## 2. Categoria A — Correção de Citações Obsoletas (Zeradas)

As 4 citações obsoletas identificadas decorriam de caminhos desatualizados por fatiamentos estruturais anteriores:

| Arquivo e Linha | Citação Anterior (Desatualizada) | Citação Canônica Corrigida | Diagnóstico Causal |
| :--- | :--- | :--- | :--- |
| **`01_core/.../math/layout/frac.rs:173`** | `fraction.rs:60` | `lab/typst-original/crates/typst-layout/src/math/fraction.rs:60` | O arquivo `fraction.rs` pertence à crate `typst-layout` no Vanilla. |
| **`01_core/.../math/layout/tests.rs:3783`** | `compiler/layout/metrics.rs:59` | `01_core/src/compiler/layout/metrics.rs:59` | Caminho pré-fatiamento do Crystalline atualizado para a camada `01_core`. |
| **`01_core/.../math/layout/underover.rs:117`** | `scripts.rs:300` | `lab/typst-original/crates/typst-layout/src/math/scripts.rs:300` | `compute_limit_shifts` reside em `typst-layout/src/math/scripts.rs`. |
| **`01_core/.../math/layout/underover.rs:146`** | `scripts.rs:306` | `lab/typst-original/crates/typst-layout/src/math/scripts.rs:306` | `compute_limit_shifts` inferior em `typst-layout/src/math/scripts.rs`. |

> **Lição para o checklist de fatiamento (`auditar-fatiamento.md`)**:
> Ao renomear, mover ou fatiar arquivos de código ou documentação, é obrigatório executar um *grep* de validação em todos os comentários de proveniência (`// ref:`) para garantir que os caminhos relativos continuem apontando para arquivos existentes.

---

## 3. Categoria B — Materialização de Constantes Canônicas (10 Casos Resolvidos)

Foram materializadas no código as citações canônicas de valores já confirmados como legítimos nos passos anteriores:

1. **`par.leading = 0.65em` (8 ocorrências em código de produção)**:
   - **Proveniência**: `lab/typst-original/crates/typst-library/src/model/par.rs:210` (`#[default(Em::new(0.65).into())]`).
   - **Locais**: `cursor.rs:352`, `cursor.rs:361`, `enum_item.rs:44`, `list_item.rs:43`, `sequence.rs:132`, `sub_frame.rs:205`, `sub_frame.rs:213`, `03_infra/src/shaper.rs:474`.
2. **`matrix.column_gap` (`0.5em`) e `matrix.row_gap` (`0.2em`) (2 ocorrências)**:
   - **Proveniência**: `lab/typst-original/crates/typst-library/src/math/matrix.rs:15-16` (`DEFAULT_ROW_GAP` e `DEFAULT_COL_GAP`).
   - **Locais**: `01_core/src/compiler/math/layout/matrix.rs:26, 29`.

---

## 4. Categoria C — Reconciliação Exata dos 62 Escalares Remanescentes

A conta fecha com precisão matemática: **$72 \text{ iniciais} - 10 \text{ resolvidos na Categoria B} = \mathbf{62 \text{ remanescentes}}$**, divididos em:

| Classe de Escalar | Qtd | Arquivos e Linhas Típicas | Diagnóstico e Destino |
| :--- | :---: | :--- | :--- |
| **1. Centragem Geométrica e Margens Simétricas** | **30** | `columns.rs:146, 161`, `cursor.rs:471, 475, 709`, `footnote_flush.rs:60`, `equation.rs:176` | Operações triviais `(total - width) / 2.0` e `2.0 * margin` (esquerda+direita). |
| **2. Testes e Mock Metrics** | **17** | `01_core/src/compiler/layout/tests.rs` (17 locais) | Asserções de teste utilizando `FixedMetrics` (`size * 0.65` e `2.0 * margin`). |
| **3. Achados de Layout Catalogados no P1053** | **4** | `quote.rs:37, 47` (`1.5`), `raw.rs:111` (`0.9`), `term_item.rs:27` (`1.5`) | Itens medidos empiricamente no Passo 1053 para alinhamento em passos dedicados. |
| **4. Fallback de Métrica Monoespaçada** | **2** | `03_infra/src/font_metrics.rs:451, 1361` (`0.6`) | Fallback monoespaçado para glifos ausentes na face. |
| **5. Outros Escalares de Domínio** | **9** | `frac.rs:53` (`2.0 * padding`), `decorations.rs:36` (`0.05`), `equation.rs:334` (`1.2`), `mod.rs:880, 993` | Espaçamentos de frações, espessura mínima de traço e layout math. |
| **Total Reconciliado** | **62** | | $\mathbf{30 + 17 + 4 + 2 + 9 = 62}$ |

---

## 5. Fase B — Decisão da Convenção de Proveniência: Citação Auto-Contida

> **Decisão Metodológica de Convenção**:
> Para evitar fragilidade em refatorações futuras, adota-se a **repetição da citação externa canônica completa (`// ref: lab/typst-original/.../file.rs:line`) em todas as ocorrências de um valor reutilizado**, em vez de remissões relativas internas (`// mesmo valor que file.rs:line`).
> 
> **Justificativa**: Remissões internas criam acoplamento entre arquivos e tornam-se obsoletas assim que o arquivo de origem for renomeado ou fatiado. A citação externa direta para a especificação do Vanilla é 100% auto-contida e estável.

---

## 6. Validação do Workspace

- **Citações Obsoletas**: **`0` (100% eliminadas)**.
- **Crystalline Linter**: `0 erros`.
- **Cargo Test Workspace**: **`5.930 aprovados, 0 falhas (100% PASS)`**.
