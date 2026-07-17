# P435 — Fecho de débito: DEBT-57 subset (`rules/stdlib/shapes.rs`)

---

## Contexto

**DEBT-57** registava ~70 funções stdlib sem spec L0 dedicada. Os subsets `structural.rs` (P430), `layout.rs` (P432), `calc.rs` (P433) e `assert.rs` (P434) foram fechados. O próximo subset mais fechável é `rules/stdlib/shapes.rs` — **6 funções nativas** (`rect`, `ellipse`, `circle`, `line`, `polygon`, `curve`), todas com consumers reais em L1.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/shapes.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas? | 6 funções | — |
| Todas têm consumer real? | 6/6 implementadas (P78–P79, P277, P293–P294) | ✅ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** S (~20 min; 6 entradas; formato estabelecido P430–P434).

---

## ADR-0107 — Paridade linguagem

Contrato documental: cada função nativa em `shapes.rs` deve ter seu contrato L0 (assinatura, argumentos posicionais/nomeados, semântica, paridade vanilla, limitações graded, testes canônicos) em `00_nucleo/prompts/engine/stdlib/shapes.md`.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/engine/stdlib/shapes.md` com 6 secções (1 por função).
2. Cada secção: assinatura, args (`width`, `height`, `fill`, `stroke`, `radius`, etc.), semântica, paridade vanilla, limitações, testes canônicos.
3. Atualizar `rules/stdlib/_comum.md` — remover `shapes.rs` da lista.
4. Atualizar cabeçalho `@prompt` de `01_core/src/engine/stdlib/shapes.rs` para apontar `shapes.md`.
5. `DEBT.md` atualizado com nota "subset shapes.rs fechado em P435".

---

## Scope-out explícito

- `cmyk` / `oklab` / `oklch` / `linear_rgb` / `hsl` / `hsv` — color spaces não-RGB ausentes; fora do escopo de `shapes.rs`.
- `gradient` — `Value::Gradient` ausente; fora do escopo.
- Outros ficheiros stdlib sem spec L0 (`transforms.rs`, `gradients.rs`, `foundations.rs`) — fora do escopo do P435.

---

## Critério de fecho

- [ ] `00_nucleo/prompts/engine/stdlib/shapes.md` criado com 6 secções.
- [ ] Cada função documenta: assinatura, args, semântica, paridade vanilla, limitações, testes canônicos.
- [ ] `_comum.md` atualizado (shapes.rs removido da lista).
- [ ] `shapes.rs` cabeçalho `@prompt` aponta `shapes.md`.
- [ ] `DEBT.md` atualizado com nota de fecho P435.
- [ ] `crystalline-lint` zero novas violações.
- [ ] `cargo test --workspace` verde (zero código modificado).
- [ ] DEBT-57 atualizado: "subset shapes.rs fechado em P435".

---

**Próximo passo:** Com P435 fechado, continuamos com **DEBT-57** subset `transforms.rs`/`gradients.rs`/`foundations.rs` ou pivotamos para **DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva). Indique se quer ajustar o escopo do P435.
