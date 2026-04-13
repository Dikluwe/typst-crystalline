# L0 — Layout: Figuras e Legendas
Hash do Código: 91c9007c

## Módulo
`01_core/src/rules/layout/figure.rs`

## Propósito
Encapsula o braço `Content::Figure` do Layouter. Responsável por desenhar
o corpo da figura e, se existir, a legenda (caption) numerada.

## Regras de negócio
- O contador visual avança em `step_flat("figure")` se `numbering_active["figure"]`.
- O prefixo da legenda segue o formato "Figura N: ".
- O corpo (`body`) é desenhado primeiro, seguido do prefixo e do `caption`.
- Figura sem caption não desenha prefixo numérico.
- Não escreve em `resolved_labels` — isso é responsabilidade de `introspect.rs`.
- A dupla contagem (introspecção + layout) é intencional: a Passagem 1 rastreia
  o estado final do documento; a Passagem 2 desenha os números iterativamente.

## Critérios de verificação
- Figura numerada com caption → prefixo "Figura 1: " antes do texto da legenda.
- Figura sem caption → sem prefixo numérico.
- Duas figuras numeradas → prefixos "Figura 1: " e "Figura 2: " correctos.
