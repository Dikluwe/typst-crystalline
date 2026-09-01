# P1292 — recibo RED dos oráculos independentes

Data/hora do ack/testes v11: 2026-09-01T09:57:05-03:00

Papel: autor independente de oráculos/testes, lotes A–D. Este recibo não é
veredito e não inspeciona implementação candidata.

## Proveniência e selos de entrada

- baseline Git: 0eb39f8ecb48930515f2cadb6a378450855b5a72;
- manifest v11 de autorização:
  782c97a5d9752a61e5f65d51b0d46ebac9a8fab19664b307458560e20a0f10fc;
- seal v11: 256f5cbee59ae11c7bf2a1150865a95694753a1dfd24197de837a4f464f52daa;
- receipt do contrato v11: 4cd26f7bd20a4e343f4f59231584454202975d0da9fea9c7eb85a173af7e6148;
- amendment 10: 8aac16ad279e22495ed3ce805462a2ed36a44d30cef6df4b62a16b85f52c512f;
- amendment 9: 92e83d0afb8d323823b0815eca62e6a8653d354e782284149651a38fa91d487e;
- amendment 6: 764d2282d0d5067375b7c32796fe7ef2e3db70f81da18c2ea6dfe87cbade357a;
- amendment 5: f9b266fa286524cd326de63024e068d42bf478a218336080b27770d5b79531f7;
- amendment 4: fbda5f854fcf30df07f0f103901fb958a3c02cf403dceca62231e9b0f67c05a2;
- amendment 3: 47656b9d645a75c8ec6f53bdd63b1c06ad269f16eb11e9313fbbd5e99f4c3060;
- amendment 1: 05f6d4bda1fd5a962c17aa530d7f15c5eff0409a9fe209744b385542ca0907b5;
- lots v11: 867b5fe0e61e3bb69a4e54aec4a98710c4acb12c2de03b2f5098d9c890aa79e0;
- comparison v11: cbcac2aa1efe454a20e9542515211f7fabba4048ee2824cec3d2c2ff37207c01;
- medição vanilla: dd8009cac21d3ccadd061ef9ea4ce1a83a8f9eda75264a9c90f54b7dad2bb929;
- hash canônico do contrato v11:
  671fd53142624d9cceaa28d4ebaf5676eb07f4bfa5d3432d1fd2e030ad29d9f2.

O resselo v4 preserva todos os valores de linguagem A–D. Acrescenta somente
controles B discriminatórios exigidos pelo novo contrato: plain-x, mixed
baseline, avanço da linha seguinte e frame/regra italic-f. Nenhuma expectativa
foi adaptada à candidata refutada.

A árvore estava não commitada. As alterações preexistentes eram os 15 L0s
selados modificados, os dois novos L0s flush, os recibos/manifest/seal de P1292
e os artefatos de baseline. O único artefato de teste criado por este papel foi
o ficheiro abaixo; nenhum código produtivo ou L0 foi editado.

Estado exato observado na execução RED baseline anterior
(M = modificado, ?? = não rastreado):

    M  00_nucleo/prompts/compiler/eval.md
    M  00_nucleo/prompts/compiler/eval/math.md
    M  00_nucleo/prompts/compiler/eval/repr.md
    M  00_nucleo/prompts/compiler/layout.md
    M  00_nucleo/prompts/compiler/math/layout/_comum.md
    M  00_nucleo/prompts/compiler/math/layout/cancel.md
    M  00_nucleo/prompts/compiler/math/layout/underline.md
    M  00_nucleo/prompts/compiler/math/layout/vec.md
    M  00_nucleo/prompts/compiler/stdlib/layout.md
    M  00_nucleo/prompts/compiler/stdlib/structural/math.md
    M  00_nucleo/prompts/entities/content.md
    M  00_nucleo/prompts/entities/elements/_comum.md
    M  00_nucleo/prompts/entities/elements/math_cancel.md
    M  00_nucleo/prompts/entities/elements/math_underline.md
    M  00_nucleo/prompts/entities/elements/math_vec.md
    ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
    ?? 00_nucleo/diagnosticos/p1292-contract-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-contract-seal.json
    ?? 00_nucleo/diagnosticos/p1292-manifest.json
    ?? 00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-red-tests-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md
    ?? 00_nucleo/prompts/compiler/layout/flush.md
    ?? 00_nucleo/prompts/entities/elements/flush.md
    ?? 04_wiring/tests/p1292_contract.rs

## Artefato congelado

| Artefato | Linhas | Bytes | SHA-256 |
|---|---:|---:|---|
| 04_wiring/tests/p1292_contract.rs | 718 | 27.871 | fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa |
| 01_core/src/entities/elements/math_underline.rs | 58 | 1.646 | 6a08c552d47f7a90263b6f3e447f15ee9e42f17885201a866db910edddfa4948 |

Bloco cfg(test) independente de math_underline.rs, extraído desde
#[cfg(test)] até EOF: SHA-256
b5ab3f6635c5c36cb037c585062f35f723ed16789ce7dcaf51fab91fb6d3a334.

Não há fixture persistente: cada fonte Typst é literal no teste e os ficheiros
temporários são criados sob /tmp. A partir deste hash, o teste/oráculo fica
imutável para a implementação e para o adversário.

## Cobertura selada

- A — cancel: superfície/repr/defaults e todos os named explícitos; erros
  exatos; callback 0deg e contexto efetivo; cross com duas linhas; span
  original, ausência de documento em erro e ordem background. A cardinalidade
  de duas requisições e o mesmo default positivo são cobertos pelos testes
  internos P1291 herdados registados abaixo.
- B — math.underline: superfície/repr/identidade distinta do underline textual;
  erros exatos; dimensões display/text/script/cramped; correção itálica negativa
  na largura da regra; constantes fallback 166/66/66du e conversão a 12pt.
- C — math.vec: cardinalidades zero/um/muitos; defaults e repr explícito de
  delim/align/gap; erros exatos; equivalência sintaxe/função; fences e
  alinhamento discriminatórios; gap percentual finito, colapso em auto, termo
  absoluto e termo misto.
- D — place.flush: namespace direto e preservado por with, repr e erros exatos;
  no-op sem floats; top/bottom/misto; prefixo versus float posterior; controle
  sem flush; posições/clearance; layouter aninhado ativo.

Nenhum vetor requerido foi classificado como Unknown.

## Compilação do harness

Comando:

    cargo test -p typst-wiring --test p1292_contract --no-run

Código: 0.

Saída relevante:

    Finished test profile [unoptimized + debuginfo]
    Executable tests/p1292_contract.rs

Captura: /tmp/p1292-reseal-no-run.out, 763 linhas, 32.795 bytes, SHA-256
d5250ea43ba87fa25b550400413d6d8a97f818a58b16f84e05872bcb84132815.

## Evidência herdada para cross

O witness removido tentava distinguir a segunda callback por a menor que
90deg. Isso era inválido: ambas as callbacks de cross recebem o mesmo default
positivo. A conclusão anterior de uma única chamada está explicitamente
retratada.

Comandos válidos executados:

    cargo test -p typst-core p1291_cross_reusa_default_positivo_e_ignora_inverted_na_chamada -- --nocapture
    cargo test -p typst-core p1291_identity_preorder_e_cross_duas_chamadas -- --nocapture

Resultados:

    p1291_cross_reusa_default_positivo_e_ignora_inverted_na_chamada ... ok
    test result: ok. 1 passed; 0 failed; 5349 filtered out

    p1291_identity_preorder_e_cross_duas_chamadas ... ok
    test result: ok. 1 passed; 0 failed; 5349 filtered out

Capturas:

- /tmp/p1292-inherited-default.out: SHA-256
  3fd64a8f7178d5af6d0b191365bb08a23cccd69116956d3011cbff63b256ab93;
- /tmp/p1292-inherited-identity.out: SHA-256
  e0d143d4ebb56c03ec107e465c901a80cb044b937a448f3ba5c971996f05ce8b.

Uma primeira tentativa com --exact selecionou zero testes porque o nome
completo inclui o caminho do módulo; foi descartada e não recebe crédito.

## Execução RED focal

Comando exato:

    cargo test -p typst-wiring --test p1292_contract -- --nocapture > /tmp/p1292-reseal-red.out 2>&1

Código: 101. Captura completa: /tmp/p1292-reseal-red.out, 863 linhas,
37.278 bytes, SHA-256
96656f86e2b8cfc270218bcb6f902ae2007ac1d4f3af081b193966ce860e35c1.

Resumo de stdout/stderr:

    running 9 tests
    p1292_b_underbar_fallback_constants_are_not_generic_line_defaults ... ok
    p1292_a_cancel_callback_cross_context_span_and_background ... ok
    A surface: module 'math' does not contain field "cancel"
    B surface: module 'math' does not contain field "underline"
    B geometry: left Array []; right display/text/script/cramped vanilla
    C surface: module 'math' does not contain field "vec"
    C geometry: left Array []; right finite/auto/absolute/mixed vanilla
    D surface: cannot access fields on type function
    D geometry: cannot access fields on type function
    test result: FAILED. 2 passed; 7 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 1.63s
    error: test failed, to rerun pass -p typst-wiring --test p1292_contract

## Motivo semântico do RED

Há pelo menos um RED correto por lote:

- A: math.cancel ainda não é membro público. Os testes herdados confirmam duas
  requisições em preorder e o mesmo default positivo nas duas callbacks; não há
  conclusão de chamada única;
- B: math.underline está ausente e a consulta geométrica produz forma vazia em
  vez das quatro medidas MATH congeladas;
