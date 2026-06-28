---

# P482 — Trilha 5 Fase 1: `ShapedGlyph` + `FrameItem::TextShaped` + `shaper.rs`

> **Passo:** 482
> **Data:** 2026-06-27
> **Foco:** Materializar shaping real com rustybuzz: struct `ShapedGlyph`, variant `FrameItem::TextShaped`, módulo `03_infra/src/shaper.rs` com post-processing pass, e arms em export.
> **Trilha:** 5 — Shaping / rustybuzz — Fase 1.
> **Tipo:** Materialização L.
> **Tamanho:** L (~3–4h).
> **ADR-0120 PROPOSTO** (P481) — autoriza `FrameItem::TextShaped` como novo variant; Opção A1 (post-processing L3); `cursor.rs` não muda.
> **ADR-0039 EM VIGOR** — `TextStyle` como struct resolvido; bridge `From<&StyleChain>`.
> **ADR-0029 EM VIGOR** — L1 puro; `font_slot_idx: Option<usize>` é campo opaco sem I/O.

---

## Contexto

P481 identificou os 4 grupos de sondas com `file:line` completos:

- **Write sites:** 7 sites L1 + 1 L3. Path principal: `cursor.rs:86`.
- **Read sites:** ~19 matches exaustivos + ~120 tests não-exaustivos.
- **rustybuzz 0.20** confirmado. API: `UnicodeBuffer + shape() → GlyphBuffer`.
- **Bloqueador:** `cursor.rs` (L1) não tem acesso a bytes de fonte. Solução: post-processing pass em `shaper.rs` (L3) entre layout e export.
- **ADR-0120** redigida. `cursor.rs` não muda — o shaper converte `FrameItem::Text → FrameItem::TextShaped` em L3.

---

## Tarefas (ordem de implementação)

### 1. `ShapedGlyph` struct (`01_core/src/entities/layout_types.rs`)

```rust
/// **P482** — Glifo shaped por rustybuzz. Campos em unidades de fonte (units_per_em).
/// Convertidos para pt no export: `x_advance_pt = x_advance as f64 / units_per_em * font_size_pt`.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    /// ID do glifo na fonte (índice na tabela de glifos).
    pub glyph_id:  u16,
    /// Advance horizontal em unidades de fonte (≥0 para LTR).
    pub x_advance: i32,
    /// Offset horizontal (kerning, marcas). Somado à posição de cursor.
    pub x_offset:  i32,
    /// Offset vertical (diacríticos, subscript). Somado ao baseline.
    pub y_offset:  i32,
    /// Índice byte no string original (para reconstrução ToUnicode CMap).
    pub cluster:   u32,
    /// Codepoint Unicode — derivado de `cluster` no shaper.
    pub char_code: char,
}
```

**Localização:** após `TextStyle` em `layout_types.rs`. Antes de `FrameItem`.

### 2. `FrameItem::TextShaped` variant (`01_core/src/entities/layout_types.rs`)

Adicionar ao enum `FrameItem` (preservar `Text` — sem remoção):

```rust
pub enum FrameItem {
    Text {
        pos:   Point,
        text:  EcoString,
        style: TextStyle,
    },
    /// **P482** — Texto com shaping real (rustybuzz). Substitui `Text`
    /// depois da passagem do shaper (L3). `Text` preservado para fallback
    /// e para casos onde shaping falha (fonte não carregada, etc.).
    TextShaped {
        pos:    Point,
        glyphs: Vec<ShapedGlyph>,
        style:  TextStyle,
        /// Texto original — preservado para acessibilidade (ToUnicode CMap)
        /// e para plain_text_items.
        text:   EcoString,
    },
    // ... variantes existentes ...
}
```

### 3. `font_slot_idx: Option<usize>` em `TextStyle`

**Ficheiro:** `01_core/src/entities/layout_types.rs`

```rust
pub struct TextStyle {
    // ... campos existentes ...
    /// **P482** — Índice do slot de fonte activo no momento do emit do FrameItem::Text.
    /// Preenchido pelo Layouter L3 via `FontBook::active_slot_idx()`.
    /// `None` = fonte padrão (Helvetica fallback).
    /// Campo opaco sem I/O — paridade ADR-0029.
    pub font_slot_idx: Option<usize>,
}
```

