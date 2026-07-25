# Passo 905 — `/` em argumentos de chamada de função em modo matemático produz saída malformada

**Precede este passo**: `typst-passo-899-relatorio.md`, achado incidental na Parte B ("um argumento
contendo `/` dentro de QUALQUER chamada de função em modo math... produz saída malformada").
Confirmado pré-existente (reproduz com `sqrt(x/y)`, função não tocada por P899), agora afetando
também `abs`/`norm`/`floor`/`ceil`/`round`/`binom` (implementadas em P899) — sobe de prioridade por
isso, não é mais um caso isolado de `sqrt`.

**Pré-condição de árvore**: `git status`. Confirmar se P893 (Fase B, agora confirmada) está em
curso — ficheiros prováveis não se sobrepõem (`eval/math.rs`/parsing de args vs `font_metrics.rs`),
mas conferir.

---

## Sintoma (reler P899 para o caso exato antes de assumir os detalhes abaixo)

`$ sqrt(x/y) $` (e agora `abs(x/y)`, `floor(x/y)`, etc.) produz saída malformada — só o primeiro
operando aparece, com um glifo estranho por baixo. Isolar o caso mínimo exato antes de investigar
(pode não ser literalmente `x/y`, confirmar contra o que P899 reproduziu).

## Fase A — diagnóstico

1. Confirmar como argumentos de chamada de função em modo math são parseados/avaliados hoje
   (`eval/math.rs`, o mesmo despacho hardcoded usado por `sqrt`/`abs`/etc. desde P899) — o problema
   é de parsing (`/` dentro de parênteses de chamada sendo interpretado como divisão de fração em
   vez de argumento literal) ou de avaliação (o argumento é parseado certo, mas `eval_math_arg_value`
   ou equivalente processa `/` de forma que produz o glifo estranho)?
2. Confirmar como o vanilla trata isto (`lab/typst-original/`) — `/` dentro de argumento de função
   deveria produzir uma fração normal como conteúdo desse argumento (`sqrt(x/y)` = raiz de "x sobre
   y", não um erro), ou há alguma regra especial de precedência que o cristalino está a violar?
3. Confirmar se isto afeta só chamadas hardcoded (`sqrt`/`abs`/etc.) ou também o despacho namespaced
   genérico (`bb(x/y)`, etc.) — testar pelo menos um caso de cada categoria.
4. Isolar o caso mínimo mais simples possível (`$ sqrt(1/2) $`?) e capturar exatamente o que aparece
   hoje (via `mutool trace` ou visual) para ter um alvo de "antes" concreto.

## Fase B — Implementação (protocolo de dois agentes se a causa envolver parsing/precedência —
risco de regressão em outros usos de `/`; TDD directo se for isolado ao caminho de argumentos de
função)

1. Teste que falhe primeiro: `sqrt(x/y)` (e pelo menos mais um caso das funções de P899) produz
   conteúdo correto — fração como argumento, renderizada normalmente dentro do `sqrt`/`abs`/etc.
2. Implementar a correção no ponto confirmado pela Fase A.
3. Suíte completa verde, discriminada por crate. **Atenção especial**: `/` é usado extensivamente
   fora de argumentos de função (frações normais `a/b`) — confirmar que a correção não afeta esse
   caso, que já funciona.
4. Recompilar as secções que usam estas funções com `/` dentro (procurar no `.typ` de 30 secções;
   se nenhuma usar, criar caso de teste próprio) e confirmar visualmente.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`. Lembrar que ruído de ~2-4% em todos os 7
cenários simultaneamente (incluindo controlo não relacionado) já se confirmou ser variabilidade de
máquina em P896-899, não regressão — não gastar tempo re-investigando isso se o padrão se repetir
de novo, a menos que só os cenários relacionados com este passo subam (aí sim investigar).

## Resultado esperado

- Header de linhagem atualizado.
- Teste(s) novo(s) cobrindo `/` dentro de pelo menos 2 funções diferentes das implementadas em P899.
- Relatório com: causa exata confirmada (parsing vs avaliação), confirmação de que frações normais
  fora de chamada de função continuam funcionando, confirmação visual, benchmark completo.
