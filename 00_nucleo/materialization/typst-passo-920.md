# Passo 920 — gap de `underover.rs` sem constante explícita; `fraction_denom_gap` sem consumidor confirmado

**Precede este passo**: `typst-passo-906-relatorio.md` (achado original do gap de `underover`) e
`typst-passo-918-relatorio.md` (achado lateral: `fraction_denom_gap` parece não ter consumidor em
`frac.rs`). Dois achados tratados juntos por estarem na mesma vizinhança de código
(`MathConstants`, gaps de empilhamento) e por serem pequenos — separar se um crescer.

**Pré-condição de árvore**: `git status`. Confirmar P918/919 (se já commitado) presentes.

---

## Parte A — gap de `underover.rs`

`layout_underover` empilha por `height()` directo, sem constante de gap da tabela MATH (vanilla
usa `underbar_vertical_gap`/`overbar_vertical_gap` explícitos, per achado original de P906).

### Fase A
1. Ler a fórmula exacta do vanilla para `underbar_vertical_gap`/`overbar_vertical_gap` — confirmar
   se são dois campos distintos (over vs under, valores diferentes) ou o mesmo campo reaproveitado
   nos dois lados. Confirmar se já existem em `MathConstants` do cristalino (prováveis candidatos
   já extraídos em P893) ou precisam de campo novo.
2. Confirmar se `stack_tight_above` (extraído em P918) já é o ponto certo para injectar o gap, ou
   se precisa de uma variante/parâmetro adicional.

### Fase B (TDD directo, mapeamento de constante — não geometria nova, `stack_tight_above` já
existe e já está auditado)
1. Teste que falhe primeiro: gap real medido, não zero.
2. Implementar.
3. Suíte verde, discriminada por crate.
4. `mutool trace` num caso real (`underbrace`/`overbrace`) confirmando o gap novo, comparado ao
   vanilla — recibo, não afirmação (`L11`).

## Parte B — `fraction_denom_gap` sem consumidor confirmado

`frac.rs` usa `fraction_num_gap` tanto para o lado do numerador quanto do denominador —
`fraction_denom_gap` parece nunca ser lido. Pode ser bug (campo errado usado duas vezes) ou campo
morto (vanilla também não distingue na prática, campo existe só por completude da tabela MATH).

### Fase A
1. Ler a fórmula real do vanilla para o gap de numerador e de denominador — confirmar se são
   sempre o mesmo valor na prática (nesse caso, usar só `fraction_num_gap` nos dois lados não é
   bug, é coincidência de a tabela MATH ter os dois campos iguais na fonte usada) ou se divergem
   (nesse caso, é bug real — `frac.rs` está a aplicar o gap errado no lado do denominador).
2. Confirmar com `fontTools` os valores reais de `fraction_num_gap`/`fraction_denom_gap` em
   `NewCMMath-Regular.otf` — se forem numericamente iguais nessa fonte específica, o bug pode
   existir mas ser invisível com esta fonte, o que muda a prioridade (real mas não observável
   hoje) sem mudar se vale corrigir.

### Fase B (só se a Fase A confirmar divergência real)
1. Teste com ground-truth dos dois campos lidos separadamente.
2. Corrigir `frac.rs` para usar `fraction_denom_gap` no lado do denominador.
3. Suíte verde. `mutool trace` confirmando a mudança (se os valores da fonte real divergirem;
   senão, registar que a correcção é correcta mas sem efeito visível nesta fonte, e seguir mesmo
   assim por ser a fórmula certa per `ADR-0123`).

## Fase C — Regressão (conjunta para as duas partes)

Benchmark completo, 7 cenários, `--warmup 5 -m 20`, attestation completa (`L11`).

## Resultado esperado

- Parte A: gap real de `underover` implementado, recibo comparando ao vanilla.
- Parte B: veredicto sobre se é bug real ou campo coincidentemente igual nesta fonte; corrigido se
  for bug, independentemente de ter efeito visível na fonte actual.
- Benchmark completo atestado.
