# P1286 — recibo independente de medição vanilla

**Natureza:** baseline empírico e documental; não contém decisão arquitetural,
contrato L0, implementação, testes do candidato nem veredito.

## 1. Papel, regime e isolamento

- Regime da skill `tekt-materializacao-segregada`: protocolo completo, limitado à
  fase de baseline/autor do contrato.
- Executor: agente `/root/medicao_p1286`, ambiente local compartilhado, em
  2026-08-30.
- Entradas permitidas efetivamente lidas: instruções do repositório; skill e as
  referências `papeis-e-capacidades.md` e `artefatos-e-gates.md`; ADR-0107,
  ADR-0108, ADR-0114, ADR-0121 e ADR-0125; fonte quarantinada
  `lab/typst-original`; binários `/usr/local/bin/typst` e `target/debug/typst`;
  artefatos temporários em `/tmp`.
- Escrita autorizada no repositório: somente este recibo. As sondas e saídas
  transitórias foram escritas exclusivamente sob `/tmp`.
- Entradas deliberadamente não lidas: `00_nucleo/materialization/`,
  `00_nucleo/context/`, Prompts L0 e implementação/testes cristalinos. O binário
  cristalino foi executado como caixa-preta apenas depois de congelar a matriz
  vanilla.
- Contexto herdado: pedido do papel medidor, a identidade ratificada declarada
  pelo dono e as instruções superiores. Não foi recebida saída privada do
  implementador.
- Linguagem de atestação: **executado sem atestação de isolamento do host**. A
  ordem e as capacidades foram respeitadas pelo executor, mas o checkout é
  compartilhado e não há sandbox/worktree separado que prove isolamento físico.

## 2. Política de `Unknown`

`Unknown` é preservado quando: a identidade imutável não pode ser provada pelo
artefato disponível; o observável exige AT/leitor PDF não instrumentado; a
construção não foi sondada; ou uma ferramenta de inspeção não distingue
semântica de mecânica. `Unknown` nunca é convertido em aceitação, equivalência
ou ausência de comportamento.

## 3. Proveniência congelada

### 3.1 Estado e binários

Medição iniciada em `2026-08-30T18:15:21-03:00`; snapshot de fecho desta remessa
em `2026-08-30T18:21:54-03:00` (America/Sao_Paulo).

```text
$ git rev-parse HEAD
53d21c5a602f4045a769a0ab0c935baa5ecd3b88

$ /usr/local/bin/typst --version
typst 0.15.1 (e0e8ca4d)

$ target/debug/typst --version
typst 0.15.1 (53d21c5a)

$ sha256sum /usr/local/bin/typst target/debug/typst
7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8  /usr/local/bin/typst
08e0b7cd1967b00a26178c7daa240989f0c1e0e7c418ddc268c552b427a43509  target/debug/typst
```

O alvo ratificado `upstream/main a51e02804` é uma entrada declarada pelo dono.
O checkout `lab/typst-original` não tem `.git` próprio; portanto a associação
dos bytes locais a `a51e02804` é **Unknown como prova independente**. Ela é
fortalecida pelos hashes dos ficheiros medidos abaixo, mas hash de ficheiro não
prova commit. O segundo binário citado no pedido não estava presente:

```text
$ sha256sum lab/typst-original/target/release/typst
sha256sum: lab/typst-original/target/release/typst: No such file or directory
exit=1
```

Consequentemente, todas as execuções vanilla deste recibo usam
`/usr/local/bin/typst` identificado pelo SHA-256 acima. A string `--version` não
é usada sozinha como prova de proveniência.

### 3.2 Hashes da fonte vanilla efetivamente citada

