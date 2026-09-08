# P1318 — recibo A/B independente

PASS no fragmento público congelado: 6.156 comparações, zero falhas e zero
Unknown, em normal/repeat/reverse. Executado sem atestação de isolamento técnico.
Não constitui selo de refinamento, mutation score ou paridade geral CSV.

## Autoridade e entradas

Testador `/root/p1318_tests`, contexto próprio limitado ao L0 loading inteiro,
skill `tekt-materializacao-segregada` e suas duas referências, ADRs aplicáveis,
fonte vanilla loading/csv.rs, diag.rs, syntax lines.rs/lexer.rs, artefatos
p1317-ab e suas fixtures históricas, binários e git HEAD/status/diff STAT.
Owner loading.rs, testes locais, diff produtivo e recibos com fonte embutida
não foram lidos. Escritas limitadas a p1318-ab-* em diagnósticos e fixtures
`/tmp/p1318-ab-fixtures`. O filesystem é compartilhado; as capacidades declaradas
não são um mecanismo de isolamento atestado.

L0 normativo SHA-256
`2b23a5adf4c7b62afbfb5ee9b8899bb756f36c2eb158c2ddb7e2795e49312ee6`,
excluindo somente a linha canônica `Hash do Código`. Freeze registrado em
2026-09-08T15:32:37.413021+00:00, antes da liberação/aplicação do candidato,
conforme sequência de mensagens das autoridades. O teste compara a mensagem
diagnóstica como observável de língua, conforme ADR-0107/0108 e L0 P1318.

| Entrada | SHA-256 |
|---|---|
| Baseline `/tmp/p1317-target.y5u9ah/release/typst` | `9fcb4cbe830c74ec589506b0b86f52982abfdaab558f62cbf7a8f7de8665ccab` |
| Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| Candidato `/tmp/p1318-target.eAgQwp/release/typst` | `0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c` |
| Runner final p1318-ab-runner.py | `6a84d37f2860b19688a4250b113217508158844f96b7f3ef807dbf85682f5f65` |
| Casos p1318-ab-cases.json | `2d0b10eeb955946baa4a943a67f101ebf8d5a5dcd7f2fc4b0ff90a7cb4cb74b3` |
| Freeze p1318-ab-freeze.json | `aede13464afe720b4dffe501a9cab59f22f92a6101c12e9be7878c083abcb334` |
| Baseline validado p1318-ab-baseline-runs-r1.json | `06625b05fc79e023e26079b7871a399afddf3c15b48b2daac52e2528ff3f2be1` |
| Candidato p1318-ab-candidate-runs.json | `c5907504691f3d2f524804624b3db343058e2db93d4004422332e8ecba168ef0` |
| Comparação p1318-ab-comparison.json | `9ed11dfe247403eb6fd58942c4a774693e6fdc105b4ba425b2051667b686f1a1` |

## Proveniência e abrangência

Todas as contagens abaixo referem-se ao HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado.
Os recibos JSON guardam timestamps por processo, argv, cwd, exit/stdout/stderr
integrais e estado anterior/posterior. Baseline começou em
2026-09-08T15:29:11.859156+00:00, com diff/stat:

```text
00_nucleo/prompts/compiler/stdlib/loading.md | 261 ++++++++++++++-
01_core/src/compiler/stdlib/loading.rs       | 484 ++++++++++++++++++++++++++-
2 files changed, 728 insertions(+), 17 deletions(-)
```

Candidato medido de 2026-09-08T15:42:08.395881+00:00 até
2026-09-08T15:46:09.393724+00:00, com diff/stat:

```text
00_nucleo/prompts/compiler/stdlib/loading.md | 261 ++++++++++++-
01_core/src/compiler/stdlib/loading.rs       | 551 ++++++++++++++++++++++++++-
2 files changed, 789 insertions(+), 23 deletions(-)
```

São 513 casos: os 387 históricos P1317 e 126 novos, nos perfis default, html,
a11y e html+a11y. Antes de aplicar o delta, os 1.548 replays históricos foram
comparados integralmente à execução candidata P1317 anterior, incluindo fonte,
cwd e observação pública. Não houve deriva histórica.

