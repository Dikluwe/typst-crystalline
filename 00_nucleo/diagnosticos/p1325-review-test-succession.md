# P1325 — revisão da sucessão test-only

Veredito ex ante `PASS_TEST_ONLY_SUCCESSION_AND_DELTA`, aguardando integração
e gates. Regime A/B sem atestação técnica de isolamento, sem selo; revisor
somente lê artefatos julgados e escreve `p1325-review-*`.

## Causa medida

`p1325-workspace-tests.json`, SHA-256
`d276d5a90995ece30bebbb2805141f80ce029254570ecaa595d9401f0c131ead`,
registra os três testes de P1301/P1303/P1306 que ainda exigem Dict total.
Esses testes falham depois da correção de produto expressamente autorizada
pelo L0 P1325; os demais testes core passaram. O L0 de testes foi lido
integralmente pelo revisor e exigia explicitamente aquelas âncoras antigas.
Logo era necessária uma sucessão normativa adicional; não cabe ignorar
falhas nem modificar silenciosamente o par test-only protegido.

Baseline de sucessão SHA-256
`0271fef1319916b703bc0df54061bc9028685fa83e618e0d7b57c539207d943a`
registra `2026-09-09T01:10:31.823484+00:00`, HEAD/diff/stat, fonte e L0
originais. Manifesto separado SHA-256
`6c9e00228545f6101ccff3af7057d7983fc4f376a6ddedf8184055949e3ab685`
abre exclusivamente `compiler/eval/tests.rs` e seu L0, congelando o par
runtime P1325 inteiro e preservando o manifesto/oráculos A/B originais.
L0 test-only normativo conferido:
`71573020241802fb93317588d19c10d0d9f49b9da09d9a85014defbc089ce08b`.

## Julgamento do L0 e delta antes da integração

O adendo test-only sucede expressamente as três proteções de Dict total e
preserva expressões, mensagens, hints, helpers, perfis e demais controles.
Não autoriza mudar produto/API/default/fase. Classificação contínua ADR-0127
é adequada; o conjunto original de owners era incompleto quanto à migração
das regressões históricas, fato registrado sem reescrever o manifesto.

O corpo do teste ainda foi observado idêntico ao baseline, normalizando
somente o header @prompt-hash, depois de redigido o novo L0. O recorder
ampliado confere o manifesto e só excepciona os dois paths test-only exatos;
ele exige o runtime congelado byte a byte. A mudança do recorder é explícita
e seu hash confere com o manifesto sucessor.

Delta independente `p1325-ab-legacy-delta.patch`, SHA-256
`b76e23a2535a83b3d6271d5f32f8eb637f1e8f7cfd999dba0f1cf301f8a9c542`,
contém exatamente três substituições de range de Dict e um rename
descritivo autorizado. Não remove testes, relaxa helpers nem altera outros
controles. As expectativas derivam do identificador nas expressões
inalteradas e da medição vanilla nos quatro perfis;
`p1325-ab-legacy-measure.json`, SHA-256
`d42384304b8529f53361a93038aa799f477473d3a87a1ed51fc17fd24581e3f7`.
Não foram geradas a partir do candidato.

Freeze independente conferido:
`4906f9f39edabef0aac0ecd70e4da4d6cd32755c5b23491304196cb4f4d82af7`.
O esperado para source test-only sem header é
`0b22e2167e25bb24b9d63fcfb45edc3d826e3c58fac3fd87355b137cc7cd9e53`.
Integração literal, linhagem e execução de grupos e workspace completo
ainda devem ser verificadas; essa migração não é novo RED pré-candidato
do produto. A/B runtime original permanece prova independente inalterada.
