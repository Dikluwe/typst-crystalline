# P1326 — freeze e RED válidos antes de C

Manifesto `800db40f2300d0fda0275d06365c5f3019ca0848577ee54a9c834975b231146f`.
Freeze `p1326-ab-freeze.json` SHA-256
`8d2484bc219b16f8a9aa106ec4efff2595bc174c80f888c0da14098385a48064`,
UTC `2026-09-09T01:32:47.106064+00:00`, auditado antes de C.
Todos os onze artefatos listados conferem SHA-256; L0 normalizado confere
`4b3789c3decc0463ba553937ce7595c3a9c0ecc5fae545726ed009ad928d7451`.
Desintegrar literalmente snippet e sucessor legado e retirar somente header
de hash restaura a fonte original do baseline, provando ausência do delta
funcional candidato durante esta auditoria. Isso não prova isolamento técnico.

CLI baseline `p1326-ab-baseline.json` SHA-256
`7d137c0fa1b776b294717b46e24fcea2105617f91662134dce35f7e11c71b08d`:
23 casos nos quatro perfis, 36 RequiredCorrection, 48 PreservedParity,
8 ResidualPreserveBaseline, nenhum Unknown. Todas as saídas integrais estão
congeladas; runner final compara exit/stdout/stderr integralmente com vanilla
para Closure e baseline para os demais. Os residuais são explicitamente
raw Content text ausente e ordem de argumento com panic.

R1 `p1326-ab-tests-r1.rs` SHA-256
`5a1a16d7bc74d14dd305640db4c88fbdbdbce223f5c4e972b2332dd3a7f3e008`
altera exclusivamente literal do controle JSON; medição bilateral anterior
comprova o formato pretty. A expectativa corrigida não veio de C.
Sucessor legado conserva hash `765e4e40f8203c33d42aa267b6ff763ae38d871337033a3183636c2bf4d87ee5`.

RED R1 `p1326-unit-red-r1.json` SHA-256
`7526f89f71f959c58bd7aa99a921f7f124517559db2c6f382a79abcaf82287ee`,
`2026-09-09T01:33:35.724973+00:00`–`01:34:58.739963+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado;
inventários produtivos before/after idênticos, diff/stat no recibo.
Comando release/locked do owner compilou e executou 18 testes: 15 passaram,
3 falharam somente pela mensagem Closure contratada. JSON positivo e demais
fronteiras passaram; objeção r0 resolvida. Nenhuma falha de build/harness é
contada como RED. Recibo r0 permanece imutável.

Gate pré-código de linhagem `p1326-pre-code-lineage.json` registra
`crystalline-lint --checks v5,v15,v26 --fail-on warning .`, exit 0 e nenhuma
violação. Reseal anterior alterou apenas header do owner para `97a2773e`.

Veredito: entrada de C liberada no recorte L0 Closure/With vigente; ainda são
necessários GREEN, comparação final e gates arquiteturais. Nenhuma autorização
nova de API/default/fase. A/B executado sem atestação de isolamento; sem selo
de refinamento e sem alegação de paridade geral.