O freeze contém 2.052 expectativas: 788 RED de sufixo e 1.264 preservações.
Dos casos de sufixo, 756 observações exigem também diagnóstico vanilla integral;
32 são colisões de parsing com excesso, explicitamente normativas. Nessas
colisões o vanilla original dá excesso; uma segunda sonda vanilla sem excesso
fornece apenas a posição, com argv e saída integral registrados separadamente.
Nenhuma diferença de diagnóstico é removida para alegar paridade.

Cada expectativa de mudança conserva exit/stdout e o stderr completo do baseline,
acrescentando somente ` at L:C` antes do parêntese final da causa. L/C medidos
foram congelados antes do candidato. Cobertura inclui LF, CRLF entre CR/LF, CR,
VT/FF/NEL/LS/PS, campos multilinha, BOM, Unicode de quatro bytes, Utf8 em
cabeçalho/dados e UnequalLengths anterior a um byte inválido posterior; modos
array/dictionary, With/Args/spread/sink/detached e preservações Path/Str reais,
valores, opções, casts, outros loaders e encoders.

Testemunhas congeladas: `a,b\r\n1` e cabeçalho Unicode equivalente dão 1:5;
LS no cabeçalho antes de CRLF dá 2:5; com byte inválido somente depois do erro
anterior, passa a 1:1 e conserva UnequalLengths. Assim, ordinal, linha do parser,
contagem exclusiva de LF para texto e validação antecipada não substituem a
obrigação observada.

## Incidente do writer e custo

Uma execução baseline/vanilla: 2.872 processos, 61,724 s registrados no JSON.
O writer r0 usava `splitlines()` na construção do patch e converteu U+2028 em
quebra literal em oito saídas JSON de dois controles válidos. Esse erro de
serialização não é RED do produto. O bruto `p1318-ab-baseline-runs.json` permanece
imutável, SHA-256 `b5f02700f103a87a8f65bbf23654c18dd794339868929edd04b42bd59bcae059`.
O runner r0 também foi preservado. A correção usa `split('\n')`.

`p1318-ab-writer-repair.py` produziu o artefato r1 sem repetir o corpus: todos os
oito registros afetados foram validados por nova execução focal de exit/stdout/
stderr, argv e cwd completos, 8 processos/0,325 s. Há registro causal em
`writer_revision` do baseline r1, além de p1318-ab-writer-focal.json. A validação
dos JSONs e o replay histórico passaram antes do freeze. Zero expectativas
foram alteradas; houve uma correção do protocolo de gravação, com ganho
observável de oito registros antes ilegíveis para oito verificados.

Candidato: uma execução com as três ordens, 6.156 processos/240,932 s.
Total das execuções de produto deste A/B: 9.036 processos e cerca de 302,981 s
de duração registrada dos lotes; não é benchmark de performance. Nenhuma revisão
de caso/oráculo depois do freeze, nem segunda execução integral após PASS.
O gate final revalidou todos os hashes congelados e o L0 normativo. Unknown
bloqueia em identidade ambígua, entrada ausente/duplicada, timeout, crash,
construção não suportada ou deriva; não foi convertido em sucesso.

## Reprodução sem sobrescrever evidência

Na raiz `/repos/Antigravity/typst-crystalline`, com os binários e fixtures do host
preservados, executar com acesso ao `/tmp` do host. Os nomes `reproduction-01`
abaixo devem estar livres; escolher novo sufixo em outra reprodução. Não executar
prepare/freeze novamente sobre os artefatos canônicos.

```bash
python3 00_nucleo/diagnosticos/p1318-ab-runner.py run \
  --binary /tmp/p1318-target.eAgQwp/release/typst \
  --binary-sha256 0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c \
  --orders normal,repeat,reverse \
  --output 00_nucleo/diagnosticos/p1318-ab-reproduction-01-runs.json
python3 00_nucleo/diagnosticos/p1318-ab-runner.py compare \
  --freeze 00_nucleo/diagnosticos/p1318-ab-freeze.json \
  --measurement 00_nucleo/diagnosticos/p1318-ab-reproduction-01-runs.json \
  --output 00_nucleo/diagnosticos/p1318-ab-reproduction-01-comparison.json
```

## Limites

A CLI não prova assinatura/semântica do decode_csv público direto, Args Rust
sintético sem occurrences, fallback sem posição/overflow, número de chamadas
World ou parser único. Essas obrigações pertencem à validação local e auditoria
de outra autoridade. Este testador não inspecionou suas provas nem emite veredito
arquitetural. PASS limita-se às versões, casos, perfis e observáveis identificados.
