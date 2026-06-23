# P424 — Consumer PDF: `FrameItem::Link` → Annotation URI (S-M)

**Título**: PDF link consumer — conversão de `FrameItem::Link` em annotation URI no output PDF  
**Tipo**: Materialização (S-M) — consumer PDF writer + infraestrutura de annotation  
**Bloqueadores**: P422 (FrameItem::Link criado) e P425-A7 (FrameItem::Link propagado em typst-infra) como pré-requisitos  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P422 (link render visual), P425-A7 (bloqueador: pos/size necessário)

---

## FASE A.0 — Sonda do substrato (obrigatória; 5 min)

Execute os 6 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: 4/6 mínimo; itens 5 e 6 nice-to-have. Se qualquer um dos 4 obrigatórios falhar → **parar imediatamente** e reclassificar.

```bash
# 1. FrameItem::Link existe em typst-infra?
grep -rn "FrameItem::Link\|Link {" typst-infra/src/ | head -10

# 2. PDF writer existe no projeto?
grep -rn "pdf\|PDF\|writer" typst-infra/src/ | head -20

# 3. O PDF writer já emite annotations (link, destino)?
grep -rn "annotation\|Annot\|URI\|Link" typst-infra/src/ | head -20

# 4. FrameItem tem posição/size (bounding box) no layout?
grep -rn "struct FrameItem" 01_core/src/entities/layout_types.rs

# 5. [NICE-TO-HAVE] Existe infraestrutura de transform/position em FrameItem?
grep -rn "pos\|position\|size\|rect\|bbox" 01_core/src/entities/layout_types.rs | head -10

# 6. [NICE-TO-HAVE] O PDF writer consome outros FrameItems (text, shape, image)?
grep -rn "FrameItem::" typst-infra/src/ | head -20
```

**Output esperado**:
1. ≥1 hit com `FrameItem::Link` ou `Link {` em typst-infra
2. ≥1 hit com módulo de PDF writer
3. ≥0 hits (annotations podem não existir ainda — este é o gap)
4. ≥1 hit com `struct FrameItem` ou enum mostrando fields
5. ≥0 hits (posição/size pode ser externa ao FrameItem)
6. ≥1 hit com outros FrameItems sendo consumidos pelo PDF writer

**Se (1) falhar** → `FrameItem::Link` não está propagado em typst-infra; reclassificar para S (reabrir P425-A7).  
**Se (2) falhar** → PDF writer não existe; reclassificar para L (criar infraestrutura de PDF do zero).  
**Se (4) mostrar que FrameItem NÃO tem pos/size** → decisão arquitetural necessária; verificar se pos/size é computada no consumer.

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/typst-infra/pdf-writer.md` + `rules/layout/link.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `#link("https://example.com")[text]` produz um **PDF com annotation de URI**:
- **Semântica**: a área do texto do link é clicável no PDF reader; o click navega para o URL.
- **Sintaxe**: `#link("url")[body]` → PDF annotation.
- **Morfologia**: o `FrameItem::Link` contém URL + items do body; o PDF writer deve converter isso em uma annotation `Link` do PDF (tipo URI).

**O que NÃO é paridade (mecânica; diverge de propósito):**
- O formato exato da annotation no PDF (PDF 1.4 vs 1.7, dictionary keys, etc.).
- A precisão da bounding box (pode ser aproximada se o layout não expuser pos/size exato).
- A ordem de escrita no PDF stream (annotations podem ser escritas no final ou inline).

### A.1.2 — Decisão arquitetural: pos/size no FrameItem::Link (bloqueador P425-A7)

| Opção | Descrição | Magnitude | Risco | Nota |
|-------|-----------|-----------|-------|------|
| **α** — Adicionar pos/size a FrameItem::Link | `FrameItem::Link` ganha `pos: Point` e `size: Size` calculados durante layout | S | Baixo | Paridade estrutural clara; PDF writer consome diretamente |
| **β** — Computar bounding box no PDF writer | O PDF writer percorre os `items` do `FrameItem::Link` e calcula bbox acumulada | S | Médio | Não altera FrameItem; mas requer recursão no writer; bbox pode ser imprecisa |
| **γ** — Usar pos/size do primeiro item | Assumir que o primeiro item do Link tem a pos/size representativa | S | Alto | Incorreto se body tem múltiplos items ou espaços |

