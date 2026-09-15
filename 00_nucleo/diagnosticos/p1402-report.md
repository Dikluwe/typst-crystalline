# P1402 — verificação de `outline.entry`

Estado medido: working tree não commitado sobre
`be15d0ab73af089860d03db390508edf21a4bdac`, em
`2026-09-14T22:35:46-03:00`. Executado sem atestação de isolamento.

## Resultado

`PASS_FRAGMENT`. O cristalino expõe `outline.entry` e
`outline.entry.body` como identidades nativas estáticas. O construtor conserva
`level`, o elemento completo e `fill`; `body()` preserva a morfologia do
heading e rejeita elemento ainda não outlinable.

A materialização introspectiva produz uma entrada por heading incluído por
outline: as sondas limitadas retornaram 1 no caso principal, 0 sem outline, 2
com dois outlines, 1 sob `depth: 1` e 0 para `outlined: false`. Os campos do
caso principal coincidiram com o vanila ratificado `586e1bd43`, inclusive
`repeat(body: [.], gap: 0.15em)`.

O primeiro build foi abortado pelo teto fixo de 2 GiB. Sem elevar o limite, a
repetição com um job, incremental desligado e 512 unidades de geração terminou
com sucesso. Cada sonda teve teto de 512 MiB e timeout de 15 s; a suíte
monolítica conhecida por exceder a memória não foi executada e não recebe
crédito. O linter terminou com exit 0 e zero violations após o resselo. O
catálogo negativo matou 10 de 10 mutantes contratuais.

O veredito não cobre `indented`, `inner`, `page`, `prefix`, targets não-heading
nem paridade geral da linguagem.
