# P422 — `link` Render Visual: hiperlinks clicáveis no output (S)

**Título**: Link render visual — `Content::Link` emitido como hyperlink anotado no frame  
**Tipo**: Materialização (S) — consumer layout + infraestrutura de annotation/link no frame  
**Bloqueadores**: Nenhum externo; `Content::Link` existe como entidade  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P418 (Bibliography CSL), P421 (repr completo)

---

## FASE A.0 — Sonda do substrato (obrigatória; 3 min)

Execute os 5 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: todos os 5 produzem output conforme esperado. Se qualquer um falhar → **parar imediatamente** e reclassificar.

```bash
# 1. Content::Link existe no enum?
grep -rn "Content::Link\|struct LinkElem" 01_core/src/entities/content.rs 01_core/src/entities/elements/ | head -10

# 2. LinkElem tem url e body?
grep -rn "url\|body" 01_core/src/entities/elements/link.rs 2>/dev/null | head -10

# 3. FrameItem tem variant para hyperlink/annotation?
grep -rn "enum FrameItem" 01_core/src/entities/layout_types.rs

# 4. Layouter layouta Content::Link atualmente?
grep -rn "Content::Link" 01_core/src/rules/layout/ | head -10

# 5. Existe infraestrutura de cor/azul para links?
grep -rn "rgb\|blue\|color\|fill" 01_core/src/rules/layout/ | grep -i link | head -10
```

**Output esperado**:
1. ≥1 hit com `Link` ou `LinkElem`
2. ≥1 hit com `url` e `body` (ou `child`)
3. ≥1 hit com `enum FrameItem` e lista de variants
4. ≥0 hits (pode ser que Link ainda não seja layoutado — este é o gap)
5. ≥0 hits (cor para links pode não existir ainda)

**Se (1) falhar** → `LinkElem` não existe; reclassificar para S (criar struct + variant Content).  
**Se (3) falhar** → `FrameItem` enum não existe; reclassificar para L (infraestrutura de layout ausente).  
**Se (4) mostrar que Link já é layoutado como texto com URL visível** → verificar se é stub (só texto) ou real (hyperlink anotado). Se for stub → S; se for real → reclassificar para "já implementado".

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/entities/elements/link.md` + `rules/layout/link.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `#link("https://example.com")[Clique aqui]` é uma **construção linguística** de hyperlink:
- **Semântica**: o texto do body é renderizado visualmente; o URL é associado como metadado de hyperlink; o leitor de PDF torna o texto clicável.
- **Sintaxe**: `#link("url")[body]` ou `#link("url")` (body implícito = URL).
- **Morfologia**: `LinkElem` tem `url: EcoString` e `body: Content`.

**O que NÃO é paridade (mecânica; diverge de propósito):**
- A cor exata do texto (vanilla usa azul padrão; crystalline pode usar outra cor ou nenhuma).
- O sublinhado (vanilla não sublinha por padrão; alguns renderizadores simulam).
- A estrutura interna da annotation no PDF (PDF annotation vs outro mecanismo).
- O comportamento de hover (renderizador-dependente).

### A.1.2 — Decisão arquitetural: atomização forma B (ADR-0109)

- `entities/elements/link.rs` — `LinkElem` struct puro (`url`, `body`).
- `rules/layout/link.rs` — free function `layout_link(layouter, &LinkElem)` (forma B).
- `entities/layout_types.rs` — `FrameItem::Link { url, body_items }` ou reutilizar `FrameItem::Group` com metadado.

**Não usar Opção A** (`impl LinkElem { fn layout(...) }` em `entities/`) — import reverso para `Layouter` (ADR-0109, rejeitado).

### A.1.3 — Estratégia de implementação

| Opção | Descrição | Magnitude | Risco | Nota |
|-------|-----------|-----------|-------|------|
| **α** — FrameItem::Link novo | Adicionar `FrameItem::Link { url: EcoString, items: Vec<FrameItem> }` ao enum | **S** | **Baixo** | Paridade estrutural clara; consumer downstream (PDF writer) sabe que é link |
| **β** — FrameItem::Group com metadado | Reutilizar `FrameItem::Group` e adicionar campo `link: Option<EcoString>` | S | Baixo | Menos variants no enum; metadado misturado com clip_mask |
| **γ** — Apenas texto azul sem annotation | Renderizar body como texto com cor azul; sem metadado de hyperlink | S | **Alto** | Não é paridade — o link não é clicável no PDF |

**Decisão recomendada**: **Opção α** — `FrameItem::Link` novo.  
**Razão ADR-0107**: a paridade linguagem exige que o URL seja **preservado como metadado** no output, não apenas como cor visual. Um texto azul sem annotation não é um hyperlink.

### A.1.4 — Estrutura de dados

```rust
// entities/elements/link.rs (existente ou novo)
pub struct LinkElem {
    pub url: EcoString,
    pub body: Box<Content>,
}

// entities/layout_types.rs — novo variant em FrameItem
pub enum FrameItem {
    // ... existentes
    Link {
        url: EcoString,
        items: Vec<FrameItem>, // body renderizado
    },
    // ... existentes
}
```

### A.1.5 — Algoritmo de layout

1. **Layout do body**:
   - `layout_content(layouter, &link.body)` → produz `Vec<FrameItem>` (texto, espaços, etc.).
   - O body pode conter texto formatado (bold, italic, etc.) — preservar.

2. **Envolver em FrameItem::Link**:
   - `FrameItem::Link { url: link.url.clone(), items: body_items }`.
   - O `Layouter` emite este item no frame atual.

