# L0 — place
Hash do Código: 002271a7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/place.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/place.rs:22`, `01_core/src/compiler/layout/place.rs:141`, `01_core/src/compiler/layout/place.rs:154`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the Place adapter plus float fitting, reservation and final emission. Merge missing alignment axes from the cell context. Non-float placement delegates geometry to `00_nucleo/prompts/compiler/layout/placement.md`; without explicit vertical alignment it follows the current ordinary flow origin, including pending-line semantic top edge, rather than always the physical page top. Apply that correction locally to this occurrence's items, decorations and vertical fixups; retain out-of-flow cursor behavior.

For a float, lay out/measure the body in the active isolated subframe, resolve clearance against effective font size and queue the existing DeferredFloat with dimensions, decoration and orphan alignment metadata. The language's default clearance arrives from its constructor as relative 1.5em; the adapter's absent-field fallback remains zero. The current DeferredFloat does not store dx/dy/scope, which are not applied by this float path: do not claim their support or broaden the private struct here.

Admit only an ordered fitting prefix using effective usable region and current material flow. Include adjacent clearance when flow or an earlier admitted float exists; retain the existing defensive admission of the first float on an empty region. Once realized, top/bottom reservation includes future-flow clearance. Anchor top floats at the top and accumulate downward, bottom floats at the physical bottom and accumulate upward; auto height retains its current top-placement fallback. Body, orphan paths and decoration share the final reference frame. Preserve existing top-reserve translations and bottom effective-region reduction, without applying clearance twice.

Ask `00_nucleo/prompts/compiler/layout/cursor.md` for region progress and `00_nucleo/prompts/compiler/layout/flow_suffix.md`/`00_nucleo/prompts/compiler/layout.md` for full suffix recomposition; do not emit in-flow at the cursor, inspect a future Content or duplicate replayed occurrences. Direct private HTML markers are non-material for fitting but remain geometrically transported. Oversize/unmeasured float combinations are not full parity guarantees.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/place.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos. Excluir separador da origem vertical medida.

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