**Decisão recomendada**: **Opção α** — adicionar `pos: Point` e `size: Size` ao `FrameItem::Link`.  
**Razão ADR-0108**: medir o custo. A opção α é uma adição de 2 fields ao struct existente. O layout já calcula posição e tamanho dos items; é trivial propagar para o `FrameItem::Link`. A opção β é mais complexa e imprecisa. A opção γ é incorreta.

**Nota**: se `Point` e `Size` não existirem como tipos, usar `f64` pares `(x, y)` e `(width, height)` ou tipos existentes do projeto.

### A.1.3 — Decisão arquitetural: atomização forma B (ADR-0109)

- `entities/layout_types.rs` — `FrameItem::Link` com `pos` + `size` (dados).
- `rules/layout/link.rs` — `layout_link` calcula e preenche `pos`/`size` (forma B).
- `typst-infra/src/pdf/` — free function `write_link_annotation(writer, &FrameItem::Link)` (forma B).

**Não usar Opção A** (método `impl FrameItem::Link { fn write_pdf(...) }` em `entities/`) — import reverso para PDF writer (ADR-0109, rejeitado).

### A.1.4 — Estrutura de dados

```rust
// entities/layout_types.rs — FrameItem::Link atualizado
pub enum FrameItem {
    // ... existentes
    Link {
        url: EcoString,
        items: Vec<FrameItem>,
        pos: Point,    // NOVO: posição do canto superior-esquerdo
        size: Size,    // NOVO: largura × altura
    },
    // ... existentes
}

// Tipos auxiliares (se não existirem)
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Size {
    pub width: f64,
    pub height: f64,
}
```

### A.1.5 — Algoritmo de layout + PDF

1. **Layout (atualização de P422)**:
   - `layout_link` renderiza o body → `Vec<FrameItem>`.
   - Calcula `pos` como a posição do primeiro item (ou mínimo x, y de todos os items).
   - Calcula `size` como a bbox acumulada de todos os items (max x+width, max y+height).
   - Emite `FrameItem::Link { url, items, pos, size }`.

2. **PDF writer (novo)**:
   - O PDF writer itera sobre `FrameItem`s da página.
   - Quando encontra `FrameItem::Link`:
     a. Escreve os `items` normalmente (texto, shapes, etc.) para o content stream.
     b. Cria uma annotation dictionary no PDF:
        ```
        /Type /Annot
        /Subtype /Link
        /Rect [x y (x+width) (y+height)]
        /Border [0 0 0]
        /A << /Type /Action /S /URI /URI (url) >>
        ```
     c. Adiciona a annotation ao array de annotations da página.

3. **Consumer downstream**:
   - Se o PDF writer usa uma crate externa (ex.: `pdf-writer`, `printpdf`), usar a API da crate para adicionar URI annotation.
   - Se o PDF writer é próprio, construir o dictionary manualmente.

### A.1.6 — Paridade vanilla

- Vanilla: `#link("url")[text]` → PDF com annotation URI na área do texto; clicável no reader.
- Cristalino P424: paridade comportamental (annotation URI na área do texto). A mecânica (formato PDF, precisão da bbox, ordem de escrita) é livre.

### A.1.7 — Scope-out explícito

- Link interno (`#link("#label")`) → annotation tipo `GoTo` em vez de `URI` — scope-out; requer infraestrutura de destinos/named destinations.
- Link para página (`#link("<page>")`) — scope-out.
- Border/estilo visual da annotation (vanilla não desenha borda) — scope-out; `/Border [0 0 0]` é aceitável.
- Hover tooltip no PDF — scope-out; renderizador-dependente.
- QuadPoints (vertices da área clicável para textos multi-line) — scope-out; usar `Rect` simples.

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `pdf-writer.md` + `layout/link.md`
2. Sonda A.0 produziu 4/6 OK (mínimo)
3. Decisão α (pos/size no FrameItem::Link) validada — tipos Point/Size existem ou são criáveis
4. PDF writer existe e consome FrameItems

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Entities (`entities/layout_types.rs`)

Atualizar `FrameItem::Link`:
```rust
Link {
    url: EcoString,
    items: Vec<FrameItem>,
    pos: Point,  // ou (f64, f64)
    size: Size,  // ou (f64, f64)
},
```

Atualizar todos os `match` sobre `FrameItem` (13 locais, P425 já atualizou; verificar se há mais).

### B.2 — Layout (`rules/layout/link.rs` — forma B, ADR-0109)

