# P457 — Table of Contents (`outline()`)

> **Passo:** 457  
> **Data:** 2026-06-25  
> **Foco:** Materializar a função nativa `outline()` para gerar sumário automático do documento, listando headings numerados com page numbers.  
> **Trilha:** 1 — Numeração restante (depende de P451 heading numbering).  
> **ADR-0109:** Reaproveita `CounterRegistry` (P451) e `format_counter` (P451); requer introspecção de headings.

---

## Contexto

O Typst vanilla suporta `#outline()` para gerar sumário automático com títulos de headings e números de página. O cristalino tem `Content::Heading` com numeração (P451) e rendering básico, mas **não tem** mecanismo de introspecção para colectar headings do documento nem função `outline()`. Este passo materializa o subset minimal: `outline()` que lista headings de nível 1–3 com título + número + page number placeholder (ou real, se pagination existir).

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Heading` com numeração existe? | Sim — P451 (heading numbering via chain + CounterRegistry) | ✅ |
| `CounterRegistry` / `Introspector` existe? | Sim — P451/P177/P184C | ✅ |
| `format_counter` existe? | Sim — P451 | ✅ |
| Introspecção de headings (colectar do documento)? | Parcial — `Introspector` tem `query_by_kind`? | 🟡 |
| `outline()` função nativa? | Não | ❌ |
| Page numbers no layout? | Parcial — pagination básica existe? | 🟡 |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S-M (~30 min; colecção de headings + formatação de sumário + layout + tests).

**Nota:** O roteiro de conclusão (anexado) pede três verificações antes de avançar:
1. Gate ADR-0117 no `crystalline-lint` — implementado ou proposta?
2. `Selector::Where` — realmente ausente ou inventário desatualizado face a P417/P423?
3. `link` no inventário — linha 213/342 marca "sem render visual" (parcial), mas P422–424/P452 materializaram render + annotation PDF.

Estas verificações são feitas como parte desta sonda A.0 (coerente com ADR-0117 cláusula 1).

---

## Toques pontuais

### 1. Introspecção de headings (`rules/introspect.rs` ou `entities/introspector.rs`)

Reaproveitar `Introspector` existente (P177/P184C/P451):

```rust
// Introspector já deve suportar query por kind/location
pub trait Introspector {
    fn query_by_kind(&self, kind: NodeKind) -> Vec<(&Content, Location)>;
    fn position_of(&self, loc: Location) -> Option<Position>;  // page/line/column
}
```

- Verificar se `query_by_kind(NodeKind::Heading)` retorna todos os headings do documento em ordem.
- Se `Introspector` não tem `query_by_kind`, implementar como walk do documento `Content` colectando `Content::Heading`.

**Decisão:** Se `query_by_kind` existe e funciona, usar. Se não, implementar walk minimal de colecção de headings.

### 2. `outline()` como função nativa (`rules/stdlib/structural.rs`)

```rust
fn native_outline(
    title: Option<Content>,      // Título do sumário (default: "Contents")
    depth: Option<usize>,         // Profundidade máxima (default: 3)
    indent: Option<bool>,         // Indentar sub-headings (default: true)
) -> Content
```

- Registar no stdlib scope como `"outline"`.
- Retorna `Content::Outline(OutlineElem { title, depth, indent })`.

**Scope-out de parâmetros:** `fill` (leader dots), `target` (selector personalizado), `title` como string simples (não `Content` rico). Subset minimal.

### 3. `Content::Outline` como nova variante (`entities/content.rs`)

```rust
pub struct OutlineElem {
    pub title: Option<Content>,
    pub depth: usize,
    pub indent: bool,
}

