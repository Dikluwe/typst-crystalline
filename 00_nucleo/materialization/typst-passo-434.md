# P434 — Fecho de débito: DEBT-57 subset (`rules/stdlib/assert.rs`)

---

## Contexto

**DEBT-57** registava ~70 funções stdlib sem spec L0 dedicada. Os subsets `structural.rs` (P430), `layout.rs` (P432) e `calc.rs` (P433) foram fechados. O próximo subset mais fechável é `rules/stdlib/assert.rs` — **1 função nativa** (`assert`), trivial e XS.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/assert.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas? | 1 função (`assert`) | — |
| Tem consumer real? | Sim — `native_assert` em `stdlib/assert.rs` | ✅ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** XS (~10 min; 1 entrada; formato estabelecido P430–P433).

---

## ADR-0107 — Paridade linguagem

Contrato documental: `assert(cond, msg)` deve ter seu contrato L0 (assinatura, argumentos, semântica, paridade vanilla, testes canônicos) em `00_nucleo/prompts/rules/stdlib/assert.md`.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/rules/stdlib/assert.md` com 1 secção.
2. Secção: assinatura, args (`cond: bool`, `msg: str`), semântica (panic se `cond == false`), paridade vanilla, testes canônicos.
3. Atualizar `rules/stdlib/_comum.md` — remover `assert.rs` da lista.
4. Atualizar cabeçalho `@prompt` de `01_core/src/rules/stdlib/assert.rs` para apontar `assert.md`.
5. `DEBT.md` atualizado com nota "subset assert.rs fechado em P434".

---

## Scope-out explícito

- Outros ficheiros stdlib sem spec L0 (`shapes.rs`, `transforms.rs`, `gradients.rs`, `foundations.rs`) — fora do escopo do P434.

---

## Critério de fecho

- [ ] `00_nucleo/prompts/rules/stdlib/assert.md` criado com 1 secção.
- [ ] `assert.md` documenta: assinatura, args, semântica, paridade vanilla, testes canônicos.
- [ ] `_comum.md` atualizado (assert.rs removido da lista).
- [ ] `assert.rs` cabeçalho `@prompt` aponta `assert.md`.
- [ ] `DEBT.md` atualizado com nota de fecho P434.
- [ ] `crystalline-lint` zero novas violações.
- [ ] `cargo test --workspace` verde (zero código modificado).
- [ ] DEBT-57 atualizado: "subset assert.rs fechado em P434".

---

**Próximo passo:** Com P434 fechado, continuamos com **DEBT-57** subset `shapes.rs`/`transforms.rs`/`gradients.rs` ou pivotamos para **DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva). Indique se quer ajustar o escopo do P434.
