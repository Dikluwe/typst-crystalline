# L0 — Layout: Figuras e Legendas
Hash do Código: b3d26917

## Módulo
`01_core/src/compiler/layout/figure.rs`

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
  Exemplos: lang `"en"` → `"Figure"`, lang `"pt"` → `"Figura"`.
  O prefixo completo segue o formato `"{supplement} {formatted}: "`.

> **Fonte de paridade dos prefixos i18n (P1031)** — as traduções são dados literais do
> vanilla ratificado (`e0e8ca4d`), em `crates/typst-library/translations/<lang>.txt`,
> linha 1 de cada ficheiro:
>
> | Lang | Ficheiro:linha | Valor |
> |---|---|---|
> | `en` | `translations/en.txt:1` | `figure = Figure` |
> | `pt` | `translations/pt.txt:1` | `figure = Figura` |
> | `de` | `translations/de.txt:1` | `figure = Abbildung` |
>
> Os três valores do L0 conferem. Citação literal.
>
> **Duas divergências medidas (2026-08-13)** — vanilla `/usr/local/bin/typst`
> (`typst 0.15.1 (e0e8ca4d)`) vs cristalino `target/release/typst` (fonte em HEAD
> `4f64e4e69`, árvore só com edições em `00_nucleo/prompts/**`).
> Documento: `#figure(rect(width: 10pt, height: 10pt), caption: [Uma coisa])`.
>
> | Caso | Vanilla | Cristalino |
> |---|---|---|
> | sem `#set figure(numbering:)`, sem `#set text(lang:)` | `Figure 1: Uma coisa` | `Uma coisa` |
> | `#set figure(numbering: "1")`, sem `lang` | `Figure 1: Uma coisa` | `Figura 1: Uma coisa` |
> | `#set figure(numbering: "1")` + `#set text(lang: "en")` | `Figure 1: Uma coisa` | `Figure 1: Uma coisa` (coincide) |
>
> **ACHADO ESCALADO 1 — `figure.numbering` tem default na linguagem, e o cristalino não o
> aplica.** Vanilla: `crates/typst-library/src/model/figure.rs:296-299` —
> `#[default(Some(NumberingPattern::from_str("1").unwrap().into()))] pub numbering:
> Option<Numbering>`, com doc comment *"How to number the figure. Accepts a numbering
> pattern or function taking a single number."* Ou seja, uma figura com caption é numerada
> por defeito. O cristalino exige `#set figure(numbering: "1")` explícito.
>
> **ACHADO ESCALADO 2 — a língua por defeito do cristalino não é `en`.** Vanilla:
> `crates/typst-library/src/text/mod.rs:473` — `#[default(Lang::ENGLISH)] pub lang: Lang`
> (`Lang::ENGLISH` em `text/lang.rs:232`). A regra "lang `None` → `Figura`" que constava
> acima descrevia fielmente o cristalino, mas contradiz a linguagem; foi removida da
> lista de exemplos. Com `lang` explícito os dois binários coincidem, o que localiza a
> divergência no **default**, não na tabela de traduções.
>
> Ambos são mudança de comportamento por defeito → gate ADR-0127 e passo próprio.
> **Não implementados aqui.**
- O corpo (`body`) é desenhado primeiro, seguido do prefixo e do `caption`.
- Figura sem caption não desenha prefixo numérico.
- Não escreve em `resolved_labels` — isso é responsabilidade de `introspect.rs`.
- A dupla contagem (introspecção + layout) é intencional: a Passagem 1 rastreia
  o estado final do documento; a Passagem 2 desenha os números iterativamente.

## Import adicionado (P470)
```rust
use crate::compiler::lang::figure_supplement::figure_supplement_for_lang;
```

## Critérios de verificação
- Figura numerada, caption, pattern `"1"`, sem lang → prefixo `"Figura 1: "`.
- Figura numerada, caption, pattern `"1"`, lang `"en"` → prefixo `"Figure 1: "`.
- Figura numerada, caption, pattern `"I."`, lang `"de"` → prefixo `"Abbildung I.: "`.
- Figura numerada, caption, pattern `"(a)"` → prefixo `"Figura (a): "` (sem lang → PT).
- Figura numerada, caption, pattern inválido → fallback `"Figura 1: "` (sem lang).
- Figura sem caption → sem prefixo numérico.
- Duas figuras sequenciais com mesmo pattern → `"Figura 1: "` e `"Figura 2: "`.

## §P488 — Registo de página para LoF

Quando `numbering_pattern.is_some()` (figura contada com caption), após calcular
`figure_number`, regista a página actual em `layouter.runtime.figure_page_numbers`:

```rust
let page = layouter.current_page_number();
layouter.runtime.figure_page_numbers.push(page);
```

Inserção em ordem de documento — positional match com `figures_for_lof`
no `layout_lof` (§P488 em `layout_outline.md`). Figuras sem caption ou sem
numbering não são registadas (coerência com `figures_for_lof` que só tem
elementos `is_counted`).

---

## P1034 — `figure.numbering` tem default `"1"`

**Achado 5 do P1031**, medido: `#figure(..., caption: [Uma coisa])` sem `numbering` dá
`Figure 1: Uma coisa` no vanilla e `Uma coisa` (sem número) no cristalino. O vanilla declara
`#[default(Some(NumberingPattern::from_str("1")))]` em `FigureElem`.

O cristalino lê o padrão da chain (`custom("figure.numbering")`, de-bake P365). A leitura
tratava **ausente** e **`none`** como o mesmo caso, e o resultado era não numerar. Passam a
ser casos distintos:

| `custom("figure.numbering")` | antes | P1034 |
|---|---|---|
| `Some(Value::Str(p))` | numera com `p` | numera com `p` |
| `Some(Value::None)` — `#set figure(numbering: none)` | não numera | não numera |
| ausente | não numera | **numera com `"1"`** |

O mesmo gate existe no `introspect` (`walk`, arm `Figure`): `is_counted` deixa de exigir
`Some(Value::Str(_))` e passa a excluir só `Some(Value::None)` — sem isto o contador não
avança e a legenda numerada fica sem número. As duas leituras têm de concordar; são a mesma
regra em dois sítios (gate de contagem e render do prefixo).