pub enum Content {
    // ... existing variants
    Outline(OutlineElem),  // NOVO
}
```

### 4. Eval de `outline()` (`rules/eval/rules.rs`)

- `native_outline` constrói `Content::Outline`.
- O eval **não** expande o outline em tempo de eval — isso requer 2-pass (introspecção após layout completo).
- **Decisão:** Implementar como 2-pass? Ou 1-pass com colecção de headings no próprio eval?

**Análise:**
- O Typst vanilla faz 2-pass: primeiro layout completo, depois introspecção, depois outline renderizado.
- O cristalino tem `Introspector` (P177/P184C) que parece suportar 2-pass (`layout_with_introspector`).
- **Decisão:** Usar 2-pass se infraestrutura existir. Se não, implementar 1-pass com colecção de headings durante o eval walk (menos preciso para page numbers, mas suficiente para sumário sem page numbers).

**Simplificação para subset minimal:** 1-pass com colecção de headings durante eval. Page numbers são scope-out ou placeholder (`"?"`).

### 5. Layout de outline (`engine/layout/outline.rs` ou `engine/layout/mod.rs`)

- Ao encontrar `Content::Outline`:
  1. Colecionar headings do documento (via `Introspector` ou lista pré-computada no eval context).
  2. Para cada heading (até `depth`):
     - Formatar número: `format_counter(&heading.number, "1.")` (ou pattern do heading).
     - Formatar título: heading body como `TextItem`.
     - Formatar page number: placeholder `"?"` (scope-out; requer pagination completa).
     - Combinar: `"1. Título do Heading ................ ?"`.
  3. Renderizar como `FrameItem::Text` linhas sequenciais, com indentação para níveis > 1.
  4. Envolver em `FrameItem::Group`.

**Indentação:**
- Nível 1: sem indent (`0em`).
- Nível 2: `1em` indent.
- Nível 3: `2em` indent.
- Usar `TextItem` com leading spaces ou `FrameItem::Group` com offset X.

**Leader dots:** Scope-out. Apenas espaço simples entre título e page number.

### 6. Page numbers — scope-out ou placeholder

- O cristalino pode não ter pagination completa (page numbers por frame).
- **Decisão:** Page numbers como placeholder `"?"` ou omitidos. O sumário lista títulos + números de heading, sem page numbers.
- **Alternativa:** Se `Introspector::position_of` retorna page number, usar. Verificar na sonda A.0.

### 7. Tests

- **L1 (eval):** 2 testes — `native_outline()` emite `Content::Outline`; `native_outline(title: Some("Sumário"))` preserva título.
- **L2 (layout):** 2 testes — outline com 2 headings nível 1 renderiza 2 linhas; outline com heading nível 2 indenta.
- **L3 (E2E):** 2 testes — documento com 3 headings gera outline com 3 entradas; outline respeita `depth=2` (omite nível 3).

### 8. Spec L0

- `00_nucleo/prompts/entities/content.md` — `Content::Outline`.
- `00_nucleo/prompts/engine/stdlib/structural.md` — `outline(title?, depth?, indent?)`.
- `00_nucleo/prompts/engine/layout/outline.md` — layout de sumário.

---

## Scope-out explícito

- **Page numbers reais** — scope-out; placeholder `"?"` ou omitido. Requer pagination completa (Trilha 7).
- **Leader dots (`fill`)** — scope-out; apenas espaço simples.
- **Target selector personalizado** — scope-out; apenas headings.
- **Outline de figures/tables/equations** — scope-out; apenas headings.
- **Clickable links no outline** — scope-out; depende de `label`/`ref` (Trilha 2).
- **Title como `Content` rico** — scope-out; title é `TextItem` simples.
- **2-pass layout completo** — scope-out; usa 1-pass com colecção de headings durante eval (simplificação aceitável para subset minimal).

---

## Critério de fecho

- [ ] `Content::Outline` adicionado como variante de `Content`.
- [ ] `native_outline` registada no stdlib como `"outline"`.
- [ ] Eval coleciona headings do documento (1-pass ou via `Introspector`).
- [ ] Layout renderiza sumário com títulos + números de heading (até `depth`).
- [ ] Indentação para sub-headings (nível 2+).
- [ ] Page numbers como placeholder ou omitidos.
- [ ] 6 tests verdes (2 L1 + 2 L2 + 2 L3).
- [ ] Spec L0 actualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| 2-pass layout completo para page numbers reais | Requer infraestrutura de pagination completa (Trilha 7), fora de escopo. 1-pass com placeholder é suficiente para paridade vanilla básica. |
| `outline` como `Content::Sequence` de `Heading` references | Perde a semântica de sumário; dificulta formatação especial (indent, leader dots futuro). `Content::Outline` é mais limpo. |
| Colecção de headings no layout (não no eval) | Layout não tem acesso à estrutura do documento; eval tem. Coerente com P451/P454 (eval computa contadores). |
| Page numbers via `CounterRegistry` | Page counter é diferente de heading/figure/equation counters; requer pagination. Scope-out. |

---

**Aguardando sua indicação:**

1. **Executar o P457** (table of contents / outline, ~30 min)?
2. **Pivotar para outra frente** (Trilha 2: label/ref, Trilha 1: table numbering, Trilha 3: sonda Selector::Where, Trilha 8: refinos de stdlib)?
3. **Ajustar o escopo** do P457 (adicionar page numbers reais se `Introspector::position_of` existir, ou reduzir para apenas heading titles sem números)?
