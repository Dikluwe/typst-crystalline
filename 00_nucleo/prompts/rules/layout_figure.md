# L0 — Layout: Figuras e Legendas
Hash do Código: 91c9007c

## Módulo
`01_core/src/rules/layout/figure.rs`

## Propósito
Encapsula o braço `Content::Figure` do Layouter. Responsável por desenhar
o corpo da figura e, se existir, a legenda (caption) numerada.

## Regras de negócio
- O gate de numeração é lido da chain via `custom("figure.numbering")`.
  O valor é um `Value::Str(pattern)` que define o formato do número.
- O número é obtido via `Introspector::figure_number_at_index(kind_key, idx)`
  (fallback heurístico `idx + 1`).
- O número formatado é produzido por `format_counter(&[figure_number], pattern)`;
  se o pattern for inválido/vazio, faz-se fallback para arábico.
- O prefixo da legenda segue o formato "Figura {formatted}: ".
- O corpo (`body`) é desenhado primeiro, seguido do prefixo e do `caption`.
- Figura sem caption não desenha prefixo numérico.
- Não escreve em `resolved_labels` — isso é responsabilidade de `introspect.rs`.
- A dupla contagem (introspecção + layout) é intencional: a Passagem 1 rastreia
  o estado final do documento; a Passagem 2 desenha os números iterativamente.

## Critérios de verificação
- Figura numerada com caption e pattern `"1"` → prefixo "Figura 1: ".
- Figura numerada com caption e pattern `"I."` → prefixo "Figura I.: ".
- Figura numerada com caption e pattern `"(a)"` → prefixo "Figura (a): ".
- Figura numerada com caption e pattern `"A."` → prefixo "Figura A.: ".
- Figura numerada com caption e pattern inválido → fallback "Figura 1: ".
- Figura sem caption → sem prefixo numérico.
- Duas figuras numeradas sequenciais com mesmo pattern → "Figura 1: " e
  "Figura 2: " (ou formato equivalente do pattern).
