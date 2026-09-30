# L0 — references
Hash do Código: 790cd40e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/references/unreferencable-label.toml sha256:5dde6e7f84150a121e671dab0d174798e8b56f24355147c1bcc471d6babd69ed

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/references.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/references.rs:77`, `01_core/src/compiler/layout/references.rs:283`, `01_core/src/compiler/layout/references.rs:30`, `01_core/src/compiler/layout/references.rs:54`, `01_core/src/compiler/layout/references.rs:249`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Label and Ref layout, not introspection stores or language tables. Label renders its body transparently, then records the current physical page and its captured point in runtime maps. Ref to an existing bibliography key delegates to `00_nucleo/prompts/compiler/layout/cite.md` with the supplement.

For ordinary references, preserve current validation precedence: unreferencable kind diagnostics before generic existence checks, then heading/equation numbering checks and numeric/legacy resolution. Emit the exact current errors and hints, with detached spans because the element lacks a span. Numeric counter-key/location formatting takes precedence; legacy figure/resolved-label data remains fallback. Missing labels are diagnosed, not silently changed back to a question mark. Preserve internal destination links and the current pending/emitted-item collection behavior.

An explicit supplement, including empty Content, overrides the implicit one. Equation defaults consume its causal realized supplement; figure defaults and legacy figure fallback delegate to the language owner. Heading defaults first extract the causal prefix from resolved label text only when it ends in ASCII space plus the same formatted number; otherwise use the language owner's deterministic current-style fallback. Do not add bilingual tables, kind-label catalogs or infer a target language that was not carried. A non-empty prefix joins number with U+00A0 once.

For Page form, preserve existing known-label/page lookup, missing-numbering diagnostic/hint, explicit-versus-PageStore supplement precedence and internal link. Function numbering consumes PageStore.reference_numbering without Func execution. The current Pattern branch still formats the physical page index locally; its conflict with the older shared unary logical-store promise is unresolved. A not-yet-known page retains the existing linked question-mark path rather than fabricated numbering. This cleanup does not authorize changes to numbering views or labels.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
