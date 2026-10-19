# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper/tests.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `b4d63473221aef1c65850f1b5729f83879c9b6d1e119f4b4b7e6a8450bbd5fac`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 2d5c64ad
prompt: infra/shaper/tests
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

# Prompt L0 — infra/shaper/tests

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:1128-2399` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Possuir a suíte unitária movida integralmente, mantendo nomes, fixtures, fontes reais e mocks, e acesso restrito aos helpers do módulo pai. Os testes cobrem travessia, bidi, fallback, variações, tracking, matemática e reconciliação; não são lógica produtiva nem uma nova API.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.

## Testes (≥6)

- `p482_shape_document_preserves_text_sem_font`: sem `style.font` → Text preservado.
- `p482_shape_document_preserves_text_content`: conteúdo do Text preservado.
- `p482_byte_idx_to_char_ascii`: índices ASCII correctos.
- `p482_byte_idx_to_char_utf8`: multi-byte UTF-8 (é = 2 bytes).
- `p482_shaped_glyph_clone_eq`: ShapedGlyph clone+eq.
- `p482_shape_document_group_children_passthrough`: Text dentro de Group preservado sem font.
