P430 — Fecho de débito: DEBT-57 subset (Spec L0 para `rules/stdlib/structural.rs`)

---

### ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda DEBT-57:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/structural.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas no ficheiro? | 21 funções (strong, emph, raw, heading, divider, terms, quote, table, table_cell, table_header, table_footer, bibliography, cite, footnote, accent, cancel, underover, op, grid, grid_cell, grid_header, grid_footer) | — |
| Quais já têm consumer real em L1? | strong ✓, emph ✓, raw ✓, heading ✓, divider ✓, terms ✓, quote ✓, table ✓, table_cell ✓, table_header ✓, table_footer ✓, bibliography ✓, cite ✓, footnote ✓, accent ✓, cancel ✓, underover ✓, op ✓, grid ✓, grid_cell ✓, grid_header ✓, grid_footer ✓ | 21/21 implementadas |
| Existe ADR que proíba spec L0? | Não — ADR-0104 autoriza prompts por ficheiro | ✅ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** DEBT-57 = **M** total, mas fatiável por ficheiro. O subset `structural.rs` = **S-M** (~1h; 21 entradas documentais).

---

### ADR-0107 — Paridade linguagem

O contrato é **documental**, não funcional: cada função nativa em `structural.rs` deve ter seu contrato L0 (argumentos, semântica, erros, paridade vanilla) escrito em `00_nucleo/prompts/engine/stdlib/structural.md`, desvinculando a especificação do prompt grosso `stdlib/_comum.md`.

---

### ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/engine/stdlib/structural.md` com 21 secções (1 por função nativa).
2. Cada secção: assinatura, argumentos posicionais/nomeados, semântica, paridade vanilla, limitações conhecidas (graded), testes canônicos.
3. Atualizar `rules/stdlib.md` (prompt grosso) para apontar `structural.rs → structural.md` em vez de `_comum.md`.
4. `crystalline-lint` zero violations (prompts órfãos atualizados).

---

### Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Formato do spec | Markdown por função, em ficheiro único `structural.md` | Paridade com `stdlib/layout.md`, `stdlib/calc.md` (precedente P314/P96.5) |
| Granularidade | 1 sub-secção por função nativa | Fatiável; permite revisão cirúrgica futura |
| Scope-out explícito | CSL styling (bibliography/cite), shaping real (accent/cancel), multi-region grid | Preservado como graded; não é parte deste fecho documental |
| Hash L0 | Atualizado em `structural.md` + referência cruzada em `stdlib.md` | ADR-0080 EM VIGOR (L0 minimal para refactors) |

---

### Scope-out explícito

- CSL styling completo (bibliography/cite) — continua scope-out; DEBT-55 permanece parcial.
- Shaping OpenType real (accent/cancel/underover/op) — continua scope-out; ADR-0054 graded.
- `grid` multi-region flow real — scope-out per ADR-0078 IMPLEMENTADO.
- Outros ficheiros stdlib sem spec L0 (layout.rs, calc.rs, etc.) — fora do escopo do P430; permanecem em DEBT-57.

---

### Critério de fecho

- [ ] `00_nucleo/prompts/engine/stdlib/structural.md` criado com 21 secções.
- [ ] Cada função documenta: assinatura, args, semântica, paridade vanilla, limitações.
- [ ] `rules/stdlib.md` atualizado para apontar `structural.md` em vez de `_comum.md`.
- [ ] DEBT-57 atualizado em `DEBT.md` com nota "subset structural.rs fechado em P430".
- [ ] `crystalline-lint` zero violations (prompts órfãos atualizados).
- [ ] `cargo test --workspace` verde (zero código modificado; tests baseline preservados).

---

**Próximo passo:** Com P430 fechado, continuamos com **DEBT-55** (ADR-0062 hayagriva + probe) ou **DEBT-50** (refactor bake-in pré-requisito). Indique se quer pivotar para código ou manter o ritmo documental.
