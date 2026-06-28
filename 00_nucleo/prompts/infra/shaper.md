---
prompt: infra/shaper
layer: L3
created: 2026-06-27
passo: P482
adr: ADR-0120
---

# Prompt L0 — `shaper.rs` (Trilha 5 Fase 1)
Hash do Código: 63feb005

## Propósito

Post-processing shaping pass: converte `FrameItem::Text` →
`FrameItem::TextShaped` via rustybuzz após layout e antes de export.
Executado em L3 (`03_infra/src/shaper.rs`).

Decisão arquitectural: ADR-0120 Opção A1 — o shaping não pode ocorrer
em `cursor.rs` (L1) porque os bytes de fonte vivem em L3.

## API pública

```rust
pub fn shape_document(world: &dyn World, doc: PagedDocument) -> PagedDocument
```

Converte todos os `FrameItem::Text` de um `PagedDocument` em
`FrameItem::TextShaped` via rustybuzz.

Itens sem fonte resolvida (`style.font == None` ou lookup falha) são
preservados como `FrameItem::Text` (fallback Helvetica).

## Pipeline interno

```
shape_document(world, doc) → shape_page → shape_item → try_shape
```

- `shape_item`: desce recursivamente em `Group` e `Link`.
  `Text` com `style.font.is_some()` é candidato a shaping.
- `try_shape`: resolve slot via `resolve_slot`, obtém bytes via
  `world.font(slot_idx)`, constrói `rustybuzz::Face::from_slice(data, 0)`,
  shape com `rustybuzz::shape(&rb_face, &[], buffer)`, mapeia
  glyph_infos + glyph_positions → `Vec<ShapedGlyph>`.

## Resolução de fonte

```rust
fn resolve_slot(world: &dyn World, font_list: &FontList) -> Option<usize>
```

Itera `font_list.as_slice()`, para cada família chama
`world.book().select_pattern(&family.name, &FontVariant::default())`.
Verifica que `world.font(idx)` devolve `Some` antes de aceitar o slot.
Mesma lógica de `resolve_font` em `pipeline.rs`.

## `byte_idx_to_char`

```rust
fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char>
```

Devolve o codepoint que começa no byte `byte_idx` de `s` (UTF-8).
Usado para popular `ShapedGlyph.char_code` a partir de `cluster`.

## Scope-out (Fase 1)

- RTL/bidi — Fase 3 (ADR-0120).
- OpenType features explícitas — `features = &[]` usa GSUB/GPOS padrão.
- Font fallback multi-família além da primeira que resolve.
- Face index > 0 (TTC fonts) — Fase 1 usa sempre index 0.
- `FrameItem::TextShaped` shaping (recursão) — não re-shape já shaped.

## Testes (≥6)

- `p482_shape_document_preserves_text_sem_font`: sem `style.font` → Text preservado.
- `p482_shape_document_preserves_text_content`: conteúdo do Text preservado.
- `p482_byte_idx_to_char_ascii`: índices ASCII correctos.
- `p482_byte_idx_to_char_utf8`: multi-byte UTF-8 (é = 2 bytes).
- `p482_shaped_glyph_clone_eq`: ShapedGlyph clone+eq.
- `p482_shape_document_group_children_passthrough`: Text dentro de Group preservado sem font.

## P484 — Fase 3: RTL básico via unicode-bidi

**Data:** 2026-06-28

`unicode-bidi = "0.3"` adicionado ao workspace. `try_shape` agora divide o texto
em runs bidirectionais antes de shape via `bidi_runs(text) -> Vec<BidiRun>`.

### `BidiRun`

```rust
struct BidiRun {
    text:       String,  // substring do texto original para este run
    rtl:        bool,    // true se run RTL (árabe, hebraico, etc.)
    byte_start: usize,   // offset byte no string original (para cluster abs)
}
```

### `bidi_runs`

```rust
fn bidi_runs(text: &str) -> Vec<BidiRun>
```

Usa `unicode_bidi::BidiInfo::new(text, None)` + `visual_runs(para, line)`
para obter runs na ordem visual correcta. Para texto LTR puro: 1 run.
Para texto misto/RTL: múltiplos runs na ordem correcta de renderização.