- C: math.vec está ausente e a consulta geométrica produz forma vazia em vez de
  resolver gap finito/auto;
- D: place ainda é função sem namespace acessível, portanto place.flush não
  existe e não pode drenar o prefixo de floats.

As falhas são de membro ausente, no-op/forma vazia ou comportamento semântico
específico. O harness Rust compilou integralmente; nenhuma falha depende de
tipo, variante ou função Rust futura.

## Reconhecimento v2 — execução na candidata pós-A

Estado Git: HEAD
0eb39f8ecb48930515f2cadb6a378450855b5a72 com working tree não commitada.
Estado exato observado:

    M  00_nucleo/prompts/compiler/eval.md
    M  00_nucleo/prompts/compiler/eval/math.md
    M  00_nucleo/prompts/compiler/eval/repr.md
    M  00_nucleo/prompts/compiler/introspect.md
    M  00_nucleo/prompts/compiler/layout.md
    M  00_nucleo/prompts/compiler/math/layout/_comum.md
    M  00_nucleo/prompts/compiler/math/layout/cancel.md
    M  00_nucleo/prompts/compiler/math/layout/spacing.md
    M  00_nucleo/prompts/compiler/math/layout/underline.md
    M  00_nucleo/prompts/compiler/math/layout/vec.md
    M  00_nucleo/prompts/compiler/stdlib/layout.md
    M  00_nucleo/prompts/compiler/stdlib/structural/math.md
    M  00_nucleo/prompts/entities/content.md
    M  00_nucleo/prompts/entities/elements/_comum.md
    M  00_nucleo/prompts/entities/elements/math_cancel.md
    M  00_nucleo/prompts/entities/elements/math_underline.md
    M  00_nucleo/prompts/entities/elements/math_vec.md
    M  00_nucleo/prompts/infra/query-helpers.md
    M  01_core/src/compiler/eval/repr.rs
    M  01_core/src/compiler/math/layout/cancel.rs
    M  01_core/src/compiler/math/layout/mod.rs
    M  01_core/src/compiler/math/layout/tests.rs
    M  01_core/src/compiler/stdlib/structural/math.rs
    M  01_core/src/entities/content.rs
    M  01_core/src/entities/elements/math_cancel.rs
    M  03_infra/src/layout.rs
    ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
    ?? 00_nucleo/diagnosticos/p1292-contract-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-contract-seal.json
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-a.md
    ?? 00_nucleo/diagnosticos/p1292-l0-dependency-amendment-1.md
    ?? 00_nucleo/diagnosticos/p1292-manifest.json
    ?? 00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-red-tests-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md
    ?? 00_nucleo/prompts/compiler/layout/flush.md
    ?? 00_nucleo/prompts/entities/elements/flush.md
    ?? 04_wiring/tests/p1292_contract.rs

Compilação do harness:

    cargo test -p typst-wiring --test p1292_contract --no-run

Código 0. Captura /tmp/p1292-v2-post-a-no-run.out: 762 linhas,
32.713 bytes, SHA-256
9ae669d585462988fa48093e1159ee301ba74e3a868c348d85832f368943f8e7.

Execução:

    cargo test -p typst-wiring --test p1292_contract -- --nocapture

Código 101. Captura /tmp/p1292-v2-post-a.out: 849 linhas, 36.948 bytes,
SHA-256 b1e9fe6526cedbfd51496f104af588d76910faa8b645b77cfc58e16220143be2.

Resultado:

    running 9 tests
    p1292_a_cancel_surface_repr_and_errors ... ok
    p1292_a_cancel_callback_cross_context_span_and_background ... ok
    p1292_b_underbar_fallback_constants_are_not_generic_line_defaults ... ok
    B surface: module 'math' does not contain field "underline"
    B geometry: left Array []; right medidas vanilla congeladas
    C surface: module 'math' does not contain field "vec"
    C geometry: left Array []; right medidas vanilla congeladas
    D surface/geometria: cannot access fields on type function
    test result: FAILED. 3 passed; 6 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 1.78s

Classificação: lote A GREEN; lotes B, C e D permanecem RED por razões
semânticas específicas. Unknown = 0. Nenhuma linha do oráculo foi alterada
para obter este resultado; este bloco apenas reconhece as novas entradas
causais v2 e mede a candidata pós-A contra o mesmo SHA protegido.

## Resselo v3 — transportes materializados

O owner L0 do oráculo tem SHA-256
41dd2e0ed56814baf94acf565d1fa59eb64f1f11b9d11cd1ea4937a998f2d0a1.
O consumer recebeu o header L4 com prompt-hash 41dd2e0e. A identidade B agora
desserializa diretamente o boolean JSON; B usa raiz SVG page-auto; C usa
rect/path SVG colorido produzido por measure contextual; D usa PDF
multipágina e pdftotext -bbox. Não resta uso de query(metadata).

Estado Git: HEAD
0eb39f8ecb48930515f2cadb6a378450855b5a72 com working tree não commitada.
Estado exato observado:

    M  00_nucleo/prompts/compiler/eval.md
    M  00_nucleo/prompts/compiler/eval/math.md
    M  00_nucleo/prompts/compiler/eval/repr.md
    M  00_nucleo/prompts/compiler/introspect.md
    M  00_nucleo/prompts/compiler/layout.md
    M  00_nucleo/prompts/compiler/layout/equation.md
    M  00_nucleo/prompts/compiler/math/layout/_comum.md
    M  00_nucleo/prompts/compiler/math/layout/cancel.md
    M  00_nucleo/prompts/compiler/math/layout/spacing.md
    M  00_nucleo/prompts/compiler/math/layout/underline.md
    M  00_nucleo/prompts/compiler/math/layout/vec.md
    M  00_nucleo/prompts/compiler/stdlib/layout.md
    M  00_nucleo/prompts/compiler/stdlib/structural/math.md
    M  00_nucleo/prompts/entities/content.md
    M  00_nucleo/prompts/entities/elements/_comum.md
    M  00_nucleo/prompts/entities/elements/math_cancel.md
    M  00_nucleo/prompts/entities/elements/math_underline.md
    M  00_nucleo/prompts/entities/elements/math_vec.md
    M  00_nucleo/prompts/infra/query-helpers.md
    M  01_core/src/compiler/eval/math.rs
    M  01_core/src/compiler/eval/repr.rs
    M  01_core/src/compiler/introspect.rs
    M  01_core/src/compiler/introspect/locatable.rs
    M  01_core/src/compiler/layout/mod.rs
    M  01_core/src/compiler/math/layout/cancel.rs
    M  01_core/src/compiler/math/layout/mod.rs
    M  01_core/src/compiler/math/layout/spacing.rs
    M  01_core/src/compiler/math/layout/tests.rs
    M  01_core/src/compiler/stdlib/structural/math.rs
    M  01_core/src/entities/content.rs
    M  01_core/src/entities/elements/math_cancel.rs
    M  01_core/src/entities/elements/mod.rs
    M  03_infra/src/layout.rs
    M  03_infra/src/query_helpers.rs
    ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
    ?? 00_nucleo/diagnosticos/p1292-contract-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-contract-seal.json
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-a.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-b.md
    ?? 00_nucleo/diagnosticos/p1292-l0-dependency-amendment-1.md
    ?? 00_nucleo/diagnosticos/p1292-l0-oracle-amendment-2.md
    ?? 00_nucleo/diagnosticos/p1292-manifest.json
    ?? 00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-red-tests-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md
    ?? 00_nucleo/prompts/compiler/layout/flush.md
    ?? 00_nucleo/prompts/entities/elements/flush.md
    ?? 00_nucleo/prompts/wiring/tests/p1292_contract.md
    ?? 01_core/src/compiler/math/layout/underline.rs
    ?? 01_core/src/entities/elements/math_underline.rs
    ?? 04_wiring/tests/p1292_contract.rs

### Harness e RED

Comandos:

    cargo test -p typst-wiring --test p1292_contract --no-run
    cargo test -p typst-wiring --test p1292_contract -- --nocapture

O primeiro terminou com código 0. Captura /tmp/p1292-v3-no-run.out:
762 linhas, 32.713 bytes, SHA-256
79889a715c43088c6e0f721f0277b1e15fe523a69646a18e7f4f5dcfea8ff72f.

O segundo terminou com código 101. Captura /tmp/p1292-v3-red.out:
842 linhas, 35.364 bytes, SHA-256
3058c676daf632340397f17c70505ad63d5aaf6508f209eab49df2ec4b87e8ae.

Resultado:

    running 9 tests
    p1292_a_cancel_surface_repr_and_errors ... ok
    p1292_a_cancel_callback_cross_context_span_and_background ... ok
    p1292_b_math_underline_surface_identity_and_errors ... ok
    p1292_b_underbar_fallback_constants_are_not_generic_line_defaults ... ok
    B geometry: text inline extent expected 7.513±0.0001, got 7.238
    C surface/shape: module 'math' does not contain field "vec"
    D surface/PDF: cannot access fields on type function
    test result: FAILED. 4 passed; 5 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 1.82s

Classificação: A GREEN. B surface, identidade booleana e erros GREEN; B
geometria RED exclusivamente pela extensão inline ausente. C e D permanecem
RED por membros públicos ausentes, já depois de os controles materializados
do harness serem válidos. Unknown = 0.

### Owner inline e V1/V2/V5

O único acréscimo em
01_core/src/entities/elements/math_underline.rs está sob cfg(test); não houve
mudança de lógica produtiva.

