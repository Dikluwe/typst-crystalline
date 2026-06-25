# P460 — `label<x>`: Destinos nomeados para referências cruzadas

> **Passo:** 460  
> **Data:** 2026-06-25  
> **Foco:** Materializar a função nativa `label` para criar destinos nomeados no documento, permitindo referências cruzadas internas.  
> **Trilha:** 2 — Referências cruzadas e navegação interna (depende de Trilha 1 completa: heading P451, figure P454, equation P456, TOC P457, table P459).  
> **ADR-0110:** Hyperlinks e referências cruzadas em documentos estruturados (continuação de P452).

---

## Contexto

O Typst vanilla suporta `#label<sec1>` para marcar um ponto no documento como destino nomeado, e `#ref<sec1>` ou `@sec1` para referenciar esse destino (exibindo o número do heading/figure/equation/table associado). O cristalino tem `Content::Link` (P452) para hyperlinks externos, mas **não tem** mecanismo de destinos nomeados internos nem referências cruzadas. Este passo materializa o primeiro lado do par: `label` como destino nomeado.

**Nota:** `label` e `ref` são dependentes um do outro, mas `label` pode ser materializado primeiro porque:
1. É um marcador passivo (não requer resolução).
2. O layout precisa de `label` para posicionar o destino no PDF (`/Dests`).
3. `ref` (P461) depende de `label` existir para resolver.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Link` existe (hyperlinks externos)? | Sim — P452 | ✅ |
| Export PDF de `/Annot` com `/Subtype /Link` existe? | Sim — P452 | ✅ |
| `label`/`ref` existe em stdlib? | Não | ❌ |
| Introspecção de elementos numerados (heading, figure, equation, table) existe? | Sim — CounterRegistry + format_counter (P451/P454/P456/P459) | ✅ |
| `Introspector` com `position_of` / `query_by_kind` existe? | Parcial — verificar na implementação | 🟡 |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S (~20 min; entidade `LabelElem` + registo de destino + export PDF `/Dests` + tests).

---

## Toques pontuais

### 1. Entidade `Label` (`entities/elements/label.rs` ou `entities/content.rs`)

```rust
pub struct LabelElem {
    pub name: EcoString,      // "sec1", "fig1", "eq1", etc.
    pub body: Content,        // Conteúdo ao qual o label está associado
}

pub enum Content {
    // ... existing variants
    Label(LabelElem),  // NOVO
}
```

- `label` é um **marcador passivo** — não altera o rendering do `body`.
- O `body` é o conteúdo "ao lado" do qual o label está posicionado (heading, figure, equation, table, ou texto arbitrário).
- No Typst vanilla, `label` é um elemento vazio (`#label<sec1>`) ou associado a um elemento (`= Heading #label<sec1>`). No cristalino, modelamos como `Content::Label { name, body }` onde `body` é o conteúdo que o label "envolve".

**Decisão arquitectural:** O Typst vanilla permite `= Heading #label<sec1>` (label após heading) e `#ref<sec1>` resolve para o número do heading. Isso implica que o label está associado ao heading **anterior** ou **parent**. No cristalino, simplificamos: `label` é um elemento que envolve o conteúdo ao qual se refere (`Content::Label { name, body }`). O layout renderiza `body` normalmente e registra o label como destino.

### 2. `native_label` (`rules/stdlib/structural.rs` ou `rules/stdlib/interactive.rs`)

```rust
fn native_label(name: EcoString, body: Content) -> Content {
    Content::Label(LabelElem { name, body })
}
```

- Registar no stdlib scope como `"label"`.
- Sintaxe Typst: `#label<sec1>` é sugar para `label("sec1")` com body implícito (próximo elemento). No cristalino, usamos `label("sec1", body)` explicitamente.

**Scope-out de sintaxe:** `#label<sec1>` como sugar não é suportado neste passo; usa-se `label("sec1", body)` como função nativa. A sintaxe sugar é lexer/parser — scope-out para passo futuro.

### 3. Layout de `label` (`rules/layout/mod.rs` ou `rules/layout/label.rs`)

- Ao encontrar `Content::Label`:
  1. Renderizar `body` normalmente → obtém `Frame`.
  2. Registar o label como destino:
     - Nome: `label.name`.
     - Posição: `frame.pos` (ponto de inserção do label no documento).
     - Página: página actual (ou 0 se pagination não existir).
  3. Envolver em `FrameItem::Group` (ou retornar o `Frame` do body diretamente, com metadado de label).

**Decisão:** O label não altera o layout visual. O body é renderizado como se o label não existisse. O label é um metadado posicional.

### 4. Export PDF de `/Dests` (`03_infra/src/export.rs`)

```rust
// No final do export de cada página (ou no final do documento):
// Gerar array de destinos nomeados para o PDF catalog

// Estrutura PDF /Dests:
// /Dests <<
//   /sec1 [page_obj_ref /XYZ x y zoom]
//   /fig1 [page_obj_ref /XYZ x y zoom]
// >>
```

- Cada label registado no layout gera uma entrada em `/Dests`.
- `page_obj_ref`: referência ao objeto da página onde o label está.
- `x`, `y`: coordenadas do label na página (em points, sistema de coordenadas PDF).
- `zoom`: `null` (default zoom).

