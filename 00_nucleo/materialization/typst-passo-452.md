# P452 — Links e Hyperlinks

> **Passo:** 452  
> **Data:** 2026-06-24  
> **Foco:** Materializar a função nativa `link` para criar hyperlinks clicáveis no PDF exportado, e a infraestrutura de `label`/`ref` para referências cruzadas internas.  
> **ADR-0110:** Hyperlinks e referências cruzadas em documentos estruturados.

---

## Contexto

O Typst vanilla suporta `#link("https://example.com")[texto]` para hyperlinks externos e `#label<sec1>` + `#ref<sec1>` para referências cruzadas internas. O cristalino tem `FrameItem::Text` e `FrameItem::Rect`/`Line`/`Shape`, mas **não tem** nenhum mecanismo de link nem anotação de PDF. Este passo materializa o subset minimal: `link(url, body)` como hyperlink externo no PDF.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `link` existe em stdlib? | Não | ❌ |
| `label`/`ref` existe? | Não | ❌ |
| Infra de anotação PDF existe? | Não — zero código de URI/Link annotation | ❌ |
| `FrameItem` suporta metadados? | Parcial — `FrameItem` é enum simples, sem metadados genéricos | 🟡 |
| Export PDF de texto existe? | Sim — `TJ` / `Tj` operators | ✅ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** M (~35 min; novo `FrameItem::Link` + anotação PDF + função nativa + tests).

---

## Toques pontuais

### 1. Entidade `Link` (`entities/content.rs` ou novo `entities/link.rs`)

```rust
pub struct LinkElem {
    pub url: EcoString,      // "https://..." ou "#sec1" (interno)
    pub body: Content,
}
```

- `Content::Link(LinkElem)` como nova variante de `Content`.
- Ou reaproveitar `Content::Styled` com `Style::Link(EcoString)` — decisão a tomar.

**Decisão:** Nova variante `Content::Link` é preferível porque:
- `link` tem `body` (como `heading`), não é apenas um estilo sobre texto existente.
- `label`/`ref` são conceitualmente diferentes de estilo tipográfico.
- Alinha com `Content::Heading`, `Content::Bibliography` (elementos estruturais).

### 2. `FrameItem::Link` (`entities/layout_types.rs`)

```rust
pub enum FrameItem {
    Text(TextItem),
    Shape(ShapeItem),   // Rect, Line, etc.
    Link(LinkItem),     // NOVO
}

pub struct LinkItem {
    pub url: EcoString,
    pub pos: Point,
    pub size: Size,
    pub body: Frame,     // Frame interno com o conteúdo visual (texto, etc.)
}
```

- `LinkItem` envolve um `Frame` (como `GroupItem` envolve `Frame` em alguns engines).
- No layout, `link` cria um `FrameItem::Link` contendo o `Frame` do `body` renderizado.
- No export PDF, gera **Link Annotation** (`/Type /Annot /Subtype /Link /Rect [...] /A << /S /URI /URI (url) >>`).

### 3. Função nativa `link` (`rules/stdlib/structural.rs` ou `rules/stdlib/interactive.rs`)

```rust
fn native_link(url: EcoString, body: Content) -> Content {
    Content::Link(LinkElem { url, body })
}
```

- Registar no stdlib scope como `"link"`.
- `url` é `EcoString` (não validado como URL — passado ao PDF tal qual).

### 4. Layout de `Content::Link` (`rules/layout/mod.rs` ou `rules/layout/link.rs`)

- Renderizar `body` normalmente → obtém `Frame`.
- Envolver em `FrameItem::Link { url, pos: frame.pos, size: frame.size, body: frame }`.
- Posicionar no cursor como um bloco inline (se `body` é inline) ou block (se `body` é block).

### 5. Export PDF de Link Annotation (`03_infra/src/export.rs`)

```rust
// No loop de export de FrameItems:
FrameItem::Link(link) => {
    // Renderizar body primeiro (texto, shapes, etc.)
    export_frame(&link.body)?;
    // Adicionar Link Annotation ao page annotations array
    let rect = [link.pos.x, link.pos.y, link.pos.x + link.size.width, link.pos.y + link.size.height];
    write!(w, "<< /Type /Annot /Subtype /Link /Rect [{}] /A << /S /URI /URI ({}) >> /Border [0 0 0] >>
", rect, escape_pdf_string(&link.url))?;
}
```

- **PDF operator:** Link annotations são objetos separados no `/Annots` array da page, não operators de content stream.
- Alternativa: usar `/A << /S /GoTo /D (dest) >>` para links internos (scope-out por ora; apenas URI externo).

### 6. `label` / `ref` — scope-out para P453

- `label` requer tabela de símbolos no documento (destinos nomeados).
- `ref` requer resolução de destinos e geração de `/Dests` no PDF.
- Ambos são dependentes de infraestrutura de introspection/numbering que ainda não está madura o suficiente.

### 7. Tests

- **L1 (eval):** 2 testes — `native_link` emite `Content::Link`; `link` com body complexo preserva estrutura.
- **L2 (layout):** 2 testes — `Content::Link` renderiza body e envolve em `FrameItem::Link`; posição e tamanho correctos.
- **L3 (E2E PDF):** 1 teste — export PDF contém `/Subtype /Link` e `/URI (https://example.com)`.

### 8. Spec L0

- `00_nucleo/prompts/entities/content.md` — `Content::Link`.
- `00_nucleo/prompts/entities/layout_types.md` — `FrameItem::Link`, `LinkItem`.
- `00_nucleo/prompts/rules/stdlib/interactive.md` — `link(url, body)`.
- `00_nucleo/prompts/03_infra/export.md` — Link Annotation no PDF.

---

## Scope-out explícito

- **`label` / `ref`** — scope-out; P453. Requer destinos nomeados e tabela de símbolos.
- **Links internos (`/GoTo`)** — scope-out; apenas URI externo (`/URI`).
- **URL validation** — scope-out; string passada tal qual ao PDF.
- **Link styling (underline, color)** — scope-out; body do link é renderizado como está. O Typst vanilla aplica underline+blue por defeito, mas isso é show rule — scope-out.
- **Tooltip / hover text** — scope-out.
- **QuadPoints** — scope-out; `/Rect` simples suficiente.

---

## Critério de fecho

- [ ] `Content::Link` adicionado como variante de `Content`.
- [ ] `FrameItem::Link` adicionado com `LinkItem { url, pos, size, body }`.
- [ ] `native_link` registada no stdlib como `"link"`.
- [ ] Layout renderiza `body` e envolve em `FrameItem::Link`.
- [ ] Export PDF gera `/Subtype /Link` annotation com `/URI`.
- [ ] 5 tests verdes (2 L1 + 2 L2 + 1 L3).
- [ ] Spec L0 actualizada (4 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| `Style::Link(EcoString)` em vez de `Content::Link` | `link` tem `body` e comporta-se como elemento estrutural, não estilo tipográfico. Reaproveitar `Styled` confundiria com `strong`/`emph`. |
| Link como operator de content stream (PDF) | Link annotations são objetos de page, não operators de content stream. A abordagem de `/Annots` é a correcta e única. |
| Suportar `label`/`ref` neste passo | Duplica esforço e adiciona complexidade de destinos nomeados. Melhor separar: `link` externo é independente, `label`/`ref` interno é dependente. |

---

**Aguardando sua indicação:**

1. **Executar o P452** (links/hyperlinks, ~35 min)?
2. **Pivotar para outra frente** (figure numbering — reaproveita contador P451, table of contents, DEBT-42 benchmark, label/ref)?
3. **Ajustar o escopo** do P452?
