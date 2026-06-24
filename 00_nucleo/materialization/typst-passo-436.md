# P436 — Fecho de débito: DEBT-57 subset (`rules/stdlib/transforms.rs`)

---

## Contexto

**DEBT-57** registava ~70 funções stdlib sem spec L0 dedicada. Os subsets `structural.rs` (P430), `layout.rs` (P432), `calc.rs` (P433), `assert.rs` (P434) e `shapes.rs` (P435) foram fechados. O próximo subset mais fechável é `rules/stdlib/transforms.rs` — **4 funções nativas** (`move`, `rotate`, `scale`, `skew`), todas com consumers reais em L1.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/transforms.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas? | 4 funções | — |
| Todas têm consumer real? | 4/4 implementadas (P78, P156F) | ✅ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** XS-S (~15 min; 4 entradas; formato estabelecido P430–P435).

---

## ADR-0107 — Paridade linguagem

Contrato documental: cada função nativa em `transforms.rs` deve ter seu contrato L0 (assinatura, argumentos posicionais/nomeados, semântica, paridade vanilla, limitações graded, testes canônicos) em `00_nucleo/prompts/rules/stdlib/transforms.md`.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/rules/stdlib/transforms.md` com 4 secções (1 por função).
2. Cada secção: assinatura, args (`angle`, `dx`, `dy`, `amount`, `body`), semântica, paridade vanilla, limitações, testes canônicos.
3. Atualizar `rules/stdlib/_comum.md` — remover `transforms.rs` da lista.
4. Atualizar cabeçalho `@prompt` de `01_core/src/rules/stdlib/transforms.rs` para apontar `transforms.md`.
5. `DEBT.md` atualizado com nota "subset transforms.rs fechado em P436".

---

## Scope-out explícito

- `origin` argumento em `rotate`/`scale`/`skew` — scope-out (paridade vanilla).
- Outros ficheiros stdlib sem spec L0 (`gradients.rs`, `foundations.rs`) — fora do escopo do P436.

---

## Critério de fecho

- [ ] `00_nucleo/prompts/rules/stdlib/transforms.md` criado com 4 secções.
- [ ] Cada função documenta: assinatura, args, semântica, paridade vanilla, limitações, testes canônicos.
- [ ] `_comum.md` atualizado (transforms.rs removido da lista).
- [ ] `transforms.rs` cabeçalho `@prompt` aponta `transforms.md`.
- [ ] `DEBT.md` atualizado com nota de fecho P436.
- [ ] `crystalline-lint` zero novas violações.
- [ ] `cargo test --workspace` verde (zero código modificado).
- [ ] DEBT-57 atualizado: "subset transforms.rs fechado em P436".

---

**Próximo passo:** Com P436 fechado, continuamos com **DEBT-57** subset `gradients.rs`/`foundations.rs` ou pivotamos para **DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva). Indique se quer ajustar o escopo do P436.
