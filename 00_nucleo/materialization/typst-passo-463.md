# P463 — PDF `/GoTo` links internos: Conectar `ref` a `/Dests`

> **Passo:** 463  
> **Data:** 2026-06-25  
> **Foco:** Conectar as referências `ref` (P462) aos destinos nomeados `/Dests` (P460) via annotations PDF `/GoTo`, tornando as referências clicáveis no documento exportado.  
> **Trilha:** 2 — Referências cruzadas e navegação interna (fecho da Trilha 2; depende de P460 `label` + P462 `ref`).  
> **ADR-0110:** Hyperlinks e referências cruzadas em documentos estruturados (continuação de P460/P462).

---

## Contexto

O Typst vanilla suporta referências clicáveis: quando o leitor clica em `@sec1` ou `#ref<sec1>`, o PDF navega para o destino nomeado criado por `#label<sec1>`. O cristalino tem:

- `Content::Label` (P460) — cria destinos nomeados no PDF via `/Dests`.
- `Content::Ref` (P462) — resolve para texto com número do elemento referenciado.
- Export PDF de `/Dests` (P460) — emite `/Names /Dests << /sec1 [page_ref /XYZ x y null] >>`.

Mas **não tem** a conexão entre `ref` e `/Dests` — o texto da referência não é um link clicável no PDF. Este passo materializa o `/GoTo` annotation que conecta o `ref` ao seu destino.

**Nota:** Com P463, a Trilha 2 (referências cruzadas e navegação interna) fica **COMPLETA**: `label` (P460) → `ref` (P462) → `/GoTo` links (P463). Isso desbloqueia consumidores como List of Figures/Tables (LoF/LoT) e bibliografia back-references (Trilha 6).

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Label` + `/Dests` existe (P460)? | Sim — `PagedDocument.extracted_label_positions` | ✅ |
| `Content::Ref` + resolução de número existe (P462)? | Sim — resolve via `Introspector.counter_key_for_label` | ✅ |
| Export PDF de `/Annot` com `/Subtype /Link` existe (P452)? | Sim — hyperlinks externos via `/URI` | ✅ |
| `/GoTo` annotation (link interno) no PDF? | Não — zero código de `/GoTo` | ❌ |
| `FrameItem::Link` existe (P452)? | Sim — `LinkItem { url, items, pos, size }` | ✅ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S (~20 min; adaptar `FrameItem::Link` para suportar `/GoTo` + layout de `ref` como link + export PDF + tests).

---

## Toques pontuais

### 1. Adaptar `FrameItem::Link` para suportar destinos internos

**Ficheiro:** `entities/layout_types.rs` (ou onde `LinkItem` está definido)

O `LinkItem` actual (P452) suporta apenas URLs externas:

```rust
pub struct LinkItem {
    pub url: EcoString,        // "https://..." — externo
    pub items: Vec<FrameItem>, // Conteúdo visual
    pub pos: Point,
    pub size: Size,
}
```

**Adaptação:** Adicionar enum de destino:

```rust
pub enum LinkTarget {
    Url(EcoString),           // Externo: /URI
    Destination(Label),       // Interno: /GoTo (Label = nome do destino)
}

pub struct LinkItem {
    pub target: LinkTarget,    // NOVO: Url ou Destination
    pub items: Vec<FrameItem>,
    pub pos: Point,
    pub size: Size,
}
```

**Decisão:** Alterar `LinkItem` para usar `LinkTarget` em vez de `url: EcoString`. Isso é uma mudança de tipo que afeta:
- P452 (hyperlinks externos): `LinkTarget::Url(url)` em vez de `url`.
- P463 (links internos): `LinkTarget::Destination(Label(name))`.

**Impacto:** Refacto mecânico em P452 para usar `LinkTarget::Url`. Adicionar braço `Destination` para P463.

### 2. Layout de `Content::Ref` como link clicável

**Ficheiro:** `rules/layout/ref.rs` (ou `rules/layout/references.rs`)

O layout de `ref` (P462) renderiza o texto resolvido como `FrameItem::Text`. Para torná-lo clicável, envolver em `FrameItem::Link`:

```rust
// No layout de Content::Ref:
let resolved_text = /* texto resolvido: "Fig. 1", "Table 2", etc. */;
let frame = Frame::new(pos, size);
frame.items.push(FrameItem::Text(TextItem { text: resolved_text, ... }));

