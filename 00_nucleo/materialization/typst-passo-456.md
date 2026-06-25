# P456 — Equation numbering

> **Passo:** 456  
> **Data:** 2026-06-25  
> **Foco:** Materializar numeração automática de equações matemáticas (`$...$` e `$...$`) com contador independente e referência cruzada básica.  
> **ADR-0109:** Reaproveita `CounterRegistry` e `format_counter` (P451/P454); contador de equation é independente de heading e figure.

---

## Contexto

O Typst vanilla numera equações automaticamente quando `set math.equation(numbering: "(1)")` está activo. O cristalino tem `Content::Math` (entidade) e rendering básico desde os primeiros passos, mas **não tem** contador de equation nem número renderizado ao lado da equação. Este passo materializa o mecanismo de contador + numeração para equações, reaproveitando a infraestrutura `CounterRegistry` (P451) e `format_counter` (P451/P454).

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Math` existe? | Sim — entidade com `body: Content`, `block: bool` | ✅ |
| Rendering de math existe? | Sim — layout básico via `FrameItem::Text` (math mode) | ✅ |
| `CounterRegistry` existe? | Sim — P451/P177/P184C | ✅ |
| `format_counter` existe? | Sim — P451 (`entities/counter_format.rs`) | ✅ |
| `numbering` como parâmetro de equation? | Não — `native_equation` (se existe) não aceita `numbering` | ❌ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S (~20 min; contador de equation + formatação + layout lateral + tests).

---

## Toques pontuais

### 1. Contador de equation no `CounterRegistry`

Reaproveitar `CounterRegistry` (P451) com chave `"equation"`:

```rust
// Em eval, ao encontrar Content::Math com block=true e numbering=Some(pattern):
let equation_number = counter_registry.step("equation");
let formatted = format_counter(&equation_number, pattern.as_deref());
```

- Contador de equation é **independente** de heading e figure.
- Equações inline (`block: false`) **não são numeradas** — apenas block equations.
- `step("equation")` incrementa um contador de 1 dimensão (não hierárquico).

### 2. `MathElem` com `numbering` (`entities/elements/math.rs` ou `entities/content.rs`)

```rust
pub struct MathElem {
    pub body: Content,
    pub block: bool,
    pub numbering: Option<EcoString>,  // NOVO
}
```

**Decisão:** Campo em `MathElem` é preferível (análogo a `FigureElem` com `numbering` em P454, mas que foi rejeitado em favor da chain). No entanto, para math, o Typst vanilla usa `#set math.equation(numbering: "(1)")` que é um `set` rule na chain. **Decisão a tomar:** campo em `MathElem` vs chave na `StyleChain` (`math.equation.numbering`).

**Análise:**
- O Typst vanilla trata `math.equation` como um elemento com propriedades que podem ser set via `set` rule. No cristalino, isso mapeia para a `StyleChain` (como `figure.numbering` em P365/P454).
- **Decisão:** Usar a `StyleChain` com chave `math.equation.numbering` (análogo a `figure.numbering` e `heading.numbering.pattern`), não campo em `MathElem`. Isto é coerente com a arquitectura estabelecida em P365 e evita contradizer a ADR-0117 cláusula 4.

### 3. `native_equation` com `numbering` (ou `set math.equation` via StyleChain)

```rust
// Em rules/stdlib/math.rs (se existe) ou rules/stdlib/structural.rs:
// O Typst vanilla usa:
// #set math.equation(numbering: "(1)")
// Isso é um set rule que empurra na StyleChain.
```

- Não é função nativa com parâmetro; é `set` rule na `StyleChain`.
- O eval lê `math.equation.numbering` da `StyleChain` ao processar `Content::Math` com `block: true`.
- Se `Some(pattern)`, computa número e passa para layout.

**Decisão:** Não adicionar `native_equation` com parâmetro `numbering`. Em vez disso, o eval verifica a `StyleChain` por `math.equation.numbering` (chave custom) ao processar `Content::Math`. Isto é coerente com P365 (padrão na chain) e P454 (figure numbering na chain).

### 4. Layout de equation com número lateral (`rules/layout/math.rs` ou `rules/layout/mod.rs`)

