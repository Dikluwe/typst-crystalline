# L0 — context block
Hash do Código: faae0241

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/context_block.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/context_block.rs:11`, `01_core/src/compiler/layout/context_block.rs:42`, `01_core/src/compiler/layout/context_block.rs:47`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Ordinary paginated layout is a no-op and never executes the closure. When detailed HTML context capture is enabled, append the private marker supplied by `00_nucleo/prompts/core/compiler/layout/html_frame.md`, identified by the element id, at `(cursor_x, cursor_y - ascender)` using the active injected metrics. Do not advance the cursor or extract positions here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Posições no referencial composto — 2026-10-14

### Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz, fontes/binários/HEAD/working tree/UTC
preservados: Box, inset2pt, prefixo e aninhamento publicam (0,0), mesmo
quando a tinta se desloca. A diferença da baseline do context direto é
uma questão separada: não copiar o vanilla apenas por diferir. mod.rs:
1131-1148/1180-1200 escreve pontos locais em runtime.positions; Box agrupa
o corpo em outro referencial; finish.rs:295-302 sela o mapa sem recompor
esses pontos. Realized escreve com or_insert; labels têm mapa paralelo.
HTML já transporta markers privados pelo frame em html_frame.rs:30-199.
O registro single-pass e sealing por sub-store são preservados, ADR-0073/
0074 §§posição/modelo híbrido; não se adota introspecção global vanilla.
A inspeção do núcleo coordinates e seus owners mostrou usos absolutos
em grid e explicitamente locais em Place/Box. A qualificação do núcleo
conserva ambos e exige transformação até a página, sem dupla origem.
Snapshot anterior: posicoes-compostas-2026-10-14/before.json.gz.
Geometria/identidade são observáveis; o carrier e seu índice são mecânica.

### Contrato deste owner

A operação produtiva continua no-op para página e opt-in apenas para marker HTML, sem executar closure. Seu teste deve distinguir carrier HTML desse registro paginado emitido pelo hub: neutralidade, cursor e ausência de execução continuam obrigatórios, mas o comprimento total do Vec não é contrato de linguagem.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.

## Origem geométrica da ilha e do carrier — 2026-10-14

### Medição antes da decisão

place-origem-local-2026-10-14/html-failures.log reproduz quatro falhas P1493 existentes: aqui() publica y11.2 em vez de20 dentro de Place(Box HTML), diferença de ascender8.8 em FixedMetrics. O entrypoint html_frame.rs:219 compõe em baseline legada; context_block.rs:16–22 subtrai ascender ativo mesmo quando Place/Box usam origem geométrica0. Os testes de pipeline preservam explicitamente x30/y20, nenhuma página e fixpoint do mesmo target. O estado anterior está em html-before.json.gz; workspace-interrupted conserva a primeira ocorrência e o estado integral. Captura e desenho devem usar a mesma geometria, como exige o núcleo coordinates; não é motivo para converter HTML em pipeline paginado nem mudar ADR-0128.

### Responsabilidade exclusiva

No subframe, registrar o cursor físico sem descontar ascender: o caller já possui a tradução de recomposição, inclusive a retirada da origem legada. Fora de subframe, preservar a fórmula legada com métricas ativas. Não executar closure, avançar cursor nem mudar a captura paginada. Isso substitui a subtração incondicional de ascender quando o body usa origem geométrica.

Correção interna ADR-0127 para preservar o contrato HTML já vigente. As expectativas P1493 permanecem iguais. Validar todos os testes p1493, contexto/HTML e suíte completa, além dos controles PDF/SVG e gates de linhagem; sem mudança pública ou de fase.

Refutador anterior à revisão: html-green-attempt1.log mantém dois testes de transform/link vermelhos: a translação5 vira-3.8. Transform recompõe o frame legado retirando sua origem8.8; normalizar o carrier antes disso a retira duas vezes. Assim, um marker em subframe deve portar a coordenada física do cursor como os demais itens, e não uma coordenada pré-normalizada. A ilha HTML já entra em origem geométrica. Os dois testes permanecem inalterados.
