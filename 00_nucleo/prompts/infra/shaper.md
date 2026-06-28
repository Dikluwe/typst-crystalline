---
prompt: infra/shaper
layer: L3
created: 2026-06-27
passo: P482
adr: ADR-0120
---

# Prompt L0 — `shaper.rs` (Trilha 5 Fase 1)
Hash do Código: afeb3b10

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