### `try_shape` pós-P484

Substitui `buffer.guess_segment_properties()` por:

```rust
for run in bidi_runs(text) {
    buffer.set_direction(if run.rtl { Direction::RightToLeft }
                         else       { Direction::LeftToRight });
    // shape + ajustar abs_cluster = run.byte_start + info.cluster
}
```

### Scope-out P484

- Múltiplos parágrafos (`paragraphs[0]` assume 1 parágrafo por `FrameItem::Text`).
- Texto vertical (CJK rotated).
- Corpus RTL no lab/parity — testes unitários L3 cobrem.
- Remoção de `FrameItem::Text` — requer ADR nova (colisão ADR-0029).

### Testes adicionados P484

- `p484_bidi_runs_ltr_unico_run`: texto inglês → 1 run LTR.
- `p484_bidi_runs_vazio_zero_runs`: texto vazio → 0 runs.
- `p484_bidi_runs_arabico_rtl`: árabe → run RTL detectado.
- `p484_try_shape_rtl_sem_fonte_nao_panic`: árabe sem fonte → sem panic.
- `p484_bidi_runs_misto_ingles_arabico`: texto misto → ≥2 runs.
- `p484_bidi_runs_byte_start_correcto`: byte_start correcto.

---

## P483 — Fase 2: font padrão + cobertura ≥95%

**Data:** 2026-06-28

`try_shape` actua quando `style.font.is_some()`. Pós-P483, `From<&StyleChain>
for TextStyle` preenche sempre `font` com pelo menos `FontList("Helvetica")`
(fallback padrão). Assim o shaper tenta actuar em todo o texto, não apenas
em texto com `#set text(font:...)` explícito.

Comportamento defensivo preservado: se `resolve_slot` não encontra a fonte
no `FontBook` (fonte não carregada, Type1), `try_shape` retorna `None` e o
item permanece como `FrameItem::Text`.

### Cobertura esperada em produção

Em produção com `SystemWorld` e fontes do sistema carregadas: ≥95% do texto
normal será `TextShaped`. `FrameItem::Text` resta apenas para fontes ausentes.

### Testes adicionados P483

- `p483_text_com_font_helvetica_tenta_shape_mas_sem_fontes_preserva_text`
- `p483_text_sem_font_nao_tenta_shape`
- `p483_shaped_glyph_debug_display`

## §P485 — `units_per_em` em `try_shape`

**P485** adiciona extracção de `units_per_em` de `rb_face.units_per_em()` (retorna
`i32` via rustybuzz; cast `.max(1) as u16`). O valor é armazenado em
`FrameItem::TextShaped.units_per_em: u16` e usado em `emit_shaped_pdf` para
converter `x_advance` (font units) em unidades TJ do PDF.

```rust
// Em try_shape, após construir rb_face:
let units_per_em = rb_face.units_per_em().max(1) as u16;
// ...
Some(FrameItem::TextShaped { pos: *pos, glyphs: all_glyphs, style: style.clone(),
                              text: text.clone(), units_per_em })
```

### Testes adicionados P485

- `p485_shape_document_sem_fonte_nao_produz_textshaped`
- `p485_units_per_em_cast_seguro`

## §P486 — Features OpenType confirmadas (Sub-item A)

**P486** confirma via sonda do código-fonte de rustybuzz 0.20.1 (`ot_shape.rs:86-91`)
que `liga`, `kern` e `calt` estão em `HORIZONTAL_FEATURES` com flags `F_GLOBAL`/
`F_GLOBAL_HAS_FALLBACK` — activados por defeito para texto horizontal, independentemente
do parâmetro `user_features` passado a `rustybuzz::shape`.

```rust
// P486 — liga/kern/calt activados por defeito via HORIZONTAL_FEATURES
// (rustybuzz 0.20.1 ot_shape.rs:86-91). features = &[] é suficiente.
let output = rustybuzz::shape(&rb_face, &[], buffer);
```

Sub-item A é apenas documentação — nenhum código novo é necessário.

### Testes adicionados P486

- `p486_features_default_confirmado`: `features = &[]` (len=0) documenta invariante.
