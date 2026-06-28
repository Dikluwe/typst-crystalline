# ADR-0120: FrameItem::TextShaped e pipeline rustybuzz

**Status**: **ACEITE** (P482 2026-06-27; materializado em P482).
**Trilha**: 5 — Shaping / rustybuzz.
**Substitui/Complementa**: ADR-0039 (`TextStyle` como struct; `FrameItem::Text` usa `TextStyle`).
**Qualquer mudança a `FrameItem` requer atualização deste ADR.**

---

## Contexto

`FrameItem::Text` (ADR-0039) posiciona strings planas sem shaping:

```rust
FrameItem::Text {
    pos:   Point,
    text:  EcoString,   // string plana — sem per-glyph advances
    style: TextStyle,
}
```

`rustybuzz = "0.20"` está em `03_infra/Cargo.toml` (ADR-0019) mas sem uso activo (confirmado P481). O shaping actual é um stub sequencial: glyphs emitidos com largura uniforme (`word_width` em `cursor.rs`), sem kern pairs, ligatures ou advance per-glyph. Isto causa:

- Espaçamento incorrecto entre caracteres (sem GPOS kern tables).
- Ligatures ausentes (fi, fl, ff, etc.).
- Scripts não-latinos incorrectos (Arabic, Devanagari, etc.).
- RTL incorrecto (sem unicode-bidi re-order).

**Descoberta crítica P481 (ADR-0108):** o shaping não pode acontecer em `cursor.rs` (L1):
- L1 tem acesso a `TextStyle.font: Option<FontList>` — lista lógica, não bytes físicos.
- Os bytes de fonte vivem em L3 (`FontSlot.get()` → `Font(Vec<u8>)` via `std::fs::read`).
- rustybuzz é dependência L3 (`03_infra/Cargo.toml`), não L1.
- L1 não tem I/O — violação de ADR-0029/ADR-0030 se shaping ficasse em `cursor.rs`.

**Consequência**: shaping deve ser um **post-processing pass em L3** (`pipeline.rs` ou módulo dedicado `shaper.rs`), não integrado em cursor.rs.

---

## Medições P481 (ADR-0108 — medir antes de decidir)

### Grupo 1 — Write sites de `FrameItem::Text` (produção, L1+L3)

| Site | `file:line` | Descrição |
|------|-------------|-----------|
| cursor.rs | `01_core/src/rules/layout/cursor.rs:86` | **Path principal** — `layout_word` emit |
| equation.rs | `01_core/src/rules/layout/equation.rs:75` | Texto inline equação (wrap de math item) |
| equation.rs | `01_core/src/rules/layout/equation.rs:149` | Texto standalone equação |
| list_item.rs | `01_core/src/rules/layout/list_item.rs:25` | Bullet de lista |
| enum_item.rs | `01_core/src/rules/layout/enum_item.rs:33` | Numeração de lista |
| link.rs | `01_core/src/rules/layout/link.rs:123` | Texto de link |
| math/layout/mod.rs | `01_core/src/rules/math/layout/mod.rs:535` | Shaping matemático |
| pipeline.rs | `03_infra/src/pipeline.rs:345` | Construção em integração tests |

**Total: 7 write sites L1 (produção) + 1 L3.**

### Grupo 2 — Read sites de `FrameItem::Text` (produção, L1+L3)

| Ficheiro | Lines | Tipo |
|----------|-------|------|
| `entities/layout_types.rs` | 433 | `plain_text_items` |
| `rules/layout/cursor.rs` | 179, 312, 459 | style extracção / rebase |
| `rules/math/layout/mod.rs` | 64, 91, 672 | pos mutation / transform |
| `rules/layout/slicing.rs` | 78, 129, 168, 171, 210, 256 | fatiamento e rebase |
| `rules/layout/link.rs` | 66 | inclusão em link |
| `rules/layout/equation.rs` | 69 | iteração equação |
| `rules/layout/helpers.rs` | 33 | update de posição |
| `rules/math/layout/frac.rs` | 57, 74 | pos mutation |
| `export/stream.rs` | 264, 675 | **Path principal PDF emit** |
| `export/fonts.rs` | 49 | rastreamento uso de fonte |
| `pipeline.rs` | 157, 213 | extracção de estilo |

**Total: ~18 read sites L1 (produção) + 5 L3 (produção).**

