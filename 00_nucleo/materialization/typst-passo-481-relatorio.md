# Relatório P481 — Sonda profunda Trilha 5: arquitectura de shaping rustybuzz

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P481 (Sonda arquitectural + ADR-0120)
**Materialização:** Sonda completa (grupos 1–4) + ADR-0120 PROPOSTO. Zero código de produção.

---

## 1. Resumo

Sonda arquitectural completa para Trilha 5 (shaping rustybuzz). Todos os 4 grupos de sondas executados com `file:line`. ADR-0120 redigida em estado PROPOSTO.

**Descoberta crítica**: o shaping não pode acontecer em `cursor.rs` (L1) — bytes de fonte vivem em L3. A arquitectura correcta é um **post-processing shaping pass em `03_infra/src/shaper.rs`** (L3), invocado pelo pipeline após layout e antes do export.

**Magnitude corrigida**: L-XL (8–10h total); Fase 1 (P482) é L (~3–4h), não XL como estimado inicialmente.

---

## 2. Grupo 1 — Write sites de `FrameItem::Text` (ADR-0108)

### 2.1 Resultado da sonda

```
grep -rn "FrameItem::Text {" 01_core/ 03_infra/
```

| Site | `file:line` | Tipo |
|------|-------------|------|
| `cursor.rs` | `01_core/src/engine/layout/cursor.rs:86` | **Path principal** — `layout_word` emit |
| `equation.rs` | `01_core/src/engine/layout/equation.rs:75` | Wrap de math item em texto inline |
| `equation.rs` | `01_core/src/engine/layout/equation.rs:149` | Texto standalone equação |
| `list_item.rs` | `01_core/src/engine/layout/list_item.rs:25` | Bullet de lista |
| `enum_item.rs` | `01_core/src/engine/layout/enum_item.rs:33` | Numeração de lista numerada |
| `link.rs` | `01_core/src/engine/layout/link.rs:123` | Texto de link |
| `math/layout/mod.rs` | `01_core/src/engine/math/layout/mod.rs:535` | Shaping matemático |
| `pipeline.rs` | `03_infra/src/pipeline.rs:345` | Construção fixture integração |

**Total: 7 write sites L1 (produção) + 1 L3.**

### 2.2 `cursor.rs:86` — path principal

```rust
self.regions.current.current_line.push(FrameItem::Text {
    pos: Point { x: self.regions.current.cursor_x, y: self.baseline_y() },
    text,
    style: self.style.clone(),
});
```

Este é o único site que emite texto corrente. Todos os outros são casos especiais (math, listas, links). O shaper L3 converterá exactamente estes `FrameItem::Text` em `FrameItem::TextShaped`.

---

## 3. Grupo 2 — Read sites de `FrameItem::Text` (ADR-0108)

### 3.1 Sites de produção (L1)

| Ficheiro | Lines | Operação |
|----------|-------|----------|
| `entities/layout_types.rs` | 433 | `plain_text_items` — concatenação de texto |
| `engine/layout/cursor.rs` | 179, 312, 459 | extracção de style / rebase de posição |
| `rules/math/layout/mod.rs` | 64, 91, 672 | mutação de pos / transformação de item |
| `engine/layout/slicing.rs` | 78, 129, 168, 171, 210, 256 | fatiamento e rebase |
| `engine/layout/link.rs` | 66 | inclusão de texto em link |
| `engine/layout/equation.rs` | 69 | iteração de equação |
| `engine/layout/helpers.rs` | 33 | update de posição |
| `rules/math/layout/frac.rs` | 57, 74 | mutação de pos (fracções) |

### 3.2 Sites de produção (L3)

| Ficheiro | Lines | Operação |
|----------|-------|----------|
| `export/stream.rs` | 264, 675 | **Path principal PDF emit** |
| `export/fonts.rs` | 49 | rastreamento de chars usados para glyph map |
| `pipeline.rs` | 157, 213 | extracção de estilo em formatação |

### 3.3 Lab/parity

| Ficheiro | Lines | Operação |
|----------|-------|----------|
| `lab/parity/src/frame_dto.rs` | 85, 183 | DTO para comparação parity |

### 3.4 Impacto de compilação ao adicionar `FrameItem::TextShaped`

Adicionar nova variante a enum causa erro Rust em `match` exhaustivos sem `TextShaped` arm. Sites de match exhaustivo que necessitam de arm adicional:

| Ficheiro | Sites | Acção em P482 |
|----------|-------|---------------|
| `export/stream.rs:264, 675` | 2 | Arm `TextShaped` — emit glyphs CID |
| `export/fonts.rs:49` | 1 | Arm `TextShaped` — rastrear glyph_ids |
| `pipeline.rs:157, 213` | 2 | Arm `TextShaped` — extracção de estilo |
| `cursor.rs:312, 459` | 2 | `TextShaped { .. } => unreachable!()` |
| `math/layout/mod.rs:64, 91` | 2 | Arm `TextShaped` — pos mutation |
| `slicing.rs:78, 129` | 2 | Arm `TextShaped` — rebase |
| `helpers.rs:33` | 1 | Arm `TextShaped` |
| `link.rs:66` | 1 | Arm `TextShaped` |
| `equation.rs:69` | 1 | Arm `TextShaped` |
| `layout_types.rs:430` | 1 | `plain_text_items` — usar `text` de TextShaped |
| `math/layout/frac.rs:57, 74` | 2 | Arm `TextShaped` — pos mutation |
| `frame_dto.rs:85, 183` | 2 | Arm `TextShaped` — DTO |

**Total: ~19 ficheiros com arms novos necessários.** Tests: ~120 sites — maioria usa `if let FrameItem::Text { .. }` ou `matches!()` — não causam erro de compilação.

---

## 4. Grupo 3 — rustybuzz integration points

### 4.1 Versão e disponibilidade

| Item | Resultado | `file:line` |
|------|-----------|-------------|
| `rustybuzz` versão | `"0.20"` | `Cargo.toml:25` |
| `ttf-parser` versão | `"0.25"` | `Cargo.toml:24` |
| Compatibilidade | ✓ rustybuzz 0.20 usa ttf-parser 0.25 internamente | doc rustybuzz |
| Uso activo cristalino | **Nenhum** — só declarado em `03_infra/Cargo.toml:26` | grep confirmado |

### 4.2 API rustybuzz (confirmada via `lab/krilla-reference/crates/krilla/src/text/shape.rs`)

```rust
// Referência: lab/krilla-reference/crates/krilla/src/text/shape.rs:1-87

// 1. Criar face dos bytes
let mut rb_font = rustybuzz::Face::from_slice(font_data, face_index).unwrap();

// 2. Criar buffer e carregar texto
let mut buffer = UnicodeBuffer::new();
buffer.push_str(text);
buffer.guess_segment_properties(); // detecta LTR/RTL, script, language

// 3. Shape
let output = rustybuzz::shape(&rb_font, &[], buffer); // features=[] usa padrão GSUB/GPOS

// 4. Iterar resultados
let positions = output.glyph_positions();  // [GlyphPosition] — x_advance: i32, x_offset: i32, y_offset: i32
let infos = output.glyph_infos();          // [GlyphInfo] — glyph_id: u32, cluster: u32
```

### 4.3 Acesso aos bytes de fonte

- `Font::as_slice() -> &[u8]` — disponível em `01_core/src/entities/world_types.rs:34`.
- `FontSlot.get() -> Option<Font>` — em `03_infra/src/fonts.rs:36` — lê do disco em L3.
- `TextStyle.font: Option<FontList>` — lista lógica (nome/weight/style), não bytes.

**Bloqueador identificado**: `cursor.rs` (L1) não tem como resolver `FontList` → bytes. O shaper (L3) recebe `PagedDocument` pós-layout e usa `World::font()` para resolver a fonte por slot index. Requer que `TextStyle` transporte `font_slot_idx: Option<usize>` para o shaper saber qual slot usar (Sub-opção i da ADR-0120).

### 4.4 `font_slot_idx` em `TextStyle`

`TextStyle` em `01_core/src/entities/layout_types.rs:123` tem campo `font: Option<FontList>`. Para o shaper, precisamos de `font_slot_idx: Option<usize>` — preenchido em L3 no momento do layout (o `Layouter` em L3 sabe qual slot da `FontBook` está activo).

Esta é uma mudança L1 limpa (campo `usize` opaco, sem I/O — ADR-0029/ADR-0030 cumpridas). Decidida em P482.

---

## 5. Grupo 4 — impacto `unicode-bidi` (RTL)

| Item | Resultado | Fonte |
|------|-----------|-------|
| `unicode-bidi` em workspace | **Ausente** | `grep -rn "unicode.bidi" Cargo.toml` |
| `Dir::RTL` em `cursor.rs` | **Ausente** | grep confirmado |
| `bidi` em `layout/mod.rs` | **Ausente** | grep confirmado |
| `guess_segment_properties()` | Detecta LTR/RTL via Unicode properties Unicode Bidirectional Algorithm | rustybuzz API |
| Impacto RTL em Fase 1 | Nulo — `guess_segment_properties()` basta para LTR correcto | |
| Scope RTL | Fase 3 genuína | ADR-0120 scope-out |

`unicode-bidi` existe apenas em `lab/typst-original/crates/typst-layout/Cargo.toml` — fora do workspace cristalino. RTL é genuinamente Fase 3.

---

## 6. ADR-0120 — `FrameItem::TextShaped` e pipeline rustybuzz

ADR redigida em `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md`.

**Decisão principal**: Opção A1 — post-processing shaping pass em L3 (`shaper.rs`), não integrado em `cursor.rs` (L1). `cursor.rs` não muda a sua lógica.

