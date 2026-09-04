# P1300 — receipt do gate L0

**Estado do gate:** `CONFIRMADO`
**Classificação:** `ADR0127_PUBLIC_CONTRACT_AND_COMPAT_BREAK`
**Regime:** protocolo completo de materialização segregada Tekt
**Fase executada:** medição, autoria L0 e receipt pré-contrato
**Instante do receipt:** `2026-09-03T19:03:44,448229959-03:00`

**Confirmação humana:** o dono respondeu `Continue` em
`2026-09-03T19:08:05,765418888-03:00`, em resposta direta ao pedido de
confirmação explícita do diff L0 P1300. Esta confirmação autoriza iniciar a
cadeia pós-gate, sem ampliar o escopo nem autorizar staging, commit ou push.

Nenhum papel P1–P8 pós-confirmação foi iniciado. Esta fase foi executada por uma única
autoridade no filesystem compartilhado; portanto ainda não há alegação de isolamento
técnico, contrato selado, teste independente, implementação ou certificado.

## 1. Baseline e working tree antes das escritas P1300

- `HEAD`: `1f082370e59939de7b57992e137a9f74bfb6758f`
- branch: `Tekt`
- captura: `2026-09-03T19:01:51.133010-03:00`
- `git diff HEAD --stat`: vazio
- SHA-256 dos bytes de `git diff HEAD --stat`:
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- SHA-256 dos bytes de `git status --short`:
  `0892a43b35a9382f46c4c6e066728d8a0e420fd923720ed4f18eeb61c1c9b1ca`

`git status --short` integral antes das escritas P1300:

```text
?? 00_nucleo/diagnosticos/p1299-baseline-status.txt
?? 00_nucleo/diagnosticos/p1299-certificate.json
?? 00_nucleo/diagnosticos/p1299-crystalline-default.json
?? 00_nucleo/diagnosticos/p1299-crystalline-html.json
?? 00_nucleo/diagnosticos/p1299-decision-report.md
?? 00_nucleo/diagnosticos/p1299-feature-matrix.json
?? 00_nucleo/diagnosticos/p1299-inventory-default.json
?? 00_nucleo/diagnosticos/p1299-inventory-html.json
?? 00_nucleo/diagnosticos/p1299-manifest.json
?? 00_nucleo/diagnosticos/p1299-owner-ledger.tsv
?? 00_nucleo/diagnosticos/p1299-probe-catalog.json
?? 00_nucleo/diagnosticos/p1299-run-matrix.py
?? 00_nucleo/diagnosticos/test_p1299_run_matrix.py
?? 00_nucleo/materialization/typst-passo-1299.md
?? 00_nucleo/materialization/typst-passo-1300.md
```

Os itens não rastreados acima são preexistentes e foram preservados. Nenhum path foi
adicionado ao índice, commitado, removido ou limpo.

## 2. Inputs protegidos

| Artefato | SHA-256 reobservado | Resultado |
|---|---|---|
| `00_nucleo/materialization/typst-passo-1299.md` | `574c0f77207a86bbe656bf191120c9603924d0c555e5b411760756a6b303b158` | coincide |
| `00_nucleo/diagnosticos/p1299-certificate.json` | `c9d18805b081e55b34caaf02e39ddfc15fd76c5d48849dd82ca112552cdac754` | coincide |
| `00_nucleo/diagnosticos/p1299-decision-report.md` | `332425ce95df73ce8ea8e54e84aec31d37286a6dec8b07ce9b6b5f77e33eeb2e` | coincide |
| `00_nucleo/diagnosticos/p1299-owner-ledger.tsv` | `79261fe7f8b377b2fb28781b15e88828c8b87843e5b1d0d99cdf1e7a7355fa69` | coincide |
| `00_nucleo/diagnosticos/p1299-feature-matrix.json` | `58f08430c479ba5dea093945301785a89e77f20e9af0809ae05c6641a7ecdc11` | coincide |
| `00_nucleo/prompts/compiler/stdlib/color.md` | `fecc619d1b336a6f8ca8c39c22a302babdec4daf22d3387ce97ce28e36d87dc3` | coincide antes do diff L0 |
| `00_nucleo/prompts/compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` | coincide antes do diff L0 |
| `00_nucleo/prompts/compiler/eval/tests.md` | `4e5963393178ccec06850ca16e84db530675c6760e977d68d68839e1cf891848` | coincide antes do diff L0 |
| `01_core/src/compiler/stdlib/color.rs` | `d40197461efdee472734ef850e7a681821eb490624856bf15510d1b8a9336fed` | coincide |
| `01_core/src/compiler/eval/mod.rs` | `cafdcbf690f5ad8020bbe3da4457a759397c29ac091dc1624f33e14f5127c5b4` | coincide |
| `01_core/src/compiler/eval/tests.rs` | `b6b87d6786874a39b3071c667476d062a4fc41a9298c6519d9434e0ae637df20` | coincide |
| `lab/typst-original/crates/typst-library/src/lib.rs` | `0f43c11dd22fc2da64e6f526d20f4257f8e324b087c70b7986d13694f5abb5ee` | coincide |

