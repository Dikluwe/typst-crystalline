# Achado: Divergência de Largura e Gutter em Math Cases (`cases.rs`)

**Data**: 2026-08-14
**Origem**: Investigação de isolamento diferencial do Passo 1041 (DENY 3, `spacing.rs:72`)
**Módulo responsável**: `01_core/src/compiler/math/layout/cases.rs` (`layout_cases`)
**Estado**: `CATALOGADO (Pendente de Passo Dedicado)`
**Requer Gate**: Sim (`ADR-0127` — alteração de posicionamento visual/layout)

---

## 1. Descrição do Problema

Durante a validação diferencial do Passo 1041 contra o Typst Vanilla 0.15.1, observou-se que equações compostas em display mode contendo blocos `$cases(...)$` sofrem um deslocamento horizontal sistemático de ~1.65 pt em todos os elementos da equação.

A investigação de isolamento comprovou que:
1. Os operadores e nós matemáticos puros (`frac`, `sqrt`, `+`) possuem **paridade exata de 0.00 pt** no espaçamento inter-símbolo (`spacing.rs:72` / `MathClass::Normal`);
2. A causa do deslocamento reside exclusivamente no bloco `cases`, cuja largura total agregada diverge do Typst Vanilla em virtude das constantes de espaçamento em `01_core/src/compiler/math/layout/cases.rs`:
   - `padding = style.size * 0.1` (espaçamento entre o delimitador `{` esticado e a primeira coluna de células);
   - `col_gap = style.size * 0.5` (gutter horizontal entre colunas adjacentes da grelha de `cases`);
   - Avanço horizontal do glifo do delimitador `{` esticado.

---

## 2. Medição Empírica (Vanilla 0.15.1 vs. Crystalline)

| Cenário de Teste | Número de Colunas | Largura Vanilla | Largura Crystalline | Delta de Largura ($\Delta w$) | Impacto de Centralização ($\Delta x = \Delta w / 2$) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| `$cases(1, 2)$` | 1 coluna | 13.75 pt | 14.85 pt | **+1.10 pt** | +0.55 pt |
| `$cases(1 "if" x > 0, 0 "otherwise")$` | 2 colunas | 62.71 pt | 63.81 pt | **+1.10 pt** | +0.55 pt a +1.65 pt (composto) |
| `$cases(1 & "if" & x > 0, ...)$` | 5 colunas | 53.94 pt | 58.74 pt | **+4.80 pt** | +2.40 pt |

---

## 3. Causa Raiz e Próximos Passos

- **Causa Raiz**: O Vanilla Typst implementa `CasesElem` utilizando o modelo de tabela matemática `resolve_cases` / `resolve_cells` com parâmetros OpenType Math específicos de `delim_gap` e `CasesElem::gap`. No Crystalline, `cases.rs` utiliza valores fixos proporcionais ao tamanho da fonte (`col_gap = 0.5em`, `row_gap = 0.2em`, `padding = 0.1em`).
- **Ação Proposta**: Criar um passo de materialização dedicado para calibrar as constantes de `cases.rs` contra a especificação e o comportamento exato do `CasesElem` no Vanilla, submetendo a alteração ao gate visual do `ADR-0127`.