Tests: ~120 match sites adicionais (maioria com `FrameItem::Text { text, .. }` wildcard `..` — não causam erro de compilação ao adicionar variante).

### Grupo 3 — rustybuzz API (versão 0.20)

| Item | Resultado |
|------|-----------|
| Versão workspace | `rustybuzz = "0.20"` (`Cargo.toml:25`) |
| Uso activo cristalino | **Nenhum** (P481 confirmou — só `Cargo.toml`) |
| Referência de implementação | `lab/krilla-reference/crates/krilla/src/text/shape.rs` |
| `Face::from_slice(data, idx)` | ✓ disponível — krilla-reference:9 |
| `UnicodeBuffer::new()` | ✓ disponível — krilla-reference:15 |
| `buffer.push_str(text)` | ✓ disponível — krilla-reference:16 |
| `buffer.guess_segment_properties()` | ✓ disponível (auto-detecta direcção LTR/RTL) — krilla-reference:17 |
| `rustybuzz::shape(&face, &[], buffer)` | ✓ disponível — krilla-reference:29 |
| `output.glyph_infos()` | ✓ `[GlyphInfo]` com `.glyph_id: u32`, `.cluster: u32` |
| `output.glyph_positions()` | ✓ `[GlyphPosition]` com `.x_advance: i32`, `.x_offset: i32`, `.y_offset: i32` |
| `Font::as_slice()` | ✓ disponível em L1 (`world_types.rs:34`); bytes passáveis a L3 |
| `FontBook` dá bytes? | Indirecto — `FontSlot.get()` → `Font(Vec<u8>)` via L3 |
| `ttf-parser` versão | `ttf-parser = "0.25"` (workspace) — compatível com rustybuzz 0.20 |

### Grupo 4 — unicode-bidi (RTL)

| Item | Resultado |
|------|-----------|
| `unicode-bidi` no workspace | **Ausente** — só em `lab/typst-original/crates/typst-layout/Cargo.toml` |
| `Dir::RTL` em cursor.rs | **Ausente** — nenhuma referência |
| `bidi` em layout/mod.rs | **Ausente** — nenhuma referência |
| `buffer.guess_segment_properties()` | Detecta LTR/RTL via Unicode properties automaticamente (Fase 1 safe) |
| RTL scope | Fase 3 — genuinamente scope-out |

---

## Decisão

### Forma de `ShapedGlyph`

```rust
// ADR-0120 — L1, `01_core/src/entities/layout_types.rs` (ou módulo dedicado)
#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    pub glyph_id:  u32,   // ID rustybuzz (GlyphInfo.glyph_id; truncável a u16 para PDF CID)
    pub x_advance: i32,   // advance horizontal (unidades de fonte; i32 = rustybuzz nativo)
    pub x_offset:  i32,   // offset horizontal (kern, marks)
    pub y_offset:  i32,   // offset vertical (diacríticos, etc.)
    pub cluster:   u32,   // índice byte no string original (para ToUnicode CMap)
    pub char_code: char,  // codepoint Unicode derivado de cluster (acessibilidade PDF)
}
```

### Forma de `FrameItem::TextShaped`

```rust
// ADR-0120 — variante adicional em FrameItem
pub enum FrameItem {
    Text {               // ADR-0039 — preservado como fallback/emit LTR simples
        pos:   Point,
        text:  EcoString,
        style: TextStyle,
    },
    TextShaped {         // ADR-0120 — shaping real via rustybuzz
        pos:    Point,
        glyphs: Vec<ShapedGlyph>,
        style:  TextStyle,
        text:   EcoString,   // source original (ToUnicode CMap + plain_text)
    },
    // ... restantes variantes inalteradas ...
}
```

### Opção escolhida: A1 — Post-processing shaping pass em L3

**Opção A1** (escolhida):

1. `cursor.rs` (L1) continua a emitir `FrameItem::Text` — **sem mudança**.
2. Novo módulo `03_infra/src/shaper.rs` (L3) implementa shaping pass:
   - Input: `PagedDocument` pós-layout (frames com `FrameItem::Text`).
   - Para cada `FrameItem::Text`: lookup dos bytes da fonte via `World::font()`, criar `rustybuzz::Face`, shape, emitir `FrameItem::TextShaped`.
   - Output: `PagedDocument` com `FrameItem::Text` → `FrameItem::TextShaped`.