**Decisão arquitectural:** O export PDF precisa de acesso à lista de labels registados. Isso pode ser feito via:
- **Opção A:** O `Layouter` mantém uma lista de `LabelDest { name, page, pos }` que é passada ao export.
- **Opção B:** O `FrameItem::Group` do label contém metadado que o export extrai.
- **Opção C:** O `Document` (ou `PagedDocument`) mantém a tabela de destinos.

**Decisão:** Opção C — o `Document` (ou estrutura de resultado do layout) mantém `labels: Vec<LabelDest>`. O export lê desta lista. Isto é coerente com a arquitectura de separação L1 (eval) / L3 (layout/export).

### 5. Tabela de destinos no `Document` (`entities/document.rs` ou `03_infra/src/export_types.rs`)

```rust
pub struct LabelDest {
    pub name: EcoString,
    pub page: usize,
    pub pos: Point,
}

pub struct PagedDocument {
    pub pages: Vec<Frame>,
    pub labels: Vec<LabelDest>,  // NOVO
}
```

- O `Layouter` popula `labels` durante o layout.
- O export PDF lê `labels` e gera `/Dests`.

### 6. Tests

- **L1 (eval):** 2 testes — `native_label("sec1", body)` emite `Content::Label`; `label` com body vazio é válido.
- **L2 (layout):** 2 testes — `Content::Label` renderiza body sem alteração visual; label registado em `PagedDocument.labels`.
- **L3 (E2E PDF):** 2 testes — export PDF contém `/Dests << /sec1 [...] >>`; label com posição correcta (x, y) na página.

### 7. Spec L0

- `00_nucleo/prompts/entities/elements/label.md` — `LabelElem { name, body }`.
- `00_nucleo/prompts/entities/document.md` — `PagedDocument` com `labels`.
- `00_nucleo/prompts/rules/stdlib/interactive.md` — `native_label(name, body)`.
- `00_nucleo/prompts/03_infra/export.md` — `/Dests` no PDF catalog.

---

## Scope-out explícito

- **`ref` / `@x`** — scope-out; P461. Requer resolução de destinos e texto da referência.
- **Links internos `/GoTo`** — scope-out; P462. Requer `/Dests` + annotation `/GoTo`.
- **Sintaxe sugar `#label<sec1>`** — scope-out; requer alteração no lexer/parser.
- **Label automático em headings/figures/equations/tables** — scope-out; requer `set heading(numbering: ...)` gerar label implícito. Pode ser adicionado como sugar futuro.
- **Label em múltiplas páginas** — scope-out; assume label numa única página.
- **Zoom em `/Dests`** — scope-out; `null` (default).
- **Destinos de região (rectângulo)** — scope-out; apenas ponto (`/XYZ`).

---

## Critério de fecho

- [ ] `LabelElem` criado com `name: EcoString` e `body: Content`.
- [ ] `Content::Label` adicionado como variante.
- [ ] `native_label` registada no stdlib como `"label"`.
- [ ] Layout renderiza `body` sem alteração visual e regista label em `PagedDocument.labels`.
- [ ] Export PDF gera `/Dests` com entradas nomeadas (`/sec1`, `/fig1`, etc.).
- [ ] 6 tests verdes (2 L1 + 2 L2 + 2 L3).
- [ ] Spec L0 actualizada (4 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| `label` como `Style` (ex: `Style::Label(EcoString)`) | Label é um marcador estrutural, não estilo tipográfico. Usar `Style` confundiria com `strong`/`emph` e complicaria o registo de destinos. |
| Label sem `body` (elemento vazio, como no vanilla `#label<sec1>`) | No cristalino, o eval precisa de associar o label a um conteúdo para posicionamento. Sem body, o label seria um `FrameItem` vazio com posição — possível, mas requer alteração no layout para "lookahead" do próximo elemento. `body` explícito é mais simples e alinha com `Content::Link` (P452). |
| Registar labels no eval (L1) em vez de no layout (L3) | O eval não tem acesso às posições físicas (page, x, y). Labels precisam de posição no layout para `/Dests`. Layout é a camada correcta. |
| `/Dests` como array em vez de dictionary | O Typst vanilla usa dictionary (`/Dests << /name [...] >>`). Array (`/Dests [name [...] name2 [...]]`) é menos comum. Seguir o vanilla. |

---

## Próximo passo (Trilha 2 continua)

Com P460, `label` está materializado. O próximo passo é:
- **P461** — `ref<x>` / `@x`: Resolução de destino + texto da referência (número do heading/figure/equation/table associado).
- **P462** — PDF links internos `/GoTo`: Conectar `ref` a `/Dests` via annotations `/GoTo`.

**Aguardando sua indicação:**

1. **Executar o P460** (label, ~20 min)?
2. **Escrever o P461** (ref — depende de P460)?
3. **Pivotar para outra trilha** (Trilha 3: Selector::Where, Trilha 4: visuais, Trilha 6: bibliografia Fase 2, Trilha 8: refinos)?
4. **Ajustar o escopo** do P460 (adicionar label automático em headings/figures)?
