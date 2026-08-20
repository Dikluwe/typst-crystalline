# Materialização — Passo 1110: Fechamento Analítico da Centragem Horizontal de Acentos sobre Expressões Compostas (Secção 33)

## Objetivo
Garantir a paridade matemática e geométrica em $X$ e $Y$ para acentos matemáticos aplicados a expressões compostas na Secção 33 (`.typ/sec_33.typ`) contra o Vanilla Typst 0.15.1.

## Arquivos Modificados
1. `01_core/src/compiler/math/layout/accent.rs`:
   - Implementada a fórmula exata de `pre_width`, `post_width`, `base_x` e `accent_x` com ancoragem simétrica para acentos esticados.

## Resultados
- **11/11 blocos da Secção 33 em Paridade Exata** (resíduos $\Delta Y \le -0.0001\text{ pt}$, $\Delta X = 0.0000\text{ pt}$ em `hat(x + y)`).
- **Suíte Total:** 5.960 testes automatizados aprovados sem regressões.