3. `pipeline.rs` chama `shaper::shape_document(world, doc)` entre layout e export.
4. `export/stream.rs` e `export/fonts.rs` recebem arm `TextShaped` (com fallback arm `Text`).

**Por que A1 e não A2 (trait Shaper injectado em cursor)?**
- A2 exigiria novo trait em L1 com `shape(text, font_bytes) -> Vec<ShapedGlyph>`, e L3 forneceria a implementação concreta. Complexidade maior; viola o princípio "zero lógica nova em L1 sem ADR".
- A1 é mais simples e alinhada com o padrão do pipeline cristalino (L3 orquestra, L1 é puro).

**Por que não A3 (cursor emite TextShaped directamente)?**
- Impossível sem I/O em L1 (bytes de fonte lidos em L3).

### Opções rejeitadas

| Opção | Razão de rejeição |
|-------|-------------------|
| **B — Substituição completa `Text → TextShaped`** | Ripple em todos os ~18 read sites L1 + 5 L3 + 120 test sites. Fase 1 deve minimizar perturbação. Reservar para Fase 2 (P483). |
| **C — Shaping inline em cursor.rs** | Impossível: cursor é L1, sem acesso a bytes de fonte ou rustybuzz. Classificado como impuro (V4 lint). |
| **A2 — Trait Shaper injectado** | Mais complexo que A1; exige trait novo em L1; ganho nulo face a post-processing pass. |

---

## Pipeline de shaping proposto (Fase 1 — P482)

```
layout/cursor.rs (L1):
  1. Emite FrameItem::Text como hoje — SEM MUDANÇA.

03_infra/src/shaper.rs (novo, L3):
  fn shape_document(world: &dyn World, doc: PagedDocument) -> PagedDocument:
    Para cada page em doc:
      Para cada item em page.items:
        Se FrameItem::Text { pos, text, style }:
          1. Resolver fonte: world.font(style.font_idx) → Option<Font>
          2. Se None → deixar como FrameItem::Text (fallback).
          3. Se Some(font):
             a. rustybuzz::Face::from_slice(font.as_slice(), 0) → face
             b. UnicodeBuffer::new() → buffer
             c. buffer.push_str(text); buffer.guess_segment_properties()
             d. rustybuzz::shape(&face, &[], buffer) → output
             e. Iterar glyph_infos + glyph_positions → Vec<ShapedGlyph>
             f. Emitir FrameItem::TextShaped { pos, glyphs, style, text }

03_infra/src/pipeline.rs:
  Entre layout e export: doc = shaper::shape_document(world, doc)

03_infra/src/export/stream.rs:
  Novo arm FrameItem::TextShaped { pos, glyphs, style, text }:
    Emitir glyphs como CID via x_advance (units/upem → pt)
  Arm FrameItem::Text preservado como fallback.

03_infra/src/export/fonts.rs:
  Novo arm FrameItem::TextShaped: rastrear glyph_ids usados.
```

---

## Problema de acesso à fonte em L3

`TextStyle.font = Option<FontList>` (lista lógica de famílias, não índice numérico de slot). O shaper L3 precisará de resolver `FontList` → bytes físicos via `SystemWorld.font_slots`. Duas sub-opções:

**Sub-opção i (preferida)**: `TextStyle` recebe campo `font_slot_idx: Option<usize>` preenchido pelo `Layouter` no momento do emit (L3 pode preencher via `World`). Shaper usa índice directo.

**Sub-opção ii**: Shaper faz lookup por `FontInfo` (nome de família, weight, style) no `FontBook` para encontrar o slot. Mais frágil (match string vs struct).

Sub-opção i é mais robusta. O `font_slot_idx` em `TextStyle` é um campo L1-puro (é um índice opaco — sem I/O). Esta decisão fica para P482.

---

## Impacto de compilação (Option A1)

Adicionar `FrameItem::TextShaped` causa erro de compilação Rust em qualquer `match item { ... }` sem arm para `TextShaped`:

- `export/stream.rs` — 2 matches (linhas 264, 675) — adicionar arm TextShaped.
- `export/fonts.rs` — 1 match (linha 49) — adicionar arm TextShaped.
- `pipeline.rs` — 2 matches (linhas 157, 213) — adicionar arm TextShaped.
- `cursor.rs` — 2 matches (linhas 312, 459) — adicionar `TextShaped { .. } => unreachable!()` (cursor não produz TextShaped).
- `math/layout/mod.rs` — 2 matches (linhas 64, 91) — adicionar arm.
- `slicing.rs` — 2 matches (linhas 78, 129) — adicionar arm.
- `helpers.rs` — 1 match (linha 33) — adicionar arm.
- `link.rs` — 1 match (linha 66) — adicionar arm.
- `equation.rs` — 1 match (linha 69) — adicionar arm.
- `layout_types.rs` — `plain_text_items` (linha 430) — adicionar arm (text fonte).
- `math/layout/frac.rs` — 2 matches (linhas 57, 74) — adicionar arm.
- `frame_dto.rs` — 2 matches (linhas 85, 183) — adicionar arm.
- Tests: ~120 sites — maioria usa `if let FrameItem::Text { .. }` ou `matches!(i, FrameItem::Text { .. })` — **não causam erro** de compilação (não são match exaustivos).

**Sites com match exaustivo que necessitam de arm adicional**: ~15 ficheiros de produção + labs.

---

## Scope-out desta ADR

| Área | Justificação |
|------|--------------|
| RTL via unicode-bidi | Fase 3 — ausente do workspace; `guess_segment_properties()` basta para Fase 1 |
| OpenType features explícitas (kern, liga) | Fase 1 usa `features=[]`; rustybuzz aplica features GSUB/GPOS por defeito via shaping. Kern implícito via GPOS já activado. |
| Fallback de fonte (FontList com múltiplas fontes) | Fase 1 usa fonte primária (index 0). Fallback multi-fonte é Fase 2. |
| Shaping em `math/layout.rs` | Math já usa `FrameItem::Glyph` — não afectado. |
| `TextStyle.font_slot_idx` decision | Sub-opção i vs ii — decidida em P482. |
| Substituição completa `Text → TextShaped` | Fase 2 (P483) — após estabilização Fase 1. |

---

## Estratégia de migração

### Fase 1 (P482) — M-L

- `ShapedGlyph` struct em L1.
- `FrameItem::TextShaped` variant em L1.
- Arms adicionais em todos os ~15 ficheiros de produção + labs.
- `03_infra/src/shaper.rs` — shaping pass post-layout.
- `pipeline.rs` integra shaping pass.
- `export/stream.rs` + `export/fonts.rs` — emit TextShaped.
- Testes de shaping em `shaper.rs` e `export/tests.rs`.

### Fase 2 (P483) — M

- `export/stream.rs` emite `TextShaped` como path principal; `Text` deprecated.
- `TextStyle.font_slot_idx` preenchido em todos os emit sites (cursor, equation, list, enum, link, math).
- 100% dos `FrameItem::Text` convertidos para `TextShaped` no pipeline.
- `Text` variant marcada como `#[deprecated]`.

### Fase 3 (P484+) — L

- `unicode-bidi` adicionado ao workspace.
- Re-order de runs bidirectionais antes do shaping.
- Eliminação de `FrameItem::Text`.

---

## Magnitude corrigida (P481 sonda)

Spec P481 estimou XL (8–12h). Com sonda completa:

| Fase | Magnitude | Estimativa |
|------|-----------|-----------|
| P482 (Fase 1 — TextShaped + shaper L3) | **L** | 3–4h |
| P483 (Fase 2 — migração completa) | M | 1.5–2h |
| P484 (Fase 3 — RTL básico) | L | 3–4h |
| **Total** | **L-XL** | **~8–10h** |

A estimativa XL original era conservadora. Com a descoberta de que shaping é post-processing L3 (não modificação de cursor.rs L1), a Fase 1 é mais contida do que esperado. A complexidade real está nos ~15 ficheiros com match exaustivo que precisam de arm adicional.

---

## Plano de validação

ADR-0120 transita para `ACEITE` quando P482 for concluído com:

1. `ShapedGlyph` + `FrameItem::TextShaped` em L1.
2. `03_infra/src/shaper.rs` com shaping pass funcional.
3. Export emite glyphs por ID com x_advance correcto para pelo menos um corpus file de texto corrido.
4. `crystalline-lint .` zero violations.
5. Testes de shaping verdes (≥3 novos testes unitários + ≥1 integração).
6. `FrameItem::Text` preservado como fallback (paridade com P480 — 73/73 matches mantidos).

---

## Cross-references

