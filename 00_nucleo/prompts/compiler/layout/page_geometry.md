# L0 — page geometry
Hash do Código: 8ce55ef4

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_geometry.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_geometry.rs:103`, `01_core/src/compiler/layout/page_geometry.rs:137`, `01_core/src/compiler/layout/page_geometry.rs:9`, `01_core/src/compiler/layout/page_geometry.rs:56`, `01_core/src/compiler/layout/page_geometry.rs:72`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Apply typed page geometry to PageConfig: explicit width/height override paper dimensions, omitted axes retain prior base dimensions, and flip swaps those bases. Preserve binding, direction and folded margin precedence. The apply helper initially resolves margins at physical page one; subsequent physical-page resolution remains in the existing regional transition path. Install no page, evaluate no callbacks and do not own a regional transition.

For auto width, derive rightmost material extent from emitted and pending items using `00_nucleo/prompts/compiler/layout/helpers.md`, include deferred equation logical widths and the current numbered-equation reservation, then add right margin with a minimum of total horizontal margins. The numbered reservation uses the equation width plus both number/gutter sides, with the existing 0.5em gutter. For auto height, use the causal last_block_descent_y or current cursor fallback plus bottom margin, with a minimum of vertical margins. These are final geometry projections, not arbitrary shrink-to-ink promises. `00_nucleo/prompts/compiler/layout/page_commit.md` and the hub call them at their existing composition points.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Geometria auto de separadores literais — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-espacos-2026-09-29/before/results.json`, `provenance.json` e `initial.json` medem 30 fontes/60 PDFs, font-path, comandos, UTC, HEAD, working tree e binários contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Só espaços literais (`#text("   ")`) com tracking 0/±0.2 produzem no Tekt `/MediaBox [0 0 0 0]` (Poppler apresenta 612×792), enquanto o vanilla mede respectivamente 9.36/9.76/8.96×8.568pt. Separadores literais finais após texto perdem a sua largura em páginas auto; os iniciais seguidos de texto já coincidem. Escopo com tamanho 18pt termina em 14.44×12.852pt vanilla para +0.2pt. Texto vazio e markup sem tinta revelam outra falta de baseline/mínimo; newline final tem divergência vertical própria e não é fechado por esta fatia.

Causa fonte: `space.rs:22-35` não transporta Literal inicial sem tracking; `page_geometry.rs:104-135` usa `helpers::line_content_right` que filtra todos os TextSeparator, por necessidade correta do alinhamento RTL, e a altura de linha só com carrier não recebe topo tipográfico; `builder.rs:291` e `builder/fonts.rs:633,1167` escrevem MediaBox zero. Vanilla `crates/typst-pdf/src/convert.rs:108-118` impõe mínimo físico de 3×3 na saída PDF. Isso justifica projeção auto separada do alinhamento RTL e mínimo compartilhado entre os três caminhos exportadores. Não inferir intenção histórica do escopo antigo.

### Contrato interno da correção

Para width:auto, combinar a extensão visual existente com a extensão causal dos TextSeparator de origem Literal emitidos/pedentes (pos.x + advance), e conservar as reservas de equação e margens. Não alterar helpers::line_content_right usado pelo alinhamento RTL, nem usar cursor global que conte espaços omitidos de outras linhas. Para height:auto quando a página contém apenas carriers Literal sem material visual, conservar ao menos o topo tipográfico medido pelo FontMetrics.text_edges do próprio estilo/size de cada carrier acima de sua origem. Páginas com material usam a altura anterior. Não impor o mínimo físico do PDF aqui; o exportador é o owner desse requisito.

Correção interna de paridade em fluxo contínuo ADR-0127, sem campo público, assinatura/flag/fase nova. Aceitação RED→GREEN para espaços 0/± e PDF mínimo, casos causais bilaterais, corpus anterior, build e lint sem erros, resselo estritamente no manifesto. Texto vazio explícito e newline final permanecem para rodada própria.

## Baseline admitida para página auto literal — medição e contrato (2026-09-29)

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/after-legacy33/results.json` mostra `#text("  ")` a 12pt com 17.136pt Tekt versus 8.568pt vanilla após fechar uma linha só Literal: `page_geometry.rs:180-201` soma o topo do texto a `pos.y`, mas o carrier já foi movido à baseline admitida por `text_baseline::admit`. Para página só de carriers Literal, a extensão causal é a própria baseline `pos.y`; conservar o maior fundo confirmado e margens. Não somar top duas vezes nem alterar alinhamento RTL ou largura auto.