Todas as identidades persistentes em `p1299-certificate.json`, tanto inputs quanto
outputs, foram recalculadas e coincidiram. O único path não reabrível era o temporário
`/tmp/p1299.k5gKsy/target/release/typst`, já removido. Ele foi reconstruído a partir do
mesmo baseline com:

```text
env TYPST_COMMIT_SHA=1f082370e59939de7b57992e137a9f74bfb6758f \
    CARGO_TARGET_DIR=/dev/shm/p1300-pre-gate.hSBewa/target \
    cargo build --release -p typst-wiring --bin typst
```

O build terminou com exit `0`; o binário fresco reproduziu exatamente o SHA-256
certificado `104bd664494b34b301c91022e3eb4efa8a8eca3278e90f17990dbebdbd08dc1a`.
O vanilla `/usr/local/bin/typst` reproduziu
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## 3. Medição focal anterior à decisão

Artefato integral:
`00_nucleo/diagnosticos/p1300-pre-gate-measurement.json`, SHA-256
`1678b1aed89bfff7581a8fdd998a32f57df18f134226e680596c587620ea3efe`.

- início: `2026-09-03T19:01:51.133010-03:00`
- fim: `2026-09-03T19:01:55.717850-03:00`
- perfis: `default`, `html`, `a11y`, `html+a11y`
- argv, features, exit, stdout, stderr, duração e estado de completude estão guardados
  por observação no JSON;
- cada perfil produziu `CRYSTALLINE_ONLY` para as seis projeções indevidas e
  `MATCH_VALUE` para as três rotas qualificadas e os dois controles agregados;
- não ocorreu `EXECUTION_UNKNOWN`.

| Observável | Resultado em todos os perfis |
|---|---|
| `hsl`, `hsv`, `linear_rgb` | `CRYSTALLINE_ONLY` |
| `std.hsl`, `std.hsv`, `std.linear_rgb` | `CRYSTALLINE_ONLY` |
| `color.hsl`, `color.hsv`, `color.linear-rgb` | `MATCH_VALUE` |
| `rgb`, `luma`, `oklab`, `oklch`, `cmyk` | `MATCH_VALUE` |
| `std.rgb`, `std.luma`, `std.oklab`, `std.oklch`, `std.cmyk` | `MATCH_VALUE` |

A medição confirma a assimetria P1299. A inferência continua sendo que os três aliases
são publicação indevida, não extensão deliberada. Aceitação de algum alias pelo vanilla
pinado ou regressão de alguma rota canônica teria refutado a inferência; nenhuma ocorreu.

## 4. Novas identidades L0 propostas

