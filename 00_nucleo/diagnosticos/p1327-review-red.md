# P1327 — revisão independente do RED pré-candidato

Regime A/B sem atestação de isolamento; continuidade da auditoria
`p1327-review-pre-c-tests.md`. Manifesto congelado
`cca33a6a95d40efa74d6dc8f7f910a711968a4c7bb771393ffc975bc9e515ea8`.

Recibo `p1327-unit-red.json`, SHA-256
`b7b598a859f57d1f368596899a457faf62e8f15ddd8062bf4da22d4b0eef9f1d`,
executado entre 2026-09-09T10:49:12.200422 e 10:50:33.655505 UTC,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado
e inventário/diff/stat before/after preservados no recibo.

Comando real: `cargo test --release --locked -p typst-core p1327 -- --nocapture`,
target `/tmp/p1327-target.k9Mq0s`. Compilação concluída e seis testes executados.
Warnings do compilador Rust não impediram execução e não são o diagnóstico
de linguagem sob teste.

## Causa verificada nas saídas privadas

- Bare std: observado side vazio, count 0 contra 1.
- Dois imports em posições distintas: observado side vazio, count 0 contra 2.
- Import seguido de erro posterior: observado side vazio, count 0 contra 1.
- Origem importada: resultado de valor já passou; observado side vazio,
  count 0 contra 1 no Source importado.
- Os dois grupos negativos, outras formas de import e erros antes do
  binding, passaram.

Resultado: 2 testes passaram, 4 falharam, exit 101, exclusivamente pela
ausência do warning contratado. Não houve erro de fixture/build ou falha
fora do recorte. O RED R0 é semanticamente válido para iniciar C depois da
integração final congelada. O corpus bilateral pré-C completo foi auditado
separadamente; o panic inicial de um grupo Rust não é tomado como prova de
execução de todas as iterações desse grupo.

## Revisão mecânica R1

`p1327-ab-r1-format.json`, SHA-256
`a69490c148f7e490fbb5e9dad131b5aecd11af221eaf8c5fba0b0ccfd79c115c`,
é aditivo, criado antes de C às 10:51:09.491126 UTC. Li o full_diff:
somente braces, trailing commas e quebras de linha de rustfmt em dois
matches das expectativas legadas. Preserva os mesmos seis casos, ranges,
asserts e branches de laterais vazios; snippet/oracle/runner R0 intactos.
Essa revisão não muda intenção ou expected nem invalida a causa RED R0.
Pai optou por repetir RED no texto final formatado; revisão do novo recibo
será aditiva e não sobrescreverá este registro.

## Adendo — RED e integração R1 confirmados

`p1327-unit-red-r1.json`, SHA-256
`80b93c673532a28b46001626e375b8f704a32e77a125756cfd08e7d467d1e6fc`,
repete o mesmo comando entre 10:51:52.997530 e 10:53:17.038661 UTC, com
proveniência completa before/after. Compilação concluída; mesmos dois grupos
de controles passam, mesmos quatro grupos falham só por count de warnings
0 contra 1/2. Exit 101 tem a mesma causa contratada e nenhum desvio novo.

Reconstruí independentemente o arquivo usando baseline original,
`canonical_legacy_successors` R1 e snippet R0 congelado. Coincide
byte a byte com o arquivo atual, SHA-256
`dd017795b6b9e9f8977d2ba2a2b3dac65979b6d7708f19bba747db4fa9e026a8`.
O corpo runtime de modules ainda coincide com o baseline excluído header.

Gate pré-C final: PASS. Corpus congelado completo, sucessão estrita,
integração exata e RED semântico no arquivo final autorizam avançar a C
dentro do owner e obrigação já aprovados. Não é veredito GREEN ou closure.
