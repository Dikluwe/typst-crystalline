# L0 — h space
Hash do Código: 95e7d349

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/h_space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/h_space.rs:22`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve absolute spacing against the active font size and advance cursor_x. Fractional spacing records the current line-item boundary and its weight, with matching natural width zero; `00_nucleo/prompts/compiler/layout/line_commit.md` owns allocation and expansion. Do not allocate vertical fractions or emit an item here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Avanço explícito e largura causal

A sonda spacing-only de logical-before-complete/results.json.gz conserva
espaço explícito sob escala absoluta. h_space.rs:27–30 só atualiza o cursor,
que não é a testemunha causal consumida por transform.rs. A troca para
esta testemunha precisa preservar a contribuição de h absoluto, inclusive
quando é o último item da linha. Registrar o extremo do cursor após o avanço
no acumulador causal existente, sem novo item, execução repetida ou estado.
Fracionários continuam alocados por line_commit, não por este módulo.
Refutar com texto seguido de h explícito sob escala absoluta e controles
sem transformação. É correção interna ADR-0127, sem contrato público novo.