Comandos:

    cargo test -p typst-core p1292_math_underline_owner_preserva_body_estrutural -- --nocapture
    crystalline-lint --checks v1,v2,v5 .

O teste inline terminou GREEN 1/1. Captura
/tmp/p1292-v3-inline-owner.out: 4.104 linhas, 218.287 bytes, SHA-256
30c62d5dc9da1c0aad730f00c1be7a68f64ef2fbd58e5722b49ecb8e181ec355.

O linter terminou com código 0: zero V1, zero V2 e zero V5 nos dois alvos
04_wiring/tests/p1292_contract.rs e
01_core/src/entities/elements/math_underline.rs. A execução global informou
cinco warnings V5 preexistentes/externos a estes alvos
(eval/mod.rs, layout/equation.rs, cancel.rs, stdlib/layout.rs e
math_cancel.rs); não foram alterados por este papel. Captura
/tmp/p1292-v3-v1-v2-v5.out: 15 linhas, 1.099 bytes, SHA-256
cf57d54787671f53cbb6739180e0d790909eea1175cadc9c9326e0c07adceabb.

git diff --check nos três caminhos autorizados terminou sem saída.

Gates complementares do owner L0:

    crystalline-lint --checks v15,v26 .
    cargo fmt --all -- --check

Ambos terminaram com código 0. V15/V26: No violations found, captura
/tmp/p1292-v3-v15-v26.out SHA-256
91a6d3da4ea876cf1389727cd10abe41cf117fa0295b588912b258b93118f0ac.
O fmt-check não produziu saída, SHA-256 vazio
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.

## Resselo v4 — controlos discriminatórios de extensão inline

Medição em `2026-09-01T01:08:19-03:00`, no HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, com working tree não
commitada. Este papel reconhece o contrato v4 selado pelo manifest
`2e0514d1da5f4597bb27598051066dbc2274e0b4d5d8df9a894895c8299bc511`,
canónico
`ac95d6003126b4b3ebc8b0128e5cbca3ac95c05d88f2f0f6ceab930d6fa598b2`,
seal `876a870be1e98474f959c69db13eff926391fafc3fb19f17a3e4425d6200a86a`
e comparação
`4ae63b34df2388100658ceec3dcfa3e3c2113e412f771b2496a6101902a1fb46`.
Os resultados públicos A–D e os valores esperados da linguagem foram
preservados; não houve adaptação ao candidato.

O oráculo passou a tornar explícitos, além dos casos text/display/script/
cramped já presentes, estes controlos v4:

- `plain $x$`, que discrimina a altura inline normalizada;
- linha mista `A $x$ B` versus `A $underline(x)$ B`, com baseline único
  compartilhado e avanço idêntico;
- linha seguinte, com baselines esperadas `[7.513, 21.901]` tanto no
  controlo plain como em underline;
- italic `f`, preservando largura do frame `6.38`, largura da regra `5.39`
  e altura normalizada `7.513`.

O transporte é exclusivamente SVG público: dimensões do root, grupos de
glyphs e elemento `line`. Não usa `query(metadata)` nem transporte
contextual.

### Proveniência da árvore v4

`git status --short` no instante da medição:

    M 00_nucleo/prompts/compiler/eval.md
    M 00_nucleo/prompts/compiler/eval/math.md
    M 00_nucleo/prompts/compiler/eval/repr.md
    M 00_nucleo/prompts/compiler/introspect.md
    M 00_nucleo/prompts/compiler/layout.md
    M 00_nucleo/prompts/compiler/layout/equation.md
    M 00_nucleo/prompts/compiler/math/layout/_comum.md
    M 00_nucleo/prompts/compiler/math/layout/cancel.md
    M 00_nucleo/prompts/compiler/math/layout/spacing.md
    M 00_nucleo/prompts/compiler/math/layout/underline.md
    M 00_nucleo/prompts/compiler/math/layout/vec.md
    M 00_nucleo/prompts/compiler/stdlib/layout.md
    M 00_nucleo/prompts/compiler/stdlib/structural/math.md
    M 00_nucleo/prompts/entities/content.md
    M 00_nucleo/prompts/entities/elements/_comum.md
    M 00_nucleo/prompts/entities/elements/math_cancel.md
    M 00_nucleo/prompts/entities/elements/math_underline.md
    M 00_nucleo/prompts/entities/elements/math_vec.md
    M 00_nucleo/prompts/infra/query-helpers.md
    M 01_core/src/compiler/eval/math.rs
    M 01_core/src/compiler/eval/repr.rs
    M 01_core/src/compiler/introspect.rs
    M 01_core/src/compiler/introspect/locatable.rs
    M 01_core/src/compiler/layout/equation.rs
    M 01_core/src/compiler/layout/mod.rs
    M 01_core/src/compiler/math/layout/cancel.rs
    M 01_core/src/compiler/math/layout/mod.rs
    M 01_core/src/compiler/math/layout/spacing.rs
    M 01_core/src/compiler/math/layout/tests.rs
    M 01_core/src/compiler/stdlib/structural/math.rs
    M 01_core/src/entities/content.rs
    M 01_core/src/entities/elements/math_cancel.rs
    M 01_core/src/entities/elements/mod.rs
    M 03_infra/src/layout.rs
    M 03_infra/src/query_helpers.rs
    ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
    ?? 00_nucleo/diagnosticos/p1292-contract-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-contract-seal.json
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-a.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-b.md
    ?? 00_nucleo/diagnosticos/p1292-l0-dependency-amendment-1.md
    ?? 00_nucleo/diagnosticos/p1292-l0-inline-extent-amendment-3.md
    ?? 00_nucleo/diagnosticos/p1292-l0-oracle-amendment-2.md
    ?? 00_nucleo/diagnosticos/p1292-manifest.json
    ?? 00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-red-tests-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md
    ?? 00_nucleo/prompts/compiler/layout/flush.md
    ?? 00_nucleo/prompts/entities/elements/flush.md
    ?? 00_nucleo/prompts/wiring/tests/p1292_contract.md
    ?? 01_core/src/compiler/math/layout/underline.rs
    ?? 01_core/src/entities/elements/math_underline.rs
    ?? 04_wiring/tests/p1292_contract.rs

### Artefactos v4

- `04_wiring/tests/p1292_contract.rs`: 637 linhas, 24.990 bytes,
  SHA-256
  `e6a6f09b757572303ca9116848450952713efa8d4cf189bf1afc4b6bec5689c2`.
- `01_core/src/entities/elements/math_underline.rs`: byte-identical ao
  resselo v3, 58 linhas, 1.646 bytes, SHA-256
  `6a08c552d47f7a90263b6f3e447f15ee9e42f17885201a866db910edddfa4948`.
- bloco independente desde `#[cfg(test)]`: byte-identical ao resselo v3,
  SHA-256
  `b5ab3f6635c5c36cb037c585062f35f723ed16789ce7dcaf51fab91fb6d3a334`.

### Harness v4 e classificação causal

Comandos:

    cargo test -p typst-wiring --test p1292_contract --no-run
    cargo test -p typst-wiring --test p1292_contract -- --nocapture

O primeiro terminou com código 0. Captura `/tmp/p1292-v4-no-run.out`:
763 linhas, 32.795 bytes, SHA-256
`c3c08ccb6fc7ff4a9b7708896ed86adf38ba596515bda5aec2bb9026dfb5db63`.

O segundo terminou com código 101. Captura `/tmp/p1292-v4-red.out`:
852 linhas, 36.012 bytes, SHA-256
`b93bbf57a7a7492b97db5327f0dbaabd48afd549ad32917224b28f73fc07e3f6`.

Resultado: 11 testes, 4 GREEN e 7 RED.

- A: superfície/repr/erros e callback GREEN.
- B: superfície/identidade/erros e constantes fallback GREEN. Os três RED
  geométricos são discriminatórios da normalização crua: text esperava
  `7.513` e recebeu `9.735`; italic `f` esperava altura `7.513` e recebeu
  `12.386`; o controlo plain `$x$` esperava `7.513` e recebeu `7.359`.
  Logo o RED de B não é falha de transporte.
- C: dois RED semânticos pelo membro público `math.vec` ausente.
- D: dois RED semânticos pelo membro público `place.flush` ausente.
- Unknown = 0.

### Gates v4

Comandos:

    cargo test -p typst-core p1292_math_underline_owner_preserva_body_estrutural -- --nocapture
    crystalline-lint --checks v1,v2,v5 .
    crystalline-lint --checks v15,v26 .
    cargo fmt --all -- --check

O teste inline terminou GREEN 1/1. Captura
`/tmp/p1292-v4-inline-owner.out`: 4.104 linhas, 218.286 bytes, SHA-256
`84b44e33deb934a7fae91b7d059d8755cde23f7f77264521b74d307674cb1494`.

V1/V2/V5 terminou com código 0 e zero ocorrências nos dois alvos protegidos.
A execução global informou seis warnings V5 externos a estes alvos
(`eval/mod.rs`, `layout/equation.rs`, `cancel.rs`,
`math/layout/underline.rs`, `stdlib/layout.rs`, `math_cancel.rs`), que este
papel não alterou. Captura `/tmp/p1292-v4-v1-v2-v5.out`: 18 linhas, 1.325
bytes, SHA-256
`5fb9792f318862b82a23a771a8e3a01d706774acd6b7bdcb9ddac78dc5c92fae`.

V15/V26 terminou com código 0 e `No violations found`; captura
`/tmp/p1292-v4-v15-v26.out`, SHA-256
`91a6d3da4ea876cf1389727cd10abe41cf117fa0295b588912b258b93118f0ac`.
O fmt-check terminou com código 0 e captura vazia, SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

