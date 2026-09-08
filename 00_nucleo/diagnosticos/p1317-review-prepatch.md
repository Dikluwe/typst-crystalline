# P1317 — GO anterior ao patch produtivo

Revisor `/root/p1317_review`; mesmas capacidades e limitações do preflight.
**GO** para materialização do recorte P1317, após RED local e freeze A/B.
Não é veredito final, selo de refinamento ou atestação técnica de isolamento.

RED `p1317-unit-red.json`, SHA-256
`7d5f5a4af287cbe4ca0d78ffc6f4263a280e87d9457c04a1cdae2510a9b5eb18`:
`cargo test -p typst-core --release p1317 --lib`, target
`/tmp/p1317-target.y5u9ah`, exit 101. Execução de
2026-09-08T14:58:56.834477+00:00 a 2026-09-08T15:02:10.760506+00:00.
Três falhas nas asserções de mensagem (decode, Bytes nativo e Path/Str via
World); controle de UnequalLengths/Unicode válido passa. Falha genuína,
sem erro de compilação usado como RED. Fonte e L0 idênticos antes/depois:
source SHA-256 `7be6d6b1f6892f45e9760a1022900da2420a6d848164f77ab2f6f1a9aafe8866`,
L0 SHA-256 `75a92fc68daac14ab093e1b6838a9401aea17efe575acdebbc971491cda15e71`.
Estado do HEAD e diff/stat integral constam no recibo.

A comparação literal ao snapshot de `p1317-measurement.json` em
2026-09-08T15:00:41.059Z confirmou produção inalterada e preservação integral
do sufixo antigo de testes, incluindo P1315/P1316. Os quatro testes novos
foram somente acrescentados no início do módulo de testes.

Freeze independente `p1317-ab-freeze.json`, SHA-256
`0a701c7f10630c94c131f6e2923b45272a72290d29545592e49d8f370d6169f1`,
de 2026-09-08T15:02:45.464967+00:00: 387 casos em quatro perfis, 1548
expectativas, 208 RED observáveis. A revisão em 2026-09-08T15:03:28.365Z
conferiu os 24 inputs protegidos e ambos os binários; zero divergências.
O source ainda tinha o mesmo hash do RED, logo nenhum candidato produtivo
precedeu esta revisão. Hash normativo L0 congelado:
`059aad946b30515b24ecfdef4c8490ab735d1bb86d374ce1371c899130ab0d51`;
única exclusão declarada: linha canônica `Hash do Código`.

Runner e casos examinados: seleção Utf8 declarada antes do candidato;
baseline precisa chegar à causa UTF-8, vanilla precisa confirmar a causa
ou o erro de excesso explicitamente normativo. Comparação independente das
expectativas ao baseline verificou substituição apenas da primeira linha
nos casos Utf8 e observação integral nos demais. Replays são conferidos
contra candidato P1316 antes da transformação; nenhuma remoção de spans,
traces ou sufixos vanilla é usada para alegar igualdade total.

Controles incluem header/dados em ambos os modos, byte isolado/overlong/
surrogate/truncamento/acima do limite, quoting multilinha, With/Args/map/sink,
named anterior, excesso, Path/Str com arquivo real, UnequalLengths antes de
Utf8, Unicode válido, opções/I/O e outros loaders. Path/Str são declarados
detached no baseline; a inspeção local via World cobre o complemento puro.
Unknown bloqueia: identidade, drift, processo, ausência ou duplicata não
viram sucesso implícito. Replays normal/repeat/reverse e gates finais pendentes.