- Ao encontrar `Content::Math` com `block: true` e `math.equation.numbering = Some(pattern)`:
  1. Eval (ou layout) computa número via `CounterRegistry` (chave `"equation"`).
  2. Formata: `format_counter(&[n], pattern)` → `"(1)"`, `"[I]"`, etc.
  3. Layout renderiza:
     - Equação centrada (ou alinhada conforme `align` na chain) como `FrameItem::Group`.
     - Número posicionado à **direita** do frame da equação, alinhado verticalmente ao centro.
     - Número como `FrameItem::Text` com estilo do math mode (ou estilo regular, dependendo do vanilla).
  4. Envolver equação + número em `FrameItem::Group` com `pos` e `size` agregados.

**Posicionamento do número:**
- O Typst vanilla posiciona o número à direita, alinhado verticalmente ao centro da equação.
- Margem direita: offset do número em relação ao limite direito do frame da equação.
- Se a equação + número exceder a largura da página, o vanilla pode quebrar linha — scope-out para este passo (assumir que cabe).

### 5. Pattern suportados (subset)

Mesmo subset de P451/P454:
- `"(1)"` → `(1)`, `(2)`, `(3)`
- `"[1]"` → `[1]`, `[2]`, `[3]`
- `"I."` → Romanos maiúsculos
- `"(a)"` → Letras minúsculas
- `"A."` → Letras maiúsculas

**Scope-out:** `"①"` (círculos), `"一"` (kanji), etc.

### 6. Tests

- **L1 (eval):** 2 testes — `Content::Math` com `block: true` e `math.equation.numbering = Some("(1)")` emite `MathElem` com número computado; `block: false` ignora numbering.
- **L2 (layout):** 2 testes — block equation com número posicionado à direita; inline equation sem número.
- **L3 (E2E):** 2 testes — duas equações block sequenciais numeram `(1)`, `(2)`; equação com pattern `"[I]"` usa romanos.

### 7. Spec L0

- `00_nucleo/prompts/entities/math.md` — `MathElem` (sem campo `numbering`; padrão na chain).
- `00_nucleo/prompts/rules/layout/math.md` — equation block com número lateral.
- `00_nucleo/prompts/rules/eval/math.md` — leitura de `math.equation.numbering` da chain.

---

## Scope-out explícito

- **Equation inline numbering** — scope-out; apenas block equations são numeradas (comportamento vanilla).
- **Numbering posicionado à esquerda** — scope-out; apenas à direita (comportamento vanilla default).
- **Numbering em duas colunas (esquerda e direita)** — scope-out; apenas número à direita.
- **Cross-reference (`@eq1`)** — scope-out; depende de `label`/`ref` (P457+).
- **Sub-equations (1a, 1b)** — scope-out; contador hierárquico de 2 dimensões.
- **Equation alignment com número** — scope-out; assume equação centrada simples.
- **Numbering em math mode (fonte math)** — scope-out; número renderizado em fonte regular (simplificação aceitável para subset minimal).
- **`#set math.equation(numbering: none)` para desactivar** — scope-out; `None` na chain já suprime.

---

## Critério de fecho

- [ ] Eval lê `math.equation.numbering` da `StyleChain` ao processar `Content::Math`.
- [ ] Block equation com `numbering = Some(pattern)` computa número via `CounterRegistry` (chave `"equation"`).
- [ ] Layout renderiza número à direita da equação, alinhado verticalmente ao centro.
- [ ] Inline equation ignora numbering (não numerada).
- [ ] 6 tests verdes (2 L1 + 2 L2 + 2 L3).
- [ ] Spec L0 actualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| Campo `numbering` em `MathElem` (análogo ao que P454 propôs para `FigureElem`) | Rejeitado em P454 em favor da chain (P365). Aplicar a `MathElem` seria inconsistente com a arquitectura vigente e violaria a ADR-0117 cláusula 4. |
| `native_equation(body, numbering?)` como função nativa | O Typst vanilla não expõe `equation` como função nativa com parâmetro; é um `set` rule. Usar função nativa seria divergência de paridade vanilla. |
| Contador de equation no layout (não no eval) | O layout não tem acesso à ordem sequencial de equações no documento; o eval tem. Computar no eval é coerente com P451/P454. |
| Número como `FrameItem::Text` em math mode | Simplificação para subset minimal; fonte regular é aceitável e evita complexidade de math mode para números simples. |

---

**Aguardando sua indicação:**

1. **Executar o P456** (equation numbering, ~20 min)?
2. **Pivotar para outra frente** (table of contents, label/ref, DEBT-42 benchmark)?
3. **Ajustar o escopo** do P456?