```text
5ecdcbff06730194a01fc8d8da1acc5564ea914efdefb6b550d5cd0dfe286a1d  lab/typst-original/crates/typst-library/src/text/smartquote.rs
5816aaffa4a339799fec9e69f6a87b31953a7401b3bcb711b6bc83beac9d84ba  lab/typst-original/crates/typst-library/src/visualize/line.rs
875df8a8b9775d305bd05be60cb3e2889a671053990bb543ec7f70912ff7d35f  lab/typst-original/crates/typst-layout/src/shapes.rs
80473eba7460cb0f398e7937946e6412c1a8cb1cadffe00580aea9b48f6c0713  lab/typst-original/crates/typst-library/src/visualize/color.rs
525d2505b229baa9ae1a6078de290bddaeba23a4378304c695aa36c55f63aaed  lab/typst-original/crates/typst-library/src/pdf/attach.rs
dba1d5f5e4fe3e8c24376cd96616be0f183926fa15b6303ee75c3806e8567d51  lab/typst-original/crates/typst-library/src/pdf/accessibility.rs
7d88f3efd8579ccd8b91070c408a6ba139715acd9a01911e5d43e7e513005a13  lab/typst-original/crates/typst-pdf/src/attach.rs
72b34f1640483893fd2f5da814f6260921b3786cfa60dd1046073330af12e634  lab/typst-original/crates/typst-layout/src/rules.rs
81fb72ab34cdf9bb24629e32133c00859a386680544473f125cf845a586f1045  lab/typst-original/crates/typst-pdf/src/tags/tree/build.rs
9eb9bb6154de578d872117cbca7590752598de09ee1810cd80eaeed1b41caef2  lab/typst-original/crates/typst-pdf/src/tags/util/mod.rs
```

### 3.3 Working tree da medição

A árvore estava não commitada antes desta medição. O comando exigido pela
ADR-0121, no snapshot de fecho, produziu:

```text
$ git diff HEAD --stat
 .gitignore                                         |     12 +-
 .typ/sec_04.typ                                    |     24 +
 .typ/sec_07.typ                                    |     21 +
 .typ/sec_09.typ                                    |     23 +
 .typ/sec_10.typ                                    |     25 +
 .typ/sec_12.typ                                    |     20 +
 .typ/sec_16.typ                                    |     22 +
 .typ/sec_17.typ                                    |     18 +
 .typ/sec_18.typ                                    |     18 +
 .typ/sec_22.typ                                    |     21 +
 .typ/sec_27.typ                                    |     17 +
 .../diagnosticos/p1282-crystalline-default.json    |  21432 ++
 00_nucleo/diagnosticos/p1282-crystalline-html.json |  22266 ++
 .../diagnosticos/p1282-inventory-default.json      | 128832 +++++++++++
 00_nucleo/diagnosticos/p1282-inventory-html.json   | 213769 ++++++++++++++++++
 00_nucleo/diagnosticos/p1282-probes-default.json   |   4012 +
 00_nucleo/diagnosticos/p1282-probes-html.json      |   4802 +
 00_nucleo/diagnosticos/p1282-summary.json          |  47835 ++++
 00_nucleo/diagnosticos/p1282-vanilla-default.json  |  57158 +++++
 00_nucleo/diagnosticos/p1282-vanilla-html.json     | 138527 ++++++++++++
 .../diagnosticos/typst-passo-1281-relatorio.md     |    164 +
 .../diagnosticos/typst-passo-1282-relatorio.md     |    259 +
 00_nucleo/prompts/compiler/eval.md                 |     21 +-
 .../prompts/compiler/eval/bindings/field_access.md |     65 +-
 .../compiler/eval/bindings/value_methods.md        |     30 +-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |     81 +-
 00_nucleo/prompts/compiler/eval/math.md            |     28 +-
 00_nucleo/prompts/compiler/eval/repr.md            |     13 +-
 00_nucleo/prompts/compiler/eval/rules.md           |     26 +-
 .../prompts/compiler/eval/selector_matching.md     |     57 +-
 00_nucleo/prompts/compiler/stdlib/collections.md   |     86 +-
 00_nucleo/prompts/compiler/stdlib/color.md         |     76 +-
 00_nucleo/prompts/compiler/stdlib/emoji.md         |     53 +-
 .../prompts/compiler/stdlib/foundations/color.md   |    109 +-
 .../compiler/stdlib/foundations/selector.md        |     25 +-
 .../prompts/compiler/stdlib/foundations/str.md     |     23 +-
 .../prompts/compiler/stdlib/structural/math.md     |     17 +
 .../prompts/compiler/stdlib/structural/outline.md  |     22 +
 00_nucleo/prompts/compiler/stdlib/sym.md           |     55 +-
 00_nucleo/prompts/entities/color.md                |     13 +
 00_nucleo/prompts/entities/selector.md             |     22 +
 00_nucleo/prompts/infra/export-fixtures.md         |     12 +-
 00_nucleo/prompts/infra/query-helpers.md           |     43 +
 00_nucleo/prompts/shell/cli.md                     |    140 +-
 00_nucleo/prompts/wiring.md                        |     25 +
 01_core/Cargo.toml                                 |      1 +
 01_core/src/compiler/eval/bindings/field_access.rs |     45 +-
 01_core/src/compiler/eval/bindings/mod.rs          |      2 +-
 .../src/compiler/eval/bindings/value_methods.rs    |     40 +-
 01_core/src/compiler/eval/call_dispatch.rs         |    555 +-
 01_core/src/compiler/eval/math.rs                  |     13 +-
 01_core/src/compiler/eval/mod.rs                   |     10 +-
 01_core/src/compiler/eval/repr.rs                  |     57 +-
 01_core/src/compiler/eval/rules.rs                 |     79 +-
 01_core/src/compiler/eval/selector_matching.rs     |    104 +-
 01_core/src/compiler/eval/tests.rs                 |     91 +-
 01_core/src/compiler/stdlib/collections.rs         |   1355 +-
 01_core/src/compiler/stdlib/color.rs               |     33 +-
 01_core/src/compiler/stdlib/emoji.rs               |    680 +-
 01_core/src/compiler/stdlib/foundations/color.rs   |    184 +-
 .../src/compiler/stdlib/foundations/selector.rs    |     84 +-
 01_core/src/compiler/stdlib/foundations/str.rs     |      6 +-
 01_core/src/compiler/stdlib/mod.rs                 |     40 +-
 01_core/src/compiler/stdlib/state.rs               |      2 +-
 01_core/src/compiler/stdlib/structural/math.rs     |     56 +-
 01_core/src/compiler/stdlib/structural/outline.rs  |      2 +-
 01_core/src/compiler/stdlib/sym.rs                 |    189 +-
 01_core/src/entities/color.rs                      |      2 +-
 01_core/src/entities/selector.rs                   |      2 +-
 02_shell/src/cli.rs                                |    794 +-
 03_infra/fixtures/fonts/LICENSE-NotoSans.md        |     93 +
 03_infra/fixtures/fonts/NotoSans-Regular.ttf       |    Bin 0 -> 556216 bytes
 03_infra/fixtures/p307b/MANIFEST.md                |     36 +-
 03_infra/src/export/svg.rs                         |      6 +-
 03_infra/src/p307b_snapshot_tests.rs               |      6 +-
 03_infra/src/query_helpers.rs                      |     90 +-
 04_wiring/src/main.rs                              |     66 +-
 04_wiring/tests/cli.rs                             |     33 +
 Cargo.lock                                         |     67 +
 Cargo.toml                                         |      1 +
 crystalline.toml                                  |      5 +
 lab/parity/matrix/manifest.yaml                    |      6 +-
 lab/parity/matrix/runner.py                        |     92 +-
 lab/parity/matrix/test_runner.py                   |    120 +
 lab/surface-inventory/Cargo.lock                   |     83 +-
 lab/surface-inventory/extra_seeds.json             |     47 +
 lab/surface-inventory/merge.py                     |    407 +-
 lab/surface-inventory/probes.json                  |      4 +-
 lab/surface-inventory/run_probes.py                |    193 +-
 lab/surface-inventory/src/main.rs                  |    235 +-
 lab/surface-inventory/summarize.py                 |    201 +
 lab/surface-inventory/test_merge.py                |    234 +
 lab/surface-inventory/test_run_probes.py           |     36 +
 .../crates/p1140-inventory/src/main.rs             |    192 +-
 94 files changed, 645608 insertions(+), 1390 deletions(-)
```

`git diff HEAD --stat` não inclui ficheiros untracked; a contagem acima não é
uma alegação sobre todos os ficheiros da árvore. Nenhum desses ficheiros foi
editado por este papel.

## 4. Convenções dos transcriptos

- `stdout=∅` e `stderr=∅` significam stream vazio.
- Nas tabelas de erro, o texto em `stderr` é a mensagem primária exata; a moldura
  de source/caret só é condensada como `@ ficheiro:linha:coluna`.
- `typst query` escreve o valor JSON em stdout e um aviso de depreciação em
  stderr. Esse aviso foi igual em todas as queries:
  `warning: the typst query subcommand is deprecated` e hint para `typst eval`.
- O HTML foi usado para observar morfologia textual de smart quotes; o próprio
  binário emitiu em stderr o aviso exato
  `warning: html export is under active development and incomplete` com exit 0.
- SVG/PDF/QDF são lentes para geometria/tagging observável, não critérios de
  igualdade byte-a-byte.