## Resselo v5 — largura pareada e transportes display/cramped

Medição final em `2026-09-01T01:26:59-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Este resselo reconhece o contrato canônico v5
`5f5b4ab885529a213497d274705c66e0d62e3195e2bcfdf02fb98209a5edb7fc`,
seal `42029c04946e45a36c39825720e71f32fde192aa1e93e8f2a408470c56c8fd7d`
e comparison
`4d20beae612d8e9d027fd0b732d87617b7a02b93298b761e36867b2340529175`.

Autorizações causais sucessivas:

- manifest
  `dac1a9fad250ba88f1984edd1904c75757c1762aed1f4039f25cdfaaaf647a2f`:
  substituição das larguras absolutas whole-wrapper Script/Cramped por pares
  base/underline no mesmo renderer e delta horizontal zero;
- manifest
  `ab918d2ac9b4591f7f8ac12e55fa63dc983148a20bb95b17e4bf86f46dc3dcd8`:
  troca exclusiva do wrapper display aninhado pela sintaxe canônica
  `$ underline(x) $`, mantendo `6.292 × 7.359` e uma regra;
- manifest final
  `3986bb59245625c47f0b83f5928ffa42e8c91b103eb6ba2115427c36a5598806`:
  correção da cardinalidade contraditória Cramped para as duas linhas já
  seladas, underline `4.4583pt` e fraction bar `4.5276pt`.

Não houve relaxamento de valor. Permanecem exatas as alturas Text `7.513`,
Display `7.359`, Script `8.459` e Cramped `9.537`; as dimensões/baselines
mixed, avanço da linha seguinte, frame/regra italic, superfície, identidade,
erros e todos os vetores A/C/D são os mesmos. Script usa o par
`$x_(y)$`/`$x_(underline(y))$`; Cramped usa
`$1/y$`/`$1 / underline(y)$`; ambos exigem delta de root width `0` com
tolerância `0.0001`.

### Proveniência da árvore v5

`git status --short` no instante final:

    M 00_nucleo/prompts/compiler/eval.md
    M 00_nucleo/prompts/compiler/eval/math.md
    M 00_nucleo/prompts/compiler/eval/repr.md
    M 00_nucleo/prompts/compiler/introspect.md
    M 00_nucleo/prompts/compiler/layout.md
    M 00_nucleo/prompts/compiler/layout/equation.md
    M 00_nucleo/prompts/compiler/math/layout/_comum.md
    M 00_nucleo/prompts/compiler/math/layout/cancel.md
    M 00_nucleo/prompts/compiler/math/layout/spacing.md
    M 00_nucleo/prompts/compiler/math/layout/underline.md
    M 00_nucleo/prompts/compiler/math/layout/vec.md
    M 00_nucleo/prompts/compiler/stdlib/layout.md
    M 00_nucleo/prompts/compiler/stdlib/structural/math.md
    M 00_nucleo/prompts/entities/content.md
    M 00_nucleo/prompts/entities/elements/_comum.md
    M 00_nucleo/prompts/entities/elements/math_cancel.md
    M 00_nucleo/prompts/entities/elements/math_underline.md
    M 00_nucleo/prompts/entities/elements/math_vec.md
    M 00_nucleo/prompts/infra/query-helpers.md
    M 01_core/src/compiler/eval/math.rs
    M 01_core/src/compiler/eval/repr.rs
    M 01_core/src/compiler/introspect.rs
    M 01_core/src/compiler/introspect/locatable.rs
    M 01_core/src/compiler/layout/equation.rs
    M 01_core/src/compiler/layout/mod.rs
    M 01_core/src/compiler/math/layout/cancel.rs
    M 01_core/src/compiler/math/layout/mod.rs
    M 01_core/src/compiler/math/layout/spacing.rs
    M 01_core/src/compiler/math/layout/tests.rs
    M 01_core/src/compiler/stdlib/structural/math.rs
    M 01_core/src/entities/content.rs
    M 01_core/src/entities/elements/math_cancel.rs
    M 01_core/src/entities/elements/mod.rs
    M 03_infra/src/layout.rs
    M 03_infra/src/query_helpers.rs
    ?? 00_nucleo/diagnosticos/p1292-b-wrapper-width-amendment-4.md
    ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
    ?? 00_nucleo/diagnosticos/p1292-contract-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-contract-seal.json
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-a.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-b.md
    ?? 00_nucleo/diagnosticos/p1292-l0-dependency-amendment-1.md
    ?? 00_nucleo/diagnosticos/p1292-l0-inline-extent-amendment-3.md
    ?? 00_nucleo/diagnosticos/p1292-l0-oracle-amendment-2.md
    ?? 00_nucleo/diagnosticos/p1292-manifest.json
    ?? 00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-red-tests-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md
    ?? 00_nucleo/prompts/compiler/layout/flush.md
    ?? 00_nucleo/prompts/entities/elements/flush.md
    ?? 00_nucleo/prompts/wiring/tests/p1292_contract.md
    ?? 01_core/src/compiler/math/layout/underline.rs
    ?? 01_core/src/entities/elements/math_underline.rs
    ?? 04_wiring/tests/p1292_contract.rs

### Execução final v5

Comandos:

    cargo test -p typst-wiring --test p1292_contract --no-run
    cargo test -p typst-wiring --test p1292_contract -- --nocapture

O primeiro terminou com código 0. Captura `/tmp/p1292-v5c-no-run.out`:
763 linhas, 32.795 bytes, SHA-256
`1b0a9af8ad9e87a35d515532eeb92741e355f2237ffcc0564843f232bafeb4c4`.

O segundo terminou com código 101 apenas pelos RED requeridos de C/D.
Captura `/tmp/p1292-v5c-red.out`: 840 linhas, 35.240 bytes, SHA-256
`578dd9a62d1e24e240b8f64b457bf4bbd699dc7b5e2701056e008a04d721b1f3`.

Resultado final: 11 testes, 7 GREEN e 4 RED.

- A: dois testes GREEN.
- B: cinco testes GREEN, incluindo width pareado Script/Cramped, vertical
  exato dos quatro estilos, display canônico, duas linhas Cramped com ambas
  as larguras seladas, mixed/avanço e italic frame/rule.
- C: dois RED semânticos por `math.vec` ausente.
- D: dois RED semânticos por `place.flush` ausente.
- Unknown = 0.

Artefato final `04_wiring/tests/p1292_contract.rs`: 663 linhas, 26.176 bytes,
SHA-256
`73fad840f9ac04e0cda0404dc7412c9f5d04a6acde1ec1fc789c605ec530f429`.
O ficheiro owner e o bloco `cfg(test)` independente permanecem inalterados,
respetivamente
`6a08c552d47f7a90263b6f3e447f15ee9e42f17885201a866db910edddfa4948`
e `b5ab3f6635c5c36cb037c585062f35f723ed16789ce7dcaf51fab91fb6d3a334`.

Gates finais:

    cargo fmt --all -- --check
    crystalline-lint --checks v1,v2,v5 .

Ambos terminaram com código 0. O fmt não produziu saída, SHA-256 vazio
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
V1/V2/V5 tem zero ocorrência no oráculo/owner protegidos; a execução global
registou cinco warnings V5 externos aos alvos. Captura
`/tmp/p1292-v5c-v1-v2-v5.out`: 15 linhas, 1.099 bytes, SHA-256
`a3dff1971e47cb82542048c675dec48ff1bbf78b75f874b0b91dc8081d028b40`.

## Resselo v6 — região efetiva de vec e morfologia pública

Medição final em `2026-09-01T02:06:04-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Entradas v6: contrato canônico
`c5eb479c218354af0910c17eaf24447122b8d91775d70c3031950c426ff27f3b`,
seal `28b5eec0abb668408b68672a0b8230628f08393022d9602827a4b04e540e5410`,
comparison
`bf3b06f4413f4021a138bd3bbdd489c15583fee29d5b7b3784703c5a0624f28a`
e amendment-5
`f9b266fa286524cd326de63024e068d42bf478a218336080b27770d5b79531f7`.

Autorizações causais:

- manifest
  `57d73478019b7ad5f1a3750c8ee08d0bec048f908fb5c7b194f9b91177797aa8`:
  retirar somente o transporte C que usava named `measure`, compilar o vec
  diretamente em páginas finite/auto e extrair duas baselines-filho;
- manifest final
  `6413da8c4bdd943de7645200d31b692a4a4fa3e77113f2709399cf38e881e1c4`:
  substituir a igualdade proibida de bytes SVG por `repr` público JSON,
  sem normalizar o resultado do candidato.

O transporte C final não usa `measure`, metadata nem query. Para cada gap
`0%`, `10%`, `1em` e `10% + 1em`, compila diretamente:

    #set page(width:auto,height:<100pt|auto>,margin:0pt)
    #math.equation(math.vec([A], [B], gap: <gap>))

Cada SVG exige root finito, root finite exatamente `100pt` quando aplicável,
exatamente duas baselines-filho e os deltas selados:

- finite: `7.6692`, `17.6692`, `18.6692`, `28.6692`;
- auto: `7.6692`, `7.6692`, `18.6692`, `18.6692`.

O controlo syntax compara `repr($vec(a,b)$.body)` e
`repr(math.vec([a],[b]))` contra o JSON exato
`"vec(children: ([a], [b]))"`, congelando cardinalidade dois, ordem `a,b` e
morfologia content. O candidato devolve para a sintaxe
`"vec(children: (a, b))"`; isso permanece RED, não foi normalizado.