| Prompt proprietário | SHA-256 anterior | SHA-256 proposto |
|---|---|---|
| `00_nucleo/prompts/compiler/stdlib/color.md` | `fecc619d1b336a6f8ca8c39c22a302babdec4daf22d3387ce97ce28e36d87dc3` | `8115021062602c8a3663797b3fcfe0f17eb423f54a8ae7b437b260804ad58462` |
| `00_nucleo/prompts/compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` | `3bf911a0882e35f713cf8742f70ea921086a9a95ede6b02b8b938ab92becfc96` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `4e5963393178ccec06850ca16e84db530675c6760e977d68d68839e1cf891848` | `d4d6382d351e084869445fe58e1cb484e762d5b5dfc843b7974cd21149289104` |

Os campos `Hash do Código` permanecem intencionalmente com o consumer baseline. Nenhum
`@prompt-hash`, `Hash do Código` ou hash de Núcleo foi ressellado nesta fase, e
`crystalline-lint --fix-hashes` não foi executado.

## 5. Diff L0 integral submetido à confirmação

```diff
diff --git a/00_nucleo/prompts/compiler/eval.md b/00_nucleo/prompts/compiler/eval.md
index c8127dc59..7d62c1e3a 100644
--- a/00_nucleo/prompts/compiler/eval.md
+++ b/00_nucleo/prompts/compiler/eval.md
@@ -51,6 +51,43 @@ de introspecção pré-show e conteúdo final pós-show permanecem distintos;
 evento residual no entrypoint é erro. Mudança pública, de default ou fase para
 no gate ADR-0127.

+### P1300 — conjunto global fechado de constructors de cor
+
+#### Medição anterior à decisão
+
+Em `2026-09-03T19:01:51.133010-03:00`–`19:01:55.717850-03:00`, no baseline
+`1f082370e59939de7b57992e137a9f74bfb6758f`, a matriz bilateral
+`00_nucleo/diagnosticos/p1300-pre-gate-measurement.json` (SHA-256
+`1678b1aed89bfff7581a8fdd998a32f57df18f134226e680596c587620ea3efe`)
+classificou, nos quatro perfis `default`, `html`, `a11y` e `html+a11y`, os
+nomes bare `hsl`, `hsv`, `linear_rgb` e seus pares sob `std` como
+`CRYSTALLINE_ONLY`. No mesmo corpus, `color.hsl`, `color.hsv`,
+`color.linear-rgb` e os cinco globals ratificados, bare e sob `std`, foram
+`MATCH_VALUE`, sem `EXECUTION_UNKNOWN`.
+
+A fonte vanilla pinada `a51e02804`, em
+`lab/typst-original/crates/typst-library/src/lib.rs:397-401`, registra somente
+`luma`, `oklab`, `oklch`, `rgb` e `cmyk` no scope global. É inferência que a
+causa da assimetria é o conjunto extra registrado no scope base cristalino.
+Aceitação vanilla de algum alias, ou regressão de uma rota qualificada,
+refutaria a inferência; nenhuma foi observada.
+
+#### Decisão
+
+`make_stdlib_with_features` registra exatamente cinco constructors globais de
+cor: `rgb`, `luma`, `cmyk`, `oklab` e `oklch`. Não registra `hsl`, `hsv` ou
+`linear_rgb`, nem inventa o spelling global `linear-rgb`.
+
+Como `std` projeta a mesma stdlib não sombreada, `std.hsl`, `std.hsv`,
+`std.linear_rgb` e qualquer spelling alternativo `std.linear-rgb` também não
+constituem bindings. Esta regra independe das features `html` e
+`a11y-extras`.
+
+O conjunto global não altera os oito fields qualificados de `color`, suas
+nativas, cores predefinidas, operadores ou `color.space()`. A remoção dos três
+aliases públicos é quebra de compatibilidade e permanece bloqueada pelo gate
+humano ADR-0127 antes de qualquer código.
+
 ## Aceitação

 Entrypoints, scope base, duas passagens, dispatcher, spans e fluxo residual são
diff --git a/00_nucleo/prompts/compiler/eval/tests.md b/00_nucleo/prompts/compiler/eval/tests.md
index 135cfddce..3221078b7 100644
--- a/00_nucleo/prompts/compiler/eval/tests.md
+++ b/00_nucleo/prompts/compiler/eval/tests.md
@@ -17,6 +17,47 @@ morfologia e mensagens, não mecânica Rust incidental.

 Regressões têm controles e proveniência do vanilla quando decidem paridade.

+## P1300 — regressão dos constructors globais de cor
+
+### Medição anterior à decisão
+
+Em `2026-09-03T19:01:51.133010-03:00`–`19:01:55.717850-03:00`, no baseline
+`1f082370e59939de7b57992e137a9f74bfb6758f`, a matriz bilateral
+`00_nucleo/diagnosticos/p1300-pre-gate-measurement.json` (SHA-256
+`1678b1aed89bfff7581a8fdd998a32f57df18f134226e680596c587620ea3efe`)
+mediu `hsl`, `hsv`, `linear_rgb` e seus três pares sob `std` somente no
+cristalino, mas preservou bilateralmente `color.hsl`, `color.hsv`,
+`color.linear-rgb` e os cinco constructors globais ratificados. Os resultados
+foram iguais nos perfis `default`, `html`, `a11y` e `html+a11y`, sem
+`EXECUTION_UNKNOWN`.
+
+É inferência que regressões negativas e controles positivos no mesmo teste
+distinguem remoção dos aliases de apagamento das nativas ou de uma única
+projeção root/`std`. Um perfil em que a disponibilidade varie, uma rota
+qualificada ausente ou um global ratificado ausente refutaria a inferência.
+
+### Decisão e aceitação
+
+Após confirmação humana do gate ADR-0127, regressões permanentes devem:
+
+- exigir que `hsl`, `hsv` e `linear_rgb` falhem como variáveis desconhecidas
+  nos quatro perfis;
+- exigir que `std.hsl`, `std.hsv` e `std.linear_rgb` falhem como fields
+  ausentes nos quatro perfis;
+- comparar classe, mensagem, hints e span público de cada erro negativo com o
+  vanilla ratificado;
+- exigir que `color.hsl`, `color.hsv` e `color.linear-rgb` continuem funções
+  chamáveis, com os valores e repr vigentes;
+- exigir que `rgb`, `luma`, `cmyk`, `oklab` e `oklch`, tanto bare quanto sob
+  `std`, continuem funções nos quatro perfis;
+- impedir que a correção apague `native_hsl`, `native_hsv` ou
+  `native_linear_rgb`, esconda qualquer rota qualificada, ou corrija somente
+  uma das projeções root/`std`.
+
+Esses testes observam superfície da linguagem e diagnósticos públicos, não
+ordem interna de inserção, endereço de function pointer ou estrutura Rust.
+Antes da confirmação humana, este contrato não autoriza escrever o teste RED.
+
 P1250B retifica o oracle P744 depois de P1253: as constantes públicas
 nomeadas preservam literais `f32`, portanto `red.mix(blue, space: rgb)` expõe
 `#805b87` no vanilla ratificado. O valor `#805a88` pertence ao caso distinto
