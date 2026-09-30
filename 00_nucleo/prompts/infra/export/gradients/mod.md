# L0 — infra/export/gradients/mod
Hash do Código: 42947746

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/mod.rs:54` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Gradient discovery façade

Own PatternRef, GradientObjectKind/GradientObject, RectKey/DedupKey, bbox/key helpers,
scan_all_gradients, pattern_resources_for_page and restricted child reexports.
Keep private static child modules; neither formatting functions nor sampling/
mesh algorithms are duplicated here.

Traverse page background, body and foreground recursively through Group/Link/
Semantic. Register Shape.fill and Shape.stroke Gradient paints in encounter order
with the same effective key; reuse matching fill/stroke identity once.
DedupKey combines gradient Arc pointer with effective bbox quantized by the existing
RectKey representation. Group passes its local bbox context; the shape's own
parent_bbox_at_emit wins over the inherited override.

Spot gradients additionally retain the painted shape rect for placement and lookup,
separate from a container's relative-parent bbox. The same spot gradient reused
at different shape locations must retain those placements; process keys preserve
their existing policy. Keep sequential allocated pattern/shading/function IDs and
recursive page resource lookup synchronized with discovery, without collisions
or unresolved references. Do not include nonvisual alt as paint.

## Delegated owners

- `00_nucleo/prompts/infra/export/gradients/linear.md`: axial coordinates/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/radial.md`: focal circles/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/cmyk.md`: CMYK sampling/conversion.
- `00_nucleo/prompts/infra/export/gradients/relative.md`: relative tuple transforms.
- `00_nucleo/prompts/infra/export/gradients/adaptive.md`: PDF counts/SVG adaptive stops.
- `00_nucleo/prompts/infra/export/gradients/conic.md`: Type 6 mesh payloads.
- `00_nucleo/prompts/infra/export/gradients/function_dict.md`: interpolation functions.

`00_nucleo/prompts/infra/export/builder/resources.md` owns emitted resource objects
and shading placement; `00_nucleo/prompts/infra/export/stream/shape.md` owns paint
operators. This façade performs no I/O or entity interpolation.

## Verification

Check recursive three-layer discovery, fill/stroke reuse, distinct effective boxes,
spot placements and page-local reference closure. Pointer/key mechanics preserve
the current implementation; they do not define language equality.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/gradients/mod.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator não possui paint nem recolhe gradientes.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
