# L0 — compiler/eval/rules/set/text — Set text e transporte lexical
Hash do Código: ec4982c9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:a28cd4f2dd22d2bdb2cb30085596c2a758af7770fa21eb28c38dc35e904928eb
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:f10f7afaabd1b97f3048736842df89d0ba48d4daa0ae3cd453c9be317f32ef91

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/text.rs`

## Medição antes da decisão

O dispatcher em `01_core/src/compiler/eval/rules/set.rs` no estado após a extração P1499 mantém o braço text como bloco contíguo. O baseline L1 em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` e a suíte após P1499 passaram com 6401 testes e 2 ignorados. Isto é medição estrutural, não benchmark nem nova medição bilateral com o vanilla.

## Contrato de linguagem

Preservar schema de propriedades, validação de tipos e defaults, precedência, warnings de scope-out e transporte custom text.* transparente à morfologia. text.hyphenate aceita bool/auto; os custos e a política lexical seguem os núcleos pinados.

A função deste owner é chamada estaticamente pelo dispatcher, sem mudança na ordem dos braços. Preservar erros, spans, avaliação de argumentos, identidade de valores, ausência versus presença de estilos e fase de execução. As decisões históricas e medições permanecem em `00_nucleo/prompts/compiler/eval/rules.md`.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade linguística conforme ADR-0107/0108; owner 1:1 e pins conforme ADR-0129/0130.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

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

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Set text deixa de ignorar Size Length/Color posicionais: seleção tipada e preferência nomeada; validar sobras, preservando avaliação única, ordem e ocorrências causais. text.size transporta Length sem resolver em. A dobra é do owner StyleChain, não deste dispatcher. Captura/lexicalidade e demais propriedades permanecem.

A seleção da expressão AST da propriedade font classifica explicitamente as três
variantes Arg. O grupo V19 de argumentos sem expressão font correspondente é
coeso: usa a ocorrência já avaliada, sem reavaliar positional/spread/named alheio.
Validar fill sólido nomeado; None e demais não-paints usam o erro do cast Paint.
Gradient/Tiling e os validadores font legados permanecem frentes separadas.

Preservar especificamente o transporte parcial anterior de Gradient/Tiling: as duas variantes constituem grupo V19 explícito e coeso, sem efeito neste owner até a frente Paint. Não confundir esse scope-out com cast inválido; None e demais não-paints agora falham. Não fechar a frente Paint por meio de cor sólida ou mensagem contraditória.

O SARIF da primeira materialização TS1 registrou dois V17 novos neste owner (lookup AST e validação da array font). Desdobrar a seleção exaustiva da variante e as condições no corpo do braço, sem supressão e preservando diagnósticos/avaliação. O grupo V19 residual Pos/Spread é coeso: não possuem expressão AST nomeada font, usando a ocorrência avaliada.

## Pin do núcleo de seleção — 2026-10-18

As medições e o refinamento de forma/RTL em
00_nucleo/diagnosticos/bibliography-hanging-2026-10-18 precedem o novo pin.
Este owner recebe somente o pin do núcleo compartilhado; seu código executável,
API, fase e demais responsabilidades não mudam nesta etapa.