`TextStyle::default()` → `font_slot_idx: None`.
`From<&StyleChain> for TextStyle` — preservar `font_slot_idx: None` (será preenchido pelo Layouter L3 após resolução).

**Onde é preenchido:** em `03_infra/src/pipeline.rs` (ou `Layouter` L3) antes de emitir `FrameItem::Text`, chamar `FontBook::resolve_slot(&text_style.font)` e armazenar o índice.

### 4. Arms `TextShaped` nos ~19 sites de match exaustivo

**Regra de arm:**

| Contexto | Arm `TextShaped` |
|----------|-----------------|
| `cursor.rs:312, 459` (leitura de style) | `FrameItem::TextShaped { style, .. } => style` |
| `layout_types.rs:430` (`plain_text_items`) | `FrameItem::TextShaped { text, .. } => text.as_str()` |
| `math/layout/mod.rs:64, 91` (mutação pos) | `FrameItem::TextShaped { pos, .. } => *pos = new_pos` |
| `slicing.rs:78, 129, 168, 171, 210, 256` (rebase) | `FrameItem::TextShaped { pos, .. } => pos.x += delta.x` |
| `link.rs:66` (inclusão) | passthrough — incluir `TextShaped` no link frame |
| `equation.rs:69` (iteração) | `FrameItem::TextShaped { text, style, .. } => /* idem Text */` |
| `helpers.rs:33` (update pos) | `FrameItem::TextShaped { pos, .. } => *pos = …` |
| `math/layout/frac.rs:57, 74` (pos mutation) | idem `mod.rs` |
| `pipeline.rs:157, 213` (extracção estilo) | `TextShaped { style, .. } => style.clone()` |
| `frame_dto.rs:85, 183` (parity DTO) | incluir `text` no DTO |
| `export/stream.rs:264, 675` | **arm funcional — ver §5** |
| `export/fonts.rs:49` | **arm funcional — ver §5** |

**Padrão de implementação:** nos sites passthrough, o arm `TextShaped` replica o comportamento do arm `Text` usando os campos comuns (`pos`, `style`, `text`). Nenhum site precisa de aceder a `glyphs` excepto os dois de export.

### 5. `03_infra/src/shaper.rs` (novo ficheiro)

```rust
//! **P482** — Post-processing shaping pass.
//! Converte `FrameItem::Text` → `FrameItem::TextShaped` via rustybuzz.
//! Executado em L3 após layout e antes de export.
//! ADR-0120 Opção A1.

use rustybuzz::{UnicodeBuffer, Face as RbFace};
use crate::world::SystemWorld;
use typst_core::entities::layout_types::{FrameItem, ShapedGlyph, PagedDocument};

/// Converte todos os `FrameItem::Text` de um `PagedDocument` em
/// `FrameItem::TextShaped` via rustybuzz.
/// Itens sem fonte resolvida (`font_slot_idx = None`) são preservados como `Text`.
pub fn shape_document(world: &SystemWorld, mut doc: PagedDocument) -> PagedDocument {
    for page in &mut doc.pages {
        shape_frame(world, &mut page.frame);
    }
    doc
}

fn shape_frame(world: &SystemWorld, frame: &mut Frame) {
    for item in frame.items.iter_mut() {
        match item {
            FrameItem::Text { pos, text, style } if style.font_slot_idx.is_some() => {
                let slot_idx = style.font_slot_idx.unwrap();
                if let Some(shaped) = shape_text(world, pos, text, style, slot_idx) {
                    *item = shaped;
                }
                // Fallback: se shape_text retorna None, `Text` preservado.
            }
            FrameItem::Group { frame: inner, .. } => {
                shape_frame(world, inner);
            }
            _ => {}
        }
    }
}

fn shape_text(
    world:    &SystemWorld,
    pos:      &Point,
    text:     &EcoString,
    style:    &TextStyle,
    slot_idx: usize,
) -> Option<FrameItem> {
    // 1. Obter bytes da fonte
    let font = world.font(slot_idx)?;
    let font_data = font.as_slice();
    let face_idx  = font.face_index();

    // 2. Construir rustybuzz::Face
    let rb_face = RbFace::from_slice(font_data, face_idx)?;
    let units_per_em = rb_face.units_per_em() as i32;

    // 3. Shape
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text.as_str());
    buffer.guess_segment_properties();
    let output = rustybuzz::shape(&rb_face, &[], buffer);

    // 4. Construir Vec<ShapedGlyph>
    let infos     = output.glyph_infos();
    let positions = output.glyph_positions();
    let chars: Vec<char> = text.chars().collect();

    let glyphs: Vec<ShapedGlyph> = infos.iter().zip(positions.iter()).map(|(info, pos_g)| {
        // cluster = byte index; derivar char
        let char_code = byte_idx_to_char(text.as_str(), info.cluster as usize)
            .unwrap_or('\u{FFFD}');
        ShapedGlyph {
            glyph_id:  info.glyph_id as u16,
            x_advance: pos_g.x_advance,
            x_offset:  pos_g.x_offset,
            y_offset:  pos_g.y_offset,
            cluster:   info.cluster,
            char_code,
        }
    }).collect();

    Some(FrameItem::TextShaped {
        pos:    *pos,
        glyphs,
        style:  style.clone(),
        text:   text.clone(),
    })
}

fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char> {
    s[byte_idx..].chars().next()
}
```

