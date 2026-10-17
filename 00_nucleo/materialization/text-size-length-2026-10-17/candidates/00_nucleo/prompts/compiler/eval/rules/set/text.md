# L0 — compiler/eval/rules/set/text — Set text e transporte lexical
Hash do Código: 0f4067d8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:8a628c506765c5d67144248103049ab49d2f1402312cb210c25ac9902b0f2682
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/text.rs`

## Medição antes da decisão

O dispatcher em `01_core/src/compiler/eval/rules/set.rs` no estado após a extração P1499 mantém o braço text como bloco contíguo. O baseline L1 em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` e a suíte após P1499 passaram com 6401 testes e 2 ignorados. Isto é medição estrutural, não benchmark nem nova medição bilateral com o vanilla.

## Contrato de linguagem

Preservar schema de propriedades, validação de tipos e defaults, precedência, warnings de scope-out e transporte custom text.* transparente à morfologia. text.hyphenate aceita bool/auto; os custos e a política lexical seguem os núcleos pinados.

A função deste owner é chamada estaticamente pelo dispatcher, sem mudança na ordem dos braços. Preservar erros, spans, avaliação de argumentos, identidade de valores, ausência versus presença de estilos e fase de execução. As decisões históricas e medições permanecem em `00_nucleo/prompts/compiler/eval/rules.md`.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade linguística conforme ADR-0107/0108; owner 1:1 e pins conforme ADR-0129/0130.


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

Set text deixa de ignorar Size Length/Color posicionais: seleção tipada e preferência nomeada; validar sobras, preservando avaliação única, ordem e ocorrências causais. text.size transporta Length sem resolver em. A dobra é do owner StyleChain, não deste dispatcher. Captura/lexicalidade e demais propriedades permanecem.
