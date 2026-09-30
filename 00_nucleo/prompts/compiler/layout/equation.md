# L0 — equation
Hash do Código: 1eeb0cb4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/equation.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/equation.rs:158`, `01_core/src/compiler/layout/equation.rs:479`, `01_core/src/compiler/layout/equation.rs:871`, `01_core/src/compiler/layout/equation.rs:38`, `01_core/src/compiler/layout/equation.rs:658`, `01_core/src/compiler/layout/equation.rs:673`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own block/inline Equation dispatch, the current math fallback, math-to-FrameItem composition, Formula carrier, equation number placement and deferred equation fixups. Mathematical node layout and callback occurrence matching belong to their existing math owners, not to this prompt.

Install math=true, New Computer Modern Math family, weight 450 and Display/Text math-size according to blockness, inheriting the existing outer style fields. Keep the current lexical chain and supplied stable equation Location, effective region height and attempt-wide callback pass when constructing MathLayouter. A directly constructed Layouter retains its callback-free fallback pass. Do not load fonts, call Func, allocate a second equation Location or reconstruct math callback indexes here.

Lay out and measure the math body once. Use that same raw extent and returned items for composition. Block numbering may extend ascent to the active top edge; preserve collapse state across mathematical measurement. Inline equations stay on the text baseline, not displaced by the math axis. Resolve their paragraph contribution once as the current bound/text-edge ascent/descent versus raw extent minus 0.7 times effective paragraph leading, with nonnegative descent, then submit it to Cursor's canonical inline-extents aggregator. Do not rescan glyphs or lay out again to estimate line bounds.

Block equations close pending text, center within the available finite width (including a negative offset for an overwide equation), and preserve the current region-top suppression of above spacing and the ordinary block boundary/collapse protocol elsewhere. Default above/below is resolved from `00_nucleo/prompts/compiler/layout/vanilla_defaults.md`; track equation descent for subsequent blocks. Auto width defers centering/number placement to this owner's pending fixups rather than using infinite coordinates.

Translate supported math FrameItems into the actual equation baseline/origin, then wrap them in an identity Group whose inner width/height are the logical measured extent, inside a Formula Semantic envelope with Inline/Block placement and the supplied String alt or no alt. Keep prescripts/negative child coordinates and trailing logical extent: cursor advance is based on the full extent, not a sum of Rust glyph advances or a first-child bound. The envelope does not create layout twice or change visual order.

For an enabled block number, consume pre-realized equation-numbering Content from the introspector, with the current causal pattern/counter fallback and empty fallback; this is not layout-time function execution. Preserve the current plain-text width projection and marginal Text emission for numbering; rich function-produced numbering is not newly guaranteed by this cleanup. Resolve number-align's physical start/end with direction and top/bottom/horizon against the actual measured multi-line equation baselines/extent. Keep finite alignment and the existing number/gutter reservation interface with `00_nucleo/prompts/compiler/layout/page_geometry.md`. Deferred fixups apply after final page width is known, preserving indexes/reference frame and clearing their buffers once.

Bare supported Math variants use this math bridge without inventing a new native dispatcher or interpretation for Lr/Mid/Primes/Stretch/Underover. MathAlignPoint outside that structured context remains the explicit central no-op. Direct private HTML markers do not make a pending material line. Unmeasured layout/callback/rich-numbering combinations remain limitations, not success inferred from a non-empty frame.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/equation.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos. Em conteúdo externo de equação, deslocar carrier sem somar advance ao cursor.

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