- As referências `text/...`, `visualize/...` e `pdf/...` abaixo são relativas a
  `lab/typst-original/crates/typst-library/src/`; as restantes começam no crate
  explicitamente nomeado.

## 5. Fatia 1 — `smartquote`

### 5.1 Medição da fonte

- `text/smartquote.rs:32-36`: elemento `smartquote`; `double: bool`, default
  `true`.
- `text/smartquote.rs:38-49`: `enabled: bool`, default `true`.
- `text/smartquote.rs:51-63`: `alternative: bool`, default `false`; não atua em
  línguas sem alternativa nem quando quotes explícitas foram configuradas.
- `text/smartquote.rs:65-89`: `quotes` aceita `auto` (default), string, array ou
  dicionário `single`/`double`, e os membros do dicionário aceitam `auto`, string
  ou array.
- `text/smartquote.rs:225-317`: seleção por língua/região/alternative e overlay
  das quotes explícitas.
- `text/smartquote.rs:319-331`: `double` escolhe o par single/double; quando
  desativada, a forma fallback é ASCII `"` ou `'`.
- `text/smartquote.rs:342-367`: string deve conter exatamente dois grapheme
  clusters Unicode.
- `text/smartquote.rs:369-419`: array deve conter exatamente duas strings;
  dicionário só conhece `double` e `single`, cada ausente vira `auto`; uma
  string/array simples personaliza `double` e deixa `single: auto`.

### 5.2 Casos válidos e morfologia observada

Comando-base:

```text
/usr/local/bin/typst compile --features html --format html --pretty INPUT OUTPUT
```

Todos os casos abaixo: `exit=0`, `stdout=∅`; `stderr` contém somente o aviso HTML
descrito em §4.

| Entrada relevante | Texto HTML observado |
|---|---|
| `text(lang: "de")`, defaults, `"Default"` | `<p>„Default“.</p>` |
| alemão + `alternative: true`, `"Alt"` | `<p>»Alt«.</p>` |
| o anterior + `quotes: "()"`, `"Explicit"` | `<p>(Explicit).</p>`; quotes explícitas prevalecem |
| `quotes: auto`, `smartquote(double: true, enabled: false)` | `"` |
| `quotes: auto`, `smartquote(double: false, enabled: false)` | `'` |
| `quotes: ("[[", "]]")`, `"Array"` | `[[Array]]` |
| `quotes: (single: ("<", ">"), double: auto)`, `'Single' and "Double"` em alemão alternativo | `<Single> and »Double«` |

### 5.3 Limites e erros

Comando de cada linha:
`/usr/local/bin/typst compile INPUT /tmp/out.pdf`.

| Entrada | exit | stdout | stderr primário exato |
|---|---:|---|---|
| `quotes: "x"` | 1 | ∅ | `error: expected 2 characters, found 1 character` @ `p1286-smartquote-bad-string.typ:1:24` |
| `quotes: ("x",)` | 1 | ∅ | `error: expected 2 quotes, found 1 quote` @ `p1286-smartquote-bad-array.typ:1:24` |
| `quotes: (triple: ("x", "y"))` | 1 | ∅ | `error: unexpected key "triple", valid keys are "double" and "single"` @ `p1286-smartquote-bad-dict.typ:1:24` |

**Medido:** os resultados acima. **Inferido da fonte:** string conta grapheme
clusters, não scalar values ou bytes. Refutação: uma string de dois graphemes
que o binário rejeite, ou de outra cardinalidade que aceite.

## 6. Fatia 2 — `line(start:, end:)`

### 6.1 Medição da fonte

- `visualize/line.rs:19-30`: `start` é `Axes<Rel<Length>>`; `end` é opcional;
  `length` default é `30pt` e só é respeitado quando `end` é `none`.
- `visualize/line.rs:32-34`: `angle` também só é respeitado quando `end` é
  `none`; a sonda sem `angle` abaixo observa direção de zero graus.
- `typst-layout/src/shapes.rs:28-40`: ambos os pontos são resolvidos contra o
  tamanho da região; com `end`, `delta = end - start`; sem `end`, o delta vem de
  `length × (cos(angle), sin(angle))`.
- `typst-layout/src/shapes.rs:42-51`: o frame inclui `start`, `start + delta` e
  zero; a geometria é `Line(delta)` inserida em `start`. Infinito é rejeitado.

### 6.2 Geometria observada em SVG

Comando-base:
`/usr/local/bin/typst compile --format svg --pretty INPUT OUTPUT`.
Todos: `exit=0`, `stdout=∅`, `stderr=∅`.

