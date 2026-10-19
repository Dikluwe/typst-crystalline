# Diagnóstico P1140.9 — `repr` de spacing e breaks

**Data:** 2026-08-24
**Commit base:** `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`
**Estado:** working tree não commitado

## Resultado

Foram corrigidos os observáveis `repr` de `h`, `v`, `pagebreak` e `colbreak`.
O cristalino agora preserva tanto o valor de `weak` quanto a presença explícita
do argumento, distinguindo omissão de `weak: false` como o vanilla ratificado.

## Implementação

- `HSpaceElem`, `VSpaceElem`, `PagebreakElem` e `ColbreakElem` ganharam o campo
  público `weak_explicit: bool`, aprovado no gate ADR-0127;
- os construtores Rust históricos mantiveram assinatura e comportamento;
- construtores adicionais transportam a presença exata a partir do eval;
- o extrator comum de `h`/`v` devolve valor e presença;
- `pagebreak` e `colbreak` preservam presença a partir de `Args::named`;
- layout continua consultando somente `weak`;
- `repr_content` usa `repr_length`, formatação canônica de `fr`, ordem
  `weak`/`to` e strings `even`/`odd`;
- `PartialEq` e `Hash` incluem a presença, pois ela muda um observável da
  linguagem.

## RED → GREEN

Os dois testes P1140.9 falharam antes da implementação:

- spacing: recebido `hspace`, esperado `h(amount: 1pt)`;
- breaks: recebido `pagebreak`, esperado `pagebreak()`.

Depois da implementação, ambos passaram. Foram também acrescentados testes
diretos para comprimento combinado, fração decimal, presença explícita e
paridade.

## Sondas finais

Às `2026-08-24T11:21:29-03:00`, 19 expressões foram executadas tanto com
`/usr/local/bin/typst` (vanilla pinado `a51e02804`) quanto com
`./target/debug/typst`. O comparador não encontrou divergências.

O conjunto incluiu:

- `h` em `pt`, `em`, `fr`, zero, `weak: true` e `weak: false`;
- `v` em `pt`, `em`, zero, `weak: true` e `weak: false`;
- `pagebreak` vazio, weak true/false, `to: odd` e weak+`to: even`;
- `colbreak` vazio e weak true/false.

Proveniência dessa medição:

- commit base: `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`;
- working tree não commitado;
- `git diff HEAD --stat`: **28 ficheiros alterados, 516 inserções e 105
  remoções**;
- `git status --short | wc -l`: **30 entradas**.

## Validação

- testes P1140.9: **2 passados, 0 falhas**;
- `cargo test -p typst-core --lib`: **5147 passados, 0 falhas**;
- `cargo test -p typst-infra --lib`: **828 passados, 0 falhas**;
- `cargo build --workspace`: passou;
- `cargo fmt --all -- --check`: passou;
- `crystalline-lint .`: zero violações e zero drift;
- `git diff --check`: passou.

Os hashes L0 foram ressellados nos cinco ficheiros que já possuíam
`@prompt-hash`. `stdlib/layout.rs` continua sem `@prompt-hash`, condição
preexistente registrada no passo; nenhum hash foi inventado.

## Achado restante

`linebreak()` permanece ausente do scope global: o vanilla devolve
`repr(linebreak()) == "linebreak()"`, enquanto o cristalino reporta
`unknown variable linebreak`. Essa falha é de binding/construtor público, não
de formatação dos quatro elementos corrigidos, e deve ser o próximo passo
atomizado.