3. **Cor visual (opcional)**:
   - Se o sistema de cores já suporta `rgb("#0000ff")` ou similar, aplicar cor azul ao texto do body antes do layout.
   - Se não: scope-out de cor; o link será clicável mas sem cor diferente (paridade comportamental reduzida, mas honesta).

4. **Consumer downstream** (PDF writer, não neste passo):
   - O PDF writer itera sobre `FrameItem`s; quando encontra `FrameItem::Link`, emite annotation de tipo URI com o URL.
   - **Scope-out**: se o PDF writer não existe ainda, o `FrameItem::Link` é infraestrutura preparatória; o consumer será implementado em passo futuro.

### A.1.6 — Paridade vanilla

- Vanilla: `#link("url")[text]` → texto renderizado com URL como annotation de hyperlink; cor azul padrão; clicável no PDF.
- Cristalino P422: paridade estrutural (body renderizado + URL preservado como metadado); cor azul scope-out se infraestrutura de cor não estiver madura; clicável no PDF scope-out se consumer PDF não existir.

### A.1.7 — Scope-out explícito

- Cor azul do texto — scope-out se sistema de cor não estiver maduro; o link funciona sem cor.
- Sublinhado — scope-out; vanilla não sublinha por padrão.
- Consumer PDF (annotation de URI) — scope-out se PDF writer não existir; `FrameItem::Link` é infraestrutura preparatória.
- Hover tooltip — scope-out; renderizador-dependente.
- Link interno (`#link("#label")`) — scope-out; requer infraestrutura de labels/anchors no PDF.
- Link para página (`#link("<page>")`) — scope-out.

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `link.md` + `layout/link.md`
2. Sonda A.0 produziu 5/5 OK
3. Decisão α (FrameItem::Link novo) validada
4. `FrameItem` enum aceita novo variant sem breaking change (verificar se é `non_exhaustive` ou se todos os `match` precisam ser atualizados)

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Entities

1. **`entities/layout_types.rs`** — adicionar variant:
   ```rust
   Link {
       url: EcoString,
       items: Vec<FrameItem>,
   },
   ```
   Atualizar todos os `match` sobre `FrameItem` (verificar em sonda A.0 item 3). Se o enum for `non_exhaustive`, apenas adicionar o variant. Se não, atualizar todos os arms com `todo!()` ou implementação honesta.

2. **`entities/elements/link.rs`** — verificar se `LinkElem` já existe; se não, criar:
   ```rust
   #[derive(Clone, Debug, PartialEq)]
   pub struct LinkElem {
       pub url: EcoString,
       pub body: Box<Content>,
   }
   ```

### B.2 — Layout (`rules/layout/link.rs` — forma B, ADR-0109)

```rust
pub(super) fn layout_link<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &LinkElem,
) {
    // 1. Layout do body (preserva formatação interna)
    let body_items = layouter.layout_sub_frame(&elem.body);

    // 2. Envolver em FrameItem::Link
    layouter.push_item(FrameItem::Link {
        url: elem.url.clone(),
        items: body_items,
    });
}
```

**Nota**: `layout_sub_frame` é uma operação hipotética que renderiza o body em um sub-frame e retorna items. Se não existir, usar o pattern do P418/P421: `layout_content` com acumulação temporária.

### B.3 — Consumer update (`rules/layout/mod.rs`)

Adicionar arm ao `match content` do `layout_content`:
```rust
Content::Link(link) => link::layout_link(self, link),
```

### B.4 — Tests

**Mínimo 8 tests**:
- 2 unit `FrameItem::Link`: construção, clone, debug
- 2 unit `layout_link`: body simples (texto), body formatado (bold + italic)
- 2 unit `LinkElem`: construção, clone, debug, partial_eq
- 2 E2E: `#link("https://example.com")[text]` → `FrameItem::Link` com URL correto e body items; `#link("https://example.com")` (body implícito = URL) → `FrameItem::Link` com URL como body

---

## FASE C — Validação

```bash
cargo test -p typst-core --lib -- link
# → 8 passed; 0 failed; 0 ignored

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
```

**Critério de fecho**:
- [ ] 8 tests verdes
- [ ] Lint zero errors; drift sincronizado
- [ ] `Content::Link` layoutado como `FrameItem::Link`
- [ ] URL preservado no metadado do `FrameItem::Link`
- [ ] Body renderizado corretamente dentro do `FrameItem::Link`
- [ ] Nenhum vtable/`dyn` introduzido (ADR-0109 / ADR-0026)
- [ ] `match` exaustivo preservado (todos os `match` sobre `FrameItem` atualizados)
- [ ] Lógica atomizada em free function (forma B)
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 verifica se `FrameItem` enum aceita novo variant sem breaking change. Se todos os `match` precisarem ser atualizados e forem muitos, reclassificar para M.
- **Paridade linguagem (ADR-0107)**: O contrato é body renderizado + URL como metadado. A cor é mecânica livre; o clicável é consumer-dependente. O `FrameItem::Link` é a infraestrutura honesta que permite o consumer downstream.
- **Atomização (ADR-0109)**: `LinkElem` é struct puro; `layout_link` é free function; `FrameItem::Link` é novo variant no enum de layout.
- **Honestidade epistêmica**: Se o PDF writer não existir, o `FrameItem::Link` é infraestrutura "preparatória" — não é "quase funciona", é "a base está lá para o consumer futuro". Documentar claramente no scope-out.
- **Próximo passo P423**: consumer PDF (annotation URI) se PDF writer existir; ou `text.lang` rustybuzz (XL, scope-out); ou outro gap do Inventário.