### Proveniência da árvore v6

`git status --short` no instante final:

    M 00_nucleo/prompts/compiler/eval.md
    M 00_nucleo/prompts/compiler/eval/math.md
    M 00_nucleo/prompts/compiler/eval/repr.md
    M 00_nucleo/prompts/compiler/introspect.md
    M 00_nucleo/prompts/compiler/layout.md
    M 00_nucleo/prompts/compiler/layout/equation.md
    M 00_nucleo/prompts/compiler/math/layout/_comum.md
    M 00_nucleo/prompts/compiler/math/layout/cancel.md
    M 00_nucleo/prompts/compiler/math/layout/spacing.md
    M 00_nucleo/prompts/compiler/math/layout/underline.md
    M 00_nucleo/prompts/compiler/math/layout/vec.md
    M 00_nucleo/prompts/compiler/stdlib/layout.md
    M 00_nucleo/prompts/compiler/stdlib/structural/math.md
    M 00_nucleo/prompts/entities/content.md
    M 00_nucleo/prompts/entities/elements/_comum.md
    M 00_nucleo/prompts/entities/elements/math_cancel.md
    M 00_nucleo/prompts/entities/elements/math_underline.md
    M 00_nucleo/prompts/entities/elements/math_vec.md
    M 00_nucleo/prompts/infra/query-helpers.md
    M 01_core/src/compiler/eval/math.rs
    M 01_core/src/compiler/eval/repr.rs
    M 01_core/src/compiler/introspect.rs
    M 01_core/src/compiler/introspect/locatable.rs
    M 01_core/src/compiler/layout/equation.rs
    M 01_core/src/compiler/layout/mod.rs
    M 01_core/src/compiler/math/layout/cancel.rs
    M 01_core/src/compiler/math/layout/mod.rs
    M 01_core/src/compiler/math/layout/spacing.rs
    M 01_core/src/compiler/math/layout/tests.rs
    M 01_core/src/compiler/stdlib/structural/math.rs
    M 01_core/src/entities/content.rs
    M 01_core/src/entities/elements/math_cancel.rs
    M 01_core/src/entities/elements/mod.rs
    M 03_infra/src/layout.rs
    M 03_infra/src/query_helpers.rs
    ?? 00_nucleo/diagnosticos/p1292-b-wrapper-width-amendment-4.md
    ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
    ?? 00_nucleo/diagnosticos/p1292-c-region-transport-amendment-5.md
    ?? 00_nucleo/diagnosticos/p1292-contract-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-contract-seal.json
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-a.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-b.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-c.md
    ?? 00_nucleo/diagnosticos/p1292-l0-dependency-amendment-1.md
    ?? 00_nucleo/diagnosticos/p1292-l0-inline-extent-amendment-3.md
    ?? 00_nucleo/diagnosticos/p1292-l0-oracle-amendment-2.md
    ?? 00_nucleo/diagnosticos/p1292-manifest.json
    ?? 00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-red-tests-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md
    ?? 00_nucleo/prompts/compiler/layout/flush.md
    ?? 00_nucleo/prompts/entities/elements/flush.md
    ?? 00_nucleo/prompts/wiring/tests/p1292_contract.md
    ?? 01_core/src/compiler/math/layout/underline.rs
    ?? 01_core/src/compiler/math/layout/vec.rs
    ?? 01_core/src/entities/elements/math_underline.rs
    ?? 01_core/src/entities/elements/math_vec.rs
    ?? 04_wiring/tests/p1292_contract.rs

### Execução final v6

Comandos:

    cargo test -p typst-wiring --test p1292_contract --no-run
    cargo test -p typst-wiring --test p1292_contract -- --nocapture

O primeiro terminou com código 0. Captura `/tmp/p1292-v6b-no-run.out`:
763 linhas, 32.795 bytes, SHA-256
`4d7b2a68efa1ab89d19884e094575470de1e6ba8ec9c33aba84226e3c51bf621`.

O segundo terminou com código 101 apenas pelos três RED requeridos. Captura
`/tmp/p1292-v6b-red.out`: 818 linhas, 34.758 bytes, SHA-256
`24f3d07f3df42a4b03a5817adfc0de5eedce3b9d32846015bb7269cead65cdb3`.

Resultado: 11 testes, 8 GREEN e 3 RED.

- A e B integralmente GREEN.
- C superfície/repr/erros e os oito casos regionais GREEN.
- C syntax morphology RED exato: candidato
  `vec(children: (a, b))`, esperado `vec(children: ([a], [b]))`.
- D mantém dois RED semânticos porque `place.flush` está ausente.
- Unknown = 0.

Artefato final `04_wiring/tests/p1292_contract.rs`: 658 linhas, 26.043 bytes,
SHA-256
`063f1cb4c7d3a6d3fa06a5bff78da5369ceecbd6b251f319cb10707b8f4eb6c5`.
O owner underline e seu bloco `cfg(test)` independente permanecem
byte-identical, respetivamente
`6a08c552d47f7a90263b6f3e447f15ee9e42f17885201a866db910edddfa4948`
e `b5ab3f6635c5c36cb037c585062f35f723ed16789ce7dcaf51fab91fb6d3a334`.

Gates finais:

    cargo fmt --all -- --check
    crystalline-lint --checks v1,v2,v5 .

Ambos terminaram com código 0. O fmt não produziu saída, SHA-256 vazio
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
V1/V2/V5 tem zero ocorrência no oráculo e owner protegido; quatro warnings
V5 externos ficaram registados. Captura `/tmp/p1292-v6b-v1-v2-v5.out`: 12
linhas, 879 bytes, SHA-256
`fe5e07cec86209326545a3320cc0b0cefca23b5447a01641e4b2195cf73d9193`.

## Resselo v7 — edges da projeção morfológica C

Medição final em `2026-09-01T02:22:29-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Entradas: manifest ack
`36a9aefd66b257b98a33b3469086d131efd76c7714c3d7188075e18191602d9a`,
contrato canônico
`415d8abb1cd5df02fcc08fd2ea90c2d423b67751ef0a09818fb8cc511b225244`,
seal `dc59cefb3aa50cbf38d38b4452aacdddf3b5eeb5a1484d9281206cdc17c65bcc`
e amendment-6
`764d2282d0d5067375b7c32796fe7ef2e3db70f81da18c2ea6dfe87cbade357a`.

O `git status --short` tinha exatamente o mesmo conjunto listado na secção
v6, mais o novo artefato causal não rastreado
`00_nucleo/diagnosticos/p1292-c-syntax-repr-amendment-6.md`; não houve outra
diferença no conjunto de paths. O estado continua não commitado e inclui
alterações P1292 de outros papéis, preservadas por este autor.

O oráculo já continha o discriminador de identificadores diretos. Foram
acrescentados no mesmo teste, sem nova contagem de testes, os edge controls
v7 que faltavam:

- números `1,23` (`MathText`) versus `[1],[23]`;
- identificadores multigrapheme resolvidos `alpha,beta` versus `[α],[β]`;
- texto quoted `"foo","bar"` versus markup `[foo],[bar]`;
- filhos estruturados `(a+b),(c)` versus os mesmos math-bodies, exigindo a
  forma própria `lr(body: sequence(...))` sem projeção recursiva;
- markup `strong([a]), emph([b])`, exigindo suas repr próprias sem double-wrap.

Cada par usa somente `repr` público JSON e exige a forma vanilla exata. O par
de identificadores permanece primeiro, logo o candidato atual conserva o RED
selado exato em vez de deslocar a testemunha para outro edge:
`"vec(children: (a, b))"` contra
`"vec(children: ([a], [b]))"`.

### Execução v7

Comandos:

    cargo test -p typst-wiring --test p1292_contract --no-run
    cargo test -p typst-wiring --test p1292_contract -- --nocapture

O primeiro terminou com código 0. Captura `/tmp/p1292-v7-no-run.out`: 763
linhas, 32.795 bytes, SHA-256
`9fb3aaf01ab7add78fe74ec7fe067079c5f6c9c8a910b955453a45dd97342042`.

O segundo terminou com código 101 somente pelos três RED requeridos. Captura
`/tmp/p1292-v7-red.out`: 818 linhas, 34.755 bytes, SHA-256
`d8923f21cffc3d7fedc3a78ae2e525b16d2fc67f921fa4a5ae175b001b0d6ef0`.

Resultado: 11 testes, 8 GREEN e 3 RED. A-B, C surface/repr/errors e C region
GREEN; C syntax morphology RED exato; D dois RED semânticos por
`place.flush` ausente; Unknown = 0.

Artefato final `04_wiring/tests/p1292_contract.rs`: 702 linhas, 27.236 bytes,
SHA-256
`6ca930ea9b03c8389c9b9bf0d749e90251ccf81322411b7625f26e4fc48a0533`.
O owner underline e seu bloco `cfg(test)` independente seguem byte-identical:
`6a08c552d47f7a90263b6f3e447f15ee9e42f17885201a866db910edddfa4948`
e `b5ab3f6635c5c36cb037c585062f35f723ed16789ce7dcaf51fab91fb6d3a334`.

Gates:

    cargo fmt --all -- --check
    crystalline-lint --checks v1,v2,v5 .

Ambos terminaram com código 0. Fmt vazio, SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
V1/V2/V5 tem zero ocorrência no oráculo/owner protegido; cinco warnings V5
externos, incluindo o owner `repr.rs` deliberadamente ainda não materializado,
ficaram registados. Captura `/tmp/p1292-v7-v1-v2-v5.out`: 15 linhas, 1.093
bytes, SHA-256
`d03236401f2e22c3d04066b6827badde27481857a1a958c4820df88b01d8b172`.

## Reparo estreito v7 — MathLr estruturado fora do lote

Medição em `2026-09-01T02:32:26-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Autorização `structured_edge_scope_repair` no manifest SHA-256
`c65230702fcff93212adab60531b5c106b69b7958995d49569d7eb89d91780d9`.
O conjunto de paths de `git status --short` era exatamente o mesmo listado
na secção v7; somente o conteúdo do manifest/oráculo/recibo avançou dentro
dos paths já presentes.

