# L0 — tests
Hash do Código: ead66ab7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L1, test-only
**Exclusive consumer:** `01_core/src/compiler/layout/tests.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/tests.rs:31`, `01_core/src/compiler/layout/tests.rs:38`, `01_core/src/compiler/layout/tests.rs:48`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own only the existing test-only worlds/metrics, affine observation helpers and layout regression suite. Preserve assertions for rendered text/morphology, page/position/style, diagnostic behavior and architectural delegation. Geometric observers compose ancestor affine transforms, Group position/matrix and nested wrappers once into global coordinates; Semantic is transparent and Link follows its own convention. Variant-only tests need not project geometry.

Keep the absolute Block fixtures lifted into the current relative carrier without changing their values/tolerances. Preserve full-region Block followed by text, terminal-no-empty-page and relative/fractional sizing controls. Preserve the independent float clearance-anchor versus reserve/fitting controls, the atomic post-marker suffix including Place and later floats, and controls without marker/float or with explicit block spacing; do not weaken them to page totals or inspect private checkpoint flags as a substitute for observable position.

Retain mixed-size/no-wrap and certified one-wrap fractional-prefix controls, both orders across the outer size, equal/unequal size cases, and opaque compound-style/multiword negative controls. Keep cases alignment points reaching layout, without suppressing panic or deleting align markers to make a test green. Preserve the structural regression for thin static descendant dispatch arms and the exhaustive match; it checks architecture, not claimed performance.

This document neither authorizes production changes nor requires tests that are absent to be declared implemented. Existing historical probe coordinates and executions are archived in the diagnostic; future quantitative parity/performance claims need exact source/binary provenance, not aggregate test status.

## Acceptance and limits

Preserve the existing tests and their observable assertions. This documentation operation does not modify fixtures, tolerances or Rust code.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/tests.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

A projeção geométrica dos testes trata TextSeparator como folha posicionada; o carrier não cria tinta nem altera a semântica dos outros refutadores.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.
