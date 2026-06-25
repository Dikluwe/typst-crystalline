# P451 — Heading numbering

> **Passo:** 451  
> **Data:** 2026-06-24  
> **Foco:** Implementar auto-numeração de headings (`#heading[...]` / `#1`, `#2`, etc.) com contador hierárquico e formatação configurável.  
> **ADR-0109:** Contadores hierárquicos e numeração de estrutura documental.

---

## Contexto

O Typst vanilla numera headings automaticamente quando `set heading(numbering: "1.")` está activo. O cristalino tem `Content::Heading` (entidade) e rendering básico, mas **não tem** contador de numeração nem prefixo numérico no layout. Este passo materializa o mecanismo de contador + numeração para headings.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Heading` existe? | Sim — entidade com `level: usize`, `body: Content` | ✅ |
| Rendering de heading existe? | Sim — P38 (layout básico, sem número) | ✅ |
| Contador hierárquico existe? | Não — zero infra de contador | ❌ |
| `numbering` como parâmetro de função nativa? | Não — `native_heading` não aceita `numbering` | ❌ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** M (~35 min; contador hierárquico + formatação de padrão + layout + tests).

---

## Toques pontuais

### 1. Contador hierárquico (`entities/counter.rs` ou `rules/eval/counter.rs`)

```rust
pub struct Counter {
    levels: Vec<usize>,  // [1, 2, 3] = 1.2.3
}

impl Counter {
    pub fn new() -> Self;
    pub fn step(&mut self, level: usize);  // incrementa nível, zera inferiores
    pub fn display(&self, pattern: &str) -> EcoString;  // "1." → "1.2.3."
}
```

- `step(1)` → `[1]` → `[2]`
- `step(2)` → `[2, 1]`
- `step(1)` → `[3]` (zera nível 2+)
- `step(3)` → `[3, 1, 1]`

### 2. Estado de eval com contador (`rules/eval/state.rs` ou `eval_context.rs`)

- `EvalContext` ganha `heading_counter: Counter`.
- `step_heading(level)` — chamado quando `Content::Heading` é avaliado.
- `heading_number(level, pattern) -> Option<EcoString>` — retorna `None` se `numbering` é `None`.

### 3. `native_heading` com `numbering` (`rules/stdlib/structural.rs`)

```rust
fn native_heading(
    level: usize,
    body: Content,
    numbering: Option<EcoString>,  // "1.", "1.", "(a)", etc.
) -> Content
```

- Se `numbering` é `Some`, emite `Content::Heading { level, body, numbering: Some(pattern) }`.
- Se `None`, emite sem numeração (comportamento actual).

### 4. Layout de heading com número (`rules/layout/heading.rs` ou `layout/mod.rs`)

- Ao encontrar `Content::Heading` com `numbering: Some(pattern)`:
  1. Invocar `counter.step(level)` via contexto de layout (ou receber o número pré-computado do eval).
  2. Formatar `counter.display(pattern)` → string numérica.
  3. Renderizar número como `FrameItem::Text` + espaço + `body`.
  4. O número usa o mesmo `TextStyle` do heading, possivelmente com `weight: Bold`.

**Pattern suportados (subset):**
- `"1."` → `1.`, `2.`, `1.1.`, `1.2.`
- `"1.1"` → `1.1`, `2.1`, `1.1.1`
- `"I."` → Romanos maiúsculos (I, II, III, IV, V... até 3999)
- `"(a)"` → Letras minúsculas (a, b, c... z, aa, ab)
- `"A."` → Letras maiúsculas

**Scope-out de pattern:** `"①"` (círculos), `"一"` (kanji), `"①②③"` — complexidade desnecessária para paridade vanilla básica.

### 5. Tests

- **L1 (counter unit):** 5 testes — step simples, step hierárquico, display "1.", display "I.", display "(a)", reset inferior.
- **L2 (eval):** 2 testes — `native_heading` com `numbering: Some("1.")` emite `Heading` com pattern; `numbering: None` omite.
- **L3 (layout E2E):** 3 testes — heading nível 1 com "1.", heading nível 2 com "1.1.", heading nível 1 após nível 2 reseta ("2.").

### 6. Spec L0

- `00_nucleo/prompts/entities/counter.md` — API do contador, padrões suportados.
- `00_nucleo/prompts/rules/stdlib/structural.md` — `heading(level, body, numbering?)`.
- `00_nucleo/prompts/rules/layout/heading.md` — numeração no layout.

---

## Scope-out explícito

- **Figure / table / equation counters** — scope-out; P452 ou posterior.
- **Page counter** — scope-out; depende de layout engine com pagination completa.
- **Bibliography counter** — scope-out.
- **Custom counter names** — scope-out; apenas contador implícito de heading.
- **`numbering` em `set` rule** — scope-out; apenas parâmetro de função nativa por ora.
- **Alignment do número (left/center/right/hanging indent)** — scope-out; número prefixado simples.

---

## Critério de fecho

- [ ] `Counter` implementado com `step` e `display`.
- [ ] 5 testes L1 de counter verdes.
- [ ] `native_heading` aceita `numbering: Option<EcoString>`.
- [ ] `EvalContext` mantém `heading_counter` e propaga para layout.
- [ ] Layout renderiza prefixo numérico antes do body do heading.
- [ ] 3 testes L3 de layout E2E verdes.
- [ ] Spec L0 actualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| Contador como entidade separada (`Content::Counter`) | Overkill; heading counter é o caso de uso dominante e pode ser internalizado no eval context. |
| Padrão de formatação como DSL completo (regex-like) | Complexidade desproporcionada; subset de 4 padrões cobre 95% dos casos reais. |
| Numeração computada no layout (não no eval) | Mais complexo; eval já tem a ordem sequencial de headings, layout é passivo. |

---

**Aguardando sua indicação:**

1. **Executar o P451** (heading numbering, ~35 min)?
2. **Pivotar para outra frente** (links/hyperlinks, figure numbering, DEBT-42 benchmark, table of contents)?
3. **Ajustar o escopo** do P451?
