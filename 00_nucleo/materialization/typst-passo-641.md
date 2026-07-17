---
# P641 — `#context` adia avaliação: confirmar antes de testar `state.update`

> **Passo:** 641
> **Data:** 2026-07-09
> **Foco:** P640 encontrou que `#context(...)` não avalia o corpo imediatamente — só durante o layout — e teve de reescrever os testes de `counter.display` para não depender disso. O próximo item da lista de P633 (caso 4, `state.update`) provavelmente tem o mesmo problema, porque estado costuma precisar de `#context` da mesma forma que contadores. Este passo confirma isto antes de escrever o teste de `state.update`, para não repetir o mesmo erro de metodologia.
> **Tipo:** Sonda directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.** Um teste que usa a ferramenta errada de propósito pode parecer confirmar ou refutar algo sem estar a testar a coisa certa — já quase aconteceu com `counter.display`.

---

## Sonda

### Confirmar o comportamento de `#context` directamente

```bash
grep -n "fn eval_context\|Contextual" 01_core/src/rules/eval/mod.rs | head -10
```

Confirmar onde `#context(...)` é avaliado — se cria uma closure/valor adiado, ou se avalia imediatamente, e em que ponto exacto do pipeline (eval vs. layout) essa avaliação de facto acontece.

### Confirmar se o vanilla tem o mesmo comportamento

```bash
grep -n "Contextual\|fn eval.*context" lab/typst-original/crates/typst-eval/src/*.rs 2>/dev/null | head -10
```

Confirmar se este adiamento é intencional e igual ao vanilla, ou se é uma diferença de arquitectura do cristalino.

### Testar directamente se `state.update` dentro de `#context` seria avaliado a tempo de um teste em `eval/tests.rs`

```bash
grep -n "state.update\|fn.*state" 01_core/src/rules/eval/from_tags.rs | head -10
```

Confirmar se o caso 4 de P633 (`from_tags.rs:64`, callback de `state.update` que descarta `Err`) é atingido durante o eval directo, ou só durante layout — o mesmo tipo de pergunta que P640 teve de responder para `counter.display`.

### Critério de fecho

- [ ] Comportamento de `#context` confirmado — quando avalia de facto.
- [ ] Confirmado se é igual ao vanilla ou uma diferença de arquitectura.
- [ ] Confirmado se o teste do caso 4 (`state.update`) precisa do mesmo contorno já usado por P640 para `counter.display`, ou se `from_tags.rs:64` é atingido de forma diferente (talvez já durante layout, não eval).

---

## Decisão

Se `state.update` só for atingido durante layout: o teste directo em `eval/tests.rs` (como os de P633/P640) pode não conseguir testar isto directamente — pode ser preciso um teste de integração que corra o layout completo, não só o eval, para confirmar o caso 4.

---

## Critério de fecho do passo

- [ ] Comportamento de `#context` confirmado com `file:line`.
- [ ] Método de teste certo para o caso 4 decidido, antes de o escrever.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p641.md`.
- [ ] Só depois disto avançar para a correcção do caso 4 (`state.update`), com o método de teste certo desde o início.
