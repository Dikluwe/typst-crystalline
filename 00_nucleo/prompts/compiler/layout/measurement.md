# L0 — measurement
Hash do Código: b1365934

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/measurement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/measurement.rs:27`, `01_core/src/compiler/layout/measurement.rs:64`, `01_core/src/compiler/layout/measurement.rs:354`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own constrained Content/Stack measurement and isolated intrinsic measurement with causal width, height and baseline. Constrained dispatch handles the current text, sequences, shapes/curves, spaces, containers, columns, alignments and lexical-style cases using injected metrics; unsupported cases retain their current fallback. Relative Block dimensions reuse `00_nucleo/prompts/compiler/layout/block.md` resolution against the outer available region. Do not perform an emission walk merely to allocate fractional siblings.

Preserve the public `measure_content_real` pair and internal baseline-bearing companion: build an isolated zero-margin Layouter/subframe with the supplied chain, metrics and sizer, lay out once, and derive dimensions from the produced items/cursor/inline extents. Baseline is causal composition, including boxes/equations and inset, not a plain-text-classification shortcut or unconditional height baseline. A private extent marker, where the source uses one, preserves that measured extent without a new public Content variant. Restore lexical measurement style. Do not mutate the emitting Layouter's locator, commit its pages, iterate document fixpoint or execute callbacks.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Largura intrínseca de texto em coluna Auto — 2026-10-03

### Medição anterior à decisão

O controle bilateral pinado em `00_nucleo/diagnosticos/grid-vanilla-phase-2026-10-03.md` mediu quatro colunas Auto com desvios acumulados de 0, 2,5, 5,0 e 7,5 pt nos inícios das células. Com colunas fixas, os 320 inícios coincidiram em x dentro de 0,001 pt. A sonda de linguagem mediu 2,5 pt para o espaço sob Libertinus Serif 10pt nos dois motores. Em `01_core/src/compiler/layout/measurement.rs:70-96`, a largura de `Content::Text` soma um espaço após cada palavra, inclusive a última; `01_core/src/compiler/layout/grid/tracks.rs:79` usa essa largura em Auto. A fonte vanilla pinada mede o frame da célula em `lab/typst-original/crates/typst-layout/src/grid/layouter.rs:967-1100`. A atribuição causal ainda exige ablação e controle RED→GREEN; a observação não prova equivalência para células complexas.

### Decisão e aceitação

Na medição restrita de texto, contar o espaço entre palavras consecutivas, sem adicionar um espaço artificial após a última palavra de cada linha. A seleção de quebra deve comparar a largura da próxima palavra mais o separador quando a linha já contém palavra. Preservar a restrição de largura, a altura por número de linhas e os outros braços do dispatcher. Exigir regressões para uma palavra, várias palavras e quebra; comparar a geometria Auto antes/depois com o vanilla pinado, mantendo coluna fixa como controle. Esta é correção interna de paridade, sem mudança de contrato público, comportamento padrão ou fase do pipeline (ADR-0127).
