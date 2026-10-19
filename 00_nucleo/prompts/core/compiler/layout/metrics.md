# L0 — metrics
Hash do Código: ad30432b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/metrics.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/metrics.rs:22`, `01_core/src/compiler/layout/metrics.rs:27`, `01_core/src/compiler/layout/metrics.rs:37`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Historical separation evidence: cursor measures/selects word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source→PDF observations and isolated pre-export measurements are archived in `lab/results/construction/diagnostics/auditoria-quebra-paginacao-2026-09-28/`. They expose limitations, but separation preserves those exact decisions. Initial code/L0/provenance are in `lab/results/construction/diagnostics/separacao-texto-linhas-regioes-2026-09-28/`.

Bilateral probes and exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and `before/` measure U+002D versus SHY with coincident geometry in eight controls. Inspection of this consumer's FrameItem traversal is in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the public injected FontMetrics contract and its current defaults: text advance, vertical metrics/cap/text edges, ink, shaping width, math constants, glyph variants/assembly/kern and accent/correction access. Preserve Send + Sync and the existing borrowed-trait forwarding methods without adding public methods or stronger font guarantees.

The default text-width formula belongs exclusively to `00_nucleo/prompts/core/compiler/layout/text_measure.md`; this owner preserves its delegation through the public injected method. Preserve the existing complex-script predicate. FixedMetrics remains a deterministic approximation (advance 0.6 per character at size, vertical factors 0.8/1.2, default cap edge 0.7; current x-height/ascender/descender/explicit edge handling), not production font parity. line_content_right preserves its current item projections and ignores direct private HTML markers. Shared frame-helper extents belong to `00_nucleo/prompts/core/compiler/layout/helpers.md`; do not replace this trait method's own existing projection by prose.

TextSeparator does not contribute to the visual right edge; its position/advance do not alter ink metrics. Keep the public FontMetrics API and every override/borrowed forwarding method unchanged. Default text_width delegates to the private text_measure formula and supports unsized implementers. Explicit overrides remain authoritative for word/hyphen-prefix measurement; callers must not bypass them by directly invoking the default formula.

Treat TextReplacement as a transparent visual envelope in this owner's traversals; descend into children, preserving text, order, coordinates and existing calculation, without a new algorithm or advance. The text-replacement nucleus shares invariants only; this prompt owns only its declared consumer.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve existing responsibilities, observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen control, never PDF bytes or a new selection policy.
