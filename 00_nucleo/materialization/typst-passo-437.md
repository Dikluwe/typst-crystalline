# P437 — Fecho de débito: DEBT-57 subset (`rules/stdlib/gradients.rs`)

---

## Contexto

**DEBT-57** registava ~70 funções stdlib sem spec L0 dedicada. Os subsets `structural.rs` (P430), `layout.rs` (P432), `calc.rs` (P433), `assert.rs` (P434), `shapes.rs` (P435) e `transforms.rs` (P436) foram fechados. O próximo subset mais fechável é `rules/stdlib/gradients.rs` — **3 funções nativas** (`gradient_linear`, `gradient_radial`, `gradient_conic`).

> **Nota:** as funções de gradiente estão **scope-out** em L1 (`Value::Gradient` ausente; render PDF gradient ausente). Este passo documenta o estado actual e o scope-out, fechando o subset documentalmente.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `rules/stdlib/gradients.rs` tem spec L0 dedicado? | Não — aponta para `stdlib/_comum.md` | ❌ |
| Quantas funções nativas? | 3 funções | — |
| Consumers reais em L1? | 0/3 — `Value::Gradient` ausente; render ausente | ❌ (scope-out) |
| Existe stub/planning em L0? | Não — zero referências a gradient em prompts L0 | ❌ |
| Bloqueadores? | Nenhum técnico; trabalho documental puro | ✅ |

**Reclassificação:** XS (~10 min; 3 entradas; formato estabelecido P430–P436; **documentação de scope-out**).

---

## ADR-0107 — Paridade linguagem

Contrato documental: cada função nativa em `gradients.rs` deve ter seu contrato L0 — mesmo que o contrato seja "scope-out, aguarda `Value::Gradient` + render PDF gradient". A honestidade epistémica exige que o prompt L0 declare explicitamente o que não está implementado.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/prompts/rules/stdlib/gradients.md` com 3 secções (1 por função).
2. Cada secção: assinatura vanilla, args, semântica esperada, **scope-out explícito** (`Value::Gradient` ausente; render PDF gradient ausente; ADR-0054 graded), testes canônicos (stub que retorna `Err` ou `Value::None`).
3. Atualizar `rules/stdlib/_comum.md` — remover `gradients.rs` da lista.
4. Atualizar cabeçalho `@prompt` de `01_core/src/rules/stdlib/gradients.rs` para apontar `gradients.md`.
5. `DEBT.md` atualizado com nota "subset gradients.rs fechado em P437 (documental, scope-out)".

---

## Scope-out explícito

- `Value::Gradient` — ausente em `entities/value.rs`; tipo e consumer não materializados.
- Render PDF gradient — requer `Paint::Gradient` + operadores PDF `/Sh` (shading); não implementado.
- `gradient_linear`/`gradient_radial`/`gradient_conic` — stubs em `stdlib/gradients.rs` retornam `Err` ou `Value::None`.
- `Color` space extensions (`oklab`, `oklch`, `linear_rgb`, `cmyk`, `hsl`, `hsv`) — scope-out relacionado, fora do escopo deste subset.

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Formato do spec | Markdown por função, em ficheiro único `gradients.md` | Paridade com P430–P436 |
| Honestidade do scope-out | Declarar explicitamente `Value::Gradient` ausente e render PDF gradient como bloqueadores | ADR-0033 paridade funcional; não inventar spec para código inexistente |
| Stubs em `gradients.rs` | Preservados — não modificar código funcional | Zero código modificado; apenas documentação |

---

## Critério de fecho

- [ ] `00_nucleo/prompts/rules/stdlib/gradients.md` criado com 3 secções.
- [ ] Cada função documenta: assinatura vanilla, args, semântica esperada, scope-out explícito, testes canônicos (stub).
- [ ] `_comum.md` atualizado (gradients.rs removido da lista).
- [ ] `gradients.rs` cabeçalho `@prompt` aponta `gradients.md`.
- [ ] `DEBT.md` atualizado com nota de fecho P437.
- [ ] `crystalline-lint` zero novas violações.
- [ ] `cargo test --workspace` verde (zero código funcional modificado).
- [ ] DEBT-57 atualizado: "subset gradients.rs fechado em P437 (documental, scope-out)".

---

**Próximo passo:** Com P437 fechado, **DEBT-57** reduz-se ao subset `foundations.rs` (~30 funções, o último ficheiro stdlib). Alternativas fora de DEBT-57: **DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva). Indique se quer ajustar o escopo do P437.
