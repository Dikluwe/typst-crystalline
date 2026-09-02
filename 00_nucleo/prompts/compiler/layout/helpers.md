# Prompt L0 — helpers geométricos de layout
Hash do Código: f37cdeac

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/helpers.rs`

## Contrato e aceitação

Manipular, medir e transladar FrameItems de forma pura, incluindo limites de
conteúdo e descida por Group/Link. Helpers não introduzem semântica específica
de elemento e preservam o referencial declarado pelo caller.

## P1293 — extensão horizontal de `FrameItem::Semantic` num único referencial

### Medição anterior à decisão

Proveniência numérica: recibo causal independente
`00_nucleo/diagnosticos/p1293-attach-layout-causal-measurement-receipt.md`,
SHA-256
`123a7c5b58e04a8701cb50f0c9224dd4b63ddc61c6120759391db30456277386`,
medido em `2026-09-01T19:50:26-03:00` sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, branch `Tekt`, working tree
compartilhada e não commitada (`git status --short` SHA-256
`41695492598973db685969ca8e3ee44b92ed47718b970cf40bfa54fcad913dee`;
`git diff HEAD --stat` SHA-256
`207429f7fe474cb7d337921065d9a5d56117856760efae23f04ed6fbedeaa74f`).

No consumer vigente, `item_pos` usa a posição do primeiro filho de
`FrameItem::Semantic` (`01_core/src/compiler/layout/helpers.rs:28-30`),
enquanto `item_width` usa `right(children) - min_x(children)` (`:60-69`) e
`line_content_right` recompõe o limite como `item_pos + item_width`
(`:77-87`). O wrapper de equação preserva a ordem achatada dos filhos, com a
base antes dos pre-scripts (`compiler/layout/equation.rs:454-472`). Quando um
filho fica à esquerda do primeiro, a composição produz
`first_child_x + rightmost_child_right - min_child_x`, somando novamente o
deslocamento `first_child_x - min_child_x`.

Os probes independentes isolam a condição estrutural: base, attach vazio e
post-scripts permanecem GREEN; `tl` e `bl`, que põem filho material à esquerda
do primeiro, abrem respectivamente caudas de `+6,0368pt` e `+8,9936pt`; com
dois pre-scripts prevalece o máximo do lado esquerdo. Posições relativas e
paths dos glifos permanecem. A hipótese é refutada se um `Semantic` não vazio
cujo primeiro filho não seja o mais à esquerda deixar de apresentar a
diferença algébrica acima, ou se a correção exigir transladar filhos.

Classificação ADR-0107: a extensão horizontal final é geometria observável da
linguagem; ordem de filhos e fórmula Rust são mecânica usada para localizar a
causa. ADR-0108: a obrigação abaixo deriva dessa medição `file:line`, não de
nomes de elementos ou constantes de fixture.

### Decisão

Para `FrameItem::Semantic`, posição horizontal e largura devem pertencer ao
mesmo referencial:

- `left = min(item_pos(child).x)` e
  `right = max(item_pos(child).x + item_width(child))`;
- a posição horizontal observada do wrapper é `left` e a largura é
  `right - left`, de modo que a recomposição externa resulta exatamente em
  `right`;
- filhos conservam suas posições relativas; não há tradução, reordenação nem
  semântica específica de equação/attach;
- `Semantic` vazio conserva posição e largura zero; medição vertical e os
  caminhos de `Group`/`Link` permanecem inalterados.

Aceitação: wrappers semânticos não acrescentam espaço quando o primeiro filho
não é o mais à esquerda, e continuam a medir corretamente casos em que ele já
é o mínimo. A regra é estrutural para todo `FrameItem::Semantic` e não depende
de caracteres, slots, coordenadas ou valores empíricos.

Ownership 1:1: este prompt legitima somente
`01_core/src/compiler/layout/helpers.rs`. A mudança é correção interna de
fórmula e paridade, sem trait, entidade, campo público, default ou mudança de
fase; ADR-0127: fluxo contínuo após resselo e revalidação. Qualquer necessidade
de alterar o produtor do wrapper, `FrameItem` ou o contrato público exige
parada e nova decisão.
