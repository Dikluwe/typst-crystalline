# P454 — Figure numbering

> **Passo:** 454  
> **Data:** 2026-06-24  
> **Foco:** Materializar numeração automática de figures (`#figure[...]`) com contador independente e caption prefixado.  
> **ADR-0109:** Reaproveita `CounterRegistry` do P451; contador de figure é independente do contador de heading.

---

## Contexto

O Typst vanilla numera figures automaticamente quando `set figure(numbering: "1.")` está activo. O cristalino tem `Content::Figure` (entidade) e rendering básico desde P38, mas **não tem** contador de figure nem caption prefixado com número. Este passo materializa o mecanismo de contador + numeração para figures, reaproveitando a infraestrutura `CounterRegistry` estabelecida no P451.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Figure` existe? | Sim — entidade com `body: Content`, `caption: Option<Content>` | ✅ |
| Rendering de figure existe? | Sim — P38 (layout básico, sem número) | ✅ |
| `CounterRegistry` existe? | Sim — P451/P177/P184C | ✅ |
| `format_counter` existe? | Sim — P451 (`entities/counter_format.rs`) | ✅ |
| `numbering` como parâmetro de figure? | Não — `native_figure` não aceita `numbering` | ❌ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S (~25 min; contador de figure + formatação + caption prefixado + tests).

---

## Toques pontuais

### 1. Contador de figure no `CounterRegistry`

Reaproveitar `CounterRegistry` (P451) com chave `"figure"`:

```rust
// Em eval, ao encontrar Content::Figure:
let figure_number = counter_registry.step("figure");
let formatted = format_counter(&figure_number, pattern.as_deref());
```

- Contador de figure é **independente** do contador de heading.
- `step("figure")` incrementa um contador de 1 dimensão (não hierárquico).
- `display` usa `format_counter` com pattern `"1."`, `"I."`, `"(a)"`, etc. (mesmo subset do P451).

### 2. `FigureElem` com `numbering` (`entities/elements/figure.rs` ou `entities/content.rs`)

```rust
pub struct FigureElem {
    pub body: Content,
    pub caption: Option<Content>,
    pub numbering: Option<EcoString>,  // NOVO
}
```

- Ou adicionar `numbering` como `Style` via `StyleChain` (decisão a tomar).
- **Decisão:** Campo em `FigureElem` é preferível porque `numbering` é propriedade estrutural do figure, não estilo tipográfico. Alinha com `HeadingElem` (que tem `numbering` como campo após P451).

### 3. `native_figure` com `numbering` (`rules/stdlib/structural.rs`)

```rust
fn native_figure(
    body: Content,
    caption: Option<Content>,
    numbering: Option<EcoString>,  // NOVO
) -> Content {
    Content::Figure(FigureElem { body, caption, numbering })
}
```

- Registar no stdlib scope como `"figure"`.
- Default `numbering: None` (comportamento actual: sem numeração).

### 4. Layout de figure com caption numerada (`rules/layout/figure.rs`)

- Ao encontrar `Content::Figure` com `numbering: Some(pattern)`:
  1. Invocar `counter_registry.step("figure")` (ou receber do eval).
  2. Formatar número: `format_counter(&[n], pattern)` → `"Fig. 1."` ou `"Figure 1"`.
  3. Renderizar `body` (imagem, table, etc.) como `FrameItem::Group`.
  4. Renderizar caption como `FrameItem::Text` prefixado com número formatado:
     - `TextItem` com `"Fig. 1: "` + caption body.
     - Caption posicionada abaixo do body (centro-alinhada, margem superior).
  5. Envolver body + caption em `FrameItem::Group` com `pos` e `size` agregados.

**Caption prefixo:** O Typst vanilla usa o label localizado ("Figure", "Fig.", "Abbildung", etc.) mas isso depende de i18n — scope-out. Usar prefixo simples `"Fig. "` + número + `": "` como default.

### 5. Show rule para figure (`rules/eval/rules.rs`)

- Selector `NodeKind::Figure` já deve existir (P38).
- Se `numbering` é `Some`, o eval deve:
  1. Step do contador `"figure"`.
  2. Formatar número.
  3. Prefixar caption com número formatado (ou passar número para layout).

**Decisão:** O eval computa o número e o passa para o layout como `FigureElem` enriquecido, ou o layout lê do `CounterRegistry`? 
- **Preferência:** Eval computa o número (tem acesso ao `CounterRegistry` e à ordem sequencial). Layout recebe `FigureElem` com `caption_number: Option<EcoString>` pré-computado. Isto evita que o layout precise de estado mutável.

### 6. Tests

- **L1 (eval):** 2 testes — `native_figure` com `numbering: Some("1.")` emite `FigureElem` com pattern; `numbering: None` omite.
- **L2 (layout):** 2 testes — figure com caption numerada posiciona número antes do texto; figure sem numbering não tem prefixo.
- **L3 (E2E):** 2 testes — 2 figures sequenciais numeram "Fig. 1", "Fig. 2"; figure com pattern `"I."` usa romanos.

### 7. Spec L0

- `00_nucleo/prompts/entities/figure.md` — `FigureElem` com `numbering`.
- `00_nucleo/prompts/rules/stdlib/structural.md` — `native_figure(body, caption?, numbering?)`.
- `00_nucleo/prompts/rules/layout/figure.md` — caption com prefixo numérico.

---

## Scope-out explícito

- **Table numbering** — scope-out; table é body genérico do figure, não contador separado.
- **Equation numbering** — scope-out; requer `CounterRegistry` com chave `"equation"`, mas é passo separado.
- **List of figures (LoF)** — scope-out; requer TOC infraestrutura.
- **Cross-reference (`@fig1`)** — scope-out; depende de `label`/`ref` (P455+).
- **i18n prefixo ("Figure" vs "Fig." vs "Abbildung")** — scope-out; apenas "Fig. " fixo.
- **Caption posicionada acima** — scope-out; apenas abaixo (comportamento vanilla default).
- **Sub-figures (a, b, c)** — scope-out; contador hierárquico de 2 dimensões.

---

## Critério de fecho

- [ ] `FigureElem` ganha campo `numbering: Option<EcoString>`.
- [ ] `native_figure` aceita `numbering: Option<EcoString>`.
- [ ] Eval computa número via `CounterRegistry` (chave `"figure"`) e prefixa caption.
- [ ] Layout renderiza body + caption numerada como `FrameItem::Group`.
- [ ] 6 tests verdes (2 L1 + 2 L2 + 2 L3).
- [ ] Spec L0 actualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| `Style::FigureNumbering(EcoString)` em vez de campo em `FigureElem` | `numbering` é propriedade estrutural do elemento, não estilo tipográfico. Usar `Style` confundiria com `strong`/`emph` e complicaria o `StyleChain`. |
| Contador de figure no layout (não no eval) | O layout não tem acesso à ordem sequencial de figures no documento; o eval tem. Computar no eval é mais simples e evita estado mutável no layout. |
| Caption como elemento separado (`Content::Caption`) | Overkill para subset minimal; caption é propriedade do figure no Typst vanilla. |

---

**Aguardando sua indicação:**

1. **Executar o P454** (figure numbering, ~25 min)?
2. **Pivotar para outra frente** (table of contents, equation numbering, label/ref, DEBT-2 investigação)?
3. **Ajustar o escopo** do P454?