diff --git a/00_nucleo/prompts/compiler/stdlib/color.md b/00_nucleo/prompts/compiler/stdlib/color.md
index 82a4273f2..b4eb27b57 100644
--- a/00_nucleo/prompts/compiler/stdlib/color.md
+++ b/00_nucleo/prompts/compiler/stdlib/color.md
@@ -20,11 +20,15 @@ Histórico: até P736, `color` era `Value::Dict` (P476, módulo de 4
 operadores; P477 aumentou para 6).

 Fields do tipo (20): constructors `rgb`, `linear-rgb`, `luma`, `cmyk`,
-`hsl`, `hsv`, `oklab`, `oklch` (as mesmas funções nativas registadas
-globalmente) + operadores `lighten`, `darken`, `mix`, `negate`,
+`hsl`, `hsv`, `oklab`, `oklch` + operadores `lighten`, `darken`, `mix`, `negate`,
 `saturate`, `desaturate`, `rotate`, `components`, `space` (P742) +
 `to-hex`, `transparentize`, `opacify` (P744).

+Dos oito constructors do tipo, somente `rgb`, `luma`, `cmyk`, `oklab` e
+`oklch` também pertencem ao scope global. `linear-rgb`, `hsl` e `hsv` são
+exclusivamente qualificados por `color.*`; este owner não define a composição
+do scope raiz, que pertence a `00_nucleo/prompts/compiler/eval.md`.
+
 **P1143:** o namespace contém ainda as 18 cores predefinidas ratificadas,
 totalizando 38 fields. As cores e os bindings globais consultam uma única
 tabela canônica no owner.
