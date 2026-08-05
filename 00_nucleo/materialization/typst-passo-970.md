# Passo 970 — índice de raiz (`root(3, x)`) com tamanho e posição errados

**Precede este passo**: achado 9.1 da auditoria externa (2026-08-05) — índice do cristalino: 7.7pt
(70% do texto normal), 2.2pt do topo do `√` até o índice — fica grande e quase no topo, flutuando.
Vanilla: 5.5pt (50%), 8.4pt do topo — menor, encaixado no vinco do símbolo, posição
tipograficamente correta.

**Pré-condição de árvore**: `git status`. Confirmar P969 (se já executado) presente.

---

## Fase A — confirmar a fórmula real do vanilla

1. Ler `root.rs`/`radical.rs` do vanilla — confirmar o nível de `MathSize` usado para o índice
   (parece ser mais reduzido que `Script` simples, dado 50% vs os 70% já usados noutros contextos
   de script nesta frente — confirmar se é `ScriptScript` em vez de `Script`, ou um factor próprio).
2. Confirmar a fórmula de posição vertical do índice relativa ao `√` — provavelmente ancorada a
   uma fracção da altura do radicando ou a uma constante da tabela MATH (`RadicalKernBeforeDegree`/
   `RadicalKernAfterDegree`/`RadicalDegreeBottomRaisePercent` — nomes a confirmar via `fontTools`,
   não presumir).
3. Confirmar a implementação actual do cristalino (`root.rs`) e onde diverge.

## Fase B — Implementação (TDD directo se for ajuste de fórmula pontual)

1. Teste com o tamanho/posição esperados, derivados da fórmula real e dos valores da fonte.
2. Implementar.
3. Suíte verde. Confirmação visual/medição real.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

`compare.py` nas seções com raiz (1, 13, 14, entre outras). Benchmark completo, 7 cenários,
`depois/antes`.

## Resultado esperado

- Índice de raiz no tamanho correto (per fórmula real do vanilla) e encaixado no vinco do `√`.
- Benchmark sem regressão.
