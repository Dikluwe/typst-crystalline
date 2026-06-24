# P438 — Fecho de débito: DEBT-57 subset (`rules/stdlib/foundations.rs`) + FECHO COMPLETO DEBT-57

---

## Contexto

**DEBT-57** registava ~70 funções stdlib sem spec L0 dedicada. Os subsets `structural.rs` (P430), `layout.rs` (P432), `calc.rs` (P433), `assert.rs` (P434), `shapes.rs` (P435), `transforms.rs` (P436) e `gradients.rs` (P437) foram fechados. Resta o último subset: `rules/stdlib/foundations.rs` — **~30 funções nativas** (`type`, `len`, `range`, `int`, `float`, `str`, `rgb`, `luma`, `oklab`, `oklch`, `linear_rgb`, `cmyk`, `hsl`, `hsv`, `datetime`, `decimal`, `duration`, `version`, `bytes`, `regex`, `module`, `array`, `dict`, `str_methods`, etc.), todas com consumers reais em L1.

> **Marco:** este passo fecha não apenas o subset `foundations.rs`, mas **DEBT-57 por completo** — todas as funções stdlib de L1 passam a ter spec L0 dedicada.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/foundations.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas? | ~30 funções | — |
| Todas têm consumer real? | ~30/30 implementadas (P13–P25, P99–P102, P391–P402) | ✅ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** M (~1h; ~30 entradas; formato estabelecido P430–P437; **último subset, fecha DEBT-57**).

---

## ADR-0107 — Paridade linguagem

Contrato documental: cada função nativa em `foundations.rs` deve ter seu contrato L0 (assinatura, argumentos posicionais/nomeados, semântica, paridade vanilla, limitações graded, testes canônicos) em `00_nucleo/prompts/rules/stdlib/foundations.md`.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/rules/stdlib/foundations.md` com ~30 secções (1 por função / grupo lógico).
2. Cada secção: assinatura, args, semântica, paridade vanilla, limitações, testes canônicos.
3. Atualizar `rules/stdlib/_comum.md` — remover `foundations.rs` da lista; adicionar nota de fecho completo DEBT-57.
4. Atualizar cabeçalho `@prompt` de `01_core/src/rules/stdlib/foundations.rs` para apontar `foundations.md`.
5. `DEBT.md` atualizado: **DEBT-57 reclassificado como FECHADO (P438)**.

---

## Scope-out explícito

- Color spaces não-RGB (`oklab`, `oklch`, `linear_rgb`, `cmyk`, `hsl`, `hsv`) — `Value::Color` só suporta RGB; scope-out ADR-0054 graded.
- `str` methods avançadas (`contains`, `starts_with`, `ends_with`, `split`, `trim`, etc.) — subset implementado; scope-out das restantes.
- `array` / `dict` methods avançadas — subset implementado; scope-out das restantes.
- `module` introspection avançada — scope-out.

---

## Critério de fecho

- [ ] `00_nucleo/prompts/rules/stdlib/foundations.md` criado com ~30 secções.
- [ ] Cada função documenta: assinatura, args, semântica, paridade vanilla, limitações, testes canônicos.
- [ ] `_comum.md` actualizado (foundations.rs removido; nota de fecho completo DEBT-57).
- [ ] `foundations.rs` cabeçalho `@prompt` aponta `foundations.md`.
- [ ] `DEBT.md` actualizado: **DEBT-57 reclassificado como FECHADO (P438)**.
- [ ] `crystalline-lint` zero novas violações.
- [ ] `cargo test --workspace` verde (zero código funcional modificado).
- [ ] DEBT-57 **FECHADO COMPLETAMENTE** — todos os ficheiros stdlib têm spec L0 dedicada.

---

**Próximo passo:** Com DEBT-57 fechado, os débitos em aberto restantes são: **DEBT-43** (linter type-level), **DEBT-55** (probe hayagriva), **DEBT-42** (`get_unchecked` bloqueado por benchmark). Indique se quer ajustar o escopo do P438.