| Entrada | `transform` e `d` observados |
|---|---|
| `start:(10pt,20pt), end:(40pt,50pt)` | `transform="translate(10 20)" d="M 0 0l 30 30"` |
| `start:(-10pt,-20pt), end:(30pt,40pt)` | `transform="translate(-10 -20)" d="M 0 0l 40 60"` |
| `start:(40pt,50pt), end:(10pt,20pt)` | `transform="translate(40 50)" d="M 0 0l -30 -30"` |
| `start:(10pt,20pt), end:(40pt,50pt), length:99pt, angle:180deg` | `translate(10 20)`, `M 0 0l 30 30` |
| `start:(10pt,20pt), length:30pt, angle:90deg` | `translate(10 20)`, `M 0 0l 0 30` |
| `start:(10pt,20pt), length:30pt`, sem `angle` | `translate(10 20)`, `M 0 0h 30` |
| `start:(40pt,50pt), length:-30pt, angle:0deg` | `translate(40 50)`, `M 0 0h -30` |

Os SVGs do caso simples e do caso com `length/angle` conflitantes tiveram o
mesmo SHA-256 `ca90dfa0989f7f14174f05d7e966ca5199cb5f75bb3fb4d26a4f9c90b2b6d8ef`.
O hash registra repetibilidade mecânica; a observação de língua é que `end`
determina o vetor e torna `length/angle` sem efeito.

### 6.3 Erros e fronteiras

| Entrada | exit | stdout | stderr primário exato |
|---|---:|---|---|
| `line(..., dx: 3pt)` | 1 | ∅ | `error: unexpected argument: dx` @ `p1286-line-dx.typ:1:46` |
| `line(..., dy: 3pt)` | 1 | ∅ | `error: unexpected argument: dy` @ `p1286-line-dy.typ:1:46` |
| `start: (10pt,)` | 1 | ∅ | `error: array must contain exactly two items`; hint: primeiro item X, segundo Y @ `p1286-line-bad-start.typ:1:13` |

**Medido:** origem não zero, origem negativa, vetor invertido, comprimento
negativo, precedência de `end` e inexistência de `dx`/`dy`. Percentuais mistos e
infinito não foram sondados empiricamente: **Unknown**, apesar do caminho de
resolução e erro de infinito estar visível na fonte.

## 7. Fatia 3 — `color.mix`

### 7.1 Medição da fonte

- `visualize/color.rs:1058-1089`: variádica de duas ou mais cores segundo a
  documentação; cada argumento é cor ou par array `(cor, peso)`; peso é float ou
  ratio; os pesos são relativos e não precisam somar 100%; `space` named tem
  default `auto`, normalmente Oklab.
- `visualize/color.rs:1138-1204`: espaços com hue (`HSL`, `HSV`, `Oklch`)
  rejeitam mais de duas cores; qualquer ramo rejeita soma de pesos `<= 0`; fora
  disso calcula média ponderada.
- `visualize/color.rs:2422-2459`: cor isolada recebe peso `1.0`; array deve ter
  exatamente `(Color, Weight)`; `Weight` aceita `f64` ou `Ratio`.
- `visualize/color.rs:2461-2511`: `auto` usa o mesmo spot colorant se todos os
  participantes o compartilham; caso contrário usa Oklab; zero cores também cai
  em Oklab antes de falhar pela soma.
- `visualize/color.rs:2514-2647`: spaces aceitos são `rgb`, `luma`, `cmyk`,
  `oklab`, `oklch`, `color.linear-rgb`, `color.hsl`, `color.hsv` ou spot
  colorant. Hue index existe para HSL, HSV e Oklch.

### 7.2 Casos aceites e stdout observado

Comando-base:
`/usr/local/bin/typst query /tmp/p1286-color-valid.typ '<LABEL>' --field value --one`.
Todos: `exit=0`; stderr contém somente o aviso query de §4.

