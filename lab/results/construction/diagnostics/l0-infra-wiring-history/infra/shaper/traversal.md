# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper/traversal.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `7dd7eb109f9975d9614e9f286e4658d2af3bdf9b96979dcaaa58f9b9bac44969`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 220c26b6
prompt: infra/shaper/traversal
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

# Prompt L0 — infra/shaper/traversal

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:53-68, 306-373` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Percorrer background, items e foreground nesta ordem, incluindo Group, Link e Semantic. Manter o match exaustivo estático e os nós não textuais. Criar ambos os caches por documento; delegar apenas o texto resolvido a shaping::try_shape. Preservar texto sem fonte, falhas e espaços.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.

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


## P1140.5-A — shaping dentro de grupo semântico

`shape_document` recursa por `FrameItem::Semantic.items`, preserva
kind/placement/alt sem alteração e substitui apenas os filhos textuais como já
faz em Group/Link. O wrapper não muda posições nem seleção de fontes.


## P1160 — shaping das três camadas da página

`shape_document` aplica o mesmo `shape_item` a `background`, `items` e
`foreground`. Running matter vive em `foreground`; deixá-lo como `Text` num
documento cujo body selecionou Cidfont perde o texto no artefacto PDF, embora
o `PagedDocument` pré-shaping esteja correto. A ordem das camadas e dos items
dentro de cada camada é preservada.
