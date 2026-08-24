# Prompt L0 — `stdlib/foundations/repr` — `repr(v)`
Hash do Código: 1fd34e99

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/repr.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Função

### `native_repr` — `repr(v)`

**Assinatura**: `repr(v: any) -> str`

**Semântica**: Devolve uma representação textual reconhecível do valor.
A implementação real vive em `compiler/eval/repr.rs` (`repr_value`); este nó
é apenas a nativa que a expõe no scope global.

**Testes canônicos**:
```
repr(1)             -> "1"
repr(1.5)           -> "1.5"
repr("abc")         -> "\"abc\""
repr(none)          -> "none"
repr(6pt)           -> "6pt"
repr(rgb("#ff0000")) -> "rgb(\"#ff0000\")"
repr([hello world])   -> "[hello world]"
repr([hello  world])  -> "sequence([hello], [ ], [world])"
repr()              -> Err "repr() requer 1 argumento"
```

Os dois casos de markup acima foram medidos no vanilla pinado `a51e02804` em
2026-08-24. Eles especificam a morfologia observável; não obrigam o cristalino
a imitar a representação Rust interna do vanilla (ADR-0107).

## P1140.9 — espaçamentos e quebras

Medição no vanilla pinado `a51e02804` em 2026-08-24:

```text
repr(h(1pt))                              -> "h(amount: 1pt)"
repr(h(1fr))                              -> "h(amount: 1fr)"
repr(h(1pt, weak: false))                 -> "h(amount: 1pt, weak: false)"
repr(v(1em, weak: true))                  -> "v(amount: 1em, weak: true)"
repr(pagebreak())                         -> "pagebreak()"
repr(pagebreak(weak: false, to: "even")) -> "pagebreak(weak: false, to: \"even\")"
repr(colbreak())                          -> "colbreak()"
repr(colbreak(weak: false))               -> "colbreak(weak: false)"
```

Decisão: `repr_content` formata `HSpace`, `VSpace`, `Pagebreak` e `Colbreak`
como chamadas Typst com os campos canônicos. `amount` aparece sempre em `h` e
`v`; `weak` aparece se e somente se foi explicitamente fornecido, inclusive
quando vale `false`; `pagebreak` ordena `weak` antes de `to`; `to` usa string
`"even"`/`"odd"`. `Length` usa `repr_length`; fração usa formatação numérica
canônica com sufixo `fr`. Não usar `Debug` Rust.

Classificação: o texto é linguagem; o bit que preserva presença é mecânica
interna necessária ao observável (ADR-0107). Esta decisão não altera layout.

## P1140.10 — `linebreak`

Medição no vanilla pinado `a51e02804` em 2026-08-24:

```text
repr(linebreak())                 -> "linebreak()"
repr(linebreak(justify: true))    -> "linebreak(justify: true)"
repr(linebreak(justify: false))   -> "linebreak(justify: false)"
repr([\ ])                        -> "sequence(linebreak(), [ ])"
```

Decisão: `repr_content(Linebreak)` imprime chamada vazia quando `justify` foi
omitido e inclui `justify: true|false` quando explicitamente fornecido. A mesma
regra cobre construtor e sintaxe markup. O valor default não pode ser usado
para inferir presença.

## P1140.17 — `parbreak`

Medição no vanilla pinado `a51e02804` em 2026-08-24:

```text
repr(parbreak()) → "parbreak()"
repr([a\n\nb])   → "sequence([a], parbreak(), [b])"
```

Decisão: `repr_content(Content::Parbreak)` imprime sempre a chamada canônica
`parbreak()`, independentemente de o marker vir da função pública ou de uma
linha vazia. Não adicionar bit de presença/origem: a morfologia observável é a
mesma nos dois caminhos.

## P1140.26 — `page`/`PageRun`

Medição no vanilla pinado `a51e02804` em 2026-08-24:

```text
repr(page([x])) -> "sequence(\n  pagebreak(weak: true),\n  flush(),\n  [x],\n  pagebreak(weak: true),\n)"
repr(page(width: 100pt, height: 120pt, [x]))
  -> "styled(child: sequence(\n  pagebreak(weak: true),\n  flush(),\n  [x],\n  pagebreak(weak: true),\n), ..)"
```

Decisão: `repr_content(Content::PageRun)` materializa essa morfologia. Sem
deltas explícitos imprime a sequência; com qualquer delta imprime o wrapper
`styled(child: ..., ..)`. Não listar propriedades individuais no `repr`: o
vanilla as resume literalmente como `..`.