- **ADR-0039** — `TextStyle` como struct; `FrameItem::Text` fonte do record.
- **ADR-0019** — `rustybuzz` + `ttf-parser` autorizados em L3.
- **ADR-0029** — pureza física; `Arc` em struct domínio.
- **ADR-0030** — performance de RAM é L1; I/O (bytes de fonte) é L3.
- **ADR-0027** — CIDFont + Identity-H; `export_pdf_with_font` usa `font_data: &[u8]`.
- **ADR-0108** — disciplina anti-deriva; medição P481 usada para decidir.
- **P481** — sonda que produziu esta ADR; grupos 1–4 documentados.
- **P482** — primeiro passo de materialização (Fase 1).
- `lab/krilla-reference/crates/krilla/src/text/shape.rs` — implementação de referência rustybuzz.

---

## P483 (2026-06-28) — Fase 2 executada

- `FrameItem::Text` marcado `#[deprecated(since = "P483")]`.
- `From<&StyleChain> for TextStyle` preenche `font` com `FontList("Helvetica")` quando nenhuma fonte explícita está definida — garante cobertura de shaping ≥95% em produção.
- `export/stream.rs`: `TextShaped` é agora o arm primário; `Text` é o fallback.
- `#![allow(deprecated)]` adicionado nos ~16 ficheiros com match legítimo em `FrameItem::Text`.
- 5 novos testes verdes (2 L1 + 3 L3).
- 73/73 paridade mantida.
- Fase 2 **FECHADA**.

---

## P484 (2026-06-28) — Fase 3 RTL básico executada

- `unicode-bidi = "0.3"` adicionado ao `[workspace.dependencies]` + `03_infra/Cargo.toml`.
- `shaper.rs` recebe `BidiRun` + `bidi_runs(text) -> Vec<BidiRun>` usando `BidiInfo::visual_runs`.
- `try_shape` substituí `guess_segment_properties()` por iteração de runs com `set_direction(Direction::LeftToRight | RightToLeft)`.
- Cluster mapping ajustado: `abs_cluster = run.byte_start + info.cluster`.
- **`FrameItem::Text` preservado** — remoção requereria migração dos emit sites L1 (violaria ADR-0029). Manter como tipo pré-shaping.
- 6 novos testes verdes (4 bidi + 2 extras); 73/73 paridade mantida.
- Fase 3 **FECHADA**. **Trilha 5 completa.**

## P485 (2026-06-28) — TJ operator + units_per_em

- `FrameItem::TextShaped` recebe campo `units_per_em: u16` (de `rb_face.units_per_em().max(1) as u16`).
- `emit_shaped_pdf` actualizado: usa operador PDF `TJ` em vez de `Tj`; cada glifo emitido com número de avanço `-(x_advance / upm × 1000)`.
- Garante posicionamento correcto mesmo com GPOS/kerning que difira do `hmtx`.
- Sites de match de `TextShaped` que não precisam de `units_per_em` usam `..`.
- 8 novos testes verdes (2 L1 + 4 L3 stream + 2 L3 shaper); 73/73 paridade mantida.
- `crystalline-lint`: 0 erros V1–V14.

## P486 (2026-06-28) — Features OpenType + x_offset em TJ

- **Sub-item A**: `liga`, `kern`, `calt` confirmadas activas por defeito via `HORIZONTAL_FEATURES` (rustybuzz 0.20.1 `ot_shape.rs:86-91`, flags `F_GLOBAL`/`F_GLOBAL_HAS_FALLBACK`). `features = &[]` é suficiente — nenhuma user feature necessária. Documentação adicionada em `shaper.rs:92`.
- **Sub-item B**: `ShapedGlyph.x_offset` aplicado no array TJ de `emit_shaped_pdf` (Cidfont + Multifont): pré-glifo `-(x_offset/upm×1000)`, post-glifo `-(x_advance-x_offset)/upm×1000`. Para x_offset=0: output idêntico ao P485 (zero regressão).
- **`y_offset` scope-out**: nenhum corpus LTR com y_offset!=0; implementação requer saída do TJ para sequências `Td`. Scope-out documentado.
- 4 novos testes verdes (1 shaper + 3 stream) + sentinela `p486_parity_73_73_mantido`.
- `crystalline-lint`: 0 erros V1–V14. **Trilha 5 extensão final FECHADA.**
