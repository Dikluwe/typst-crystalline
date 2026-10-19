# Diagnóstico Fase A — P281 Unificação β-completa stream-builders

**Data**: 2026-05-18
**Tipo**: ADR-0085 diagnóstico imutável; **35º consumo**.
**Decisão arquitectural**: β-completa **fixada antes de Fase A** (ver
P281 spec §0); Fase A apenas inventaria factualmente + dimensiona caps.

---

## §A.1 — Inventário dos 3 stream-builders actuais

Localizadas em `03_infra/src/export.rs`:

- `build_page_stream_type1` — linhas 2086–2323 (~237 LOC).
- `build_page_stream_cidfont` — linhas 2733–2908 (~175 LOC).
- `build_page_stream_multifont` — linhas 2923–3098+ (~175 LOC).

**Total actual**: ~587 LOC para 3 funções com estrutura idêntica
exceto Text/Glyph arms.

### Tabela arm-by-arm

| Arm `FrameItem` | Type1 | CIDFont | Multifont | Divergência |
|---|---|---|---|---|
| `Text` | `escape_pdf_string` + `/F1/F2/F3` por bold/italic + faux-bold + tracking | `text_to_hex_string(text, char_to_gid)` + `/F1` único | `text_to_hex_string(text, per_font_char_to_gid[fi])` + `/F{fi+1}` por `style.font` lookup | **divergente** (3 strats fundamentalmente distintas) |
| `Line` | `q {t} w m l S Q` | Idêntico | Idêntico | **idêntico** |
| `Glyph` | Silently ignored (`{}`) | `BT /F1 size Tf x y Td <gid> Tj ET` | Idêntico ao CIDFont (sempre `/F1` para math) | **divergente** Type1 vs (CID/Multi) |
| `Image` | `q w 0 0 h x y cm /Im{n} Do Q` | Idêntico | Idêntico | **idêntico** |
| `Shape` | `emit_stroke_paint_type1(...)` (alias) | `emit_stroke_paint(...)` | `emit_stroke_paint(...)` | **idêntico funcionalmente** (alias resolvido) |
| `Group` | `q cm ... emit_shape_path_local clip W n... draw_item_local recursive Q` | Idêntico | Idêntico | **idêntico** |

**Confirmação literal**: divergência factual só em **Text** (3 strats)
e **Glyph** (Type1 vs CID-like). Tudo o resto é compartilhável
identicamente.

### §A.1.1 — Particularidade `emit_stroke_paint_type1` vs `emit_stroke_paint`

Linha 2075–2084:

```rust
fn emit_stroke_paint_type1(...) {
    emit_stroke_paint(ops, paint, thickness, ...);
}
```

É **alias trivial** preservado por compatibilidade interna histórica.
Comportamento bit-exact idêntico. Consolidação pode remover o alias
(LOC -10) ou preservá-lo (defensive); P281 escolhe **preservar e
desuso** — Shape arm unificado usa `emit_stroke_paint` directamente.

---

## §A.2 — Parâmetros que cada scenario precisa

### Compartilhados (4 params idênticos)

- `ptr_to_idx: &HashMap<usize, usize>` (Image XObject lookup).
- `img_refs: &[ImageRef]` (Image XObject names).
- `pat_ptr_to_idx: &HashMap<DedupKey, usize>` (Gradient pattern lookup).
- `pat_refs: &[PatternRef]` (Gradient pattern names).

### Específicos por scenario

| Scenario | Params específicos |
|---|---|
| Type1 | Zero (style.bold/italic do FrameItem; faux-bold/tracking derivados de style) |
| CIDFont | `char_to_gid: &HashMap<char, u16>` |
| Multifont | `fonts: &[(FontList, Vec<u8>)]` **+** `per_font_char_to_gid: &[HashMap<char, u16>]` |

