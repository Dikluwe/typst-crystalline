# Materialização — Passo 1108: Fechamento Analítico do Colapso Heading → Equação e Transições na Secção 32

## Objetivo
Garantir a convergência analítica estrita de todos os 15 blocos verticais da Secção 32 (`.typ/sec_32.typ`) contra o Vanilla Typst 0.15.1 (`338.9089 × 201.1383 pt`), respeitando o critério de tolerância $\le \pm 0.0005\text{ pt}$ e registrando a pendência histórica do P1107 (§4 do L0).

## Arquivos Modificados
1. `01_core/src/compiler/layout/equation.rs`:
   - `ensure_initial_baseline` restrito exclusivamente para inline (`if !block`).
   - `self.initial_baseline_pending = false` setado após a primeira equação em topo de página.
   - Preservação do estado da cadeia antes de sub-layouts matemáticos.
   - Fórmula nominal de baseline em bloco: `cursor_y = Pt(prev_baseline + prev_descent + gap + ext.ascent)`.
2. `01_core/src/compiler/layout/sequence.rs`:
   - Preservação de cadeia governada por `current_line.is_empty()`, eliminando quebras causadas por wrappers estruturais transparentes do AST (`Content::Styled`, `Content::Contextual`, etc.).
3. `01_core/src/compiler/layout/cursor.rs`:
   - Removido reset incondicional de `prev_block_equation_descent` em `flush_line()`.
4. `01_core/src/compiler/math/layout/attach.rs`:
   - Desempacotamento transparente de `MathLimitsOverride` e `MathClassOverride` na avaliação de `is_text_like`.
5. `01_core/src/compiler/layout/tests.rs`:
   - Calibração das asserções de testes para os valores nominais analíticos (13.2pt e 15.95pt).

## Resultados
- **15/15 blocos com Paridade Exata** (resíduos entre $+0.0000\text{ pt}$ e $-0.0001\text{ pt}$).
- **Suíte Completa do Workspace:** 5.960 testes aprovados / 0 falhas (`typst-core` 5.082 + `typst-infra` 796 + `typst-shell` 41 + `cli` 37 + `main/eviction` 2 + `crystalline_lint` 2).
