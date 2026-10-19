# Passo 907 — dois achados de espaçamento registados em P903 (fence, e função-com-limite seguida de conteúdo)

**Precede este passo**: `typst-passo-903-relatorio.md`, secção "Achados incidentais". Ler antes de
começar. Dois achados distintos, mecanismos provavelmente diferentes — tratar como duas partes
independentes, mesma disciplina de P904.

**Pré-condição de árvore**: `git status`.

---

## Parte A — `RR | x` (fence/"mid"): gap ≈0 no cristalino, vanilla usa ≈3.65pt dos dois lados

Já registado como scope-out em P825 (`spacing.md`), reconfirmado ainda presente após P903.
Reproduzível sem qualquer texto literal envolvido — mecanismo distinto do corrigido em P903
(`Fence`/`MathClass` de `|` como separador, não `Content::Text`).

### Fase A
1. Confirmar a classe (`MathClass`) atribuída a `|` como fence/separador hoje no cristalino, e
   comparar com a classe que o vanilla usa para esse papel especificamente (`|` tem múltiplos papéis
   — delimitador de `abs`, operador "tal que"/"dado" em teoria de conjuntos, "mid" em probabilidade
   condicional — confirmar qual regra de `compute_gaps`/`spacing_between` deveria cobrir o papel de
   separador/fence, sem mudar o comportamento de `|` como delimitador de `abs(x)`, que já funciona).
2. Reler `typst-passo-825...md`/`spacing.md` para entender por que ficou registado como scope-out
   nesse passo, antes de assumir que é simples de corrigir agora.

### Fase B (TDD directo, adição de regra de espaçamento, mesmo padrão de P903 — sem necessidade de
dois agentes)
1. Teste que falhe primeiro.
2. Implementar, sem quebrar `abs(x)` (`|x|`, já correto).
3. Suíte verde, discriminada por crate.
4. Confirmar visualmente `{x in RR | x > 0}` (caso da secção 7 do `.typ` de 30 secções) e `abs(x)`
   lado a lado.

---

## Parte B — `min_(x) f(x)`: `min` sem espaço antes do conteúdo seguinte

`min(x) f(x)` no cristalino produz `min𝑓(𝑥)` (sem espaço); vanilla produz `min 𝑓(𝑥)`. Confirmado
reproduzível sem texto literal envolvido, mecanismo não investigado.

### Fase A
1. Confirmar como `min`/`max`/outras funções "com limite" (`Content::MathOp`? `Content::MathLimit`?
   — confirmar o nome real do tipo usado para operadores como `min`/`max`/`lim`/`sum`/`prod`, que já
   funcionam em vários outros lugares desta suíte de testes) são classificadas para efeitos de
   espaçamento contra o que vem a seguir — mesma família de investigação de P891/903 (classe
   ausente, ou classe presente sem regra contra o vizinho seguinte).
2. Confirmar se isto afeta só `min`/`max` com subscrito (`min_(x)`) ou também sem
   (`$ min f(x) $` sozinho, sem `_(x)`) — isolar os dois casos separadamente.

### Fase B (TDD directo)
1. Teste que falhe primeiro, para os dois casos (com e sem subscrito) confirmados pela Fase A.
2. Implementar.
3. Suíte verde, discriminada por crate.
4. Confirmar visualmente a secção 29/30 do `.typ` de 30 secções (usa `max`/`min` com subscrito).

---

## Fase C — Regressão (uma vez, para as duas partes)

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Relatório por parte: causa confirmada, teste novo, confirmação visual.
- Se qualquer uma das duas revelar-se mais complexa do que uma regra de espaçamento simples
  (por exemplo, se `|` como fence exigir desambiguação de contexto que hoje não existe): registar e
  considerar destacar para passo próprio, mesmo padrão já estabelecido nesta frente.
- Benchmark completo no fim.
