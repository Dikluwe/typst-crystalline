# Relatório P482 — Trilha 5 Fase 1: ShapedGlyph + shaper.rs + TextShaped

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P482 (Fase 1 shaping rustybuzz — ADR-0120 ACEITE)
**Materialização:** ShapedGlyph (L1) + FrameItem::TextShaped (L1) + shaper.rs (L3) + arms em ~21 sites + export arms

---

## 1. Resumo

Materialização completa da Fase 1 do épico de shaping rustybuzz (Trilha 5).

O shaper é implementado como post-processing pass L3 (`03_infra/src/shaper.rs`) que
converte `FrameItem::Text → FrameItem::TextShaped` após layout e antes de export.
A decisão arquitectural central (ADR-0120 Opção A1) é confirmada: `cursor.rs` (L1)
**não muda** — o shaper em L3 resolve a fonte a partir de `style.font` via
`world.book().select_pattern()`.

`FrameItem::Text` é preservado como fallback (fonte não carregada, Type1, style.font=None).
Paridade estrutural mantida: 73/73 matches, 0 diffs.

**ADR-0120**: transição PROPOSTO → **ACEITE** (P482 2026-06-27).

---

## 2. Decisão arquitectural confirmada

### 2.1 Sem `font_slot_idx` em `TextStyle`

P481 e a spec P482 mencionavam adicionar `pub font_slot_idx: Option<usize>` a `TextStyle`.
**Decisão tomada: NÃO adicionado.**

Razão: `font_slot_idx` é dado L3 (índice de slot no FontBook — específico à
implementação de world). Violaria pureza L1 (ADR-0029/ADR-0030). O shaper em L3
resolve o slot via `world.book().select_pattern(&family.name, &variant)` exactamente
como `resolve_font` já faz em `pipeline.rs`. Não há necessidade de propagar este
índice opaco para L1.

### 2.2 `glyph_id: u16` (não u32)

ADR-0120 propunha `glyph_id: u32` (rustybuzz nativo `GlyphInfo.glyph_id`).
Implementação usa `u16` (truncado com `as u16`) para compatibilidade com
`FrameItem::Glyph.glyph_id` e o path CIDFont existente (`<XXXX> Tj`).
Truncagem é safe para fontes com ≤ 65535 glifos (todos os TrueType normais).

### 2.3 Face index 0

`Font(Vec<u8>)` em L1 não tem `face_index()`. Fase 1 usa sempre `index=0`
— adequado para fontes TTF normais; TTC scope-out Fase 3.

---

## 3. Ficheiros alterados/criados

### L0 Prompts (criados / actualizados)

| Ficheiro | Acção |
|----------|-------|
| `00_nucleo/prompts/infra/shaper.md` | **Criado** — spec de shaper.rs |
| `00_nucleo/prompts/entities/shaped_glyph.md` | **Criado** — spec de ShapedGlyph |
| `00_nucleo/prompts/entities/layout_types.md` | Actualizado — §P482 ShapedGlyph + TextShaped |

### L1 — `01_core`

| Ficheiro | `@prompt-hash` pós-P482 | Alteração |
|----------|------------------------|-----------|
| `01_core/src/entities/layout_types.rs` | `b13692d6` | `ShapedGlyph` struct + `FrameItem::TextShaped` variant + `plain_text_items` arm |
| `01_core/src/engine/layout/helpers.rs` | (pré-existente) | `item_pos` + `translate_frame_item` — arm TextShaped |
| `01_core/src/engine/layout/slicing.rs` | (pré-existente) | `item_y_start` + `rebase_item_y` — arm TextShaped |
| `01_core/src/engine/layout/cursor.rs` | (pré-existente) | `flush_pending_floats`, `flush_pending_footnote_bodies`, `top_safe` calc, `tail_h` calc — arms TextShaped |
| `01_core/src/engine/math/layout/mod.rs` | (pré-existente) | `place`, `offset_item`, `hconcat` — arms TextShaped |
| `01_core/src/engine/layout/link.rs` | (pré-existente) | bbox calculation — arm TextShaped |
| `01_core/src/engine/layout/equation.rs` | (pré-existente) | math inline loop — arm TextShaped |
| `01_core/src/engine/math/layout/frac.rs` | (pré-existente) | num_box/den_box loops — match com TextShaped |

### L3 — `03_infra`

| Ficheiro | `@prompt-hash` pós-P482 | Alteração |
|----------|------------------------|-----------|
| `03_infra/src/shaper.rs` | `b342622c` | **Novo** — `shape_document`, `shape_page`, `shape_item`, `try_shape`, `resolve_slot`, `byte_idx_to_char` + 6 testes |
| `03_infra/src/lib.rs` | (sem hash) | `pub mod shaper;` adicionado |
| `03_infra/src/pipeline.rs` | (pré-existente) | `shape_document(world, doc)` call + `TextShaped` arms em `collect_fonts_in_items` + `first_font_in_items` |
| `03_infra/src/export/stream.rs` | (pré-existente) | `emit_shaped_pdf` helper + `TextShaped` arms em `build_page_stream` + `draw_item_local` |
| `03_infra/src/export/fonts.rs` | (pré-existente) | `collect_codepoints` + `collect_glyph_ids` — arms TextShaped |
| `03_infra/src/integration_tests.rs` | (test-only) | `frame_item_pos` — arm TextShaped |

### Lab/parity

| Ficheiro | Alteração |
|----------|-----------|
| `lab/parity/src/frame_dto.rs` | `classify_item` + `from_cristalino` — arm TextShaped (classificado como `ItemDTO::Text`) |
| `lab/parity/tests/structural_parity.rs` | Sentinela `p482_parity_73_73_mantido` adicionado |

