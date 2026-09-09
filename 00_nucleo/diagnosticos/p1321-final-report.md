# P1321 — CSV deve rejeitar argumentos antes de abrir ou interpretar dados

Estado: implementado e verificado em R2; PASS no recorte congelado.

## Correção delimitada

P1320 encontrou duas falhas de comportamento: CSV aceitava argumento excedente
quando os dados eram válidos e devolvia erro de parsing quando deveria rejeitar
o excedente. Também escolhia unknown antes de erros na fonte/opções. O recorte
P1321 corrige esse validador, incluindo missing source e uso indevido de source
como named. Não muda o parser nem a política de resolução de caminhos.

Ordem contratada: fonte/cast → delimiter → row-type → primeiro remanescente
causal → leitura/parser. O diagnóstico do remanescente aponta ao argumento
completo; erro de cast/opção aponta ao valor; missing aponta à chamada. Quando
não há positional mas existe named source, há conselho para remover `source:`.
Com positional presente, named source é apenas um remanescente desconhecido.

Continuam abertas coerções Symbol, resolução causal de strings/imports,
diagnóstico externo de arquivo UTF-8 válido e I/O legado. `array(Bytes)` é
outra frente. `csv.encode` não é dívida: P1320 confirmou sua ausência também
no vanilla ratificado. Não há alegação de paridade global CSV.