### 6. `pipeline.rs` — integrar `shaper::shape_document`

**Ficheiro:** `03_infra/src/pipeline.rs`

Entre a chamada de `layout(...)` e `export(...)`:

```rust
// P482 — shaping pass entre layout e export
let doc = typst_core::rules::layout::layout(content, world_adapter);
let doc = crate::shaper::shape_document(world, doc);
export::export_pdf(doc, world)?;
```

### 7. `export/stream.rs` — arms `TextShaped`

**Ficheiro:** `03_infra/src/export/stream.rs`

Nos dois sites de match (`264` e `675`):

```rust
FrameItem::TextShaped { pos, glyphs, style, .. } => {
    // Emit CID text string: posicionar + iterar glyphs
    let font_size_pt = style.size.0;
    let units_per_em = get_units_per_em(world, style.font_slot_idx) as f64;

    emit_text_position(writer, *pos, font_size_pt, style);

    // Emitir como hex string CID: "<GGGG...>" Tj
    let hex = glyphs.iter().map(|g| {
        format!("{:04X}", g.glyph_id)
    }).collect::<String>();
    write!(writer, "<{}> Tj\n", hex)?;
}
```

**Nota:** o arm existente de `Text` permanece para fallback (itens sem `font_slot_idx` ou onde shaping falhou).

### 8. `export/fonts.rs` — arm `TextShaped`

**Ficheiro:** `03_infra/src/export/fonts.rs:49`

```rust
FrameItem::TextShaped { glyphs, style, .. } => {
    // Rastrear glyph_ids em vez de chars
    for g in glyphs {
        used_glyphs.entry(style.font_slot_idx.unwrap_or(0))
            .or_default()
            .insert(g.glyph_id);
    }
}
```

---

## Tests

### L1 — `entities::layout_types::tests`

- `p482_shaped_glyph_clone_eq` — `ShapedGlyph { glyph_id: 1, x_advance: 600, .. }` clona e é `==`.
- `p482_frame_item_text_shaped_plain_text` — `FrameItem::TextShaped { text: "abc", .. }` → `plain_text_items` devolve `"abc"`.
- `p482_text_style_font_slot_idx_default_none` — `TextStyle::default().font_slot_idx == None`.

### L3 — `shaper::tests`

