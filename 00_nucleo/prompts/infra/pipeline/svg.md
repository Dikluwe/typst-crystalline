# L0 — infra/pipeline/svg
Hash do Código: e6913466

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/svg.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
```

## Current evidence

- `03_infra/src/pipeline/svg.rs:12` — `compile_to_svg_string`.
- `03_infra/src/pipeline/svg.rs:21` — `compile_to_svg_string_full_error`.
- `03_infra/src/pipeline/svg.rs:33` — `compile_to_svg_string_with_timings_full_error`.
- `03_infra/src/pipeline/svg.rs:47` — `compile_to_svg_string_with_timings_full_error_and_features`.
- `03_infra/src/pipeline/svg.rs:59` — `compile_to_svg_string_with_timings_full_error_and_features_and_pretty`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/svg.rs`.

## Owned contract

- Own SVG facades, resolved pretty, selected-page destination context and glyph-font associations. Compile through the common paged path; export the first page with existing empty-page fallback.
- Resolve pretty as explicit caller then document.svg.pretty then its default. Decision-free wrappers pass None and boolean compatibility wrappers pass Some. The exporter receives only final SvgOptions; do not teach it CLI/document precedence.
- Derive destinations only from labels on the selected page with corresponding extracted positions. Do not invent missing positions or move maps into Page; cross-page unresolved links remain Unknown without dangling fragments. Exporter owns ID spelling.
- Associate each normalized GlyphFontRequest with the complete FontKey resolved through FallbackFontMetrics. Distinguish base character, requested family, variant, variations and relevant math state independently of slot/address/traversal occurrence. Conflicting results invalidate an association; missing entries remain Unknown, never default slot zero.
- Collect associations across background/body/foreground and Group/Link/Semantic. Without fonts do not fabricate associations, reread sources or repeat layout. Inline `html.frame` SVG uses the HTML owner's resolved option, not format.svg.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

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
