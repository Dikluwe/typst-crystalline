# L0 — Effective inline text style
Hash do Código: e0c176fc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

Layer: L1. Exclusive consumer: `01_core/src/compiler/layout/text/style.rs`.

## Evidence before decision

`01_core/src/compiler/layout/text.rs:29-222` resolves TextStyle by typed and
custom chain precedence, including script scaling. Space previously only
used cached style at `01_core/src/compiler/layout/space.rs:14`; this is not
proof of the effective font at a lexical boundary. The approved carrier
requires actual font identity. No new parity measurement is claimed here.

## Contract

Own pure effective-style resolution extracted unchanged from Text. Preserve
field-specific precedence, typed size, custom font override, folded variations,
semantic strong/emph provenance, math fields, script scaling 0.65, explicit
script size and baseline offsets. Do not mutate layouter, do I/O, choose installed
fonts, or emit text. Text invokes once before its temporary style scope. Markup
and Newline separators invoke once at their current chain. Literal separators
already inside the resolved scope reuse that style to avoid scaling twice.
This internal descendant function introduces no public data or trait contract.


## TS1 — tamanho completo de texto (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação obrigatória

Este candidato altera dois contratos públicos, descritos em api.md. Não é
L0 vigente até aprovação explícita. Hash do Código antigo é apenas contexto,
não resselo de implementação inexistente. Materializar somente após ratificar
TS1; sem release/merge/push. Testes S1–S16 e gates sob12GiB, swap zero.

### Responsabilidade exclusiva adicional

Usar tamanho Pt já resolvido no estilo ativo; retirar a projeção morta abs-only de custom text.size sem substituir tamanhos deliberados de heading/math/script por estilo externo. Não alterar os demais folds, métricas ou emissões.
