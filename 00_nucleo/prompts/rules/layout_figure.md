# L0 — Layout: Figuras e Legendas
Hash do Código: (a calcular após P470)

## Módulo
`01_core/src/rules/layout/figure.rs`

## Propósito
Encapsula o braço `Content::Figure` do Layouter. Responsável por desenhar
o corpo da figura e, se existir, a legenda (caption) numerada com prefixo i18n.

## Regras de negócio
- O gate de numeração é lido da chain via `custom("figure.numbering")`.
  O valor é um `Value::Str(pattern)` que define o formato do número.
- O número é obtido via `Introspector::figure_number_at_index(kind_key, idx)`
  (fallback heurístico `idx + 1`).
- O número formatado é produzido por `format_counter(&[figure_number], pattern)`;
  se o pattern for inválido/vazio, faz-se fallback para arábico.
- **P470 (i18n):** O prefixo de supplement é obtido de
  `figure_supplement_for_lang(kind_key, layouter.chain.lang().as_ref())`.
  Exemplos: lang `"en"` → `"Figure"`, lang `"pt"` / `None` → `"Figura"`.
  O prefixo completo segue o formato `"{supplement} {formatted}: "`.
- O corpo (`body`) é desenhado primeiro, seguido do prefixo e do `caption`.
- Figura sem caption não desenha prefixo numérico.
- Não escreve em `resolved_labels` — isso é responsabilidade de `introspect.rs`.
- A dupla contagem (introspecção + layout) é intencional: a Passagem 1 rastreia
  o estado final do documento; a Passagem 2 desenha os números iterativamente.

## Import adicionado (P470)
```rust
use crate::rules::lang::figure_supplement::figure_supplement_for_lang;
```

## Critérios de verificação
- Figura numerada, caption, pattern `"1"`, sem lang → prefixo `"Figura 1: "`.
- Figura numerada, caption, pattern `"1"`, lang `"en"` → prefixo `"Figure 1: "`.
- Figura numerada, caption, pattern `"I."`, lang `"de"` → prefixo `"Abbildung I.: "`.
- Figura numerada, caption, pattern `"(a)"` → prefixo `"Figura (a): "` (sem lang → PT).
- Figura numerada, caption, pattern inválido → fallback `"Figura 1: "` (sem lang).
- Figura sem caption → sem prefixo numérico.
- Duas figuras sequenciais com mesmo pattern → `"Figura 1: "` e `"Figura 2: "`.