Foi substituída exclusivamente a asserção estruturada MathLr. Os pares
diretos identifier/número/alpha-beta/quoted, strong/emph, região, A-B e D
permaneceram literais. O novo controlo exige:

1. `repr($vec((a+b),(c))$.body)` e
   `repr(math.vec($(a+b)$.body,$(c)$.body))` convergentes entre si no
   candidato;
2. a forma estruturada não ser convertida na projeção direta recursiva
   `vec(children: ([(a+b)], [(c)]))`.

Medição pública exata:

- candidato, nas duas formas:
  `vec(children: ((a+b), (c)))`;
- vanilla ratificado:
  `vec(\n  children: (\n    lr(body: sequence([(], [a], [+], [b], [)])),\n    lr(body: sequence([(], [c], [)])),\n  ),\n)`.

Essa divergência é evidência preexistente do formatter genérico MathLr e
fica fora do escopo P1292. Não é declarada sucesso de paridade e não é
`Unknown` de vetor obrigatório. O vetor requerido é somente convergência das
duas entradas candidatas e ausência de projeção leaf recursiva; ambos passam.

### Execução do reparo

Comandos:

    cargo test -p typst-wiring --test p1292_contract --no-run
    cargo test -p typst-wiring --test p1292_contract -- --nocapture

O primeiro terminou com código 0. Captura
`/tmp/p1292-v7-structured-repair-no-run.out`: 763 linhas, 32.795 bytes,
SHA-256
`4d7b2a68efa1ab89d19884e094575470de1e6ba8ec9c33aba84226e3c51bf621`.

O segundo terminou com código 101 apenas pelos dois RED D requeridos.
Captura `/tmp/p1292-v7-structured-repair-red.out`: 812 linhas, 34.438 bytes,
SHA-256
`8bf62564fbf3310cada1ba5c3ac2be05bdf5ba98eab8a0cbfe3d15c46cfb127f`.

Resultado: 11 testes, 9 GREEN e 2 RED. A-B-C integralmente GREEN; somente D
mantém dois RED semânticos por `place.flush` ausente. Unknown requerido = 0.

Artefato final `04_wiring/tests/p1292_contract.rs`: 718 linhas, 27.871 bytes,
SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

Gates:

    cargo fmt --all -- --check
    crystalline-lint --checks v1,v2,v5 .

Ambos terminaram com código 0. Fmt vazio, SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
V1/V2/V5 tem zero ocorrência no oráculo protegido; quatro warnings V5
externos ficaram registados. Captura
`/tmp/p1292-v7-structured-repair-v1-v2-v5.out`: 12 linhas, 879 bytes,
SHA-256
`fe5e07cec86209326545a3320cc0b0cefca23b5447a01641e4b2195cf73d9193`.

## Ack independente do contrato v10

Medição em `2026-09-01T08:57:45-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Este papel reconhece, sem adaptar o oráculo ou as expectativas:

- manifest
  `aea3a7825f2268dee5c66f5a5c31422f39f5c36fa90e6243784bae387806a59e`;
- contrato canônico v10
  `3b1ead87735df2e4e7c72a84c899a45c61f45067bf75f808812e9a4f9056bd69`;
- seal v10
  `f798bfc18ebdcd4b9bcc684f3b95d398fbb37b072f6e8ea89e0fb7676647d630`;
- receipt do contrato
  `b79af4ec4c9a5bdc59a34b28168dfb5f78c93666f8b0cbc54557bc44df96baa7`;
- amendment-9
  `92e83d0afb8d323823b0815eca62e6a8653d354e782284149651a38fa91d487e`;
- lots
  `e096a694f5c6fad1fb5623ec2e3544ddc878fd407befde14bcf296459b87360f`;
- comparison
  `513120408189be25876812533c2860a262edc9fd3e4f00c7db67add2464d0875`.

O gate humano ADR-0127 está registado no seal: `Autorizo` em
`2026-09-01T08:46:11-03:00`, especificamente para tornar o clearance
omitido relativo `1.5em` e completar a recomposição/placement internos.

O oráculo protegido permaneceu byte-identical antes e depois da execução:
`04_wiring/tests/p1292_contract.rs`, SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

### Proveniência da árvore no ack v10

`git status --short` no instante da medição:

    M 00_nucleo/prompts/compiler/eval.md
    M 00_nucleo/prompts/compiler/eval/math.md
    M 00_nucleo/prompts/compiler/eval/repr.md
    M 00_nucleo/prompts/compiler/introspect.md
    M 00_nucleo/prompts/compiler/layout.md
    M 00_nucleo/prompts/compiler/layout/cursor.md
    M 00_nucleo/prompts/compiler/layout/equation.md
    M 00_nucleo/prompts/compiler/layout/place.md
    M 00_nucleo/prompts/compiler/math/layout/_comum.md
    M 00_nucleo/prompts/compiler/math/layout/cancel.md
    M 00_nucleo/prompts/compiler/math/layout/spacing.md
    M 00_nucleo/prompts/compiler/math/layout/underline.md
    M 00_nucleo/prompts/compiler/math/layout/vec.md
    M 00_nucleo/prompts/compiler/stdlib/_comum.md
    M 00_nucleo/prompts/compiler/stdlib/layout.md
    M 00_nucleo/prompts/compiler/stdlib/structural/math.md
    M 00_nucleo/prompts/entities/content.md
    M 00_nucleo/prompts/entities/elements/_comum.md
    M 00_nucleo/prompts/entities/elements/math_cancel.md
    M 00_nucleo/prompts/entities/elements/math_underline.md
    M 00_nucleo/prompts/entities/elements/math_vec.md
    M 00_nucleo/prompts/infra/query-helpers.md
    M 01_core/src/compiler/eval/math.rs
    M 01_core/src/compiler/eval/mod.rs
    M 01_core/src/compiler/eval/repr.rs
    M 01_core/src/compiler/introspect.rs
    M 01_core/src/compiler/introspect/locatable.rs
    M 01_core/src/compiler/layout/cursor.rs
    M 01_core/src/compiler/layout/equation.rs
    M 01_core/src/compiler/layout/mod.rs
    M 01_core/src/compiler/math/layout/cancel.rs
    M 01_core/src/compiler/math/layout/mod.rs
    M 01_core/src/compiler/math/layout/spacing.rs
    M 01_core/src/compiler/math/layout/tests.rs
    M 01_core/src/compiler/stdlib/layout.rs
    M 01_core/src/compiler/stdlib/mod.rs
    M 01_core/src/compiler/stdlib/structural/math.rs
    M 01_core/src/entities/content.rs
    M 01_core/src/entities/elements/math_cancel.rs
    M 01_core/src/entities/elements/mod.rs
    M 03_infra/src/layout.rs
    M 03_infra/src/query_helpers.rs
    ?? 00_nucleo/diagnosticos/p1292-b-wrapper-width-amendment-4.md
    ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
    ?? 00_nucleo/diagnosticos/p1292-c-region-transport-amendment-5.md
    ?? 00_nucleo/diagnosticos/p1292-c-syntax-repr-amendment-6.md
    ?? 00_nucleo/diagnosticos/p1292-contract-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-contract-seal.json
    ?? 00_nucleo/diagnosticos/p1292-d-flow-resumption-amendment-9.md
    ?? 00_nucleo/diagnosticos/p1292-d-flush-pagination-amendment-8.md
    ?? 00_nucleo/diagnosticos/p1292-d-stdlib-facade-amendment-7.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-a.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-b.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-c.md
    ?? 00_nucleo/diagnosticos/p1292-implementation-receipt-d.md
    ?? 00_nucleo/diagnosticos/p1292-l0-dependency-amendment-1.md
    ?? 00_nucleo/diagnosticos/p1292-l0-inline-extent-amendment-3.md
    ?? 00_nucleo/diagnosticos/p1292-l0-oracle-amendment-2.md
    ?? 00_nucleo/diagnosticos/p1292-manifest.json
    ?? 00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-red-tests-receipt.md
    ?? 00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md
    ?? 00_nucleo/prompts/compiler/layout/flush.md
    ?? 00_nucleo/prompts/entities/elements/flush.md
    ?? 00_nucleo/prompts/wiring/tests/p1292_contract.md
    ?? 01_core/src/compiler/layout/flush.rs
    ?? 01_core/src/compiler/math/layout/underline.rs
    ?? 01_core/src/compiler/math/layout/vec.rs
    ?? 01_core/src/entities/elements/flush.rs
    ?? 01_core/src/entities/elements/math_underline.rs
    ?? 01_core/src/entities/elements/math_vec.rs
    ?? 04_wiring/tests/p1292_contract.rs

### Execução focal v10

Comandos:

    cargo test -p typst-wiring --test p1292_contract --no-run
    cargo test -p typst-wiring --test p1292_contract -- --nocapture

O primeiro terminou com código 0. Captura
`/tmp/p1292-v10-ack-no-run.out`: 764 linhas, 32.875 bytes, SHA-256
`efeca7eb6012e17588054d2ee10d89af10396dc81d6f8b8686679d01cf6743ea`.

O segundo terminou com código 101 somente pelo RED geométrico D requerido.
Captura `/tmp/p1292-v10-ack-red.out`: 788 linhas, 33.966 bytes, SHA-256
`a57caa8bdd99098fee2cd205171c4f4e38065280a44ff52f66a9b0d415554f00`.

Resultado: 11 testes, 10 GREEN e 1 RED.

- A-B-C integralmente GREEN;
- D surface/repr/erros GREEN;
- D geometry RED em placement/resumption:
  `FLOAT_BEFORE yMin: expected 17.404±0.002, got 10.166`;
- required Unknown = 0.

Este ack não classifica o candidato v9 como preservado: o único vetor D
pendente continua causalmente RED. Nenhum código produtivo, L0, seal, teste,
ataque ou veredito foi alterado.

## Ack v11 e testes internos independentes do checkpoint

Medição final em `2026-09-01T09:57:05-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
O estado exato continha 76 entradas em `git status --short`; a captura
`/tmp/p1292-v11-status.txt` tinha 3.685 bytes e SHA-256
`631ee05b1772770b8c92f074dece2801e59e4e497ac644ffcca3ed95cb5df901`.
Em relação ao estado v10 já enumerado acima, as entradas adicionais relevantes
eram `M 00_nucleo/prompts/compiler/layout/tests.md`,
`M 01_core/src/compiler/layout/place.rs`,
`M 01_core/src/compiler/layout/tests.rs` e
`?? 00_nucleo/diagnosticos/p1292-d-checkpoint-amendment-10.md`; as demais
entradas permaneceram as já registradas no ack v10.