// Envolver em Link com destino interno:
let link_frame = Frame::new(pos, frame.size);
link_frame.items.push(FrameItem::Link(LinkItem {
    target: LinkTarget::Destination(Label(ref_elem.name.clone())),
    items: frame.items,  // Conteúdo visual do ref
    pos,
    size: frame.size,
}));
```

**Decisão:** O `ref` é sempre um link clicável? No Typst vanilla, sim — `@sec1` é clicável e navega para o label. No cristalino, simplificamos: todo `ref` é um `FrameItem::Link` com `LinkTarget::Destination`.

**Scope-out:** Opção de desactivar link em `ref` (ex: `ref("sec1", link: false)`) — não suportado neste passo.

### 3. Export PDF de `/GoTo` annotation

**Ficheiro:** `03_infra/src/export/builder.rs` (ou onde `/Annots` são emitidos)

O export PDF actual (P452) emite `/Subtype /Link` com `/A << /S /URI /URI (url) >>`. Para `/GoTo`:

```
/Annots [
  <<
    /Type /Annot
    /Subtype /Link
    /Rect [x y x+w y+h]
    /A << /S /GoTo /D /sec1 >>
    /Border [0 0 0]
  >>
]
```

- `/S /GoTo` — acção de navegação interna.
- `/D /sec1` — destino nomeado (referência a `/Names /Dests << /sec1 [...] >>`).
- O destino `/sec1` já está em `/Dests` (P460).

**Algoritmo no export:**
1. Iterar `FrameItem::Link` no frame.
2. Se `target == LinkTarget::Destination(Label(name))`:
   - Emitir annotation `/Subtype /Link` com `/A << /S /GoTo /D /name >>`.
3. Se `target == LinkTarget::Url(url)`:
   - Emitir annotation `/Subtype /Link` com `/A << /S /URI /URI (url) >>` (P452 existente).

**Decisão:** O `/GoTo` usa o nome do label como `/D /name`. O PDF reader resolve `/name` para o destino em `/Names /Dests`. Isso requer que o nome do label seja um identificador PDF válido (sem espaços, caracteres especiais). O Typst vanilla usa o nome tal qual; se o nome contiver caracteres inválidos, o vanilla escapa. Scope-out: assumir nomes de label válidos (alphanumeric + underscore + hyphen).

### 4. Coordenadas do annotation `/Rect`

O `/Rect` do annotation deve ser as coordenadas do `FrameItem::Link` no sistema de coordenadas do PDF (Y-up):

```
/Rect [pos.x page_height - (pos.y + size.height) pos.x + size.width page_height - pos.y]
```

- `pos.x`, `pos.y`: coordenadas do link no frame (Y-down interno).
- `page_height`: altura da página em points.
- Conversão: Y-up = `page_height - Y-down`.

**Decisão:** Reusar a conversão Y-down → Y-up de P460 (export de `/Dests`).

### 5. Tests

- **L2 (layout):** 2 testes — `Content::Ref` renderiza como `FrameItem::Link` com `LinkTarget::Destination`; `Content::Link` (P452) continua como `LinkTarget::Url`.
- **L3 (E2E PDF):** 3 testes — export PDF de `ref("sec1")` contém `/Subtype /Link` + `/A << /S /GoTo /D /sec1 >>`; coordenadas `/Rect` correctas; `ref` para label inexistente não emite `/GoTo` (ou emite sem destino válido).
- **L3 (E2E PDF):** 1 teste — `link("https://...")` (P452) continua emitindo `/URI` (regressão).

### 6. Spec L0

- `00_nucleo/prompts/entities/layout_types.md` — `LinkItem` com `LinkTarget` (atualizar).
- `00_nucleo/prompts/rules/layout/ref.md` — `ref` como `FrameItem::Link` com `Destination`.
- `00_nucleo/prompts/infra/export/builder.md` — `/GoTo` annotations no PDF.

---

## Scope-out explícito

- **Sintaxe sugar `@x`** — já funciona via parser (P462); não há alteração.
- **i18n de supplements** — scope-out; mantém "Fig. ", "Table ", etc.
- **Warning em label não encontrada** — scope-out; `ref` renderiza `"?"`.
- **Destinos de região (rectângulo)** — scope-out; `/GoTo` usa ponto `/XYZ` do `/Dests`.
- **Zoom em `/GoTo`** — scope-out; usa `null` (default).
- **Link styling (underline, color)** — scope-out; `ref` renderiza como texto normal (o Typst vanilla aplica estilo de link, mas isso é show rule).
- **Múltiplos destinos por label** — scope-out; um label = um destino.
- **Escaping de nomes de label inválidos** — scope-out; assumir nomes válidos.

---

## Critério de fecho

- [ ] `LinkItem` adaptado para `LinkTarget` (Url + Destination).
- [ ] P452 (hyperlinks externos) actualizado para `LinkTarget::Url` (refacto mecânico, sem regressão).
- [ ] Layout de `Content::Ref` envolve texto resolvido em `FrameItem::Link` com `LinkTarget::Destination`.
- [ ] Export PDF emite `/GoTo` annotations para `LinkTarget::Destination`.
- [ ] Export PDF continua emitindo `/URI` annotations para `LinkTarget::Url` (P452, sem regressão).
- [ ] Coordenadas `/Rect` correctas (Y-up conversion).
- [ ] 6 tests verdes (2 L2 + 4 L3).
- [ ] Spec L0 actualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 2 marcada como COMPLETA** no roteiro de conclusão.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| `FrameItem::GoTo` separado de `FrameItem::Link` | Duplica código de annotation PDF; `Link` e `GoTo` são ambos `/Subtype /Link`, diferem apenas em `/A << /S /URI >>` vs `/A << /S /GoTo >>`. Um enum `LinkTarget` é mais limpo. |
| `ref` como texto simples (não link) com opção de activar | No Typst vanilla, `ref` é sempre clicável. Não oferecer link seria divergência de paridade. |
| `/GoTo` com `/D [page_num /XYZ x y null]` em vez de nome | O Typst vanilla usa `/D /name` (referência a `/Names /Dests`). Usar coordenadas directas em `/GoTo` é menos comum e requer duplicar coordenadas do destino. Seguir o vanilla. |
| Link target como `EcoString` (nome do label) em vez de `Label` | `Label` é o tipo existente (P460) que representa um nome de destino. Usar `EcoString` seria perder tipagem. |

---

## Dívida técnica a registar (não fechar neste passo)

| Item | Impacto | Passo futuro |
|------|---------|-------------|
| `Content::Label` vs `Content::Labelled` | Dois tipos para mesmo conceito; complica walk de introspecção, layout, export | P464 (cleanup XS) |
| `label_to_counter_key` duplicado em `Introspector` e `TagIntrospector` | Manutenção dupla; um é trait, outro é impl | P464 ou refacto de introspector |
| `LinkItem` com `items: Vec<FrameItem>` vs `body: Frame` | P452 usou `items` (lista de `FrameItem`); P460 spec propôs `body: Frame`. Divergência documentada em nota retroativa P452. | Não bloqueante; manter `items`. |

---

## Próximo passo (Trilha 2 COMPLETA)

Com P463, Trilha 2 está completa. O próximo passo pode ser:
- **P464** — Consolidação `Content::Label` vs `Content::Labelled` (cleanup XS, ~10 min)
- **Trilha 3** — `Selector::Where` (sonda de estado real vs inventário)
- **Trilha 4** — Visuais: `Value::Gradient` tipo real, render PDF de gradiente
- **Trilha 6** — Bibliografia Fase 2 (estilos numéricos, back-references, `ibid`)
- **Trilha 8** — Refinos de stdlib (`repr()` completo, métodos restantes, etc.)

**Aguardando sua indicação:**

1. **Executar o P463** (PDF `/GoTo` links internos, ~20 min)?
2. **Escrever o P464** (cleanup Label/Labelled, ~10 min)?
3. **Pivotar para outra trilha** (Trilha 3, 4, 6, 8)?
4. **Ajustar o escopo** do P463 (adicionar link styling, escaping de nomes, etc.)?
