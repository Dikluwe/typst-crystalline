# L0 — helpers
Hash do Código: 536df7e5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/helpers.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/helpers.rs:18`, `01_core/src/compiler/layout/helpers.rs:54`, `01_core/src/compiler/layout/helpers.rs:94`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the existing pure FrameItem position/extent/translation/offset/path-slice helpers, the narrow legacy shape/sequence measurement fallback, heading-scale mapping and dimension projection. For a Semantic wrapper, horizontal position is the minimum non-marker child x and width is rightmost child end minus that same left edge; empty/nonvisual extent is zero. Keep existing vertical conventions and Group/Link behavior, without child reorder or automatic render semantics.

Exclude direct private context markers from visual right/bottom extents but preserve their point under all translation/offset helpers. Descendant path-slice traversal supports Group, Link and Semantic, rejects invalid/overflowing paths, and does not create geometry. Heading scales remain level one 1.4, level two 1.2, other levels 1.0.

Retain the source's existing translation/projection conventions rather than claiming fully general affine bounds or clipping: Semantic translation uses its first-child anchor; item_bottom_y retains its descender approximation; Image reconstruction preserves and translates clip_rect as specified below. Those limitations require a separate behavior decision, not silent cleanup repairs.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/helpers.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos. Usar advance somente no transporte horizontal; excluir separadores dos limites visuais direito/inferior.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/p1431_line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `00_nucleo/diagnosticos/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

Image translation preserves the clip rectangle and moves its x/y by the same delta as the image position, retaining width/height. Reconstruction must not discard cover clipping.

The dedicated cover_clip_tracks_every_translation_without_changing_extent refutator also fails after reconstruction because offset_frame_item and shift_frame_item_x/y move only the image, leaving its clip behind (`helpers.rs:285`, `helpers.rs:315`, `helpers.rs:431`). All three helpers must transport the clip by the same respective delta, including recursive Semantic/Link paths; extent remains invariant. RED is recorded in the correction diagnostic.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Posição, largura, altura, shifts, offset e translate de TextReplacement descendem nos filhos; reconstrução conserva text. Testar translação sem mudança de identidade e visitor de recurso versus extração.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
