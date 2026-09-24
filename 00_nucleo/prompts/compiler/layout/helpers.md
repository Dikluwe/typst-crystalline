# Prompt L0 — helpers geométricos de layout
Hash do Código: e633079e

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
## P1493 — neutralidade métrica do carrier HTML

### Medição anterior à decisão

`layout/html_frame.rs:30-45` representa a posição contextual por um `Semantic`
privado com shape zero-size. `helpers.rs:85-95,438-465` ainda mede a posição
desse sentinel como extensão direita/inferior, apesar de ele não existir na
página legada.

### Contrato ratificado

Os helpers de extensão ignoram somente o carrier privado P1493; os helpers de
translação continuam a deslocar recursivamente seu ponto. O sentinel não muda
largura, altura, trailing space, baseline ou alinhamento e permanece disponível
para a composição geométrica até ser removido pela fachada detalhada.
