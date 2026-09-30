# L0 — infra/pipeline/tests
Hash do Código: 1d0cd867

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/tests.rs`

## Current evidence

- `03_infra/src/pipeline/tests.rs:18` — `p1247_contexto_svg_filtra_pagina_e_posicao_homologa`.
- `03_infra/src/pipeline/tests.rs:41` — `MockWorld`.
- `03_infra/src/pipeline/tests.rs:47` — `library`.
- `03_infra/src/pipeline/tests.rs:50` — `book`.
- `03_infra/src/pipeline/tests.rs:53` — `main`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/tests.rs`.

## Owned contract

- Own only the integrated pipeline suite and fixtures, using the existing public facades and descendant helpers. Tests do not authorize production behavior or a new pipeline phase.
- Preserve integrated format precedence, feature separation, contextual snapshots, callbacks, bundle, resources and diagnostics oracles. Use structure, text, DOM, dimensions and positions rather than complete serialized byte hashes.
- Characterize fractional boxes on finite lines: suffix position for 1fr, proportional positive shares, zero share, isolated negative share and safe zero-sum/nonfinite ratios. Preserve repeat ceiling 1000, link scope and strictly-more-copies-at-greater-width relation.
- Keep repeat controls with/without prefix at equal width, distinct sibling 1fr repeats/suffixes, a later block body and nested boxes. Observe paged output, not private repeat/fr indices; do not adapt expectations to the candidate.

Verify that resource collection and resolution for a paged document containing only shapes succeed with a World whose catalog and font accessors reject any access. Construct layout before applying the rejecting World: shape layout can legitimately require baseline and cap-height metrics. This checks export resource demand independently of installed host fonts or wall-clock timing.

Verify with the real Ubuntu Sans variable fixture that unknown-axis-only resource requests retain source bytes even with an unavailable configured Python. Keep a matching `wght` control: it must still fail with the existing dependency diagnostic when the tool is absent. Do not mutate process-global environment in tests; the isolated test command supplies the missing interpreter.

## Verification and boundary

Run `cargo test -p typst-infra --lib pipeline::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/pipeline/tests.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Adaptar projeções e traversals exaustivos de fixtures ao carrier: posição é transportada; neutro para contagem de pontos, tinta e medições de comportamento existente.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — controles do contrato aprovado

Medição anterior: initial.json, before/resultados.json e red.log em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/` registram
texto/caixas e a perda de SHY. O dono confirmou o contrato
`00_nucleo/prompts/entities/layout_types.md`. A compilação de fixtures exaustivas
exige tratar a variante nova. Projeções visuais descem nos filhos; extractors
textuais usam text uma vez. Expectativas de identidade discricionária passam a
SHY, mantendo U+002D literal. Não relaxar caixas/seleção. A suíte de shaper
exercita fixture Noto obrigatória e ausência de fontes, conserva envelope e
prova que text substituto não altera glyphs/avanços nem contamina o cache.

## Regressão do PDF/UA no bundle — 2026-09-30

A medição causal em `00_nucleo/diagnosticos/auditoria-composicao-construcao-2026-09-29/probe.json` antecede a decisão: PDF isolado sob UA-1 sem título falha, bundle publica um PDF declarado UA-1 sem título. Adicionar teste integrado com a fixture World existente para exigir, no bundle, o diagnóstico exato de ausência de título e nenhum ficheiro retornado; manter controle de sucesso quando o título está presente e ausência de exigência fora de UA-1. O teste materializa somente a aceitação dos owners `infra/pipeline/pdf.md` e `infra/pipeline/bundle.md`, sem legitimar lógica produtiva por este prompt.

O controle GREEN usa `title:` nomeado em `#document("one.pdf", title: [Okay])[...]`, pois é o carrier próprio da entrada bundle; `#set document` é uma regra de eval do percurso individual, não a metadata de uma entrada. Conferir que o PDF emitido contém título e identificação UA-1. Um segundo documento sem título não herda o título do primeiro.