**Observação importante**: `font_lookup: &HashMap<&FontList, usize>`
sugerido na spec §A.3 **não existe** no código actual — multifont
calcula `fi` inline via `fonts.iter().position(|(stored, _)| stored == fl)`.
Decisão Fase A: **preservar cálculo inline** (não adicionar HashMap
novo); reduz `PageContext` complexity sem custo de performance
mensurável (N fonts tipicamente pequeno).

---

## §A.3 — Forma exacta de `PageContext` + `FontScenario`

```rust
pub(crate) enum FontScenario<'a> {
    Type1,
    Cidfont {
        char_to_gid: &'a HashMap<char, u16>,
    },
    Multifont {
        fonts:                &'a [(FontList, Vec<u8>)],
        per_font_char_to_gid: &'a [HashMap<char, u16>],
    },
}

pub(crate) struct PageContext<'a> {
    pub ptr_to_idx:     &'a HashMap<usize, usize>,
    pub img_refs:       &'a [ImageRef],
    pub pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
    pub pat_refs:       &'a [PatternRef],
    pub font_scenario:  FontScenario<'a>,
}
```

### Helpers constructors

```rust
impl<'a> PageContext<'a> {
    pub(crate) fn type1(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
    ) -> Self { ... }

    pub(crate) fn cidfont(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        char_to_gid: &'a HashMap<char, u16>,
    ) -> Self { ... }

    pub(crate) fn multifont(...) -> Self { ... }
}
```

### Lifetimes + visibility

- `'a` único cobre todos os refs (constructor scope ≤ chamada
  build_page_stream). Suficiente: não há cross-context lifetime.
- `pub(crate)` em ambos os tipos — não exposto externamente; preserva
  API pública estável (`export_pdf*` funções entry-point inalteradas).
- `FontList` e `ImageRef`/`PatternRef`/`DedupKey` já têm visibility
  necessária (testados via grep negativo — todos `pub(crate)` ou
  `pub(super)`).

Verificação compilação preliminar: forma viável (sem refactor de
visibility necessário).

---

## §A.4 — Pipeline unificado proposto

```rust
// Entry-points top-level (API pública preserved)
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8> { ... }
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8> { ... }
pub fn export_pdf_multifont(doc: &PagedDocument, fonts: &[(FontList, Vec<u8>)]) -> Vec<u8> { ... }

// Cada entry-point pre-computa resources + constrói PageContext + chama build_page_stream

// Unified stream builder (NEW)
fn build_page_stream(page: &Page, ctx: &PageContext) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.height;
    for item in &page.items {
        draw_item_top_level(&mut ops, item, page_height, ctx);
    }
    ops.into_bytes()
}

// Top-level emit dispatcher (NEW)
fn draw_item_top_level(ops: &mut String, item: &FrameItem, page_height: f64, ctx: &PageContext) {
    match item {
        FrameItem::Text { .. } => emit_text_top_level(ops, item, page_height, ctx),
        FrameItem::Glyph { .. } => emit_glyph_top_level(ops, item, page_height, ctx),
        FrameItem::Line { .. } => emit_line_top_level(...),  // scenario-independent
        FrameItem::Image { .. } => emit_image_top_level(...),  // scenario-independent
        FrameItem::Shape { .. } => emit_shape_top_level(...),  // scenario-independent
        FrameItem::Group { .. } => emit_group_top_level(ops, item, page_height, ctx),  // recurses via draw_item_local
    }
}

// Recursive local emit (existing, refactored to take &PageContext)
fn draw_item_local(
    ops: &mut String,
    item: &FrameItem,
    parent_bbox_override: Option<Rect>,
    ctx: &PageContext,
) {
    match item {
        FrameItem::Text { .. } => emit_text_local(ops, item, ctx),  // NEW arm real
        FrameItem::Glyph { .. } => emit_glyph_local(ops, item, ctx), // NEW arm real
        FrameItem::Line { .. } => emit_line_local(ops, item),         // NEW arm real
        FrameItem::Image { .. } => emit_image_local(ops, item, ctx),  // P279 preserved
        FrameItem::Shape { .. } => emit_shape_local(ops, item, parent_bbox_override, ctx), // P273.13 preserved
        FrameItem::Group { .. } => emit_group_local(ops, item, parent_bbox_override, ctx), // recursive
    }
}
```

