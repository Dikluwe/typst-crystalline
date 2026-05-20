# Prompt L0 — `infra/export/fonts` — Helpers de fontes e escape
Hash do Código: 51b65b81

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/fonts.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0027 (CIDFont + Identity-H)

---

## Contexto

Agregado de helpers para emit de texto PDF:
- `escape_pdf_string` — escape de caracteres reservados em string literals `()`.
- Cluster CIDFont: collect codepoints/glyph IDs do doc + map chars→glyphs + widths array + ToUnicode CMap + text→hex serialization.

## Restrições estruturais

- L3 puro. Lê doc (`PagedDocument`), `ttf-parser::Face`. Não toca FS nem rede.
- Tudo `pub(super)` — consumido por `super::builder` (CIDFont path) e `super::stream::emit_text_pdf` (escape).
- `escape_pdf_string` mapeia chars não-ASCII para `?` — Latin-1 only path (Helvetica). CIDFont path usa `text_to_hex_string` em vez.

## Interface

```rust
pub(super) fn escape_pdf_string(text: &str) -> String;
pub(super) fn collect_codepoints(doc: &PagedDocument) -> Vec<char>;
pub(super) fn collect_glyph_ids(doc: &PagedDocument) -> BTreeSet<u16>;
pub(super) fn map_chars_to_glyphs(face: &Face<'_>, chars: &[char]) -> Vec<(char, u16)>;
pub(super) fn widths_array(face: &Face<'_>, mappings: &[(char, u16)]) -> String;
pub(super) fn to_unicode_cmap(mappings: &[(char, u16)]) -> Vec<u8>;
pub(super) fn text_to_hex_string(text: &str, char_to_gid: &HashMap<char, u16>) -> String;
```

## Invariantes

- `collect_codepoints` e `collect_glyph_ids` atravessam `FrameItem::Group` recursivamente (P280 bug fix).
- `widths_array` usa `units_per_em` da face para converter glyph_advance → 1/1000 text space.
- `to_unicode_cmap` emite blocos de ≤100 entradas (PDF spec limit).
- `text_to_hex_string` mapeia chars sem glyph para `<0000>` (notdef).

## Critérios de verificação

- `escape_pdf_string("a(b)c")` → `"a\\(b\\)c"`.
- `to_unicode_cmap(&[])` produz CMap mínimo válido com `0 beginbfchar`.
- Tests P150+ + ToUnicode tests em `tests.rs` validam estrutura.
