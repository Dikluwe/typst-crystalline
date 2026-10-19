# Passo 1037 — Show rules não estabelecem contexto (`counter.get()` dentro de show rule)

**Tipo**: Investigar → gate → corrigir. Achado #1 do P1031, prioridade Alta. Toca
directamente a área de `eval::rules`/`show_rule_termination` já fatiada (Passos 1009,
1011, 1023) — ler os L0s existentes antes de investigar, não redescobrir a estrutura.
**Medição do P1031**: documentação vanilla (`context.typ:13`): *"Show rules provide
context"*. No cristalino, `counter.get()` dentro de uma show rule **erra**; com
`#context` explícito envolvendo a chamada, compila mas **devolve vazio**.

**Nota de correcção já feita no P1031**: o L0 de `show_rule_termination.md` tinha uma
frase ("confirmado por teste empírico", sem proveniência) que **presumia** o oposto desta
afirmação — já foi removida nesse passo (X, achado #3 da tabela Bloco A). Este passo é a
correcção de comportamento, não só de registo.

**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1033.

---

## Fase A — Ler o que já existe antes de investigar do zero

1. `compiler/eval/show_rule_termination.md` (L0) e `.rs` — confirmar o estado actual do
   mecanismo de aplicação de show rules, per o trabalho já feito nos Passos 1009/1011/
   1023/1007.
2. Confirmar exactamente o que "show rules provide context" significa no vanilla —
   mecanismo: a closure de uma show rule é avaliada como se estivesse dentro de um bloco
   `context { ... }` implícito, ou é outra coisa? Ler a documentação completa, não só a
   frase citada.
3. Reproduzir os dois casos exactos do achado — sem `#context` (deve errar como hoje, ou
   passar a funcionar directamente?) e com `#context` explícito (deve deixar de devolver
   vazio).

## Fase B — Localizar por que `#context` explícito devolve vazio

Isto é a parte mais estranha do achado — não é só "falta suporte", é "aceita mas dá
resultado errado silenciosamente". Confirmar por `file:line` o que `#context` está a
capturar quando usado dentro de uma show rule, e por que o `counter.get()` lá dentro não
vê o valor real.

## Fase C — Gate (ADR-0127, categoria 2/3)

```
Dado #show heading: it => context counter(heading).get() ...  (ou forma equivalente que
  o vanilla aceita sem #context explícito, per Fase A)
Quando renderizado
Então counter.get() devolve o valor real, batendo com vanilla

Dado a mesma construção com #context explícito (forma que hoje devolve vazio)
Quando renderizado
Então devolve o valor real, não vazio
```

Não-regressão: todos os testes de `show_rule_termination`/`eval::rules` existentes
(18 de P340 + 3 de P1007 + os de P1009/1011/1023).

## Fase D — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```

---

## Resultado esperado

Show rules estabelecem contexto correctamente para `counter`/`state`/introspecção lá
dentro, batendo com a documentação vanilla citada. Toda a suite de `eval::rules` continua
verde.
