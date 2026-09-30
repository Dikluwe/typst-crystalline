# L0 — parbreak
Hash do Código: f8fbb508

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/parbreak.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/parbreak.rs:15`, `01_core/src/compiler/layout/parbreak.rs:40`, `01_core/src/compiler/layout/parbreak.rs:92`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

For a material pending line, insert the transparent ParbreakBoundary before closing the line. Resolve `par.spacing` and `par.leading` from the active chain/style, falling back through `00_nucleo/prompts/compiler/layout/vanilla_defaults.md`; apply `max(spacing - leading, 0)` and the existing pending-block-margin collapse. Preserve `prev_block_below_pending`, `block_chain_active` and `prev_margin_is_parbreak`. Empty or direct-marker-only lines do not change flow. This visual boundary does not add painted text.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Linhas causais vazias em página auto — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/before/results.json` mede fontes→PDF com binários pinados e entradas sem newline artificial; `initial.json` guarda HEAD, working tree, hashes e snapshots. `#text("")` e a variante 18pt têm Tekt 3×3pt versus vanilla 3×8.568/12.852pt. `abc\n\n` termina em Tekt 19.872×31.536pt versus vanilla 19.872×8.568pt, enquanto `abc\n\nxyz` coincide a 19.872×31.536pt. `#text("  ")\n\nabc` fica numa linha Tekt (26.112×8.568pt), mas em duas linhas vanilla (19.872×31.536pt). Fonte: `text.rs:37-86` não emite item para Text vazio; `html_frame.rs:72` exclui todo TextSeparator de materialidade; `line_commit.rs:280-286`, `parbreak.rs:43-48` e `mod.rs:1965` usam essa materialidade para linha; `parbreak.rs:40-85` avança cursor após fechar linha e `page_geometry.rs:180-201` usa cursor se não há fundo confirmado. Vanilla `inline/linebreak.rs:695-699` oferece breakpoint obrigatório até para texto vazio. Isto mede o observável; não infere a intenção histórica nem exige a mecânica do vanilla.

### Contrato interno segregado

Parbreak fecha linha textual causal literal ou material. Após fechar uma linha, confirmar o fundo da baseline anterior como limite geométrico para página auto; o espaçamento posterior só aparece se conteúdo causal subsequente o ocupar. Preservar boundary sem tinta e colapso de margens.

Aceitação: testes RED→GREEN e matriz bilateral, regressões anteriores, build/lint sem erros; resselo somente dos owners deste manifesto.

### Refinamento medido antes da decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/after/results.json` mostrou `#text("  ")\n\n` reduzido a 3pt no Tekt, contra 8.568pt vanilla, porque a marca semântica de Parbreak entrou antes de admitir a baseline dos carriers. Se a linha anterior contém somente TextSeparator Literal, admitir sua baseline antes de adicionar a marca transparente e fechar. A marca não vira tinta nem cria largura.