- `p482_shape_text_latin_basic` — "Hello" com fonte Helvetica → Vec<ShapedGlyph> com 5 glyphs; glyph_ids não-zero; x_advance > 0.
- `p482_shape_text_empty_string` — `""` → Vec vazia sem panic.
- `p482_shape_document_preserves_text_sem_slot` — item `FrameItem::Text` sem `font_slot_idx` é preservado como `Text` (não convertido).
- `p482_shape_document_converte_text_com_slot` — item com slot → `FrameItem::TextShaped`.
- `p482_byte_idx_to_char` — `byte_idx_to_char("héllo", 0)` → `Some('h')`; `byte_idx_to_char("héllo", 1)` → `Some('é')` (é = 2 bytes UTF-8; verificar cluster correcto).

### Paridade (regressão)

- `p482_parity_73_73_mantido` — suite `lab/parity/` não deve regredir; 73/73 matches mantidos.

---

## Spec L0

### Novos

- `entities/shaped_glyph.md` — `ShapedGlyph` struct, campos, semântica, unidades.
- `infra/shaper.md` — `shape_document`, `shape_frame`, `shape_text`; pipeline A1; fallback.

### Actualizados

- `entities/layout_types.md` — `FrameItem::TextShaped` variant; `TextStyle.font_slot_idx`.
- `infra/export/stream.md` — arm `TextShaped`; emit CID hex string.
- `infra/export/fonts.md` — arm `TextShaped`; rastreamento glyph_ids.
- `infra/pipeline.md` — integração `shape_document` entre layout e export.
- ADR-0120 — transição PROPOSTO → ACEITE após implementação.

---

## Scope-out explícito

- **`cursor.rs` não muda** — zero alterações a L1 de layout. ADR-0120 A1.
- **RTL / bidi** — `guess_segment_properties()` detecta automaticamente para LTR; re-order bidireccional é Fase 3 (P484).
- **OpenType features explícitas** — `features=[]` usa padrão GSUB/GPOS da fonte; feature knobs (liga, kern, smcp) são P485+.
- **Fallback de fonte** — shaping usa apenas a fonte primária (`font_slot_idx`); fallback para segunda fonte de `FontList` é P485+.
- **`FrameItem::Text` deprecação** — permanece funcional; deprecação formal é P483.
- **Math shaping** — `math/layout.rs` usa `FrameItem::Glyph` já posicionado; não é tocado.
- **`units_per_em` cacheado** — não é optimizado neste passo; lido per-call.

---

## Critério de fecho

- [ ] Sondas pré-implementação: confirmar que `font_slot_idx` está ausente de `TextStyle`; confirmar `shaper.rs` não existe; confirmar localização exacta dos 2 sites de export — todos com `file:line`.
- [ ] `ShapedGlyph` struct implementado em `entities/layout_types.rs`.
- [ ] `FrameItem::TextShaped` variant adicionado ao enum.
- [ ] `TextStyle.font_slot_idx: Option<usize>` adicionado.
- [ ] Arms `TextShaped` nos ~19 sites de match exaustivo (compile verde sem `_` arm).
- [ ] `font_slot_idx` preenchido no Layouter L3 (`pipeline.rs` ou wrapper) para items `FrameItem::Text`.
- [ ] `03_infra/src/shaper.rs` criado com `shape_document`, `shape_frame`, `shape_text`.
- [ ] `pipeline.rs` integra `shape_document` entre layout e export.
- [ ] `export/stream.rs` tem arm `TextShaped` funcional.
- [ ] `export/fonts.rs` tem arm `TextShaped` funcional.
- [ ] 8+ testes verdes (3 L1 + 5 L3).
- [ ] `p482_parity_73_73_mantido` verde (73/73 matches preservados).
- [ ] Spec L0 actualizada (6 ficheiros).
- [ ] ADR-0120 transicionada PROPOSTO → ACEITE.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 5: Fase 1 FECHADA.** Texto latino LTR com shaping real no PDF produzido.

---

## Estado pós-P481 (para referência)

| Indicador | Estado |
|-----------|--------|
| Paridade | 73/73 matches |
| DEBTs activos | 0 |
| ADR-0120 | PROPOSTO (P481) |
| Trilhas completas | 1, 2, 3, 4, 7, 8 |
| **P482** | Trilha 5 Fase 1 — `TextShaped` + shaper | 🔄 EM PREPARAÇÃO |
