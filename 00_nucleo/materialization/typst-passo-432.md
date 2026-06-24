# P432 — Fecho de débito: DEBT-57 subset (`rules/stdlib/layout.rs`)

---

## Contexto

**DEBT-57** registava ~70 funções stdlib sem spec L0 dedicada. O subset `structural.rs` foi fechado no **P430**. O próximo subset mais fechável é `rules/stdlib/layout.rs` — 17 funções nativas (`align`, `place`, `grid`, `page`, `pad`, `hide`, `h`, `v`, `block`, `stack`, `box`, `repeat`, `columns`, `colbreak`, `measure`, `stroke`, `pagebreak`), todas com consumers reais em L1.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/layout.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas? | 17 funções | — |
| Todas têm consumer real? | 17/17 implementadas (P156C–P156L, P221, P223, P227–P232, P247, P250, P252) | ✅ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** S-M (17 entradas; ~45 min; formato estabelecido no P430).

---

## ADR-0107 — Paridade linguagem

Contrato documental: cada função nativa em `layout.rs` deve ter seu contrato L0 (assinatura, argumentos posicionais/nomeados, semântica, paridade vanilla, limitações graded, testes canônicos) em `00_nucleo/prompts/rules/stdlib/layout.md`.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/rules/stdlib/layout.md` com 17 secções (1 por função).
2. Cada secção: assinatura, args, semântica, paridade vanilla, limitações, testes canônicos.
3. Atualizar `rules/stdlib/_comum.md` — remover `layout.rs` da lista de ficheiros que apontam para o prompt comum.
4. Atualizar cabeçalho `@prompt` de `01_core/src/rules/stdlib/layout.rs` para apontar `layout.md` em último lugar.
5. Garantir que `layout.md` não fique órfão no `crystalline-lint` (re-referenciar em `layout/mod.rs` se necessário, paridade P430).
6. `DEBT.md` atualizado com nota "subset layout.rs fechado em P432".

---

## Scope-out explícito

- `measure(body)` runtime queries genuínas — continua diferido per ADR-0066.
- `columns()` / `colbreak()` multi-region flow real — scope-out per ADR-0078.
- `place()` float real — scope-out já materializado no P245, mas documentar como cumprido.
- Outros ficheiros stdlib sem spec L0 (`calc.rs`, `shapes.rs`, `transforms.rs`, etc.) — fora do escopo do P432.

---

## Critério de fecho

- [ ] `00_nucleo/prompts/rules/stdlib/layout.md` criado com 17 secções.
- [ ] Cada função documenta: assinatura, args, semântica, paridade vanilla, limitações, testes canônicos.
- [ ] `_comum.md` atualizado (layout.rs removido da lista).
- [ ] `layout.rs` cabeçalho `@prompt` aponta `layout.md`.
- [ ] `DEBT.md` atualizado com nota de fecho P432.
- [ ] `crystalline-lint` zero novas violações.
- [ ] `cargo test --workspace` verde (zero código modificado).
- [ ] DEBT-57 atualizado: "subset layout.rs fechado em P432".

---

**Próximo passo:** Com P432 fechado, continuamos com **DEBT-57** subset `calc.rs` ou **DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva). Indique se quer ajustar o escopo do P432.