### ADR

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md` | Status PROPOSTO → **ACEITE** (P482) |

---

## 4. Testes (6 novos)

### `03_infra/src/shaper.rs` (6 novos — todos verdes)

| Teste | Cobertura |
|-------|-----------|
| `p482_shape_document_preserves_text_sem_font` | `style.font = None` → Text preservado |
| `p482_shape_document_preserves_text_content` | conteúdo de Text preservado |
| `p482_byte_idx_to_char_ascii` | índices ASCII correctos |
| `p482_byte_idx_to_char_utf8` | multi-byte UTF-8 (é = 2 bytes) |
| `p482_shaped_glyph_clone_eq` | ShapedGlyph clone + PartialEq |
| `p482_shape_document_group_children_passthrough` | Text dentro de Group preservado sem font |

### Parity

| Teste | Resultado |
|-------|-----------|
| `p482_parity_73_73_mantido` | 73/73 matches, 0 diffs ✅ |

---

## 5. Medições ADR-0108 (medir antes de decidir)

| Decisão | Medição | `file:line` |
|---------|---------|-------------|
| Sem `font_slot_idx` em TextStyle | `resolve_font` em `pipeline.rs` já resolve slot via `select_pattern` | `pipeline.rs:248–261` |
| `glyph_id: u16` (não u32) | `FrameItem::Glyph.glyph_id: u16` — uniformidade | `layout_types.rs:253` |
| Face index 0 | `Font(Vec<u8>)` sem `face_index()` method | `world_types.rs:30–35` |
| `emit_shaped_pdf` tipo fallback para Type1 | `emit_text_pdf` usa `text` field — compatível | `stream.rs:121` |
| `classify_item` trata TextShaped como Text | paridade estrutural requer transparência | `frame_dto.rs:183` |

---

## 6. `crystalline-lint` resultados

```
crystalline-lint --fix-hashes .
  Fixed 2 files:
    ./01_core/src/entities/layout_types.rs  → b13692d6
    ./03_infra/src/shaper.rs                → b342622c
  Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
  ✅ 0 erros V1–V14.
  Warnings V7: prompts órfãos pré-existentes + shaped_glyph.md (doc-only, sem correspondência L1–L4 directa).
```

---

## 7. Build & tests

```
cargo build --workspace          → 0 erros
cargo test -p typst-infra        → 507 passed, 0 failed (inclui 6 testes p482)
structural_parity --test         → 13 passed, 0 failed (inclui p482_parity_73_73_mantido)
```

---

## 8. Scope-out explícito

| Área | Scope-out | Fase |
|------|-----------|------|
| RTL/bidi | `unicode-bidi` ausente; `guess_segment_properties()` basta para LTR | Fase 3 (P484) |
| OpenType features explícitas | `features = &[]` usa GSUB/GPOS padrão | Fase 2+ |
| Font fallback multi-família além da primeira | Fase 1 para simplificar | Fase 2 (P483) |
| Face index > 0 (TTC) | `Font` não expõe `face_index()` | Fase 3 |
| `FrameItem::Text` deprecated | preservado como fallback Fase 1 | Fase 2 (P483) |
| Advance em pt no emit | `x_advance` em unidades de fonte; export usa hex glyph IDs directamente | Fase 2 |

---

## 9. Critério de fecho

- [x] `ShapedGlyph` struct em `01_core/src/entities/layout_types.rs`.
- [x] `FrameItem::TextShaped` variant com `pos`, `glyphs`, `style`, `text`.
- [x] `plain_text_items` actualizado para extrair `text` de `TextShaped`.
- [x] ~21 sites de match exaustivo actualizados com arm `TextShaped`.
- [x] `03_infra/src/shaper.rs` — `shape_document` + helpers + 6 testes.
- [x] `pub mod shaper;` em `03_infra/src/lib.rs`.
- [x] `pipeline.rs` integra `shape_document` + arms `TextShaped`.
- [x] `export/stream.rs` — `emit_shaped_pdf` + arms `TextShaped`.
- [x] `export/fonts.rs` — arms `TextShaped` em `collect_codepoints` + `collect_glyph_ids`.
- [x] `lab/parity/src/frame_dto.rs` — `classify_item` + `from_cristalino` actualizados.
- [x] Sentinela `p482_parity_73_73_mantido` verde.
- [x] L0 prompts: `infra/shaper.md` (novo) + `entities/shaped_glyph.md` (novo) + `layout_types.md` (actualizado).
- [x] ADR-0120: PROPOSTO → **ACEITE**.
- [x] `cargo build --workspace` verde.
- [x] `crystalline-lint .` zero erros V1–V14.

---

## 10. Estado pós-P482

| Indicador | Estado |
|-----------|--------|
| DEBTs activos com critério de fecho | 0 |
| Trilhas completas | 1, 2, 3, 4, 7, 8 |
| Trilhas pendentes | 5 (Fase 1 ✅; Fase 2 P483; Fase 3 P484), 6 (4/5) |
| Paridade | **73/73 matches; 0 diffs; 0 errors** |
| ADR-0120 | **ACEITE** |
| **P482** | **FECHADO** |

---

## 11. Próximo passo recomendado

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P483** | Fase 2 shaping: migração completa `Text → TextShaped`; deprecar `FrameItem::Text` | M |
| **Trilha 6** | Fechar a 5ª funcionalidade pendente | Depende de conteúdo |
| **P484** | Fase 3 RTL básico via `unicode-bidi` | L |