| Label / expressão | stdout JSON exato |
|---|---|
| `three-default`: `rgb(color.mix(red,green,blue))` | `"rgb(\"#909282\")"` |
| `method-three`: `rgb(red.mix(green,blue))` | `"rgb(\"#909282\")"` |
| pesos float `1,2,3` | `"rgb(\"#61969c\")"` |
| ratios `20%,30%,10%` (soma 60%) | `"rgb(\"#999f66\")"` |
| pesos `0,5,5` | `"rgb(\"#00a59f\")"` |
| pesos `-1,1,2`, `space:rgb` (soma positiva) | `"rgb(\"#00bade\")"` |
| uma cor `color.mix(red)` | `"rgb(\"#ff4136\")"` |
| três cores, `space:rgb` | `"rgb(\"#648070\")"` |
| três cores, `space:oklab` | `"rgb(\"#909282\")"` |
| três cores, `space:luma` | `"rgb(\"#909090\")"` |
| três cores, `space:cmyk` | `"rgb(\"#6b7c76\")"` |

Embora a documentação diga “two or more”, a implementação/binário aceita uma
cor e devolve essa cor; zero argumentos chega ao erro de soma. Isto é medição de
comportamento, não alegação de intenção.

### 7.3 Rejeições

Comando de cada linha:
`/usr/local/bin/typst compile INPUT /tmp/out.pdf`.

| Entrada | exit | stdout | stderr primário exato |
|---|---:|---|---|
| zero cores | 1 | ∅ | `error: sum of weights must be positive` |
| pesos `0,0` | 1 | ∅ | `error: sum of weights must be positive` |
| pesos `-1,1` | 1 | ∅ | `error: sum of weights must be positive` |
| três cores em `color.hsl` | 1 | ∅ | `error: cannot mix more than two colors in a hue-based space` |
| três cores em `color.hsv` | 1 | ∅ | mesma mensagem |
| três cores em `oklch` | 1 | ∅ | mesma mensagem |
| array `(red,green,blue)` como um argumento | 1 | ∅ | `error: expected a color or color-weight pair` |
| peso string | 1 | ∅ | `error: expected float or ratio, found string` |
| `space:"rgb"` | 1 | ∅ | `error: expected rgb, luma, cmyk, oklab, oklch, color.linear-rgb, color.hsl, color.hsv, or spot colorant, found string` (os nomes aparecem entre backticks no stderr) |

Mistura de spot colors não foi sondada empiricamente: **Unknown** além da regra
de resolução visível em `color.rs:2461-2511`.

## 8. Fatia 4 — `pdf.attach`

### 8.1 Medição da fonte

- `pdf/attach.rs:31-46`: primeiro argumento requerido `path`, `PathOrStr`,
  resolvido relativamente à raiz virtual; o nome derivado vai para o PDF.
- `pdf/attach.rs:48-61`: segundo argumento é `Bytes`, posicional e opcional na
  chamada; se omitido, os bytes são lidos do path.
- `pdf/attach.rs:63-73`: named opcionais `relationship`, `mime-type` e
  `description`, todos default `none`.
- `pdf/attach.rs:75-86`: relationships válidos: `source`, `data`, `alternative`,
  `supplement`.
- `typst-pdf/src/attach.rs:13-63`: o exportador consulta todos os `AttachElem`,
  valida MIME, mapeia relationship (ausente = `Unspecified`), preserva bytes,
  descrição, path e data; path duplicado é erro.
- `typst-layout/src/rules.rs:824`: o elemento não gera conteúdo visual.

### 8.2 Path e bytes observados

Compilações:

```text
$ /usr/local/bin/typst compile --creation-timestamp 0 /tmp/p1286-attach-path.typ /tmp/p1286-attach-path.pdf
stdout=∅
stderr=∅
exit=0

$ /usr/local/bin/typst compile --creation-timestamp 0 /tmp/p1286-attach-bytes.typ /tmp/p1286-attach-bytes.pdf
stdout=∅
stderr=∅
exit=0
```

`pdfdetach -list` retornou `1 embedded files` e os nomes
`p1286-payload.txt`/`virtual.bin`, exit 0. Extração comprovou:

```text
$ sha256sum /tmp/p1286-payload.txt /tmp/p1286-extracted-path.txt
a1ef83d98b3ecc79e2dcc7c84cf2268dbcf387cda511a20f1f297bb68dd42905  /tmp/p1286-payload.txt
a1ef83d98b3ecc79e2dcc7c84cf2268dbcf387cda511a20f1f297bb68dd42905  /tmp/p1286-extracted-path.txt

$ od -An -t u1 /tmp/p1286-extracted-bytes.bin
   0  65 255
```

No PDF normal, `mutool show` observou `/Desc (three bytes)` no Filespec e
`/Subtype /application#2Foctet-stream` no EmbeddedFile. O path-only, com
defaults, não continha `/Desc` nem `/Subtype`. `relationship` é documentadamente
ignorado fora de PDF/A-3 e não apareceu no Filespec normal.

