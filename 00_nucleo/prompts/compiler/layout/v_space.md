# L0 — v space
Hash do Código: cae5c487

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/v_space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/v_space.rs:17`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve the element's amount against the active font size. Close a horizontally pending line using the ordinary hook, then advance cursor_y by that amount. This adapter does not add a separate collapse algorithm, float policy or page transition.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Depois do flush existente, conservar fronteira certificada (ou cursor se não houver), somar amount e registrar com cursor após avanço. Não incluir leading hipotético de linha fechada no fim físico.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.


## Fronteira vertical de Shape — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/shape-terminal-complete/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/diffstat/UTC/fontes/binários e pin vanilla 9f2b6e871.
Página width100/heightAuto/margem10 e rect20×10: Tekt mede altura43.2,
pin30. Duas formas: segunda em y25.962 contra33.2. Texto depois da forma:
baseline20.438 contra40.438. Shape→Rotate usa top13.2 contra33.2;
Rotate→Shape mantém altura30 contra53.2. shape.rs:196–266 subtrai cap-height
de um topo geométrico e shape.rs:289–304 não publica prev_line_baseline
nem last_block_descent_y. cursor.rs:90–110 e transform.rs:352–356 usam
esses campos para a próxima unidade. v_space.rs:22–31 publica apenas a
âncora de Place, sem manter o fundo terminal confirmado. A fonte pinada
flow/collect.rs:253–285 e flow/distribute.rs:302–318 distingue frame e
margem fraca terminal. São observáveis geométricos; a organização Rust
e a estratégia de buffers não são requisitos de paridade.

### Responsabilidade exclusiva

Depois de calcular o fim físico e registrar a âncora existente, quando
já houver fundo terminal confirmado e o espaço for forte, atualizar esse
fundo com o fim calculado. Não somar a margem fraca do bloco ao avanço
explícito, nem criar um fundo onde o caminho anterior ainda não o possuía.
Espaços fracos não substituem o fundo confirmado. Este reparo não certifica
colapso geral de VSpace, aceitação de negativos ou frações; essas lacunas
permanecem explícitas na continuidade da frente de fluxo vertical.

Correção interna ADR-0127: sem assinatura/campo público, fase ou modo novo.
L0 primeiro, testes RED→GREEN, resselo restrito, corpus bilateral e suíte.
Defaults de formas, alinhamento RTL, Block Auto e colapso geral de espaços
continuam abertos; este contrato não autoriza encerrar esses eixos.

A ampliação `shape-flow-before/results.json.gz` mede VSpace forte entre
Shape e Shape/texto e após Rotate: o pin soma 7pt/30pt ao fim, mas o Tekt
perde esse avanço nos leitores da baseline/fundo. Quando block_chain_active
continua verdadeiro, um VSpace forte também desloca prev_line_baseline pelo
mesmo amount. O gap fraco continua separado. Os controles weak entre itens
mostram prioridade de colapso ainda ausente; não aplicar uma regra max
genérica nem declarar esse subconjunto concluído.