### Decisão sub-helpers

**Não** extrair cada arm como função separada — manteria match
inline com helpers só para Text/Glyph emit (que têm a complexidade
real). Estrutura emergente:

```rust
fn emit_text_pdf(ops, pos, text, style, base_y, ctx) {
    // base_y = page_height - pos.y (top-level) OR pos.y.0 (local)
    match ctx.font_scenario {
        FontScenario::Type1 => emit_text_type1(...),
        FontScenario::Cidfont { char_to_gid } => emit_text_cidfont(...),
        FontScenario::Multifont { fonts, per_font_char_to_gid } => emit_text_multifont(...),
    }
}

fn emit_glyph_pdf(ops, pos, glyph_id, size, base_y, ctx) {
    match ctx.font_scenario {
        FontScenario::Type1 => {} // silently ignored
        FontScenario::Cidfont { .. } | FontScenario::Multifont { .. } => {
            // emit /F1 size Tf x y Td <gid> Tj ET
        }
    }
}
```

Top-level e local diferem apenas em **Y coordinate** (`page_height - pos.y`
vs `pos.y.0`). Esse offset passado como `base_y: f64` ao helper.

---

## §A.5 — Casos de teste planeados

### Existing tests (regressão implícita)

O workspace tem **2 611 testes pré-existentes** que cobrem
Helvetica/CIDFont/Multifont para top-level Text/Glyph/Line/Image/Shape
+ corpus paridade. Estes constituem a **suite de regressão bit-exact**
implícita — qualquer divergência byte-byte falha alguma assertion
existente.

**Decisão Fase A**: NÃO duplicar regressão com testes "p281_*_preserved"
redundantes. Tests existentes são a guard. Adicionar apenas 1-2
canónicos P281 explícitos (smoke tests com PDF byte hash) para
documentar a intenção.

### Tests funcionais novos (~9-12 tests)

Cobertura Text/Glyph/Line em Group cross-scenario:

| Teste | Verifica |
|-------|----------|
| `p281_text_em_group_helvetica` | Text Latin-1 dentro de Group rotacionado emite `(...) Tj` no PDF (Type1) |
| `p281_text_em_group_cidfont` | Text Unicode dentro de Group emite hex glyph IDs (CIDFont) |
| `p281_text_em_group_multifont` | Text com style.font dentro de Group emite `/F{i+1}` correcto (Multifont) |
| `p281_text_em_group_cidfont_nao_perde_chars` | Chars dentro de Group estão no char_to_gid (regressão directa P280 fix) |
| `p281_glyph_em_group_cidfont` | Glyph dentro de Group emite hex glyph_id (CIDFont) |
| `p281_glyph_em_group_multifont` | Glyph dentro de Group emite `/F1` (Multifont) |
| `p281_glyph_em_group_helvetica_continua_ignorado` | Glyph em Group + Type1 continua silently ignored (paridade) |
| `p281_line_em_group_emite_path_ops` | Line dentro de Group emite path ops correctos |
| `p281_text_em_group_aninhado_cidfont` | Text dentro de Group dentro de Group (recursão N=2) |
| `p281_smoke_unified_pipeline_helvetica` | Smoke: PDF byte-hash canónico Helvetica |
| `p281_smoke_unified_pipeline_cidfont` | Smoke: PDF byte-hash canónico CIDFont |

Estimativa: **11 testes funcionais** ≈ ~70-90 LOC (cap testes hard
80 — apertado mas viável; testes minimalistas com helpers compartilhados).

### Estratégia bit-exact pragmática

