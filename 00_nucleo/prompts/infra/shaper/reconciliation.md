# L0 — infra/shaper/reconciliation
Hash do Código: 862061d3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/reconciliation.rs`

## Current evidence

- `03_infra/src/shaper/reconciliation.rs:28` — `fix_line_positions`.
- `03_infra/src/shaper/reconciliation.rs:98` — `reconcile_rtl_positions`.
- `03_infra/src/shaper/reconciliation.rs:149` — `fix_line_positions_page`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/reconciliation.rs`.

## Owned contract

- Own fix_line_positions and helpers. Adjust only top-level page.items under the existing nonrecursive policy, grouping baselines within 0.5pt and ordering by physical x; preserve y.
- Skip the entire line if it contains math TextShaped or Glyph.
- LTR anchors its first item and accumulates real-minus-estimated advances while preserving layout gaps. Nontext widths remain stable under the existing supported cases.
- RTL captures `gap[i]=x[i+1]-(x[i]+w_est[i])` and `right_edge=x[last]+w_est[last]`; reconstruct `x_real[last]=right_edge-w_real[last]`, then `x_real[i]=x_real[i+1]-gap[i]-w_real[i]`. Preserve normal/justified gaps and the right edge; do not replace them with fixture constants or recompute layout decisions.
- Permit an internal helper to borrow `FallbackFontMetrics` already warmed by the same World and compilation. Keep the existing public `fix_line_positions(world, doc)` signature and its fresh-metrics behavior for all other callers. Width equations, item order and positions remain identical; only the cache lifetime changes.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/shaper/reconciliation.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Deslocar carriers com as diferenças de shaping; tratar advance como largura já aplicada (estimated == real), nunca reaplicá-lo.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Resolver x/y e larguras visual estimada/real pelos filhos e deslocá-los pelo mesmo delta mantendo text. No cálculo de shifts LTR, aplicar a diferença das larguras visuais do envelope, tal como folha textual; não estimar pela string substituta.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

Usar tracking::width para largura real das folhas TextShaped, também nos envelopes TextReplacement e no cálculo RTL já existente; remover fórmula duplicada LTR em favor de estimated_and_real_width. Carriers continuam estimated == real: intervalos já vieram do L1. Não acrescentar fronteiras novamente nesta passagem.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.