**Pipeline proposto**:
```
cursor.rs (L1) → FrameItem::Text [como hoje]
    ↓ pipeline.rs L3
shaper.rs (L3) → FrameItem::Text → FrameItem::TextShaped [rustybuzz]
    ↓
export/stream.rs (L3) → emit glyphs CID [TextShaped] | fallback string [Text]
```

**ShapedGlyph** proposta:
```rust
pub struct ShapedGlyph {
    pub glyph_id:  u32,   // GlyphInfo.glyph_id (rustybuzz)
    pub x_advance: i32,   // GlyphPosition.x_advance (unidades fonte)
    pub x_offset:  i32,   // GlyphPosition.x_offset
    pub y_offset:  i32,   // GlyphPosition.y_offset
    pub cluster:   u32,   // GlyphInfo.cluster (byte index → ToUnicode CMap)
    pub char_code: char,  // codepoint derivado de cluster
}
```

---

## 7. Magnitude corrigida do épico

| Fase | P482 estimado | Magnitude | Horas |
|------|---------------|-----------|-------|
| P482 — Fase 1: TextShaped + shaper L3 + export arm | Fase 1 | **L** | 3–4h |
| P483 — Fase 2: migração completa + deprecação Text | Fase 2 | M | 1.5–2h |
| P484 — Fase 3: RTL básico via unicode-bidi | Fase 3 | L | 3–4h |
| **Total** | P482–P484 | **L-XL** | **~8–10h** |

Spec P481 estimou XL (8–12h). Estimativa mantém-se na gama; Fase 1 é mais contida que esperado porque `cursor.rs` não muda.

---

## 8. Plano de passos P482–P484

### P482 — Fase 1 shaping (L, ~3–4h)

1. `ShapedGlyph` struct em `01_core/src/entities/layout_types.rs`.
2. `FrameItem::TextShaped` variant em `FrameItem` enum.
3. Arm `TextShaped` nos ~19 sites de match (cursor=unreachable; outros=passthrough ou funcional).
4. `TextStyle.font_slot_idx: Option<usize>` — campo opaco preenchido em L3 no layout.
5. `03_infra/src/shaper.rs` — função `shape_document(world, doc) -> PagedDocument`.
6. `pipeline.rs` integra `shaper::shape_document` entre layout e export.
7. `export/stream.rs` arm `TextShaped` — emit glyphs CID com x_advance em pt.
8. `export/fonts.rs` arm `TextShaped` — rastrear glyph_ids.
9. ≥3 testes unitários em `shaper.rs` + ≥1 integração.
10. `crystalline-lint .` zero violations.

### P483 — Fase 2 migração (M, ~1.5–2h)

1. Todos os emit sites de `FrameItem::Text` preenchem `font_slot_idx`.
2. `export/stream.rs` path principal = TextShaped; Text=deprecated.
3. `FrameItem::Text` marcado `#[deprecated]`.
4. Parity: 73/73 matches mantidos.

### P484 — Fase 3 RTL básico (L, ~3–4h)

1. `unicode-bidi` adicionado ao workspace.
2. Re-order de runs bidirectionais antes do shaping.
3. Testes com corpus árabe/hebraico.
4. `FrameItem::Text` removido (eliminar variant).

---

## 9. Critério de fecho P481

- [x] Grupo 1: todos os write sites de `FrameItem::Text` localizados com `file:line` (8 sites).
- [x] Grupo 2: todos os read sites localizados com `file:line`; ~19 sites exaustivos + ~120 tests.
- [x] Grupo 3: rustybuzz 0.20 confirmado; `Font::as_slice()` disponível; API `shape()` confirmada via krilla-reference.
- [x] Grupo 4: `unicode-bidi` ausente; RTL scope-out Fase 3 confirmado.
- [x] Bloqueador descoberto e documentado: shaping não pode ser em L1; A1 (post-processing L3) é a solução.
- [x] ADR-0120 redigida (PROPOSTO) em `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md`.
- [x] Magnitude corrigida documentada (L-XL; Fase 1 = L).
- [x] Plano P482–P484 esboçado.
- [x] Zero código de produção produzido.
- [x] `crystalline-lint` não requerido (sem modificações de código).

---

## 10. Estado pós-P481

| Indicador | Estado |
|-----------|--------|
| DEBTs activos com critério de fecho | 0 |
| Trilhas completas | 1, 2, 3, 4, 7, 8 |
| Trilhas pendentes | 5 (P482 é próximo), 6 (4/5) |
| Paridade | 73/73 matches (não afectada — zero código produção) |
| ADR-0120 | **PROPOSTO** — aguarda P482 |
| **P481** | **FECHADO** |

---

## 11. Próximo passo recomendado

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P482** | Fase 1 shaping: `ShapedGlyph` + `FrameItem::TextShaped` + `shaper.rs` L3 + export arm. A ADR-0120 já está escrita. | L |
| **P483** | Fase 2: migração completa `Text → TextShaped`. Só após P482 estabilizado. | M |
| **Trilha 6** | Fechar a 5ª funcionalidade pendente. | Depende de conteúdo. |