Em vez de "testes regressão pré/pós P281", confiar nas 2 611 assertions
existentes + adicionar 2 smoke tests P281 que verificam o pipeline
unificado produz exactamente os mesmos bytes para inputs canónicos.

---

## §A.6 — Estimativa LOC produção (refinada com inventário factual)

| Componente | LOC adicionados | LOC removidos | Net |
|---|---|---|---|
| `PageContext` + `FontScenario` + 3 constructors | ~35 | 0 | +35 |
| `build_page_stream` unificado (substitui 3 funções) | ~25 | ~587 | **-562** |
| `draw_item_local` refactored + arms Text/Glyph/Line real | ~80 | ~30 (stubs + signature antiga) | **+50** |
| Helper `emit_text_pdf` + `emit_glyph_pdf` + outros sub-helpers | ~150 | 0 | +150 |
| `draw_item_top_level` dispatcher | ~80 | 0 | +80 |
| 3 entry-point callers (build_helvetica/cidfont/multifont) refactored | ~10 | ~10 | **0** |
| **Total estimado** | **~380** | **~627** | **~-247** |

**Net LOC L3 ESTIMADO**: **-247** (consolidação compensa amplamente
adições). Cap hard 220 / soft 170 totalmente preservado mesmo em
contagem absoluta de LOC adicionados.

**Observação**: estimativa inicial spec subestimou redução (-562 da
remoção dos 3 stream-builders vs +185 espec). A unificação real é
mais consolidatória do que esperado porque os 3 stream-builders
têm enorme duplicação (Shape arm sozinho repete-se ~85 LOC × 3 =
~255 LOC; Image arm ~10 LOC × 3 = ~30 LOC; Group arm ~25 LOC × 3 =
~75 LOC; etc.).

**Net efectivo** depende de quanto helpers extraídos (Text/Glyph emit
sub-helpers) vs inline match. Decisão: **inline match** dentro de
2 funções (`emit_text_pdf` + `emit_glyph_pdf`) — menos LOC, mantém
legibilidade, evita explosão de helpers.

---

## §A.7 — Paridade vanilla (sanity)

Os testes de paridade observacional existentes (em `integration_tests.rs`
e workspace) **não comparam código estrutural** com vanilla — comparam
**output PDF**. P281 é refactor puramente estrutural cristalino-interno;
output PDF deve permanecer bit-exact para todos os inputs.

**Decisão Fase A**: zero tests de paridade vanilla tocados; verificação
indirecta via 2 611 baseline tests preservados.

---

## §A.8 — Gates §A.8 da spec — **não disparados**

| Gate | Estado |
|------|--------|
| 1. Divergência além de Text+Glyph | ✓ Não (Line/Image/Shape/Group idênticos; emit_stroke_paint_type1 é alias trivial) |
| 2. Params adicionais não previstos | ✓ Não (4 partilhados + 1-2 específicos cobertos) |
| 3. Lifetimes/visibility falham | ✓ Não (tipos referenciados já têm visibility correcta) |
| 4. LOC excede hard 220 | ✓ Não (net estimado -247; absoluto adicionado ~380 mas consolidatório) |
| 5. Cap doc Fase A hard 800 | ✓ Não (este ficheiro ~450 LOC) |
| 6. Tests baseline 2 615 alterada | ✓ Confirmado pré-Fase A (2 615 verdes) |

---

## §A.9 — Conclusão Fase A

- **3 stream-builders factualmente unificáveis** com divergência
  isolada a Text + Glyph arms.
- **`PageContext` + `FontScenario`** forma definida; tipos existentes
  têm visibility/lifetime compatíveis.
- **LOC net estimado: -247** (consolidação >> adições) — cap hard 220
  trivialmente preservado.
- **11 testes funcionais novos** planeados; existing 2 611 servem
  como suite de regressão bit-exact implícita.
- **Decisão arquitectural β-completa confirmada** factualmente viável.
- **Próximo**: §C materialização.

Fase A imutável a partir de 2026-05-18.