Inputs causais reconhecidos sem adaptar o oráculo protegido:

- manifest v11:
  `782c97a5d9752a61e5f65d51b0d46ebac9a8fab19664b307458560e20a0f10fc`;
- contrato canônico v11:
  `671fd53142624d9cceaa28d4ebaf5676eb07f4bfa5d3432d1fd2e030ad29d9f2`;
- seal v11:
  `256f5cbee59ae11c7bf2a1150865a95694753a1dfd24197de837a4f464f52daa`;
- receipt do contrato v11:
  `4cd26f7bd20a4e343f4f59231584454202975d0da9fea9c7eb85a173af7e6148`;
- amendment-10:
  `8aac16ad279e22495ed3ce805462a2ed36a44d30cef6df4b62a16b85f52c512f`;
- lots v11:
  `867b5fe0e61e3bb69a4e54aec4a98710c4acb12c2de03b2f5098d9c890aa79e0`;
- comparison v11:
  `cbcac2aa1efe454a20e9542515211f7fabba4048ee2824cec3d2c2ff37207c01`;
- 26 L0s selados, incluindo
  `00_nucleo/prompts/compiler/layout/tests.md` com SHA-256 físico
  `35f8db27e3594d5089099e72e6b3cac031da2926e7d48d68eb4c2bdb51bd32a6`
  e hash canônico de linhagem `37216fb0`.

### Revisão independente em `layout/tests.rs`

O legacy P245 que atribuía a `clearance` o movimento do anchor físico foi
substituído pelos dois controles ratificados:

1. `p1292_bottom_float_anchor_fisico_independente_do_clearance`: em região
   100pt×100pt, clearance 0pt e 20pt mantêm uma página e o mesmo anchor físico;
2. `p1292_bottom_float_clearance_altera_reserva_e_fitting`: após flow de 80pt,
   0pt mantém o float em p1 e 20pt move-o para p2, preservando o anchor local.

Foi acrescentado
`p1292_flush_checkpoint_atomico_move_todo_o_sufixo`, que exige três páginas,
cardinalidade exatamente um para `PREFLOW`, `FLOAT_BEFORE`, `AFTER_MARKER`,
`AFTER_FLOW` e `FLOAT_AFTER`, e páginas 1/2/3/3/3 respectivamente. A coleta
reconstrói tokens eventualmente segmentados entre folhas textuais do frame,
sem alterar a página exigida.

Artefato final autorizado:

- `01_core/src/compiler/layout/tests.rs`: 22.237 linhas, 849.890 bytes,
  SHA-256
  `60bf37987693cc26af0e509b3d605ee75dc2534e6fa44994a0efb8563bdee28d`;
- header de linhagem: `@prompt-hash 37216fb0`, validado sem ocorrência V5
  nesse owner.

### Execuções focais internas

Comandos:

    cargo test -q -p typst-core p1292_bottom_float -- --nocapture
    cargo test -q -p typst-core p1292_flush_checkpoint_atomico_move_todo_o_sufixo -- --nocapture

O primeiro terminou com código 0: 2 GREEN, 0 falhas, 5.362 filtrados.
Captura `/tmp/p1292-v11-core-controls.out`: 4.129 linhas, 219.504 bytes,
SHA-256
`e946edbc83e97ae2789613420f8912bd05e9b4c660961aa38f1690c94a779a32`.

O segundo terminou com código 101 pelo RED semântico requerido. Todos os cinco
tokens foram encontrados exatamente uma vez; a única asserção que falhou foi:

    AFTER_MARKER ficou na página errada
      left: 2
     right: 3

Captura `/tmp/p1292-v11-core-checkpoint-red.out`: 4.142 linhas,
220.153 bytes, SHA-256
`f03631d06df3814a7504a960e00299dbf5343e84c47ca90bc88bcea2bef28d02`.
Não houve `Unknown` requerido.

### Oráculo protegido v11, inalterado

Comando:

    cargo test -q -p typst-wiring --test p1292_contract -- --nocapture

Terminou com código 101: 11 testes, 10 GREEN e 1 RED. A única falha continuou
em `p1292_d_flush_prefix_suffix_clearance_nested_and_noop`, na obrigação
`AFTER_MARKER` p3 (`left: 2`, `right: 3`). A-B-C e D surface permaneceram
GREEN; required Unknown = 0.

Captura `/tmp/p1292-v11-oracle-red.out`: 777 linhas, 32.932 bytes, SHA-256
`c8ceb6b473c5ebd5574bff0d0513d974201fefe4af041f2e2016e71371c75245`.
O oráculo `04_wiring/tests/p1292_contract.rs` permaneceu byte-identical,
SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

### Gates mecânicos

`cargo fmt --all -- --check` e `git diff --check` terminaram com código 0.
`crystalline-lint --checks v1,v2,v5 .` terminou com código 0 e não reportou
V1/V2/V5 no owner `compiler/layout/tests.rs`; quatro warnings V5 externos ao
owner permaneceram em código produtivo alheio. Captura
`/tmp/p1292-v11-v1-v2-v5.out`: 12 linhas, 881 bytes, SHA-256
`eed4f1f11d9f64dd5e36fbfc9467082362dfd3eba469776c2a177ba7846f1e86`.

Nenhum código produtivo, L0, seal, ataque, veredito ou expectativa do oráculo
protegido foi alterado por este papel.

## Ack v12 — controles independentes de Block e Place sem Flush

Medição final em `2026-09-01T11:13:08-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada:
46 arquivos no `git diff HEAD --stat`, 3.303 inserções e 496 remoções; 78
entradas em `git status --short`.

Entradas causais v12 reconhecidas antes dos testes, sem leitura ou escrita de
código produtivo:

- manifest: `d4b5908c44ac4ef536e474efb67969e89ae52ed6a2431ea86614060c5c032839`;
- contrato canônico:
  `493f5c58ce956b5a142cbf5016db4bbd7e4dc776aaff86749be7ebd156c2e5ef`;
- seal: `d1a04bcc9351d8f63e77c35ee8bfddf581691bf44484c35b9c4975173edaae8f`;
- receipt: `8ea9dfe983d00c6939d29736443b3cdbaacb7f9c970e46fe62b9a85daa2009a8`;
- amendment-11:
  `7f5ce09fbaf6ac871d1e4fb4acab4bf60ffd2722cfeb21f3da2a600d337d72cf`;
- L0 test-only `compiler/layout/tests.md`:
  `deee32e8fadf311c9a8686004e27b098230b7ef348a4d8b09dd91f9f72d4f157`;
  linhagem canônica `9eaf493c`.

O papel materializou somente `01_core/src/compiler/layout/tests.rs` e este
recibo. A execução foi segregada por autoridade de escrita, sem atestação de
isolamento técnico porque todos os papéis compartilham o mesmo filesystem.

### Testes internos independentes

Foram acrescentados dois owners focais e preservado o controle transacional:

1. Block 30pt seguido de Block 10pt exige o gap default relativo de `1.2em`,
   isto é, +13.2pt a 11pt. O bbox ratificado correspondente é
   `AFTER_FLOW=40.604±0.002pt`; sob `FixedMetrics`, o delta esperado entre os
   tokens é 35.5pt.
2. Place não-float com linha pendente e sem Flush deve usar
   `baseline + top-edge`. O bbox ratificado é
   `AFTER_MARKER=47.842±0.002pt`; sob `FixedMetrics`, o delta owner-correct é
   7.7pt.
3. O vetor Flush/replay continua exigindo cardinalidade unitária e páginas
   PREFLOW p1, FLOAT_BEFORE p2, AFTER_FLOW/AFTER_MARKER/FLOAT_AFTER p3. Os bboxes
   `17.404`, `4.642` e `87.404pt` permanecem congelados no oráculo protegido.

Comando final:

    cargo test -q -p typst-core p1292_ -- --nocapture

Resultado: código 101, 17 testes executados, 15 GREEN e 2 RED causais, zero
`Unknown`:

- Block: esperado 35.5pt, obtido 22.3pt — falta exatamente 13.2pt;
- Place: esperado +7.7pt, obtido -20.9pt — ainda ancora antes da origem da
  linha pendente;
- checkpoint/replay e controles P1292 anteriores: GREEN.

Captura `/tmp/p1292-v12-core-red.out`: 4.134 linhas, 220.046 bytes, SHA-256
`3f2e306666b894007c2bae6a99b6f04d81140df582b99f67ca88e20c91ac8a28`.

Artefato test-only final: `01_core/src/compiler/layout/tests.rs`, 22.323
linhas, 853.795 bytes, SHA-256
`5832b044f0f7942604fa69d0d8f65b7ebec8b52eaf113155a97e4ba8b8a9671c`.

### Oráculo protegido v12, byte-idêntico

Comando:

    cargo test -q -p typst-wiring --test p1292_contract -- --nocapture

Resultado: código 101, 11 testes, 10 GREEN e 1 RED. A única falha é o
controle sem Flush: `AFTER_MARKER yMin` esperado `47.842±0.002pt`, obtido
`-9.834pt`. O vetor Flush/replay, inclusive anchors selados, permaneceu GREEN;
A–B–C e D surface permaneceram GREEN; required `Unknown=0`.

Captura `/tmp/p1292-v12-oracle-red.out`: 775 linhas, 32.949 bytes, SHA-256
`4c8f84d2c17723bf7c828797d8719712eb49692bbe272879333b8c369be4fba7`.
O oráculo permaneceu byte-idêntico, SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

### Gates mecânicos

`cargo fmt --all -- --check` e `git diff --check` terminaram com código 0.
`crystalline-lint --checks v1,v2,v5 .` terminou com código 0 e sem V1/V2/V5
no owner `compiler/layout/tests.rs`; seis warnings V5 produtivos externos ao
papel ficaram registrados. Captura `/tmp/p1292-v12-v1-v2-v5.out`, SHA-256
`f90acfc01a8ed387aca7f907b48105b33beee5c8d6923f1dff75fb27201a2c3b`.

Nenhum produto, L0, seal, contrato, oráculo protegido, ataque ou veredito foi
alterado. O produto v12 pode ser liberado ao implementador contra estes dois
REDs independentes.

## Correção test-only v12 — transporte FixedMetrics do Block

Medição final em `2026-09-01T11:51:21-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada:
47 arquivos no `git diff HEAD --stat`, 3.316 inserções e 506 remoções; 79
entradas em `git status --short`.

