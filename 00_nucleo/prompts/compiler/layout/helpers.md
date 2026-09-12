# Prompt L0 — helpers geométricos de layout
Hash do Código: d999bfa3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/helpers.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Responsabilidade

Manipular, medir e transladar `FrameItem` de forma pura. Os helpers não
introduzem semântica específica de elemento e preservam o referencial declarado
pelo caller.

## Extensão de wrappers semânticos

Para `FrameItem::Semantic`, posição e largura horizontais pertencem ao mesmo
referencial:

- `left = min(item_pos(child).x)`;
- `right = max(item_pos(child).x + item_width(child))`;
- posição horizontal = `left`;
- largura = `right - left`.

Assim, a recomposição externa termina exatamente em `right`, inclusive quando
o primeiro filho não é o mais à esquerda. Filhos conservam posições relativas:
não há tradução, reordenação ou caso nominal para equação ou attach.

`Semantic` vazio tem posição e largura zero. Medição vertical e os caminhos de
`Group` e `Link` permanecem nas suas convenções próprias.

## Aceitação

Wrappers semânticos medem o mesmo limite horizontal que renderizam, sem
acrescentar espaço pela ordem dos filhos. Casos em que o primeiro filho já é o
mínimo continuam iguais. A regra não depende de caracteres, slots, fixtures ou
valores empíricos.
