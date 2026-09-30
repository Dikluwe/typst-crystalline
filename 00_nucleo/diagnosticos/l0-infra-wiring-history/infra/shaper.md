# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `0d5d0e5543172996b62caefbcb7748cb1d7b02547893970958d10f45fc7090ba`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: ff4d04c9
prompt: infra/shaper
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

# Prompt L0 — infra/shaper

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:53-68, 2417-2423` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Fachada com módulos privados e reexports. Preservar os caminhos shape_document e fix_line_positions públicos e FaceCache, CachedFace e shaped_width internos ao crate. Não conter seleção, shaping ou reconciliação. Os contratos históricos abaixo são da fachada; implementações pertencem exclusivamente aos owners descendentes.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.

## Owners descendentes

- `03_infra/src/shaper/traversal.rs`: `00_nucleo/prompts/infra/shaper/traversal.md`.
- `03_infra/src/shaper/bidi.rs`: `00_nucleo/prompts/infra/shaper/bidi.md`.
- `03_infra/src/shaper/font_selection.rs`: `00_nucleo/prompts/infra/shaper/font_selection.md`.
- `03_infra/src/shaper/shaping.rs`: `00_nucleo/prompts/infra/shaper/shaping.md`.
- `03_infra/src/shaper/cache.rs`: `00_nucleo/prompts/infra/shaper/cache.md`.
- `03_infra/src/shaper/reconciliation.rs`: `00_nucleo/prompts/infra/shaper/reconciliation.md`.
- `03_infra/src/shaper/tests.rs`: `00_nucleo/prompts/infra/shaper/tests.md`.

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


## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-06-27 | Criação — shaping básico com rustybuzz | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-06-28 | P483 — fonte padrão + cobertura ≥95% | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-06-28 | P484 — RTL básico via unicode-bidi | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-06-30 | P515 — fallback por caractere | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-01 | P525 — variation fonts MVP | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-02 | P534 — fallback por script/cobertura | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-03 | P555 — fallback preserva classe serif/sans | `shaper.md`, `03_infra/src/shaper.rs`, `03_infra/src/fallback_fonts.rs` |
| 2026-07-05 | P568 — preservar espaços como `FrameItem::Text` | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-06 | P582 — redistribuição de posições x | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-08 | P621 — tracking nos `x_advance` | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-14 | P772o — variação de eixo no shaping | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-22 | P838 — fallback global com scoring de similaridade | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-07-23 | P875 — filtro de fallback por cobertura Unicode | `shaper.md`, `03_infra/src/shaper.rs` |
| 2026-08-26 | P1197 — catálogo de fallback individualizado; shaper retém seleção dinâmica | `shaper.md`, `03_infra/src/shaper.rs` |