A expectativa FixedMetrics `35.5pt` do ack imediatamente anterior fica
explicitamente invalidada. Ela somou `13.2pt` ao delta observado no candidato
refutado e omitiu que o contrato v12 exige transportar o fim físico integral
do frame de 30pt. A projeção unitária correta é `30pt + 1.2em @ 11pt =
43.2pt`, corroborada pelos bboxes externos ratificados e pelo oráculo
protegido GREEN. Nenhuma expectativa foi adaptada a uma mecânica privada: a
correção restaura a fórmula normativa do owner Block.

Somente `01_core/src/compiler/layout/tests.rs` e este recibo foram escritos.
Place, replay, produto, L0, manifest, contrato, seal e oráculo protegido
permaneceram congelados. A linhagem test-only continua
`@prompt-hash 9eaf493c`, sem V5 no owner.

### Core P1292

Comando:

    cargo test -q -p typst-core p1292_ -- --nocapture

Resultado: código 0, 17 GREEN, zero falhas, zero ignorados e required
`Unknown=0`. Isto inclui separadamente Block, Place e checkpoint/replay.

Captura `/tmp/p1292-v12-core-green-corrected.out`: 4.117 linhas, 218.853
bytes, SHA-256
`e3e5bbcc82e500726ba70e81b16b5eb88e00a3932bdc6c3ffb78898dbd41c906`.

Artefato test-only corrigido: `01_core/src/compiler/layout/tests.rs`, SHA-256
`92a2239354c3f82b364671ffa2d5d6a26b4767adfe3756571f7cf73b5f4f1c6c`.

### Oráculo protegido congelado

Comando:

    cargo test -q -p typst-wiring --test p1292_contract -- --nocapture

Resultado: código 0, 11 GREEN, zero falhas, zero ignorados e required
`Unknown=0`. Captura `/tmp/p1292-v12-oracle-green-corrected.out`: 763 linhas,
32.458 bytes, SHA-256
`d3f06a5d35c5bab3690cce94dcdd4584fbec2b421e4b0bfd0aa7dd5e365e0d15`.

O oráculo permaneceu byte-idêntico antes e depois da execução, SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

### Gates mecânicos

`cargo fmt --all -- --check` e `git diff --check` terminaram com código 0.
`crystalline-lint --checks v1,v2,v5 .` terminou com código 0 e sem V1/V2/V5
em `compiler/layout/tests.rs`; dois V5 produtivos externos a este papel
permaneceram registrados. Captura
`/tmp/p1292-v12-corrected-v1-v2-v5.out`: 6 linhas, 448 bytes, SHA-256
`8c89b0785469fe82cef89152afc5010b6770edda19041bfe37cb18d61c4816aa`.

Veredito deste papel: testes independentes v12 GREEN, oráculo protegido
GREEN e `Unknown=0`; a expectativa 35.5pt não deve ser usada como evidência
em nenhuma etapa posterior.

## Reabertura test-only P1030 — identidade `MathVec` no observador legado

Medição final em `2026-09-01T12:29:14-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada:
48 arquivos no `git diff HEAD --stat`, 3.326 inserções e 515 remoções; 82
entradas em `git status --short`.

Entradas normativas lidas antes da correção:

- `entities/elements/math_vec.md`, SHA-256
  `72f2ba668f201d0fb0edf7d2d6bd8f5f85aab156b883a8e644b427f7cd2d1afa`:
  `vec` preserva identidade `Content::MathVec` e `delim` próprio;
- `compiler/stdlib/structural/math.md`, SHA-256
  `087e24d091d9ab8a191374d5a1ac8c47f8e01a7e2283602f73cc0028cb53e236`:
  `math.vec` emite `Content::MathVec`;
- `compiler/eval/tests.md`, SHA-256
  `92ac9be1bcaeb65d0cda76603ba7260ffaccda2f6ff7049bf57bff24d6d83871`;
  linhagem vigente do consumer test-only `2c87f3cf`.

Não existe conflito de contrato. O helper legado `find_matrix_delim`
reconhecia somente `Content::MathMatrix` e afirmava incorretamente que `vec`
também era matriz. A correção exclusivamente test-only renomeou-o para
`find_matrix_or_vec_delim` e acrescentou leitura de `Content::MathVec(e).delim`.
As expectativas não foram relaxadas; testes de `mat`, precedência e default
continuam usando o mesmo observável.

### RED → GREEN focal

Antes da correção:

    cargo test -q -p typst-core p1030_set_vec_delim_aplica -- --nocapture

Código 101: `left: None`, `right: Some(('[', ']'))`. Captura
`/tmp/p1292-p1030-helper-red.out`: 4.130 linhas, 219.381 bytes, SHA-256
`059fa2aa5efbced9b135650110c8a99e13ae310e11cb7093823a090926b8f957`.

Depois da correção, o mesmo comando terminou com código 0, 1/1 GREEN.
Captura `/tmp/p1292-p1030-helper-green.out`: 4.117 linhas, 218.834 bytes,
SHA-256
`7a9a7c018ca572f354ef3e1b3d3fbd92548670b74b4ae0d52635c212becbc431`.

### Preservação

Comandos e resultados:

    cargo test -q -p typst-core p1030_ -- --nocapture
    cargo test -q -p typst-core p1292_ -- --nocapture
    cargo test --workspace -q

- P1030: código 0, 7/7 GREEN; captura
  `/tmp/p1292-p1030-related-green.out`, SHA-256
  `98ad7267892dfc2d2e021ac865b02e9ad2d8921d9ffd25f33f50474a381f3b70`;
- P1292 core: código 0, 17/17 GREEN; captura
  `/tmp/p1292-after-p1030-core-green.out`, SHA-256
  `dae12a9cafb90ce5ee894c6b3620217929bcb6f506242f169d639d7baff96fcf`;
- workspace: código 0, incluindo oráculo protegido 11/11 GREEN;
  captura `/tmp/p1292-after-p1030-workspace.out`: 5.102 linhas, 260.636
  bytes, SHA-256
  `9bbf3a460b34ac4ae2644b1cce4f45281fd476254a0f3fd31bdd6063a7dae1f2`.

Required `Unknown=0`. O oráculo protegido permaneceu byte-idêntico,
SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

### Gates e artefatos

`cargo fmt --all -- --check`, `git diff --check` e
`crystalline-lint --checks v1,v2,v5 .` terminaram com código 0. A captura do
linter `/tmp/p1292-after-p1030-v1-v2-v5.out` tem 1 linha, 24 bytes e SHA-256
`91a6d3da4ea876cf1389727cd10abe41cf117fa0295b588912b258b93118f0ac`.

Artefato test-only final `01_core/src/compiler/eval/tests.rs`: 17.357 linhas,
700.653 bytes, SHA-256
`7e5c362b1854214aaedb90e1e01949a78f812164da8fcf9f6a2cdcaa4b052017`.

Nenhum produto, L0, manifest, seal, contrato, oráculo, ataque ou veredito foi
alterado. Execução segregada por autoridade de escrita, sem atestação de
isolamento técnico devido ao filesystem compartilhado.