@@ -42,6 +46,42 @@ Medições vanilla que fundamentam (P736):
 P476 — fecho parcial ADR-0083 §"Operadores cor" scope-out:
 4 dos 6 operadores implementados (`saturate`/`desaturate` scope-out futuro P477).

+## P1300 — constructors qualificados e fronteira do owner
+
+### Medição anterior à decisão
+
+Em `2026-09-03T19:01:51.133010-03:00`–`19:01:55.717850-03:00`, no baseline
+`1f082370e59939de7b57992e137a9f74bfb6758f`, a matriz bilateral
+`00_nucleo/diagnosticos/p1300-pre-gate-measurement.json` (SHA-256
+`1678b1aed89bfff7581a8fdd998a32f57df18f134226e680596c587620ea3efe`)
+mediu nos perfis `default`, `html`, `a11y` e `html+a11y`:
+
+- `color.hsl`, `color.hsv` e `color.linear-rgb` como funções com valor público
+  coincidente nos dois produtos;
+- os correspondentes nomes bare e sob `std` somente no cristalino;
+- os cinco constructors globais ratificados presentes nos dois produtos.
+
+A fonte vanilla pinada `a51e02804`, em
+`lab/typst-original/crates/typst-library/src/lib.rs:397-401`, registra
+globalmente apenas `luma`, `oklab`, `oklch`, `rgb` e `cmyk`. É inferência que
+os três aliases cristalinos são publicação indevida, não extensão deliberada.
+A inferência seria refutada se o vanilla pinado aceitasse algum desses aliases
+ou se uma rota qualificada deixasse de coincidir; nenhuma refutação ocorreu.
+
+### Decisão
+
+`color_type_field` conserva exatamente os oito constructors `rgb`,
+`linear-rgb`, `luma`, `cmyk`, `hsl`, `hsv`, `oklab` e `oklch`, com as nativas,
+nomes públicos, argumentos, valores, repr e semântica vigentes. A publicação
+global fica fechada nos cinco nomes ratificados e é contrato do owner
+`compiler/eval`; `linear-rgb`, `hsl` e `hsv` permanecem acessíveis somente
+como `color.linear-rgb`, `color.hsl` e `color.hsv`.
+
+P1300 não altera `color_type_field`, nativas, `color.space()`, operadores,
+`color.map`, cores predefinidas ou métodos de instância. A remoção dos aliases
+globais é quebra de compatibilidade e permanece bloqueada pelo gate humano
+ADR-0127 antes de qualquer código.
+
 ## Função de despacho de fields (P736)

 ```rust
```

## 6. Paths ainda proibidos

Até confirmação humana explícita deste diff L0, continuam proibidos:

- qualquer edição em `01_core/src/compiler/eval/mod.rs`;
- qualquer edição em `01_core/src/compiler/stdlib/color.rs`;
- qualquer edição em `01_core/src/compiler/eval/tests.rs`;
- qualquer outro `.rs`;
- contrato, manifesto, oráculos, mutantes, selo ou teste RED P1300;
- `crystalline-lint --fix-hashes`, staging, commit ou push.

Verificações pré-parada:

- `git diff --name-only -- '*.rs'` → saída vazia;
- `git diff --check` → exit `0`, saída vazia;
- diff rastreado limitado aos três Prompts L0 submetidos acima.

O marcador `P1300_WAITING_HUMAN_L0_CONFIRMATION` encerrou a fase anterior e
foi satisfeito pela confirmação humana registrada acima.