Em PDF/A-3b:

```text
$ /usr/local/bin/typst compile --creation-timestamp 0 --pdf-standard a-3b /tmp/p1286-attach-bytes.typ /tmp/p1286-attach-bytes-a3.pdf
stdout=∅
stderr=∅
exit=0

$ mutool show /tmp/p1286-attach-bytes-a3.pdf 14
... /AFRelationship /Supplement ... /Desc (three bytes) ...
exit=0
```

### 8.3 Erros e limites

| Entrada | exit | stdout | stderr primário exato |
|---|---:|---|---|
| corpo `[hello]` como segundo arg | 1 | ∅ | `error: expected bytes, found content` |
| string `"hello"` como segundo arg | 1 | ∅ | `error: expected bytes, found string` |
| sem `path` | 1 | ∅ | `error: missing argument: path` |
| path inexistente, sem bytes | 1 | ∅ | `error: file not found (searched at /tmp/does-not-exist.bin)` |
| `relationship:"primary"` | 1 | ∅ | `error: expected "source", "data", "alternative", "supplement", or none` |
| `data:` named com path existente | 1 | ∅ | `error: unexpected argument: data` |
| bytes no lugar do path | 1 | ∅ | `error: expected path or string, found bytes` |
| MIME `"not a mime"` | 1 | ∅ | `error: invalid mime type` |
| mesmo path duas vezes | 1 | ∅ | `error: attempted to attach file same.bin twice` |
| attachment em `--pdf-standard a-2b` | 1 | ∅ | `error: PDF/A-2b error: document contains an attached file`; hint `file attachments are not supported in this export mode` |

Outras variantes de `PathOrStr`, requisitos de descrição/MIME de todos os
standards e inferência de compressão são **Unknown** empiricamente neste recibo.

## 9. Fatia 5 — `pdf.artifact`

### 9.1 Medição da fonte

- `pdf/accessibility.rs:13-37`: artifact marca conteúdo que não deve ser lido por
  Assistive Technology; a marca é transitiva para descendentes.
- `pdf/accessibility.rs:38-52`: `kind` default `other`; `body: Content` requerido.
- `pdf/accessibility.rs:55-93`: kinds: `header`, `footer`, `watermark`,
  `page-number`, `line-number`, `redaction`, `bates`, `page`,
  `pagination-other`, `layout`, `background`, `other`.
- `typst-layout/src/rules.rs:826`: regra visual devolve `body`, explicando a
  aparência de passthrough.
- `typst-pdf/src/tags/tree/build.rs:337-346`: `ArtifactElem` abre grupo artifact
  com o kind convertido; não é tratado como conteúdo normal.
- `typst-pdf/src/tags/util/mod.rs:25-44`: mapeamento exaustivo de cada kind Typst
  para o `ArtifactType` do PDF.

### 9.2 Morfologia e kinds

Query dos 12 kinds explícitos:

```text
$ /usr/local/bin/typst query /tmp/p1286-artifact-kinds.typ '<kind-probe>' --field value --pretty
[
  "(kind: \"header\", body: [x])",
  "(kind: \"footer\", body: [x])",
  "(kind: \"watermark\", body: [x])",
  "(kind: \"page-number\", body: [x])",
  "(kind: \"line-number\", body: [x])",
  "(kind: \"redaction\", body: [x])",
  "(kind: \"bates\", body: [x])",
  "(kind: \"page\", body: [x])",
  "(kind: \"pagination-other\", body: [x])",
  "(kind: \"layout\", body: [x])",
  "(kind: \"background\", body: [x])",
  "(kind: \"other\", body: [x])"
]
exit=0
```

Query do default produziu stdout `"(body: [x])"`, exit 0: o campo default não é
materializado no `fields()` implícito; a fonte fixa semanticamente o default em
`Other`.

### 9.3 Distinção observável de passthrough

Ambos os documentos — `pdf.artifact[artifact-secret]` e texto comum
`artifact-secret` — compilam com stdout/stderr vazios e exit 0. `pdftotext`
devolve nos dois `plain-before. artifact-secret plain-after.`; portanto extração
de texto por essa ferramenta **não** distingue a semântica.

Depois de `qpdf --qdf --object-streams=disable`, o stream do artifact contém:

```text
/Span<</MCID 0>>BDC
...
EMC/Artifact BMC
... artifact-secret ...
EMC/Span<</MCID 1>>BDC
```

