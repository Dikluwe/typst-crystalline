# Passo 923 — resíduo de ~4pt/linha em matrizes, não isolado por P921

**Precede este passo**: `typst-passo-921-relatorio.md`, "Achado residual, registado, não
resolvido" — depois dos dois mecanismos de P921 implementados fielmente (cada um confirmado
`file:line` contra o vanilla), sobra uma diferença de ~4pt/linha no espaçamento de matrizes,
causa não isolada.

**Diferente dos outros passos desta rodada — aqui a causa NÃO está identificada.** Este é
diagnóstico puro, sem hipótese prévia a confirmar.

**Pré-condição de árvore**: `git status`. Confirmar P921/922 (se já commitado) presentes.

---

## Fase A — isolar a causa (sem hipótese prévia — medir primeiro)

1. Reproduzir a medição de P921 (`mat(...)` 6 linhas, 20pt, `mutool trace`, vanilla real) e
   confirmar que o resíduo de ~4pt ainda existe no estado actual (depois de P922, se já fechado —
   confirmar que P922 não o afecta, área de código diferente).
2. Decompor o espaçamento total em componentes — para cada linha da grelha, medir separadamente:
   altura da linha em si (já corrigida por P921 achado 1), gap entre linhas (`math_leading`, já
   registado como candidato mas não confirmado como causa), e qualquer padding/margem adicional
   que `layout_grid_boxes`/`matrix.rs`/`cases.rs` aplique. Isolar qual componente diverge do
   vanilla, não presumir que é `math_leading` só porque já foi mencionado.
3. Se `math_leading` for de facto a causa: confirmar por que o cristalino lê da tabela MATH real
   (~0.154em) onde o vanilla usa uma constante fixa (`DEFAULT_ROW_GAP`, 0.2em, per nota de P921) —
   isto pode ser candidato ao mesmo tipo de decisão de `ADR-0123` (a "fórmula" aqui pode
   legitimamente ser "usar constante fixa", não "ler da fonte", se for isso que o vanilla faz de
   propósito, não por limitação). Confirmar a intenção do vanilla (comentário no código-fonte,
   `ADR-0108`) antes de decidir qual dos dois o cristalino deveria fazer.
4. Se não for `math_leading`: continuar a decompor até isolar a causa real, com números a cada
   passo (não avançar para a Fase B com a causa ainda incerta).

## Fase B — Implementação (só depois da Fase A confirmar a causa com números)

TDD directo ou protocolo de dois agentes, conforme a Fase A revelar (ajuste de constante vs.
geometria nova).

1. Teste com ground-truth medido do vanilla real para o componente identificado.
2. Corrigir.
3. Suíte completa verde, discriminada por crate.
4. Repetir a medição completa de P921 (`mediabox` final, espaçamento por linha) e confirmar que o
   resíduo fecha — recibo antes/depois, não afirmação.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`, attestation completa (`L11`).

## Resultado esperado

- Causa do resíduo isolada e confirmada com números, componente a componente.
- Se `math_leading` vs `DEFAULT_ROW_GAP` for a causa: decisão registada sobre qual comportamento é
  o correcto (ler da fonte vs. constante fixa), com a intenção do vanilla confirmada, não
  presumida.
- Recibo final: resíduo fechado ou razão registada para não fechar (se for divergência legítima).
- Benchmark completo atestado.
