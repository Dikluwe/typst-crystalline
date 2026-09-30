# L0 — sub frame
Hash do Código: d4a50d31

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/sub_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/sub_frame.rs:29`, `01_core/src/compiler/layout/sub_frame.rs:65`, `01_core/src/compiler/layout/sub_frame.rs:329`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

The isolated path temporarily installs a SubLayoutRegion and preserves/restores the existing outer cursor, dimensions, line/emitted buffers, page margins, pending baseline/collapse metrics, subframe/unconstrained flags, collectors and validated-prefix one-shot arm in LIFO order. Clear the outer arm locally so a body flush cannot consume it. Return the local items/height/decoration segments plus orphaned horizontal/vertical alignment paths without applying a second parent origin.

The inline path dispatches the body and drains only its pending-line tail; optionally align that tail while preserving the parent's prefix. It is not an all-state isolated path: callers such as Box own additional fraction/repeat isolation. Preserve the existing inline assertion that origin_x and width are zero and the present height fallback for material non-text items.

Direct private HTML markers are retained and returned in order but do not contribute visual had-items, fallback height or decoration metrics. Existing Semantic/Link/Group wrappers retain their old material classification. This module neither extracts ids nor removes carriers; `00_nucleo/prompts/compiler/layout/html_frame.md` owns both. No independent page dispatcher or checkpoint is introduced.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/p1431_line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `00_nucleo/diagnosticos/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

Provide a private geometric-origin path for root Place without a cell anchor or material pending line. It shares the isolated save/restore protocol, starts at y=0 with the initial text baseline pending, and defers typography to actual text. A first non-text block consumes this pending origin geometrically without font metrics. For a pending text tail, measure its height from the geometric top to the final text bottom, without subtracting the first top edge twice. Keep the existing baseline-origin path for all other callers and nested/cell placements.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

No percurso isolado, salvar/clonar State de tracking junto ao buffer exterior e restaurá-lo quando restaurar a linha. O alinhamento da cauda inline conserva a proveniência do body já emitido; não a substituir pela do prefixo durante a troca temporária usada para alinhar.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.