O passthrough contém um único `/Span<</MCID 0>>BDC ... EMC` e nenhuma ocorrência
de `/Artifact`, `BMC` ou segundo `MCID`. O comando `rg -a -n -C 3
'/Artifact|/MCID|BDC|BMC|EMC'` saiu 0 no artifact; no PDF cristalino medido em
§10, a busca por essas marcas saiu 1.

Um `kind:"header"` em PDF normal produziu:
`/Artifact<</Attached[/Top]/Subtype/Header/Type/Pagination>>BDC`.
Um `kind:"background"` com `--pdf-standard 2.0` produziu:
`/Artifact<</Type/Background>>BDC`. Ambos os pipelines tiveram exit 0.

### 9.4 Erros

| Entrada | exit | stdout | stderr primário exato |
|---|---:|---|---|
| `kind:"decorative"` | 1 | ∅ | `error: expected "header", "footer", "watermark", "page-number", "line-number", "redaction", "bates", "page", "pagination-other", "layout", "background", or "other"` |
| ausência de body | 1 | ∅ | `error: missing argument: body` |
| body inteiro `42` | 1 | ∅ | `error: expected content, found integer` |

O efeito real em leitores de ecrã/AT, copy-paste de cada kind e todos os
fallbacks por versão PDF são **Unknown**. O que está medido é a morfologia Typst,
a igualdade visual/textual nesta amostra e a diferença concreta de tagging PDF.

## 10. Caixa-preta cristalina atual — REDs observados

Esta secção foi executada somente depois de congelar os casos vanilla. Não houve
leitura do código candidato.

| Fatia / comando | exit | stdout | stderr/observável exato |
|---|---:|---|---|
| `target/debug/typst compile p1286-smartquote-alt.typ ...pdf` | 0 | ∅ | warning `smartquote: propriedade 'alternative' ainda não suportada`; `pdftotext` = `„Alt“`, enquanto vanilla = `»Alt«` |
| `target/debug/typst compile p1286-smartquote-quotes.typ ...pdf` | 0 | ∅ | stderr ∅; `pdftotext` = `(Explicit)` como o vanilla desta amostra |
| `target/debug/typst compile --format svg p1286-line-start-end.typ ...svg` | 1 | ∅ | `error: line(start): posição inicial não-zero não é suportada (scope-out) — a shape de linha cristalina é relativa à posição corrente` |
| `target/debug/typst query p1286-color-valid.typ '<three-default>' --field value --one` | 1 | ∅ | `error: color.mix() requer 2 argumentos posicionais (col1, col2), recebeu 3` |
| `target/debug/typst compile p1286-attach-bytes-min.typ ...pdf` | 1 | ∅ | `error: pdf.attach: o exportador PDF cristalino não suporta ficheiros embutidos (scope-out)` |
| `target/debug/typst compile p1286-artifact.typ ...pdf` | 0 | ∅ | stderr ∅; após QDF, `rg -a '/Artifact|/MCID|BDC|BMC|EMC'` teve stdout/stderr ∅ e exit 1 |

O CLI cristalino não aceita `--creation-timestamp` nesta build: tentativas
iniciais com essa flag saíram 2 antes de compilar. As duas sondas PDF foram
repetidas sem a flag; só estas repetições informam as observações de feature.

“RED” aqui significa apenas divergência observável nestes casos congelados. Não
é veredito sobre implementação futura nem alegação de equivalência geral das
partes que coincidiram.

## 11. Inventário final de `Unknown`

1. Associação independente dos bytes locais de `lab/typst-original` ao commit
   `a51e02804`: Unknown; falta `.git` próprio e o binário lab esperado está
   ausente.
2. Smartquote: idiomas/regiões além de alemão, profundidade máxima, primes,
   apóstrofos e todos os grapheme-cluster edge cases: Unknown empiricamente.
3. Line: percentuais mistos, infinito e clipping fora da página: Unknown
   empiricamente.
4. Color: spot colors e alpha fora dos casos listados: Unknown empiricamente.
5. Attach: todos os PDF standards, variações de `PathOrStr` e compressão:
   Unknown empiricamente.
6. Artifact: comportamento real de AT, reflow e copy-paste por kind: Unknown;
   tagging PDF é observado, não um substituto por ensaio com AT.
7. Isolamento físico do papel medidor: não atestado no checkout compartilhado.

Não se emite decisão arquitetural nem veredito neste recibo.