```rust
pub(super) fn layout_link<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &LinkElem,
) {
    // 1. Layout do body
    let body_items = layouter.layout_sub_frame(&elem.body);

    // 2. Calcular pos/size da bbox acumulada
    let (pos, size) = calculate_bbox(&body_items);

    // 3. Emitir FrameItem::Link com pos/size
    layouter.push_item(FrameItem::Link {
        url: elem.url.clone(),
        items: body_items,
        pos,
        size,
    });
}

fn calculate_bbox(items: &[FrameItem]) -> (Point, Size) {
    // Se items vazio, retornar zero
    if items.is_empty() {
        return (Point { x: 0.0, y: 0.0 }, Size { width: 0.0, height: 0.0 });
    }

    // Calcular min x, min y, max x+width, max y+height
    // Nota: isso depende de como FrameItem/TextItem expõem pos/size
    // Se não expuserem, usar heurística do layouter (cursor position)
    let min_x = items.iter().map(|item| item.pos().x).fold(f64::INFINITY, f64::min);
    let min_y = items.iter().map(|item| item.pos().y).fold(f64::INFINITY, f64::min);
    let max_x = items.iter().map(|item| item.pos().x + item.size().width).fold(0.0, f64::max);
    let max_y = items.iter().map(|item| item.pos().y + item.size().height).fold(0.0, f64::max);

    (
        Point { x: min_x, y: min_y },
        Size { width: max_x - min_x, height: max_y - min_y },
    )
}
```

**Nota**: se `FrameItem` não expuser `pos()`/`size()` como métodos, adaptar para a API existente (ex.: `TextItem` tem `pos` e `size` fields).

### B.3 — PDF Writer (`typst-infra/src/pdf/` ou similar)

```rust
// Em pdf_writer.rs ou similar
fn write_frame_item(writer: &mut PdfWriter, item: &FrameItem) {
    match item {
        FrameItem::Text(t) => write_text(writer, t),
        FrameItem::Shape(s) => write_shape(writer, s),
        // ... outros
        FrameItem::Link { url, items, pos, size } => {
            // 1. Escrever items normalmente (texto visível)
            for child in items {
                write_frame_item(writer, child);
            }

            // 2. Criar annotation URI
            let annotation = Annotation::new()
                .subtype(AnnotationSubtype::Link)
                .rect(Rect::new(pos.x, pos.y, pos.x + size.width, pos.y + size.height))
                .border(Border::new(0.0, 0.0, 0.0))
                .action(Action::uri(url.as_str()));

            writer.add_annotation(annotation);
        }
        // ... outros
    }
}
```

**Nota**: adaptar para a API específica do PDF writer do projeto (crate externa ou próprio).

### B.4 — Tests

**Mínimo 8 tests**:
- 2 unit `calculate_bbox`: items vazio, items com texto simples
- 2 unit `FrameItem::Link` com pos/size: construção, clone, debug
- 2 unit PDF annotation: estrutura do dictionary, URI correto
- 2 E2E: documento com `#link` → PDF com annotation; link sem body → PDF com annotation usando URL como texto

---

## FASE C — Validação

```bash
cargo check -p typst-core -p typst-infra
# → ok

cargo test -p typst-core --lib -- link
# → 8+ passed; 0 failed

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
```

**Critério de fecho**:
- [ ] 8 tests verdes
- [ ] Lint zero errors; drift sincronizado
- [ ] `FrameItem::Link` tem `pos` + `size`
- [ ] `layout_link` calcula bbox corretamente
- [ ] PDF writer emite annotation URI para `FrameItem::Link`
- [ ] `#link("url")[text]` → PDF clicável
- [ ] Nenhum vtable/`dyn` introduzido
- [ ] `match` exaustivo preservado
- [ ] Lógica atomizada em free functions (forma B)
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 verifica se PDF writer existe e como consome FrameItems. Se não existir, reclassificar para L.
- **Paridade linguagem (ADR-0107)**: O contrato é texto clicável → navegação para URL. A mecânica (bbox, formato PDF, dictionary) é livre.
- **Atomização (ADR-0109)**: `FrameItem::Link` é dado; `layout_link` é free function; `write_link_annotation` é free function no PDF writer.
- **Honestidade epistêmica**: Se o PDF writer não existir, este passo não pode ser feito como S-M. Reclassificar para L (criar PDF writer) ou scope-out.
- **Próximo passo P426**: `text.lang` rustybuzz (XL, scope-out) ou continuar varredura mecânica em outras camadas (typst-shell, typst-wiring).