## Medição e baseline

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093` mais working tree P1319/P1320
não commitado. Fonte/L0 completos, diff/stat e UTC estão em
`p1321-baseline.json` (SHA-256
`4b4de0feeb4c1df1b9007de81f11f6a4d1ae90b25cdc1c9ca1eadaa211e58a96`).
O preflight V5/V15/V26 saiu 0, antes da alteração normativa.

`p1321-measurement.json` (SHA-256
`f13c4ea33fe8d8f91a5ef443d3183d1cbb3f77d9ac6726c0e4c5e23c54fbef05`)
registra as sondas bilaterais pré-L0, comandos, horários, fontes e estados.
Vanilla upstream ratificado `a51e02804`, binário `/usr/local/bin/typst`, SHA
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline cristalino `/tmp/p1319-target.VqXtmj/release/typst`, SHA
`37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd`.
Target exclusivo `/tmp/p1321-target.669PuL`, cópia sem hardlinks do cache
baseline. /dev/shm não comportava o cache completo; nenhum temporário anterior
foi apagado. Versão exibida não identifica o executável por si só.

## Contrato, testes e autoridades

Seção P1321 no L0 proprietário loading, com substituição explícita das antigas
preservações de missing/unknown/excesso e precedência. O consumer R1 é
`01_core/src/compiler/stdlib/loading.rs`; R2 acrescenta o owner de dispatch
somente para transportar a chamada inteira. A revisão julgou correção interna de
paridade em fluxo contínuo ADR-0127: nenhuma API Rust, trait, flag ou fase nova.

Delta de testes escrito antes do candidato:

- novos testes de missing/named source/hint/origens, remanescente causal antes
  de leitura/parsing e precedência source/opções; fallback sintético incluído;
- P1319 mantém todas as asserções de parsing/caminho/origem sem o excedente,
  e adiciona separadamente rejeição do excedente sem leitura;
- P1316 com excesso passa a exigir `unexpected argument` e span completo do
  excedente; os outros controles de parsing P1316 permanecem;
- P1313 atualiza missing e a precedência cast sobre unknown.

Regime escolhido pela skill tekt-materializacao-segregada: A/B sem atestação
técnica de isolamento, não selo de refinamento/mutation score. Root escreve
L0/testes locais/código; `/root/p1321_ab_clean` produz os oráculos sem ler
produto; `/root/p1321_review` julga sem editar produto ou oráculos.

Incidente anterior ao freeze: o primeiro testador viu incidentalmente diff
P1319 dentro de um JSON diagnóstico. Esse papel foi encerrado sem freeze nem
execução dos binários. Seus arquivos `p1321-ab-*` são exploração descartada,
não evidência de aceitação. O A/B foi reiniciado com contexto novo e proibição
dos JSONs antigos. O novo testador também registrou criação incidental de seu
próprio pycache fora do prefixo autorizado, removido por ele; isso reforça que
não se atesta isolamento técnico do ambiente compartilhado.

Somente `p1321-ab2-*` fornece o freeze de aceitação: SHA
`b90c53cc2d982160dce2faca2f4afb6910d07b06e321b8bf8090bc51ead0d6ca`.
Norma congelada, excluindo só a linha canônica Hash do Código:
`2f6dcab26aefce4a7db207bd347b46998dd845e7205954846be4512b5c2f2cc3`.
O recibo A/B detalha as referências vanilla integrais, preservações baseline
e dois casos Symbol com delta normativo explícito. O sink tem controle próprio:
não se presume que a ordem escrita externamente seja a entregue ao consumer.

## O que o primeiro candidato resolveu — e onde falhou

O validador agora rejeita excedentes e nomes desconhecidos na ordem causal,
antes de I/O ou parsing, e respeita a precedência de source/delimiter/row-type.
Missing e named source passaram a ter mensagem e hint contratados. O teste
RED compilou e falhou nos seis casos esperados (66 passaram); o GREEN local
passou nos 72 testes. Recibos `p1321-unit-red.json`, SHA
`d750796583662024a5aac54f07bf601c87c9ea50dec145ae0b0282d7c758e8d8`,
e `p1321-unit-green.json`, SHA
`3368320a9f4de5e3948f142e79002b594947510c9cad97469b24612e5ee1d515`,
preservam estados exatos, stdout/stderr, comandos e UTC. Os testes não mudaram
entre RED e GREEN.

Build release, fmt, linhagem explícita e lint passaram; lint sem erros, com
240 warnings e 1138 infos existentes no estado medido (não zero diagnósticos).
Workspace R1: 6664 passaram, 0 falharam, 3 ignorados; recibo
`p1321-workspace-tests.json`, SHA
`16e7774da35799a7bc8c6fe81bc03897c43e96eaaf4efd6b2c8524a37bf5069c`,
UTC 2026-09-08T20:32:15.718902+00:00 a 20:36:55.342470+00:00.
Esses verdes não substituem o A/B.

O A/B independente congelou 110 casos em quatro perfis e executou três
ordens: 1224/1320 comparações passaram, 96 falharam, zero Unknown. São oito
fontes missing × quatro perfis × três ordens: mensagem correta, mas âncora
na lista de argumentos em vez da chamada inteira. Os outros 102 casos
passaram em todas as execuções, incluindo controles normativos Symbol.
Recibo bruto `p1321-ab2-candidate.json`, SHA
`d1699b5ae42327c7ba7254f5f1f2830453d0cd225fce8f79807cf329aa2a1a25`.
Portanto R1 NÃO fecha o passo. O binário medido foi preservado em
`/tmp/p1321-r1-preserved.Tdbgy8/typst`, SHA
`deb6aed85d133665b00d416fabee92aeda04196ff76b6a734ef0451bbdeaea3c`.

## Por que R2 precisa tocar o transporte da chamada

`eval_args` recebe o span da lista. O validador CSV não possui o início da
chamada para recuperar a âncora completa. O helper existente já transporta
essa informação para encoders e panic por identidade nativa; CSV não estava
no conjunto. A medição refutou a hipótese de suficiência de loading sozinho.

A reabertura acrescenta apenas native_csv ao mesmo mapeamento, legitimado
pelo L0 próprio de call_dispatch; não transfere validação para o dispatcher.
With e aliases conservam a identidade. Funções alheias chamadas csv não
recebem tratamento especial. Sem AST, Args.span recebido é preservado.
Os dois contratos foram atualizados antes do código e a revisão confirmou
fluxo contínuo ADR-0127. Não se altera nenhum resultado esperado R1 para
esconder a falha. Baseline R2 com ambos os owners e estado não commitado:
`p1321-r2-baseline.json`, SHA
`8ca670bb4ab656a29667abd67fc06bc4d227a3dcc0afa9330a5e9ad49537f065`.

## R2 — implementação e evidência final

O delta produtivo R2 é somente import/comparação de native_csv no helper
existente. A validação loading não mudou desde R1; testes dos dois owners
permanecem iguais aos seus respectivos REDs. A/B mantém as 440 expectativas
originais e acrescenta 76 controles (19 casos × quatro perfis), congelados
antes do patch. Freeze sucessor SHA
`67c6b92d62018b1cc8673fe28e75d8c55fd28f1c4e15d15ab6a4bb535ecfc8dd`.
O novo RED compilou: controle das outras identidades passou, transporte CSV
falhou; recibo `p1321-r2-unit-red.json`, SHA
`13b0dc933bc5778fdd9c4d93c7eb8327296549b3b0169fec1483ed63e43723d5`.
O parecer `p1321-review-prepatch-r2.md` precede a mudança produtiva.
GREEN R2: os dois testes passaram, sem alterar as asserções do RED;
recibo `p1321-r2-unit-green.json`, SHA
`095880f88dc4878d7b32d8ae4d1ba70ca0842a3e8c04834ad404f1485fddd2d3`.

Build R2: `cargo build --workspace --release`, exit 0, UTC
2026-09-08T20:45:21.947375+00:00 a 20:46:01.025140+00:00.
Recibo `p1321-r2-build.json`, SHA
`3cd594329afb5f93ccd461b8678e67c37d29625b15abcae8beb47f8e8ff1cad7`.
Binário `/tmp/p1321-target.669PuL/release/typst`, SHA
`756b1c85a5879ea0afb79fc35184525aebfafa6ccb926ae86679a868691f99fa`.
O estado medido é HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093 com
árvore não commitada. Os recibos R2 contêm diff/stat, UTC e texto integral
dos quatro arquivos produtivos/normativos; nenhum desses arquivos mudou
durante o build. Fontes finais: loading
`f4ebb73fac0d64d412adbda475eaef65fb45e01ac2cf44a13c5b9d9aa408bf8f`,
call_dispatch
`9b11c388cf50666beecad9a6c92fbebfb323cdf5edbb550f4be04a96e0c3e9d4`.

Linhagem recalculada explicitamente nos dois sentidos, além de V5/V15/V26:

| Owner | A efetivo | B reverso |
| --- | --- | --- |
| loading | b8c5243a | c0f17d3b |
| call_dispatch | c80d41d0 | 01a0d090 |

Recibo `p1321-r2-lineage-final.json`, SHA
`9cc28178e87a76d14c1251dfaf4a7df1e4f63c9d9ea2fa0e1160a4b02e699cf5`,
inclui hashes completos e compara norma/testes byte a byte com o RED R2.
Fmt e diff-check passaram. `crystalline-lint .` terminou exit 0, com
0 erros, 240 warnings e 1138 infos, sem aumento frente ao estado R1;
recibo `p1321-r2-lint.json`, SHA
`cbf868c9f5499ba4b1a9691bc19e4539f7a467ff7a133142982e3a00233bd3c8`.

Uma sonda extra `$csv()$` não é comparação bilateral do validador: vanilla
não resolve csv diretamente nesse namespace math, enquanto o cristalino
R1 resolve. A diferença prévia permanece fora do recorte e está integralmente
registrada em `p1321-ab2-r2-measurement.json`; não foi apagada nem contada
como sucesso. Os dois controles math por alias/With efetivamente chegam a
CSV e fazem parte da aceitação. Não se alega paridade math geral.

Focal R2: 140/140 comparações passaram (35 casos × quatro perfis), sem
Unknown ou divergência. Inclui os oito missing que falharam em R1 e as
fronteiras congeladas. Recibo `p1321-ab2-r2-focal.json`, SHA
`1b9bf87a405c47dccd0ab603bc8de51830bfc25f7bea718230a19055aecf8f94`.
Esse ganho é correção de origem medida, não atualização de expected.

A/B integral R2: 1548/1548 comparações passaram (129 casos × quatro perfis
× normal/repeat/reverse), sem falhas ou Unknown. Os resultados integrais
incluem mensagens, hints, traces, spans, stdout/stderr e exit; não foram
normalizados para esconder diferenças. Recibo `p1321-ab2-r2-full.json`, SHA
`d384d510f380b6cc67bada50a7cb6270b5a68cc01611891db728bfdea06e93c7`.
O revisor recalculou as comparações contra o freeze e confirmou o resultado,
conservando as falhas R1 como histórico: `p1321-review-r2-ab-audit.json`.

Regressão completa R2: `cargo test --workspace --release --no-fail-fast`,
exit 0, 6666 testes passaram, nenhum falhou e três doctests preexistentes
permaneceram ignorados. Recibo `p1321-r2-workspace-tests.json`, SHA
`afbc5559ef9efad6a8dcb54dcbbd6686b952eafdb0ba5a3aa6fec460379754df`,
UTC 2026-09-08T20:48:02.903704+00:00 a 20:52:38.930214+00:00,
com os quatro arquivos fonte/L0 idênticos antes e depois.

O recibo A/B final `p1321-ab2-r2-receipt.md`, SHA
`8f63928a4c3a251213967548551f2af0077327a0657e1b64daa7c9033765d717`,
consolida entradas, custo, limites e reprodução. Os runners A/B gravam nomes
fixos: reproduzir em uma cópia do repositório para não sobrescrever evidência.

## O que este passo fecha e o que ainda falta

Fecha o cluster de validação de argumentos CSV: ausência/named source,
precedência dos casts/opções, primeiro excedente/desconhecido, rejeição antes
da leitura/parser e âncora da chamada ausente nas rotas testadas. A falha R1
foi corrigida por transporte causal, sem alterar os oráculos para obter verde.

Não fecha coerções Symbol, resolução da origem de caminhos string/imports,
origem externa no diagnóstico CSV de texto UTF-8 válido nem o envelope de
I/O legado. `array(Bytes)` e a diferença de namespace math direto são frentes
separadas. O próximo passo deve escolher e medir um desses recortes antes
de propor L0; estes verdes não demonstram paridade geral do compilador.

Parecer final independente PASS, sem achados pendentes:
`p1321-review-final-r2.md`, SHA
`ac5d5c32c8e65d49dbf10805376567c41541f16224ca914f103a090a2bf0d556`.
A auditoria `p1321-review-r2-final-audit.json`, SHA
`efd070554e370f64c8dd6cc93da37f2f93016aa2e27a1c2041e46b9c5fd34938`,
verifica cada comparação, hashes e estados dos gates, além de seus resumos.
Regime A/B com revisão separada, sem atestação técnica de isolamento.

Sem commit ou push;
alterações e evidências anteriores P1319/P1320 preservadas. Nenhum temporário
foi apagado neste passo.
