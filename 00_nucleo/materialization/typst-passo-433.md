# P433 — Fecho de débito: DEBT-57 subset (`rules/stdlib/calc.rs`)

---

## Contexto

**DEBT-57** registava ~70 funções stdlib sem spec L0 dedicada. Os subsets `structural.rs` (P430) e `layout.rs` (P432) foram fechados. O próximo subset mais fechável é `rules/stdlib/calc.rs` — 21 funções nativas (`sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `exp`, `ln`, `log`, `pi`, `tau`, `e`, `inf`), com cobertura ~74% já materializada (P283).

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/calc.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas? | 21 funções (10 trig + 6 hiperbólicas + 2 exp/log + 3 constantes) | — |
| Todas têm consumer real? | 21/21 implementadas (P283) | ✅ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** S (~30 min; 21 entradas; formato estabelecido P430/P432).

---

## ADR-0107 — Paridade linguagem

Contrato documental: cada função nativa em `calc.rs` deve ter seu contrato L0 (assinatura, argumentos, semântica, domínio, paridade vanilla, limitações graded, testes canônicos) em `00_nucleo/prompts/rules/stdlib/calc.md`.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/rules/stdlib/calc.md` com 21 secções (1 por função).
2. Cada secção: assinatura, args, domínio (radianos/graus), semântica, paridade vanilla, limitações, testes canônicos.
3. Atualizar `rules/stdlib/_comum.md` — remover `calc.rs` da lista de ficheiros que apontam para o prompt comum.
4. Atualizar cabeçalho `@prompt` de `01_core/src/rules/stdlib/calc.rs` para apontar `calc.md` em último lugar.
5. `DEBT.md` atualizado com nota "subset calc.rs fechado em P433".

---

## Scope-out explícito

- Bucket 2 adiado P283 (`root`, `erf`, `fact`, `perm`, `binom`, `gcd`, `lcm`, `trunc`, `fract`, `even`, `odd`, `rem`, `div_euclid`, `rem_euclid`, `quo`, `norm`, extensões `Length`/`Angle`/`Decimal`) — continua scope-out; não é parte deste fecho.
- `log(base, value)` com aridade 2 — scope-out (hoje só `log(value)`).
- Outros ficheiros stdlib sem spec L0 (`shapes.rs`, `transforms.rs`, `gradients.rs`, `assert.rs`, `foundations.rs`) — fora do escopo do P433.

---

## Critério de fecho

- [ ] `00_nucleo/prompts/rules/stdlib/calc.md` criado com 21 secções.
- [ ] Cada função documenta: assinatura, args, domínio, semântica, paridade vanilla, limitações, testes canônicos.
- [ ] `_comum.md` atualizado (calc.rs removido da lista).
- [ ] `calc.rs` cabeçalho `@prompt` aponta `calc.md`.
- [ ] `DEBT.md` atualizado com nota de fecho P433.
- [ ] `crystalline-lint` zero novas violações.
- [ ] `cargo test --workspace` verde (zero código modificado).
- [ ] DEBT-57 atualizado: "subset calc.rs fechado em P433".

---

**Próximo passo:** Com P433 fechado, continuamos com **DEBT-57** subset `assert.rs` (XS, trivial) ou **DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva). Indique se quer ajustar o escopo do P433.
