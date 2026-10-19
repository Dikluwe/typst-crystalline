# Passo 1018 — Fechar: `counter("nome")` colide com `counter(elemento)` no `CounterRegistry`

**Tipo**: Investigar → gate (ADR-0127, categoria 2/3, mudança de comportamento por
defeito) → corrigir se aprovado. Não parar em "documentar" — parar só no ponto de decisão
real, com L0 pronto para aprovação imediata.
**Medido em P1016** (não remedir do zero): `counter("heading")`/`counter("footnote")`
devolvem o valor do contador automático do elemento (`(2,)`), onde o vanilla devolve
`(0,)` — os dois espaços de nome (string de utilizador vs função de elemento) partilham a
mesma chave no `CounterRegistry` cristalino; no vanilla são distintos.
**Pré-condição**: `git status` limpo.

---

## Fase A — Confirmar o mecanismo exacto antes de desenhar o fix

1. Ler `CounterRegistry` (`entities`/`introspect`, confirmar localização exacta) — como é
   construída a chave hoje para `counter("nome")` vs `counter(elemento)`. Confirmar por
   `file:line` que os dois caem na mesma entrada.
2. Confirmar no vanilla como os dois espaços são mantidos separados (`typst_library::
   introspection::counter` — tipo de chave, `CounterKey::Str` vs `CounterKey::Selector`
   ou equivalente).
3. Medir o alcance real: repetir a medição do P1016 para `figure` e `table`, não só
   `heading`/`footnote` — confirmar que os 4 elementos têm o mesmo defeito (presumido,
   não confirmado individualmente para `figure`/`table`).

## Fase B — L0, gate obrigatório

Escrever o L0 da correcção (separar `CounterKey` em duas variantes, ou equivalente —
decidir a forma exacta na Fase A, não aqui). Apresentar para aprovação antes de código,
per ADR-0127. **Não implementar sem confirmação explícita.**

Critérios de verificação a incluir:
```
= T1 / = T2 / #context counter("heading").get()      → (0,)   // hoje (2,)
= T1 / = T2 / #context counter(heading).get()          → (2,)   // inalterado
counter("qualquer-nome-de-utilizador").step()/.get()    → inalterado (não é elemento)
Repetir para figure, table, footnote
```

Não-regressão: todos os testes de counter existentes (heading/figure/table/footnote),
incluindo `p461_table_counter_*` e `p1016_footnote_counter_*`.

## Fase C — Implementar (só após gate) e validar

```
crystalline-lint .
cargo test --workspace
```

---

## Resultado esperado

`counter("nome")` e `counter(elemento)` deixam de colidir, para os 4 elementos que hoje
partilham a chave. Se a Fase A revelar que `figure`/`table` não têm o mesmo defeito
(hipótese a testar, não presumida), ajustar o âmbito do fix e reportar a diferença.
